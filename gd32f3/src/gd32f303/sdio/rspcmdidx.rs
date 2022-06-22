#[doc = "Register `RSPCMDIDX` reader"]
pub struct R(crate::R<RSPCMDIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSPCMDIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSPCMDIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSPCMDIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSPCMDIDX` reader - Last response command index"]
pub type RSPCMDIDX_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Last response command index"]
    #[inline(always)]
    pub fn rspcmdidx(&self) -> RSPCMDIDX_R {
        RSPCMDIDX_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Command index response register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rspcmdidx](index.html) module"]
pub struct RSPCMDIDX_SPEC;
impl crate::RegisterSpec for RSPCMDIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rspcmdidx::R](R) reader structure"]
impl crate::Readable for RSPCMDIDX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSPCMDIDX to value 0"]
impl crate::Resettable for RSPCMDIDX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
