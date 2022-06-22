#[doc = "Register `INTF` reader"]
pub struct R(crate::R<INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Global interrupt flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF0_A {
    #[doc = "0: No transfer error, half event, complete event"]
    NOEVENT = 0,
    #[doc = "1: A transfer error, half event or complete event has occured"]
    EVENT = 1,
}
impl From<GIF0_A> for bool {
    #[inline(always)]
    fn from(variant: GIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF0` reader - Global interrupt flag of channel 0"]
pub type GIF0_R = crate::BitReader<GIF0_A>;
impl GIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF0_A {
        match self.bits {
            false => GIF0_A::NOEVENT,
            true => GIF0_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == GIF0_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == GIF0_A::EVENT
    }
}
#[doc = "Full transfer finish flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTFIF0_A {
    #[doc = "0: No transfer complete event"]
    NOTCOMPLETE = 0,
    #[doc = "1: A transfer complete event has occured"]
    COMPLETE = 1,
}
impl From<FTFIF0_A> for bool {
    #[inline(always)]
    fn from(variant: FTFIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTFIF0` reader - Full transfer finish flag of channel 0"]
pub type FTFIF0_R = crate::BitReader<FTFIF0_A>;
impl FTFIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTFIF0_A {
        match self.bits {
            false => FTFIF0_A::NOTCOMPLETE,
            true => FTFIF0_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == FTFIF0_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == FTFIF0_A::COMPLETE
    }
}
#[doc = "Half transfer finish flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTFIF0_A {
    #[doc = "0: No half transfer event"]
    NOTHALF = 0,
    #[doc = "1: A half transfer event has occured"]
    HALF = 1,
}
impl From<HTFIF0_A> for bool {
    #[inline(always)]
    fn from(variant: HTFIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTFIF0` reader - Half transfer finish flag of channel 0"]
pub type HTFIF0_R = crate::BitReader<HTFIF0_A>;
impl HTFIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTFIF0_A {
        match self.bits {
            false => HTFIF0_A::NOTHALF,
            true => HTFIF0_A::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTFIF0_A::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTFIF0_A::HALF
    }
}
#[doc = "Transfer access error flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIF0_A {
    #[doc = "0: No transfer error"]
    NOERROR = 0,
    #[doc = "1: A transfer error has occured"]
    ERROR = 1,
}
impl From<ERRIF0_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIF0` reader - Transfer access error flag of channel 0"]
pub type ERRIF0_R = crate::BitReader<ERRIF0_A>;
impl ERRIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIF0_A {
        match self.bits {
            false => ERRIF0_A::NOERROR,
            true => ERRIF0_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERRIF0_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERRIF0_A::ERROR
    }
}
#[doc = "Transfer access error flag of channel 1"]
pub use ERRIF0_A as ERRIF1_A;
#[doc = "Transfer access error flag of channel 2"]
pub use ERRIF0_A as ERRIF2_A;
#[doc = "Transfer access error flag of channel 3"]
pub use ERRIF0_A as ERRIF3_A;
#[doc = "Transfer access error flag of channel 4"]
pub use ERRIF0_A as ERRIF4_A;
#[doc = "Transfer access error flag of channel 5"]
pub use ERRIF0_A as ERRIF5_A;
#[doc = "Transfer access error flag of channel 6"]
pub use ERRIF0_A as ERRIF6_A;
#[doc = "Field `ERRIF1` reader - Transfer access error flag of channel 1"]
pub use ERRIF0_R as ERRIF1_R;
#[doc = "Field `ERRIF2` reader - Transfer access error flag of channel 2"]
pub use ERRIF0_R as ERRIF2_R;
#[doc = "Field `ERRIF3` reader - Transfer access error flag of channel 3"]
pub use ERRIF0_R as ERRIF3_R;
#[doc = "Field `ERRIF4` reader - Transfer access error flag of channel 4"]
pub use ERRIF0_R as ERRIF4_R;
#[doc = "Field `ERRIF5` reader - Transfer access error flag of channel 5"]
pub use ERRIF0_R as ERRIF5_R;
#[doc = "Field `ERRIF6` reader - Transfer access error flag of channel 6"]
pub use ERRIF0_R as ERRIF6_R;
#[doc = "Full transfer finish flag of channel 1"]
pub use FTFIF0_A as FTFIF1_A;
#[doc = "Full transfer finish flag of channel 2"]
pub use FTFIF0_A as FTFIF2_A;
#[doc = "Full transfer finish flag of channel 3"]
pub use FTFIF0_A as FTFIF3_A;
#[doc = "Full transfer finish flag of channel 4"]
pub use FTFIF0_A as FTFIF4_A;
#[doc = "Full transfer finish flag of channel 5"]
pub use FTFIF0_A as FTFIF5_A;
#[doc = "Full transfer finish flag of channel 6"]
pub use FTFIF0_A as FTFIF6_A;
#[doc = "Field `FTFIF1` reader - Full transfer finish flag of channel 1"]
pub use FTFIF0_R as FTFIF1_R;
#[doc = "Field `FTFIF2` reader - Full transfer finish flag of channel 2"]
pub use FTFIF0_R as FTFIF2_R;
#[doc = "Field `FTFIF3` reader - Full transfer finish flag of channel 3"]
pub use FTFIF0_R as FTFIF3_R;
#[doc = "Field `FTFIF4` reader - Full transfer finish flag of channel 4"]
pub use FTFIF0_R as FTFIF4_R;
#[doc = "Field `FTFIF5` reader - Full transfer finish flag of channel 5"]
pub use FTFIF0_R as FTFIF5_R;
#[doc = "Field `FTFIF6` reader - Full transfer finish flag of channel 6"]
pub use FTFIF0_R as FTFIF6_R;
#[doc = "Global interrupt flag of channel 1"]
pub use GIF0_A as GIF1_A;
#[doc = "Global interrupt flag of channel 2"]
pub use GIF0_A as GIF2_A;
#[doc = "Global interrupt flag of channel 3"]
pub use GIF0_A as GIF3_A;
#[doc = "Global interrupt flag of channel 4"]
pub use GIF0_A as GIF4_A;
#[doc = "Global interrupt flag of channel 5"]
pub use GIF0_A as GIF5_A;
#[doc = "Global interrupt flag of channel 6"]
pub use GIF0_A as GIF6_A;
#[doc = "Field `GIF1` reader - Global interrupt flag of channel 1"]
pub use GIF0_R as GIF1_R;
#[doc = "Field `GIF2` reader - Global interrupt flag of channel 2"]
pub use GIF0_R as GIF2_R;
#[doc = "Field `GIF3` reader - Global interrupt flag of channel 3"]
pub use GIF0_R as GIF3_R;
#[doc = "Field `GIF4` reader - Global interrupt flag of channel 4"]
pub use GIF0_R as GIF4_R;
#[doc = "Field `GIF5` reader - Global interrupt flag of channel 5"]
pub use GIF0_R as GIF5_R;
#[doc = "Field `GIF6` reader - Global interrupt flag of channel 6"]
pub use GIF0_R as GIF6_R;
#[doc = "Half transfer finish flag of channel 1"]
pub use HTFIF0_A as HTFIF1_A;
#[doc = "Half transfer finish flag of channel 2"]
pub use HTFIF0_A as HTFIF2_A;
#[doc = "Half transfer finish flag of channel 3"]
pub use HTFIF0_A as HTFIF3_A;
#[doc = "Half transfer finish flag of channel 4"]
pub use HTFIF0_A as HTFIF4_A;
#[doc = "Half transfer finish flag of channel 5"]
pub use HTFIF0_A as HTFIF5_A;
#[doc = "Half transfer finish flag of channel 6"]
pub use HTFIF0_A as HTFIF6_A;
#[doc = "Field `HTFIF1` reader - Half transfer finish flag of channel 1"]
pub use HTFIF0_R as HTFIF1_R;
#[doc = "Field `HTFIF2` reader - Half transfer finish flag of channel 2"]
pub use HTFIF0_R as HTFIF2_R;
#[doc = "Field `HTFIF3` reader - Half transfer finish flag of channel 3"]
pub use HTFIF0_R as HTFIF3_R;
#[doc = "Field `HTFIF4` reader - Half transfer finish flag of channel 4"]
pub use HTFIF0_R as HTFIF4_R;
#[doc = "Field `HTFIF5` reader - Half transfer finish flag of channel 5"]
pub use HTFIF0_R as HTFIF5_R;
#[doc = "Field `HTFIF6` reader - Half transfer finish flag of channel 6"]
pub use HTFIF0_R as HTFIF6_R;
impl R {
    #[doc = "Bit 0 - Global interrupt flag of channel 0"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full transfer finish flag of channel 0"]
    #[inline(always)]
    pub fn ftfif0(&self) -> FTFIF0_R {
        FTFIF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer finish flag of channel 0"]
    #[inline(always)]
    pub fn htfif0(&self) -> HTFIF0_R {
        HTFIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer access error flag of channel 0"]
    #[inline(always)]
    pub fn errif0(&self) -> ERRIF0_R {
        ERRIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Global interrupt flag of channel 1"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full transfer finish flag of channel 1"]
    #[inline(always)]
    pub fn ftfif1(&self) -> FTFIF1_R {
        FTFIF1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Half transfer finish flag of channel 1"]
    #[inline(always)]
    pub fn htfif1(&self) -> HTFIF1_R {
        HTFIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer access error flag of channel 1"]
    #[inline(always)]
    pub fn errif1(&self) -> ERRIF1_R {
        ERRIF1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global interrupt flag of channel 2"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Full transfer finish flag of channel 2"]
    #[inline(always)]
    pub fn ftfif2(&self) -> FTFIF2_R {
        FTFIF2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half transfer finish flag of channel 2"]
    #[inline(always)]
    pub fn htfif2(&self) -> HTFIF2_R {
        HTFIF2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transfer access error flag of channel 2"]
    #[inline(always)]
    pub fn errif2(&self) -> ERRIF2_R {
        ERRIF2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Global interrupt flag of channel 3"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Full transfer finish flag of channel 3"]
    #[inline(always)]
    pub fn ftfif3(&self) -> FTFIF3_R {
        FTFIF3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Half transfer finish flag of channel 3"]
    #[inline(always)]
    pub fn htfif3(&self) -> HTFIF3_R {
        HTFIF3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transfer access error flag of channel 3"]
    #[inline(always)]
    pub fn errif3(&self) -> ERRIF3_R {
        ERRIF3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Global interrupt flag of channel 4"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Full transfer finish flag of channel 4"]
    #[inline(always)]
    pub fn ftfif4(&self) -> FTFIF4_R {
        FTFIF4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Half transfer finish flag of channel 4"]
    #[inline(always)]
    pub fn htfif4(&self) -> HTFIF4_R {
        HTFIF4_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transfer access error flag of channel 4"]
    #[inline(always)]
    pub fn errif4(&self) -> ERRIF4_R {
        ERRIF4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Global interrupt flag of channel 5"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Full transfer finish flag of channel 5"]
    #[inline(always)]
    pub fn ftfif5(&self) -> FTFIF5_R {
        FTFIF5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Half transfer finish flag of channel 5"]
    #[inline(always)]
    pub fn htfif5(&self) -> HTFIF5_R {
        HTFIF5_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transfer access error flag of channel 5"]
    #[inline(always)]
    pub fn errif5(&self) -> ERRIF5_R {
        ERRIF5_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Global interrupt flag of channel 6"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Full transfer finish flag of channel 6"]
    #[inline(always)]
    pub fn ftfif6(&self) -> FTFIF6_R {
        FTFIF6_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Half transfer finish flag of channel 6"]
    #[inline(always)]
    pub fn htfif6(&self) -> HTFIF6_R {
        HTFIF6_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transfer access error flag of channel 6"]
    #[inline(always)]
    pub fn errif6(&self) -> ERRIF6_R {
        ERRIF6_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "DMA interrupt flag register (DMA_INTF)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](index.html) module"]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intf::R](R) reader structure"]
impl crate::Readable for INTF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
