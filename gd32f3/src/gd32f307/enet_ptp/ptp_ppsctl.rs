#[doc = "Register `PTP_PPSCTL` reader"]
pub struct R(crate::R<PTP_PPSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_PPSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_PPSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_PPSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTP_PPSCTL` writer"]
pub struct W(crate::W<PTP_PPSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTP_PPSCTL_SPEC>;
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
impl From<crate::W<PTP_PPSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTP_PPSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSOFC` reader - PPS output frequency configure"]
pub type PPSOFC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPSOFC` writer - PPS output frequency configure"]
pub type PPSOFC_W<'a> = crate::FieldWriter<'a, u32, PTP_PPSCTL_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - PPS output frequency configure"]
    #[inline(always)]
    pub fn ppsofc(&self) -> PPSOFC_R {
        PPSOFC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PPS output frequency configure"]
    #[inline(always)]
    pub fn ppsofc(&mut self) -> PPSOFC_W {
        PPSOFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP PPS control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_ppsctl](index.html) module"]
pub struct PTP_PPSCTL_SPEC;
impl crate::RegisterSpec for PTP_PPSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_ppsctl::R](R) reader structure"]
impl crate::Readable for PTP_PPSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptp_ppsctl::W](W) writer structure"]
impl crate::Writable for PTP_PPSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTP_PPSCTL to value 0"]
impl crate::Resettable for PTP_PPSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
