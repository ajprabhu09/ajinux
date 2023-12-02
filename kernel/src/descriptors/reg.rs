pub struct CS;
pub struct DS;
pub struct SS;
pub struct ES;
pub struct FS;
pub struct GS;

pub struct CR0;

use core::arch::asm;



pub trait GetReg {
    fn get_reg() -> u16;
}

pub trait SetReg<T> {
    fn set_reg(val: T);
}

macro_rules! get_reg_impl {
    ($reg:expr, $regtyp:ty) => {
        impl GetReg for $regtyp {
            fn get_reg() -> u16 {
                let mut res: u16 = 0;
                unsafe {
                    asm!(concat!("mov {0:x},", $reg), out(reg) res);
                }
                return res;
            }
        }

        #[test_case]
        pub fn test_cs_get() {
            info!(concat!($reg,": {:?}"), <$regtyp>::get_reg());
        }
    }
}
macro_rules! set_reg64_impl {
    ($reg:expr, $regtyp:ty) => {
        impl SetReg<u64> for $regtyp {
            fn set_reg(val: u64) {
                unsafe {
                    asm!(
                        concat!("mov ", $reg,", {}"),
                        in(reg) val);
                }
            }
        }
    }
}
get_reg_impl!("cs", CS);
get_reg_impl!("ds", DS);

set_reg64_impl!("cr0", CR0);
