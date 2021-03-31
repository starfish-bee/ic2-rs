use std::convert::TryFrom;

// I2C message flags
pub const I2C_M_RD: u16 = 0x0001;
pub const I2C_M_TEN: u16 = 0x0010;
pub const I2C_M_RECV_LEN: u16 = 0x0400;
pub const I2C_M_NO_RD_ACK: u16 = 0x0800;
pub const I2C_M_IGNORE_NACK: u16 = 0x1000;
pub const I2C_M_REV_DIR_ADDR: u16 = 0x2000;
pub const I2C_M_NOSTART: u16 = 0x4000;

// i2c_rdwr_ioctl_data struct, as defined in i2c-dev.h
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct I2cReadWriteData<'a> {
    messages: *const I2cMessage,
    num: u32,
    // ensure that I2cReadWriteData does not outlive the messages
    // it points to
    _phantom: std::marker::PhantomData<&'a I2cMessage>,
}

// TODO handle buffer overflow case
impl<'a> I2cReadWriteData<'a> {
    pub fn from_messages(buffer: &'a I2cMessageBuffer) -> Self {
        let messages = buffer.buffer.as_ptr();
        let num = u32::try_from(buffer.buffer.len()).unwrap();
        Self {
            messages,
            num,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct I2cMessageBuffer<'a> {
    buffer: Vec<I2cMessage>,
    _phantom: std::marker::PhantomData<&'a I2cMessage>,
}

impl<'a> I2cMessageBuffer<'a> {
    pub fn new() -> Self {
        let buffer = Vec::new();
        Self {
            buffer,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn add_read(&mut self, addr: u16, flags: u16, buffer: &'a mut [u8]) {
        let flags = flags | I2C_M_RD;
        let len = u16::try_from(buffer.len()).unwrap();
        let buffer = buffer.as_mut_ptr();
        self.add_raw(addr, flags, len, buffer)
    }

    pub fn add_write(&mut self, addr: u16, flags: u16, buffer: &'a [u8]) {
        let flags = flags & !I2C_M_RD;
        let len = u16::try_from(buffer.len()).unwrap();
        // function guarantees I2C read flag never set, so buffer will never be written to
        let buffer = buffer.as_ptr() as *mut u8;
        self.add_raw(addr, flags, len, buffer)
    }

    pub fn add_read_reg(&mut self, addr: u16, flags: u16, register: &'a u8, buffer: &'a mut [u8]) {
        let flags = flags & !I2C_M_RD;
        let len = 1;
        let register = register as *const u8 as *mut u8;
        self.add_raw(addr, flags, len, register);
        self.add_read(addr, flags, buffer)
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
