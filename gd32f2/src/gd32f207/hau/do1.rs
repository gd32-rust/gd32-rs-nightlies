#[doc = "Register `DO1` reader"]
pub struct R(crate::R<DO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DO1` reader - message digest result of hash algorithm"]
pub type DO1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do1(&self) -> DO1_R {
        DO1_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [do1](index.html) module"]
pub struct DO1_SPEC;
impl crate::RegisterSpec for DO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [do1::R](R) reader structure"]
impl crate::Readable for DO1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DO1 to value 0"]
impl crate::Resettable for DO1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
