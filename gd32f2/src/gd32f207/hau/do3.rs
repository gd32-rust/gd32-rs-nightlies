#[doc = "Register `DO3` reader"]
pub struct R(crate::R<DO3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DO3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DO3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DO3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DO3` reader - message digest result of hash algorithm"]
pub type DO3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - message digest result of hash algorithm"]
    #[inline(always)]
    pub fn do3(&self) -> DO3_R {
        DO3_R::new(self.bits)
    }
}
#[doc = "HAU data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [do3](index.html) module"]
pub struct DO3_SPEC;
impl crate::RegisterSpec for DO3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [do3::R](R) reader structure"]
impl crate::Readable for DO3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DO3 to value 0"]
impl crate::Resettable for DO3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
