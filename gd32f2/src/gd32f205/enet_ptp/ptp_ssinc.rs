#[doc = "Register `PTP_SSINC` reader"]
pub struct R(crate::R<PTP_SSINC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_SSINC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_SSINC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_SSINC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTP_SSINC` writer"]
pub struct W(crate::W<PTP_SSINC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTP_SSINC_SPEC>;
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
impl From<crate::W<PTP_SSINC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTP_SSINC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STMSSI` reader - System time subsecond increment"]
pub type STMSSI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STMSSI` writer - System time subsecond increment"]
pub type STMSSI_W<'a> = crate::FieldWriter<'a, u32, PTP_SSINC_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - System time subsecond increment"]
    #[inline(always)]
    pub fn stmssi(&self) -> STMSSI_R {
        STMSSI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System time subsecond increment"]
    #[inline(always)]
    pub fn stmssi(&mut self) -> STMSSI_W {
        STMSSI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP subsecond increment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_ssinc](index.html) module"]
pub struct PTP_SSINC_SPEC;
impl crate::RegisterSpec for PTP_SSINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_ssinc::R](R) reader structure"]
impl crate::Readable for PTP_SSINC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptp_ssinc::W](W) writer structure"]
impl crate::Writable for PTP_SSINC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTP_SSINC to value 0"]
impl crate::Resettable for PTP_SSINC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
