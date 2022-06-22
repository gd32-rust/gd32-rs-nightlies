#[doc = "Register `DO2` reader"]
pub struct R(crate::R<DO2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DO2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DO2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DO2` reader - message digest result of hash algorithm"]
pub type DO2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do2(&self) -> DO2_R {
        DO2_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [do2](index.html) module"]
pub struct DO2_SPEC;
impl crate::RegisterSpec for DO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [do2::R](R) reader structure"]
impl crate::Readable for DO2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DO2 to value 0"]
impl crate::Resettable for DO2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
