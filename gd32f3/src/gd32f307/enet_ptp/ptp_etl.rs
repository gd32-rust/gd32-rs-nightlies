#[doc = "Register `PTP_ETL` reader"]
pub struct R(crate::R<PTP_ETL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_ETL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_ETL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_ETL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTP_ETL` writer"]
pub struct W(crate::W<PTP_ETL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTP_ETL_SPEC>;
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
impl From<crate::W<PTP_ETL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTP_ETL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETSL` reader - Expected time stamp low"]
pub type ETSL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ETSL` writer - Expected time stamp low"]
pub type ETSL_W<'a> = crate::FieldWriter<'a, u32, PTP_ETL_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Expected time stamp low"]
    #[inline(always)]
    pub fn etsl(&self) -> ETSL_R {
        ETSL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Expected time stamp low"]
    #[inline(always)]
    pub fn etsl(&mut self) -> ETSL_W {
        ETSL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP expected time low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_etl](index.html) module"]
pub struct PTP_ETL_SPEC;
impl crate::RegisterSpec for PTP_ETL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_etl::R](R) reader structure"]
impl crate::Readable for PTP_ETL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptp_etl::W](W) writer structure"]
impl crate::Writable for PTP_ETL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTP_ETL to value 0"]
impl crate::Resettable for PTP_ETL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
