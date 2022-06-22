#[doc = "Register `NECC2` reader"]
pub struct R(crate::R<NECC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NECC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NECC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NECC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC` reader - ECC result"]
pub type ECC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
#[doc = "NAND flash ECC register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [necc2](index.html) module"]
pub struct NECC2_SPEC;
impl crate::RegisterSpec for NECC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [necc2::R](R) reader structure"]
impl crate::Readable for NECC2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NECC2 to value 0"]
impl crate::Resettable for NECC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
