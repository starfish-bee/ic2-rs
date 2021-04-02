mod func;
mod messages;

pub use func::Functionality;
use libc::{c_int, c_ulong, ioctl};
use messages::{I2cMessageBuffer, I2cReadWriteData};
pub use messages::{
    I2C_M_IGNORE_NACK, I2C_M_NOSTART, I2C_M_NO_RD_ACK, I2C_M_RD, I2C_M_RECV_LEN,
    I2C_M_REV_DIR_ADDR, I2C_M_TEN,
};
use std::convert::TryFrom;
use std::os::unix::io::AsRawFd;

// supported ioctl commands
const I2C_FUNCS: c_ulong = 0x0705;
const I2C_RDWR: c_ulong = 0x0707;

#[derive(Debug)]
pub struct I2c {
    file: std::fs::File,
    addr: u16,
    func: Functionality,
}

impl I2c {
    pub fn open(addr: u16) -> Result<Self, I2cError> {
        let path = "/dev/i2c-1";
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;

        let func = Self::get_func(file.as_raw_fd())?;
        // address is too long for supported address range
        if (!func._10_bit_addr() & (addr > 0b0111_1111))
            | (func._10_bit_addr() & (addr > 0b0011_1111_1111))
        {
            return Err(I2cError::AddressError);
        };

        Ok(Self { file, addr, func })
    }

    pub fn functionality(&self) -> &Functionality {
        &self.func
    }

    pub fn i2c_read(&self, register: u8, bytes: usize) -> Result<Vec<u8>, I2cError> {
        let mut buffer = vec![0; bytes];
        let mut messages = I2cMessageBuffer::new();
        messages.add_read_reg(self.addr, 0, &register, &mut buffer[..]);
        let data = I2cReadWriteData::from_messages(&messages);
        i2c_rdwr_ioctl(&self, &data)?;
        Ok(buffer)
    }

    pub fn i2c_write(&self, register: u8, buffer: &[u8]) -> Result<(), I2cError> {
        // need to create a new buffer as first byte of buffer passed must be the register
        let mut new_buffer = Vec::with_capacity(buffer.len() + 1);
        new_buffer.push(register);
        new_buffer.extend_from_slice(buffer);

        let mut messages = I2cMessageBuffer::new();
        messages.add_write(self.addr, 0, buffer);
        let data = I2cReadWriteData::from_messages(&messages);
        i2c_rdwr_ioctl(&self, &data)
    }

    pub fn i2c_buffer(&self) -> I2cBuffer {
        I2cBuffer {
            buffer: I2cMessageBuffer::new(),
            handle: self,
        }
    }

    fn get_func(descriptor: c_int) -> Result<Functionality, I2cError> {
        let mut func = 0;
        get_err(unsafe { ioctl(descriptor, I2C_FUNCS, &mut func) })?;

        Ok(Functionality(func))
    }

    fn require_func(&self, func: c_ulong) -> Result<(), Functionality> {
        let mask = !self.functionality().0 & func;
        match mask {
            0 => Ok(()),
            mask => Err(Functionality(mask)),
        }
    }
}

#[derive(Debug)]
pub struct I2cBuffer<'a> {
    buffer: I2cMessageBuffer<'a>,
    handle: &'a I2c,
}

impl<'a> I2cBuffer<'a> {
    pub fn add_read(&mut self, flags: u16, buffer: &'a mut [u8]) {
        self.buffer.add_read(self.handle.addr, flags, buffer)
    }

    pub fn add_write(&mut self, flags: u16, buffer: &'a [u8]) {
        self.buffer.add_write(self.handle.addr, flags, buffer)
    }

    pub fn add_raw(&mut self, flags: u16, buffer: &'a mut [u8]) {
        let len = u16::try_from(buffer.len()).unwrap();
        let buffer = buffer.as_mut_ptr();
        self.buffer.add_raw(self.handle.addr, flags, len, buffer)
    }

    pub fn execute(self) -> Result<(), I2cError> {
        let data = I2cReadWriteData::from_messages(&self.buffer);
        i2c_rdwr_ioctl(&self.handle, &data)
    }
}

#[derive(Debug)]
pub enum I2cError {
    IoctlError(std::io::Error),
    AddressError,
    MissingFunctionalityError(Functionality),
}

impl std::convert::From<std::io::Error> for I2cError {
    fn from(arg: std::io::Error) -> Self {
        Self::IoctlError(arg)
    }
}

impl std::convert::From<Functionality> for I2cError {
    fn from(arg: Functionality) -> Self {
        Self::MissingFunctionalityError(arg)
    }
}

fn i2c_rdwr_ioctl(handle: &I2c, data: &I2cReadWriteData) -> Result<(), I2cError> {
    handle.require_func(func::I2C_FUNC_I2C)?;

    // SAFETY:
    // file descriptor guaranteed to point to valid open file
    // data guaranteed to outlast function call
    // parameters correctly passed as described in i2c.h and i2c-dev.h
    // hope ioctl implementation doesn't mess things up
    get_err(unsafe { ioctl(handle.file.as_raw_fd(), I2C_RDWR, data) })?;
    Ok(())
}

// wraps ioctl calls to map its return into a Result
fn get_err(code: c_int) -> Result<c_int, std::io::Error> {
    match code {
        x if x >= 0 => Ok(x),
        _ => Err(std::io::Error::last_os_error()),
    }
}

#[test]
fn test_require_funcs() {
    let mut handle = I2c::open(0x76).unwrap();
    handle.func = Functionality::new(0b10110);
    let result = handle.require_func(0b00100);
    assert_eq!(result, Ok(()));
    let result = handle.require_func(0b11001);
    assert_eq!(result, Err(Functionality::new(0b01001)));
}

// these tests require that a BME680 chip is connected to the I2C bus
// checks the BME680 chip ID register is 0x61
#[test]
fn test_i2c_read() {
    let handle = I2c::open(0x76).unwrap();
    let id = handle.i2c_read(0xD0, 1);
    println!("{:x?}", id);
    assert_eq!(id.unwrap(), vec![0x61]);
}

// checks the BME680 chip ID register is 0x61
#[test]
fn test_buffer_read() {
    let handle = I2c::open(0x76).unwrap();
    let mut data = vec![0xD0, 0];
    let (register, id) = data.split_at_mut(1);

    let mut buffer = handle.i2c_buffer();
    buffer.add_write(0, register);
    buffer.add_read(0, id);
    buffer.execute().unwrap();

    println!("{:x?}", id);
    assert_eq!(id, vec![0x61]);
}
