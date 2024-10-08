use core::fmt::Debug;
use core::ops::{BitAnd, BitOr, BitOrAssign, Not, Shl, Shr};

pub trait RegSize:
    BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitOrAssign
    + Not<Output = Self>
    + Eq
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + Copy
    + Clone
    + Debug
{
    fn zero() -> Self;
}

macro_rules! RegSize_impl_for {
    ($type:ty) => {
        impl RegSize for $type {
            fn zero() -> Self {
                0
            }
        }
    };
}

RegSize_impl_for!(u8);
RegSize_impl_for!(u16);
RegSize_impl_for!(u32);
RegSize_impl_for!(u64);
RegSize_impl_for!(u128);
RegSize_impl_for!(usize);

pub struct Field<T:RegSize> {
    pub offset: usize,
    pub mask: T,
}

impl<T: RegSize> Field<T> {
    pub const fn new(offset: usize, mask: T) -> Self {
        Field { offset, mask }
    }

    #[inline(always)]
    pub fn read(&self, reg: T) -> T {
        (reg & self.mask) >> self.offset
    }

    #[inline(always)]
    pub fn modify(&self, reg: T, value: T) -> T {
        (reg & !self.mask) | ((value << self.offset) & self.mask)
    }
}

#[macro_export]
macro_rules! register {
    (
        $reg_name:ident,       /* Register Name */
        $reg_size:ty,          /* Register Size */
        $read_instr:expr,      /* Read Instruction */
        $write_instr:expr,     /* Write Instruction */
        {                      /* Register Field */
            $($field_name:ident OFFSET($offset:expr) NUMBITS($numbits:expr) [$($field_val:ident = $val:expr),*]),*
        }
    ) => {
        use crate::regfield;
        use crate::regfunc;

        regfield!(
            $reg_name,       /* Register Name */
            $reg_size,       /* Register Size */
            {                /* Register Field */
                $(
                    $field_name OFFSET($offset) NUMBITS($numbits) [$($field_val = $val),*]
                ),*
            }
        );

        regfunc!(
            $reg_name,       /* Register Name */
            $reg_size,       /* Register Size */
            $read_instr,     /* Read */
            $write_instr     /* Write */
        );
    };
}

#[macro_export]
macro_rules! regfield {
    (
        $reg_name:ident, $type:ty, 
        { 
            $($field_name:ident OFFSET($off:expr) NUMBITS($bits:expr) 
                [ 
                    $(
                        $variant_name:ident = $variant_value:expr
                    ),* 
                ]
            ),* 
        }
    ) => {
        $(
            #[allow(non_snake_case)]
            pub mod $field_name {                
                pub const OFFSET: usize= $off;
                pub const NUMBITS: usize = $bits;
                pub const MASK: $type = ((1 as $type).wrapping_shl(NUMBITS as u32) - 1).wrapping_shl(OFFSET as u32);
                pub const SET: $type = MASK.wrapping_shr(OFFSET as u32);
                pub const CLEAR: $type = !SET;

                $(
                    pub const $variant_name: $type = $variant_value;
                )*
            }

            pub const $field_name: Field<$type> = Field::new($field_name::OFFSET, $field_name::MASK);
        )*
    };
}

#[macro_export]
macro_rules! regfunc {
    ($name:ident, $type:ty, $read_instr:expr, $write_instr:expr) => {
        use core::arch::asm;
        use crate::library::register::Field;
        
        pub struct $name;

        impl $name {
            /// Reads the raw bits of the $name register.
            #[inline(always)]
            pub fn get() -> $type {
                let reg;
                unsafe {
                    //asm!($read_instr : "=r"(reg) ::: "volatile");
                    asm!($read_instr, out(reg) reg, options(nostack));
                }
                reg
            }

            /// Writes raw bits to the $name register.
            #[inline(always)]
            pub fn set(value: $type) {
                unsafe {
                    //asm!($write_instr :: "r"(value) :: "volatile");
                    asm!($write_instr, in(reg) value, options(nostack));
                }
            }

            #[inline(always)]
            pub fn read(field: Field<$type>) -> $type {
                field.read(Self::get())
            }

            #[inline(always)]
            pub fn write(field: Field<$type>, value: $type) {
                Self::set(field.modify(Self::get(), value));
            }
        }
    };
}

mod vreg{
    register!(
        Vreg,       /* Register Name */
        u64,        /* Register Size */
        "csrr {}, 0x240",   /* Read */
        "csrw 0x240, {}",    /* Write */
        {           /* Register Field */
            VSSIE OFFSET(2) NUMBITS(1) [],
            VSTIE OFFSET(6) NUMBITS(1) [],
            VSEIE OFFSET(10) NUMBITS(1) [],
            SGEIE OFFSET(12) NUMBITS(1) [],
            MPP   OFFSET(16) NUMBITS(2) [
                USER = 0,
                SUPERVISOR = 1,
                RESERVED = 2,
                MACHINE = 3
            ]
        }
    );
}

//#[cfg(test)]
use vreg::*;

//#[test_case]
pub fn test_register() -> Result<(), &'static str> {
    let backup = Vreg::get();

    Vreg::set( (1<<2) | (1<<6) );

    println!("Vreg: 0x{:x}", Vreg::get());

    if Vreg::get() != (1<<2) | (1<<6) {
        return Err("Invalid value");
    }
    if Vreg::read(VSSIE) != 1 {
        return Err("Invalid value");
    }
    if Vreg::read(VSTIE) != 1 {
        return Err("Invalid value");
    }
    if Vreg::read(MPP) != 0 {
        return Err("Invalid value");
    }

    Vreg::write(VSSIE, 0);
    Vreg::write(MPP, MPP::MACHINE);

    if Vreg::get() != (1<<6) | (3<<16) {
        return Err("Invalid value");
    }
    if Vreg::read(VSSIE) != 0 {
        return Err("Invalid value");
    }
    if Vreg::read(VSTIE) != 1 {
        return Err("Invalid value");
    }
    if Vreg::read(MPP) != 3 {
        return Err("Invalid value");
    }

    Vreg::set(backup);
    println!("Vreg test passed");
    Ok(())
}
