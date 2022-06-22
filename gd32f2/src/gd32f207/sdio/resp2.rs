#[doc = "Register `RESP2` reader"]
pub struct R(crate::R<RESP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESP2` reader - Card state"]
pub type RESP2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Card state"]
    #[inline(always)]
    pub fn resp2(&self) -> RESP2_R {
        RESP2_R::new(self.bits)
    }
}
#[doc = "Response register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp2](index.html) module"]
pub struct RESP2_SPEC;
impl crate::RegisterSpec for RESP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp2::R](R) reader structure"]
impl crate::Readable for RESP2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP2 to value 0"]
impl crate::Resettable for RESP2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
