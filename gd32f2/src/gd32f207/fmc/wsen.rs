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
#[doc = "Register `WSEN` writer"]
pub struct W(crate::W<WSEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WSEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WSEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WSEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FMC wait state enable\n\nValue on reset: 0"]
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
#[doc = "Field `WSEN` reader - FMC wait state enable"]
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
#[doc = "Field `WSEN` writer - FMC wait state enable"]
pub type WSEN_W<'a> = crate::BitWriter<'a, u32, WSEN_SPEC, WSEN_A, 0>;
impl<'a> WSEN_W<'a> {
    #[doc = "No wait state added"]
    #[inline(always)]
    pub fn no_wait_state(self) -> &'a mut W {
        self.variant(WSEN_A::NOWAITSTATE)
    }
    #[doc = "Wait state added"]
    #[inline(always)]
    pub fn wait_state(self) -> &'a mut W {
        self.variant(WSEN_A::WAITSTATE)
    }
}
impl R {
    #[doc = "Bit 0 - FMC wait state enable"]
    #[inline(always)]
    pub fn wsen(&self) -> WSEN_R {
        WSEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMC wait state enable"]
    #[inline(always)]
    pub fn wsen(&mut self) -> WSEN_W {
        WSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wait state enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wsen](index.html) module"]
pub struct WSEN_SPEC;
impl crate::RegisterSpec for WSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wsen::R](R) reader structure"]
impl crate::Readable for WSEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wsen::W](W) writer structure"]
impl crate::Writable for WSEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WSEN to value 0"]
impl crate::Resettable for WSEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
