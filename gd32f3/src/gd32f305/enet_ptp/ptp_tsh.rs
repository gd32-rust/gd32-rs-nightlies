#[doc = "Register `PTP_TSH` reader"]
pub struct R(crate::R<PTP_TSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_TSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_TSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_TSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STMS` reader - System time second"]
pub type STMS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - System time second"]
    #[inline(always)]
    pub fn stms(&self) -> STMS_R {
        STMS_R::new(self.bits)
    }
}
#[doc = "Ethernet PTP time stamp high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_tsh](index.html) module"]
pub struct PTP_TSH_SPEC;
impl crate::RegisterSpec for PTP_TSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_tsh::R](R) reader structure"]
impl crate::Readable for PTP_TSH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTP_TSH to value 0"]
impl crate::Resettable for PTP_TSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
