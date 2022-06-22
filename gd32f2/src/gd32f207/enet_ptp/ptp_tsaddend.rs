#[doc = "Register `PTP_TSADDEND` reader"]
pub struct R(crate::R<PTP_TSADDEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_TSADDEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_TSADDEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_TSADDEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTP_TSADDEND` writer"]
pub struct W(crate::W<PTP_TSADDEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTP_TSADDEND_SPEC>;
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
impl From<crate::W<PTP_TSADDEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTP_TSADDEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMSA` reader - Time stamp addend"]
pub type TMSA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMSA` writer - Time stamp addend"]
pub type TMSA_W<'a> = crate::FieldWriter<'a, u32, PTP_TSADDEND_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Time stamp addend"]
    #[inline(always)]
    pub fn tmsa(&self) -> TMSA_R {
        TMSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp addend"]
    #[inline(always)]
    pub fn tmsa(&mut self) -> TMSA_W {
        TMSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP time stamp addend register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_tsaddend](index.html) module"]
pub struct PTP_TSADDEND_SPEC;
impl crate::RegisterSpec for PTP_TSADDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_tsaddend::R](R) reader structure"]
impl crate::Readable for PTP_TSADDEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptp_tsaddend::W](W) writer structure"]
impl crate::Writable for PTP_TSADDEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTP_TSADDEND to value 0"]
impl crate::Resettable for PTP_TSADDEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
