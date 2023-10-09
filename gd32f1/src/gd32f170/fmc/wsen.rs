#[doc = "Register `WSEN` reader"]
pub type R = crate::R<WSEN_SPEC>;
#[doc = "Field `WSEN` reader - FMC wait state enable register"]
pub type WSEN_R = crate::BitReader<WSEN_A>;
#[doc = "FMC wait state enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WSEN_A {
    #[doc = "0: No wait state added"]
    NO_WAIT_STATE = 0,
    #[doc = "1: Wait state added"]
    WAIT_STATE = 1,
}
impl From<WSEN_A> for bool {
    #[inline(always)]
    fn from(variant: WSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSEN_A {
        match self.bits {
            false => WSEN_A::NO_WAIT_STATE,
            true => WSEN_A::WAIT_STATE,
        }
    }
    #[doc = "No wait state added"]
    #[inline(always)]
    pub fn is_no_wait_state(&self) -> bool {
        *self == WSEN_A::NO_WAIT_STATE
    }
    #[doc = "Wait state added"]
    #[inline(always)]
    pub fn is_wait_state(&self) -> bool {
        *self == WSEN_A::WAIT_STATE
    }
}
#[doc = "Field `BPEN` reader - FMC bit program enable register"]
pub type BPEN_R = crate::BitReader;
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
#[doc = "Flash wait state control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsen::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WSEN_SPEC;
impl crate::RegisterSpec for WSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wsen::R`](R) reader structure"]
impl crate::Readable for WSEN_SPEC {}
#[doc = "`reset()` method sets WSEN to value 0"]
impl crate::Resettable for WSEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
