#[doc = "Register `SS` reader"]
pub struct R(crate::R<SS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SS_SPEC>) -> Self {
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
#[doc = "sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ss](index.html) module"]
pub struct SS_SPEC;
impl crate::RegisterSpec for SS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ss::R](R) reader structure"]
impl crate::Readable for SS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SS to value 0"]
impl crate::Resettable for SS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
