use libc::c_ulong;

// functionality parameters
pub const I2C_FUNC_I2C: c_ulong = 0x00000001;
pub const I2C_FUNC_10BIT_ADDR: c_ulong = 0x00000002;
pub const I2C_FUNC_PROTOCOL_MANGLING: c_ulong = 0x00000004;
pub const I2C_FUNC_SMBUS_PEC: c_ulong = 0x00000008;
pub const I2C_FUNC_SMBUS_BLOCK_PROC_CALL: c_ulong = 0x00008000;
pub const I2C_FUNC_SMBUS_QUICK: c_ulong = 0x00010000;
pub const I2C_FUNC_SMBUS_READ_BYTE: c_ulong = 0x00020000;
pub const I2C_FUNC_SMBUS_WRITE_BYTE: c_ulong = 0x00040000;
pub const I2C_FUNC_SMBUS_READ_BYTE_DATA: c_ulong = 0x00080000;
pub const I2C_FUNC_SMBUS_WRITE_BYTE_DATA: c_ulong = 0x00100000;
pub const I2C_FUNC_SMBUS_READ_WORD_DATA: c_ulong = 0x00200000;
pub const I2C_FUNC_SMBUS_WRITE_WORD_DATA: c_ulong = 0x00400000;
pub const I2C_FUNC_SMBUS_PROC_CALL: c_ulong = 0x00800000;
pub const I2C_FUNC_SMBUS_READ_BLOCK_DATA: c_ulong = 0x01000000;
pub const I2C_FUNC_SMBUS_WRITE_BLOCK_DATA: c_ulong = 0x02000000;
pub const I2C_FUNC_SMBUS_READ_BLOCK: c_ulong = 0x04000000;
pub const I2C_FUNC_SMBUS_WRITE_BLOCK: c_ulong = 0x08000000;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Functionality(pub c_ulong);

impl Functionality {
    pub fn new(func: c_ulong) -> Self {
        Functionality(func)
    }

    pub fn i2c(&self) -> bool {
        (self.0 & I2C_FUNC_I2C) > 0
    }

    pub fn _10_bit_addr(&self) -> bool {
        (self.0 & I2C_FUNC_10BIT_ADDR) > 0
    }

    pub fn protocol_mangling(&self) -> bool {
        (self.0 & I2C_FUNC_PROTOCOL_MANGLING) > 0
    }

    pub fn smbus_pec(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_PEC) > 0
    }

    pub fn smbus_block_proc_call(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_BLOCK_PROC_CALL) > 0
    }

    pub fn smbus_quick(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_QUICK) > 0
    }

    pub fn smbus_read_byte(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_READ_BYTE) > 0
    }

    pub fn smbus_write_byte(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_WRITE_BYTE) > 0
    }

    pub fn smbus_read_byte_data(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_READ_BYTE_DATA) > 0
    }

    pub fn smbus_write_byte_data(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_WRITE_BYTE_DATA) > 0
    }

    pub fn smbus_read_word_data(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_READ_WORD_DATA) > 0
    }

    pub fn smbus_write_word_data(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_WRITE_WORD_DATA) > 0
    }

    pub fn smbus_proc_call(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_PROC_CALL) > 0
    }

    pub fn smbus_read_block_data(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_READ_BLOCK_DATA) > 0
    }

    pub fn smbus_write_block_data(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_WRITE_BLOCK_DATA) > 0
    }

    pub fn smbus_read_block(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_READ_BLOCK) > 0
    }

    pub fn smbus_write_block(&self) -> bool {
        (self.0 & I2C_FUNC_SMBUS_WRITE_BLOCK) > 0
    }
}

impl std::fmt::Display for Functionality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|")?;
        if self.i2c() {
            write!(f, " I2C |")?;
        };
        if self._10_bit_addr() {
            write!(f, " 10 BIT ADDR |")?;
        };
        if self.protocol_mangling() {
            write!(f, " PROTOCOL MANGLING |")?;
        };
        if self.smbus_pec() {
            write!(f, " SMBUS PEC |")?;
        };
        if self.smbus_block_proc_call() {
            write!(f, " SMBUS BLOCK PROC CALL |")?;
        };
        if self.smbus_quick() {
            write!(f, " SMBUS QUICK |")?;
        };
        if self.smbus_read_byte() {
            write!(f, " SMBUS READ BYTE |")?;
        };
        if self.smbus_write_byte() {
            write!(f, " SMBUS WRITE BYTE |")?;
        };
        if self.smbus_read_byte_data() {
            write!(f, " SMBUS READ BYTE DATA |")?;
        };
        if self.smbus_write_byte_data() {
            write!(f, " SMBUS WRITE BYTE DATA |")?;
        };
        if self.smbus_read_word_data() {
            write!(f, " SMBUS READ WORD DATA |")?;
        };
        if self.smbus_write_word_data() {
            write!(f, " SMBUS WRITE WORD DATA |")?;
        };
        if self.smbus_proc_call() {
            write!(f, " SMBUS PROC CALL |")?;
        }
        if self.smbus_read_block_data() {
            write!(f, " SMBUS READ BLOCK DATA |")?;
        };
        if self.smbus_write_block_data() {
            write!(f, " SMBUS WRITE BLOCK DATA |")?;
        };
        if self.smbus_read_block() {
            write!(f, " SMBUS READ BLOCK |")?;
        };
        if self.smbus_write_block() {
            write!(f, " SMBUS WRITE BLOCK |")?;
        };

        Ok(())
    }
}
