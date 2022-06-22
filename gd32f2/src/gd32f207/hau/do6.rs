#[doc = "Register `DO6` reader"]
pub struct R(crate::R<DO6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DO6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DO6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DO6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DO6` reader - message digest result of hash algorithm"]
pub type DO6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do6(&self) -> DO6_R {
        DO6_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [do6](index.html) module"]
pub struct DO6_SPEC;
impl crate::RegisterSpec for DO6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [do6::R](R) reader structure"]
impl crate::Readable for DO6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DO6 to value 0"]
impl crate::Resettable for DO6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
