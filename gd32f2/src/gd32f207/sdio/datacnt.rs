#[doc = "Register `DATACNT` reader"]
pub struct R(crate::R<DATACNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATACNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATACNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATACNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATACNT` reader - Data count value"]
pub type DATACNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn datacnt(&self) -> DATACNT_R {
        DATACNT_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "Data counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datacnt](index.html) module"]
pub struct DATACNT_SPEC;
impl crate::RegisterSpec for DATACNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datacnt::R](R) reader structure"]
impl crate::Readable for DATACNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATACNT to value 0"]
impl crate::Resettable for DATACNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
