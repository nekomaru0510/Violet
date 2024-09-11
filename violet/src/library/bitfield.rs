//! bit operation

pub struct BitField {
    pub offset: u8,
    pub width: u8,
}

impl BitField {
    pub fn new() -> Self {
        BitField {
            offset: 0,
            width: 0,
        }
    }

    // Generate a bit pattern based on offset and width
    // [todo fix] val should be generic type
    pub fn pattern(&self, val: u64) -> u64 {
        let mask = (2 << (self.width - 1)) - 1;
        (val & mask) << self.offset
    }

    // Get a value based on offset and width
    // [todo fix] val should be generic type
    pub fn mask(&self, val: u64) -> u64 {
        let mask = (2 << (self.width - 1)) - 1;
        (val >> self.offset) & mask
    }
}

#[macro_export]
macro_rules! bitfield {
    ($name:ident: [$f0:expr,$f1:expr]) => {
        const $name: (usize, usize) = ($f0, $f1);
    };
}

#[macro_export]
macro_rules! bit_extract {
    ($val:expr, $field:expr) => {
        ($val & (bit_fill!($field) << $field.1)) >> $field.1
    };
}

#[macro_export]
macro_rules! bit_fill {
    ($field:expr) => {
        (1 << (($field.0 - $field.1) + 1)) - 1
    };
}

#[macro_export]
macro_rules! bit_clear {
    ($src:expr, $field:expr) => {
        ($src & !(bit_fill!($field) << $field.1))
    };
}

#[macro_export]
macro_rules! bit_set {
    ($src:expr, $field:expr, $val:expr) => {
        bit_clear!($src, $field) | ((bit_fill!($field) & $val) << $field.1)
    };
}

#[test_case]
fn test_bitfield() -> Result<(), &'static str> {
    bitfield!(RS2:[24,20]);
    if bit_extract!(0x01f0_0000, RS2) == 0x1f {
        Ok(())
    } else {
        Err("calc miss")
    }
}

#[test_case]
fn test_bitfield2() -> Result<(), &'static str> {
    bitfield!(RS2:[24,20]);
    let val = bit_set!(0x0000_0000, RS2, 0xffff);
    if bit_set!(0x0000_0000, RS2, 0xffff) == 0x01f0_0000 {
        Ok(())
    } else {
        Err("calc miss")
    }
}
