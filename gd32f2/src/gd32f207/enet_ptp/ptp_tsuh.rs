#[doc = "Register `PTP_TSUH` reader"]
pub struct R(crate::R<PTP_TSUH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_TSUH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_TSUH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_TSUH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTP_TSUH` writer"]
pub struct W(crate::W<PTP_TSUH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTP_TSUH_SPEC>;
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
impl From<crate::W<PTP_TSUH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTP_TSUH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMSUS` reader - Time stamp update second"]
pub type TMSUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMSUS` writer - Time stamp update second"]
pub type TMSUS_W<'a> = crate::FieldWriter<'a, u32, PTP_TSUH_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Time stamp update second"]
    #[inline(always)]
    pub fn tmsus(&self) -> TMSUS_R {
        TMSUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp update second"]
    #[inline(always)]
    pub fn tmsus(&mut self) -> TMSUS_W {
        TMSUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP time stamp high update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_tsuh](index.html) module"]
pub struct PTP_TSUH_SPEC;
impl crate::RegisterSpec for PTP_TSUH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_tsuh::R](R) reader structure"]
impl crate::Readable for PTP_TSUH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptp_tsuh::W](W) writer structure"]
impl crate::Writable for PTP_TSUH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTP_TSUH to value 0"]
impl crate::Resettable for PTP_TSUH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
