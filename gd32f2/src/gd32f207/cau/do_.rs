#[doc = "Register `DO` reader"]
pub struct R(crate::R<DO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DO` reader - Data output"]
pub type DO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data output"]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(self.bits)
    }
}
#[doc = "CAU data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [do_](index.html) module"]
pub struct DO_SPEC;
impl crate::RegisterSpec for DO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [do_::R](R) reader structure"]
impl crate::Readable for DO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DO to value 0"]
impl crate::Resettable for DO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
