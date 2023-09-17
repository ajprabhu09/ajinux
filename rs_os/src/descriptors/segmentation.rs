pub struct CS;
pub struct DS;
pub struct SS;
pub struct ES;
pub struct FS;
pub struct GS;

use core::arch::asm;

use crate::info;

trait GetReg {
    fn get_reg() -> u16;
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

get_reg_impl!("cs", CS);
get_reg_impl!("ds", DS);

