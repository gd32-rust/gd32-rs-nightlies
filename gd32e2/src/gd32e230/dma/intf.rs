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
#[doc = "Channel 0 Global interrupt flag\n\nValue on reset: 0"]
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
#[doc = "Field `GIF0` reader - Channel 0 Global interrupt flag"]
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
#[doc = "Channel 0 Full Transfer Finish flag\n\nValue on reset: 0"]
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
#[doc = "Field `FTFIF0` reader - Channel 0 Full Transfer Finish flag"]
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
#[doc = "Channel 0 Half Transfer Finish flag\n\nValue on reset: 0"]
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
#[doc = "Field `HTFIF0` reader - Channel 0 Half Transfer Finish flag"]
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
#[doc = "Channel 0 Error flag\n\nValue on reset: 0"]
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
#[doc = "Field `ERRIF0` reader - Channel 0 Error flag"]
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
#[doc = "Channel 1 Error flag"]
pub use ERRIF0_A as ERRIF1_A;
#[doc = "Channel 2 Error flag"]
pub use ERRIF0_A as ERRIF2_A;
#[doc = "Channel 3 Error flag"]
pub use ERRIF0_A as ERRIF3_A;
#[doc = "Channel 4 Error flag"]
pub use ERRIF0_A as ERRIF4_A;
#[doc = "Field `ERRIF1` reader - Channel 1 Error flag"]
pub use ERRIF0_R as ERRIF1_R;
#[doc = "Field `ERRIF2` reader - Channel 2 Error flag"]
pub use ERRIF0_R as ERRIF2_R;
#[doc = "Field `ERRIF3` reader - Channel 3 Error flag"]
pub use ERRIF0_R as ERRIF3_R;
#[doc = "Field `ERRIF4` reader - Channel 4 Error flag"]
pub use ERRIF0_R as ERRIF4_R;
#[doc = "Channel 1 Full Transfer Finish flag"]
pub use FTFIF0_A as FTFIF1_A;
#[doc = "Channel 2 Full Transfer Finish flag"]
pub use FTFIF0_A as FTFIF2_A;
#[doc = "Channel 3 Full Transfer Finish flag"]
pub use FTFIF0_A as FTFIF3_A;
#[doc = "Channel 4 Full Transfer Finish flag"]
pub use FTFIF0_A as FTFIF4_A;
#[doc = "Field `FTFIF1` reader - Channel 1 Full Transfer Finish flag"]
pub use FTFIF0_R as FTFIF1_R;
#[doc = "Field `FTFIF2` reader - Channel 2 Full Transfer Finish flag"]
pub use FTFIF0_R as FTFIF2_R;
#[doc = "Field `FTFIF3` reader - Channel 3 Full Transfer Finish flag"]
pub use FTFIF0_R as FTFIF3_R;
#[doc = "Field `FTFIF4` reader - Channel 4 Full Transfer Finish flag"]
pub use FTFIF0_R as FTFIF4_R;
#[doc = "Channel 1 Global interrupt flag"]
pub use GIF0_A as GIF1_A;
#[doc = "Channel 2 Global interrupt flag"]
pub use GIF0_A as GIF2_A;
#[doc = "Channel 3 Global interrupt flag"]
pub use GIF0_A as GIF3_A;
#[doc = "Channel 4 Global interrupt flag"]
pub use GIF0_A as GIF4_A;
#[doc = "Field `GIF1` reader - Channel 1 Global interrupt flag"]
pub use GIF0_R as GIF1_R;
#[doc = "Field `GIF2` reader - Channel 2 Global interrupt flag"]
pub use GIF0_R as GIF2_R;
#[doc = "Field `GIF3` reader - Channel 3 Global interrupt flag"]
pub use GIF0_R as GIF3_R;
#[doc = "Field `GIF4` reader - Channel 4 Global interrupt flag"]
pub use GIF0_R as GIF4_R;
#[doc = "Channel 1 Half Transfer Finish flag"]
pub use HTFIF0_A as HTFIF1_A;
#[doc = "Channel 2 Half Transfer Finish flag"]
pub use HTFIF0_A as HTFIF2_A;
#[doc = "Channel 3 Half Transfer Finish flag"]
pub use HTFIF0_A as HTFIF3_A;
#[doc = "Channel 4 Half Transfer Finish flag"]
pub use HTFIF0_A as HTFIF4_A;
#[doc = "Field `HTFIF1` reader - Channel 1 Half Transfer Finish flag"]
pub use HTFIF0_R as HTFIF1_R;
#[doc = "Field `HTFIF2` reader - Channel 2 Half Transfer Finish flag"]
pub use HTFIF0_R as HTFIF2_R;
#[doc = "Field `HTFIF3` reader - Channel 3 Half Transfer Finish flag"]
pub use HTFIF0_R as HTFIF3_R;
#[doc = "Field `HTFIF4` reader - Channel 4 Half Transfer Finish flag"]
pub use HTFIF0_R as HTFIF4_R;
impl R {
    #[doc = "Bit 0 - Channel 0 Global interrupt flag"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif0(&self) -> FTFIF0_R {
        FTFIF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif0(&self) -> HTFIF0_R {
        HTFIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 Error flag"]
    #[inline(always)]
    pub fn errif0(&self) -> ERRIF0_R {
        ERRIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 Global interrupt flag"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif1(&self) -> FTFIF1_R {
        FTFIF1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 1 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif1(&self) -> HTFIF1_R {
        HTFIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 1 Error flag"]
    #[inline(always)]
    pub fn errif1(&self) -> ERRIF1_R {
        ERRIF1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 2 Global interrupt flag"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 2 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif2(&self) -> FTFIF2_R {
        FTFIF2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif2(&self) -> HTFIF2_R {
        HTFIF2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 Error flag"]
    #[inline(always)]
    pub fn errif2(&self) -> ERRIF2_R {
        ERRIF2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 Global interrupt flag"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 3 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif3(&self) -> FTFIF3_R {
        FTFIF3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 3 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif3(&self) -> HTFIF3_R {
        HTFIF3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 3 Error flag"]
    #[inline(always)]
    pub fn errif3(&self) -> ERRIF3_R {
        ERRIF3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 4 Global interrupt flag"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 4 Full Transfer Finish flag"]
    #[inline(always)]
    pub fn ftfif4(&self) -> FTFIF4_R {
        FTFIF4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 4 Half Transfer Finish flag"]
    #[inline(always)]
    pub fn htfif4(&self) -> HTFIF4_R {
        HTFIF4_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 4 Error flag"]
    #[inline(always)]
    pub fn errif4(&self) -> ERRIF4_R {
        ERRIF4_R::new(((self.bits >> 19) & 1) != 0)
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
