mod func;
mod messages;

pub use func::Functionality;
use libc::{c_int, c_ulong, ioctl};
use messages::{I2cMessage, I2cReadWriteData};
use std::os::unix::io::AsRawFd;

// supported ioctl commands
const I2C_FUNCS: c_ulong = 0x0705;
const I2C_RDWR: c_ulong = 0x0707;

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

    pub fn i2c_read(&self, register: u8, buffer: &mut [u8]) -> Result<(), I2cError> {
        let messages = I2cMessage::read_reg(self.addr, register, buffer);
        let data = I2cReadWriteData::from_messages(&messages[..]);
        // SAFETY:
        // file descriptor guaranteed to point to valid open file
        // data guaranteed to outlast function call
        // parameters correctly passed as described in i2c.h and i2c-dev.h
        // hope ioctl implementation doesn't mess things up
        get_err(unsafe { ioctl(self.file.as_raw_fd(), I2C_RDWR, &data) })?;
        Ok(())
    }

    pub fn i2c_write(&self, register: u8, buffer: &[u8]) -> Result<(), I2cError> {
        let messages = I2cMessage::write_reg(self.addr, register, buffer);
        let data = I2cReadWriteData::from_messages(&messages[..]);
        // SAFETY:
        // file descriptor guaranteed to point to valid open file
        // data guaranteed to outlast function call
        // parameters correctly passed as described in i2c.h and i2c-dev.h
        // hope ioctl implementation doesn't mess things up
        get_err(unsafe { ioctl(self.file.as_raw_fd(), I2C_RDWR, &data) })?;
        Ok(())
    }

    fn get_func(descriptor: c_int) -> Result<Functionality, I2cError> {
        let mut func = 0;
        get_err(unsafe { ioctl(descriptor, I2C_FUNCS, &mut func) })?;

        Ok(Functionality(func))
    }
}

#[derive(Debug)]
pub enum I2cError {
    IoctlError(std::io::Error),
    AddressError,
}

impl std::convert::From<std::io::Error> for I2cError {
    fn from(arg: std::io::Error) -> Self {
        Self::IoctlError(arg)
    }
}

// wraps ioctl calls to map it's return into a Result
fn get_err(code: c_int) -> Result<c_int, std::io::Error> {
    match code {
        x if x >= 0 => Ok(x),
        _ => Err(std::io::Error::last_os_error()),
    }
}