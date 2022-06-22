#[doc = "Register `PTP_TSF` reader"]
pub struct R(crate::R<PTP_TSF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_TSF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_TSF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_TSF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSSCO` reader - Timestamp second counter overflow"]
pub type TSSCO_R = crate::BitReader<bool>;
#[doc = "Field `TTM` reader - Target time match"]
pub type TTM_R = crate::BitReader<bool>;
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
#[doc = "Ethernet PTP time stamp flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_tsf](index.html) module"]
pub struct PTP_TSF_SPEC;
impl crate::RegisterSpec for PTP_TSF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_tsf::R](R) reader structure"]
impl crate::Readable for PTP_TSF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTP_TSF to value 0"]
impl crate::Resettable for PTP_TSF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
