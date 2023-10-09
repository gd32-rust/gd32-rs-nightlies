#[doc = "Register `PTP_TSH` reader"]
pub type R = crate::R<PTP_TSH_SPEC>;
#[doc = "Field `STMS` reader - System time second"]
pub type STMS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - System time second"]
    #[inline(always)]
    pub fn stms(&self) -> STMS_R {
        STMS_R::new(self.bits)
    }
}
#[doc = "Ethernet PTP time stamp high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTP_TSH_SPEC;
impl crate::RegisterSpec for PTP_TSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_tsh::R`](R) reader structure"]
impl crate::Readable for PTP_TSH_SPEC {}
#[doc = "`reset()` method sets PTP_TSH to value 0"]
impl crate::Resettable for PTP_TSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
