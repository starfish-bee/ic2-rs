use std::convert::TryFrom;

// I2C message flags
const I2C_M_RD: u16 = 0x0001;
const I2C_M_TEN: u16 = 0x0010;
const I2C_M_RECV_LEN: u16 = 0x0400;
const I2C_M_NO_RD_ACK: u16 = 0x0800;
const I2C_M_IGNORE_NACK: u16 = 0x1000;
const I2C_M_REV_DIR_ADDR: u16 = 0x2000;
const I2C_M_NOSTART: u16 = 0x4000;

// i2c_rdwr_ioctl_data struct, as defined in i2c-dev.h
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct I2cReadWriteData<'a> {
    messages: *const I2cMessage,
    num: u32,
    // ensure that I2cReadWriteData does not outlive the messages
    // it points to
    phantom: std::marker::PhantomData<&'a I2cMessage>,
}

// TODO handle buffer overflow case
impl<'a> I2cReadWriteData<'a> {
    pub fn from_messages(buffer: &'a [I2cMessage]) -> Self {
        let messages = buffer.as_ptr();
        let num = u32::try_from(buffer.len()).unwrap();
        Self {
            messages,
            num,
            phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct I2cMessageBuffer {
    buffer: Vec<I2cMessage>,
}

impl I2cMessageBuffer {
    pub fn new() -> Self {
        let buffer = Vec::new();
        Self { buffer }
    }

    pub fn add_read(&mut self, addr: u16, flags: u16, buffer: &mut [u8]) {
        let flags = flags | I2C_M_RD;
        let len = u16::try_from(buffer.len()).unwrap();
        let buffer = buffer.as_mut_ptr();
        self.add_raw(addr, flags, len, buffer)
    }

    pub fn add_write(&mut self, addr: u16, flags: u16, buffer: &[u8]) {
        let flags = flags & !I2C_M_RD;
        let len = u16::try_from(buffer.len()).unwrap();
        // function guarantees I2C read flag never set, so buffer will never be written to
        let buffer = buffer.as_ptr() as *mut u8;
        self.add_raw(addr, flags, len, buffer)
    }

    fn add_raw(&mut self, addr: u16, flags: u16, len: u16, buffer: *mut u8) {
        self.buffer.push(I2cMessage {
            addr,
            flags,
            len,
            buffer,
        })
    }
}

// i2c_message struct as defined in i2c.h
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct I2cMessage {
    addr: u16,
    flags: u16,
    len: u16,
    buffer: *mut u8,
}

impl I2cMessage {
    pub fn read_reg(addr: u16, reg: u8, buffer: &mut [u8]) -> Vec<Self> {
        let mut reg = reg;
        let specify_reg = Self {
            addr,
            flags: 0,
            len: 1,
            buffer: &mut reg as *mut u8,
        };
        let read_reg = Self {
            addr,
            flags: 1,
            len: u16::try_from(buffer.len()).unwrap(),
            buffer: buffer.as_mut_ptr(),
        };

        vec![specify_reg, read_reg]
    }

    pub fn write_reg(addr: u16, reg: u8, buffer: &[u8]) -> Vec<Self> {
        let mut new_buff = Vec::with_capacity(buffer.len() + 1);
        new_buff.push(reg);
        new_buff.extend_from_slice(buffer);
        let write_reg = Self {
            addr,
            flags: I2C_M_RD,
            len: u16::try_from(new_buff.len()).unwrap(),
            buffer: new_buff.as_mut_ptr(),
        };

        vec![write_reg]
    }
}
