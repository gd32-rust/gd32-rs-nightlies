#[doc = "Register `CHC` reader"]
pub struct R(crate::R<CHC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHC` writer"]
pub struct W(crate::W<CHC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHC_SPEC>;
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
impl From<crate::W<CHC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPERR` reader - Early parity error flag"]
pub type EPERR_R = crate::BitReader<bool>;
#[doc = "Field `EPERR` writer - Early parity error flag"]
pub type EPERR_W<'a> = crate::BitWriter<'a, u32, CHC_SPEC, bool, 8>;
#[doc = "Field `HCM` reader - Hardware flow control coherence mode"]
pub type HCM_R = crate::BitReader<bool>;
#[doc = "Field `HCM` writer - Hardware flow control coherence mode"]
pub type HCM_W<'a> = crate::BitWriter<'a, u32, CHC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 8 - Early parity error flag"]
    #[inline(always)]
    pub fn eperr(&self) -> EPERR_R {
        EPERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 0 - Hardware flow control coherence mode"]
    #[inline(always)]
    pub fn hcm(&self) -> HCM_R {
        HCM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Early parity error flag"]
    #[inline(always)]
    pub fn eperr(&mut self) -> EPERR_W {
        EPERR_W::new(self)
    }
    #[doc = "Bit 0 - Hardware flow control coherence mode"]
    #[inline(always)]
    pub fn hcm(&mut self) -> HCM_W {
        HCM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "coherence control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chc](index.html) module"]
pub struct CHC_SPEC;
impl crate::RegisterSpec for CHC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chc::R](R) reader structure"]
impl crate::Readable for CHC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chc::W](W) writer structure"]
impl crate::Writable for CHC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHC to value 0"]
impl crate::Resettable for CHC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
