//! bit operation

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

#[test_case]
fn test_bitfield() -> Result<(), &'static str> {
    bitfield!(RS2:[24,20]);
    if bit_extract!(0x01f0_0000, RS2) == 0x1f {
        Ok(())
    } else {
        Err("calc miss")
    }
}
