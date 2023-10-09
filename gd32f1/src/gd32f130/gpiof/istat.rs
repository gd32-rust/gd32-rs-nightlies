#[doc = "Register `ISTAT` reader"]
pub type R = crate::R<ISTAT_SPEC>;
#[doc = "Field `ISTAT0` reader - Port input data 0"]
pub type ISTAT0_R = crate::BitReader<ISTAT0_A>;
#[doc = "Port input data 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISTAT0_A {
    #[doc = "0: Input is logic low"]
    LOW = 0,
    #[doc = "1: Input is logic high"]
    HIGH = 1,
}
impl From<ISTAT0_A> for bool {
    #[inline(always)]
    fn from(variant: ISTAT0_A) -> Self {
        variant as u8 != 0
    }
}
impl ISTAT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISTAT0_A {
        match self.bits {
            false => ISTAT0_A::LOW,
            true => ISTAT0_A::HIGH,
        }
    }
    #[doc = "Input is logic low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ISTAT0_A::LOW
    }
    #[doc = "Input is logic high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ISTAT0_A::HIGH
    }
}
#[doc = "Field `ISTAT1` reader - Port input data 1"]
pub use ISTAT0_R as ISTAT1_R;
#[doc = "Field `ISTAT2` reader - Port input data 2"]
pub use ISTAT0_R as ISTAT2_R;
#[doc = "Field `ISTAT3` reader - Port input data 3"]
pub use ISTAT0_R as ISTAT3_R;
#[doc = "Field `ISTAT4` reader - Port input data 4"]
pub use ISTAT0_R as ISTAT4_R;
#[doc = "Field `ISTAT5` reader - Port input data 5"]
pub use ISTAT0_R as ISTAT5_R;
#[doc = "Field `ISTAT6` reader - Port input data 6"]
pub use ISTAT0_R as ISTAT6_R;
#[doc = "Field `ISTAT7` reader - Port input data 7"]
pub use ISTAT0_R as ISTAT7_R;
#[doc = "Field `ISTAT8` reader - Port input data 8"]
pub use ISTAT0_R as ISTAT8_R;
#[doc = "Field `ISTAT9` reader - Port input data 9"]
pub use ISTAT0_R as ISTAT9_R;
#[doc = "Field `ISTAT10` reader - Port input data 10"]
pub use ISTAT0_R as ISTAT10_R;
#[doc = "Field `ISTAT11` reader - Port input data 11"]
pub use ISTAT0_R as ISTAT11_R;
#[doc = "Field `ISTAT12` reader - Port input data 12"]
pub use ISTAT0_R as ISTAT12_R;
#[doc = "Field `ISTAT13` reader - Port input data 13"]
pub use ISTAT0_R as ISTAT13_R;
#[doc = "Field `ISTAT14` reader - Port input data 14"]
pub use ISTAT0_R as ISTAT14_R;
#[doc = "Field `ISTAT15` reader - Port input data 14"]
pub use ISTAT0_R as ISTAT15_R;
impl R {
    #[doc = "Bit 0 - Port input data 0"]
    #[inline(always)]
    pub fn istat0(&self) -> ISTAT0_R {
        ISTAT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input data 1"]
    #[inline(always)]
    pub fn istat1(&self) -> ISTAT1_R {
        ISTAT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input data 2"]
    #[inline(always)]
    pub fn istat2(&self) -> ISTAT2_R {
        ISTAT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input data 3"]
    #[inline(always)]
    pub fn istat3(&self) -> ISTAT3_R {
        ISTAT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input data 4"]
    #[inline(always)]
    pub fn istat4(&self) -> ISTAT4_R {
        ISTAT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input data 5"]
    #[inline(always)]
    pub fn istat5(&self) -> ISTAT5_R {
        ISTAT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input data 6"]
    #[inline(always)]
    pub fn istat6(&self) -> ISTAT6_R {
        ISTAT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input data 7"]
    #[inline(always)]
    pub fn istat7(&self) -> ISTAT7_R {
        ISTAT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input data 8"]
    #[inline(always)]
    pub fn istat8(&self) -> ISTAT8_R {
        ISTAT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input data 9"]
    #[inline(always)]
    pub fn istat9(&self) -> ISTAT9_R {
        ISTAT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input data 10"]
    #[inline(always)]
    pub fn istat10(&self) -> ISTAT10_R {
        ISTAT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input data 11"]
    #[inline(always)]
    pub fn istat11(&self) -> ISTAT11_R {
        ISTAT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input data 12"]
    #[inline(always)]
    pub fn istat12(&self) -> ISTAT12_R {
        ISTAT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input data 13"]
    #[inline(always)]
    pub fn istat13(&self) -> ISTAT13_R {
        ISTAT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input data 14"]
    #[inline(always)]
    pub fn istat14(&self) -> ISTAT14_R {
        ISTAT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input data 14"]
    #[inline(always)]
    pub fn istat15(&self) -> ISTAT15_R {
        ISTAT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTAT_SPEC;
impl crate::RegisterSpec for ISTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istat::R`](R) reader structure"]
impl crate::Readable for ISTAT_SPEC {}
#[doc = "`reset()` method sets ISTAT to value 0"]
impl crate::Resettable for ISTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
