#[doc = "Register `DO7` reader"]
pub struct R(crate::R<DO7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DO7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DO7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DO7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DO7` reader - message digest result of hash algorithm"]
pub type DO7_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do7(&self) -> DO7_R {
        DO7_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [do7](index.html) module"]
pub struct DO7_SPEC;
impl crate::RegisterSpec for DO7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [do7::R](R) reader structure"]
impl crate::Readable for DO7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DO7 to value 0"]
impl crate::Resettable for DO7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
