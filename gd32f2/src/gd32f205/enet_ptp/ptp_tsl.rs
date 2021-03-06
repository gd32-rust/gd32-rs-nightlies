#[doc = "Register `PTP_TSL` reader"]
pub struct R(crate::R<PTP_TSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_TSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_TSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_TSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STMSS` reader - System time subseconds"]
pub type STMSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STS` reader - System time sign"]
pub type STS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:30 - System time subseconds"]
    #[inline(always)]
    pub fn stmss(&self) -> STMSS_R {
        STMSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - System time sign"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Ethernet PTP time stamp low register (PTP_TSL)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_tsl](index.html) module"]
pub struct PTP_TSL_SPEC;
impl crate::RegisterSpec for PTP_TSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_tsl::R](R) reader structure"]
impl crate::Readable for PTP_TSL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTP_TSL to value 0"]
impl crate::Resettable for PTP_TSL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
