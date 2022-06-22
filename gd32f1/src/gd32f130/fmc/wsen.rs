#[doc = "Register `WSEN` reader"]
pub struct R(crate::R<WSEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WSEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WSEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WSEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "FMC wait state enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSEN_A {
    #[doc = "0: No wait state added"]
    NOWAITSTATE = 0,
    #[doc = "1: Wait state added"]
    WAITSTATE = 1,
}
impl From<WSEN_A> for bool {
    #[inline(always)]
    fn from(variant: WSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSEN` reader - FMC wait state enable register"]
pub type WSEN_R = crate::BitReader<WSEN_A>;
impl WSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSEN_A {
        match self.bits {
            false => WSEN_A::NOWAITSTATE,
            true => WSEN_A::WAITSTATE,
        }
    }
    #[doc = "Checks if the value of the field is `NOWAITSTATE`"]
    #[inline(always)]
    pub fn is_no_wait_state(&self) -> bool {
        *self == WSEN_A::NOWAITSTATE
    }
    #[doc = "Checks if the value of the field is `WAITSTATE`"]
    #[inline(always)]
    pub fn is_wait_state(&self) -> bool {
        *self == WSEN_A::WAITSTATE
    }
}
#[doc = "Field `BPEN` reader - FMC bit program enable register"]
pub type BPEN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    pub fn wsen(&self) -> WSEN_R {
        WSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FMC bit program enable register"]
    #[inline(always)]
    pub fn bpen(&self) -> BPEN_R {
        BPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Flash wait state control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wsen](index.html) module"]
pub struct WSEN_SPEC;
impl crate::RegisterSpec for WSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wsen::R](R) reader structure"]
impl crate::Readable for WSEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WSEN to value 0"]
impl crate::Resettable for WSEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
