#[doc = "Register `WSEN` reader"]
pub type R = crate::R<WSEN_SPEC>;
#[doc = "Register `WSEN` writer"]
pub type W = crate::W<WSEN_SPEC>;
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
#[doc = "Field `WSEN` writer - FMC wait state enable register"]
pub type WSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WSEN_A>;
impl<'a, REG, const O: u8> WSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No wait state added"]
    #[inline(always)]
    pub fn no_wait_state(self) -> &'a mut crate::W<REG> {
        self.variant(WSEN_A::NO_WAIT_STATE)
    }
    #[doc = "Wait state added"]
    #[inline(always)]
    pub fn wait_state(self) -> &'a mut crate::W<REG> {
        self.variant(WSEN_A::WAIT_STATE)
    }
}
impl R {
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    pub fn wsen(&self) -> WSEN_R {
        WSEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMC wait state enable register"]
    #[inline(always)]
    #[must_use]
    pub fn wsen(&mut self) -> WSEN_W<WSEN_SPEC, 0> {
        WSEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Wait state enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wsen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WSEN_SPEC;
impl crate::RegisterSpec for WSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wsen::R`](R) reader structure"]
impl crate::Readable for WSEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wsen::W`](W) writer structure"]
impl crate::Writable for WSEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WSEN to value 0"]
impl crate::Resettable for WSEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
