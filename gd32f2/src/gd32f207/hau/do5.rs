#[doc = "Register `DO5` reader"]
pub struct R(crate::R<DO5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DO5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DO5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DO5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DO5` reader - message digest result of hash algorithm"]
pub type DO5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do5(&self) -> DO5_R {
        DO5_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [do5](index.html) module"]
pub struct DO5_SPEC;
impl crate::RegisterSpec for DO5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [do5::R](R) reader structure"]
impl crate::Readable for DO5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DO5 to value 0"]
impl crate::Resettable for DO5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
