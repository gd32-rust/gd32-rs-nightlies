#[doc = "Register `SSTS` reader"]
pub struct R(crate::R<SSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SSC` reader - Sub second value"]
pub type SSC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Sub second of time stamp register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssts](index.html) module"]
pub struct SSTS_SPEC;
impl crate::RegisterSpec for SSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssts::R](R) reader structure"]
impl crate::Readable for SSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSTS to value 0"]
impl crate::Resettable for SSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
