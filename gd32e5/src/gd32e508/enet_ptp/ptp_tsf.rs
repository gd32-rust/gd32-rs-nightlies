#[doc = "Register `PTP_TSF` reader"]
pub type R = crate::R<PTP_TSF_SPEC>;
#[doc = "Field `TSSCO` reader - Timestamp second counter overflow"]
pub type TSSCO_R = crate::BitReader;
#[doc = "Field `TTM` reader - Target time match"]
pub type TTM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timestamp second counter overflow"]
    #[inline(always)]
    pub fn tssco(&self) -> TSSCO_R {
        TSSCO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Target time match"]
    #[inline(always)]
    pub fn ttm(&self) -> TTM_R {
        TTM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Ethernet PTP time stamp flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTP_TSF_SPEC;
impl crate::RegisterSpec for PTP_TSF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_tsf::R`](R) reader structure"]
impl crate::Readable for PTP_TSF_SPEC {}
#[doc = "`reset()` method sets PTP_TSF to value 0"]
impl crate::Resettable for PTP_TSF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
