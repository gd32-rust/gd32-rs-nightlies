#[doc = "Register `DO4` reader"]
pub struct R(crate::R<DO4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DO4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DO4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DO4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DO4` reader - message digest result of hash algorithm"]
pub type DO4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do4(&self) -> DO4_R {
        DO4_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [do4](index.html) module"]
pub struct DO4_SPEC;
impl crate::RegisterSpec for DO4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [do4::R](R) reader structure"]
impl crate::Readable for DO4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DO4 to value 0"]
impl crate::Resettable for DO4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
