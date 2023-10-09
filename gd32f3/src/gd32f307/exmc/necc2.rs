#[doc = "Register `NECC2` reader"]
pub type R = crate::R<NECC2_SPEC>;
#[doc = "Field `ECC` reader - ECC result"]
pub type ECC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
#[doc = "NAND flash ECC register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`necc2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NECC2_SPEC;
impl crate::RegisterSpec for NECC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`necc2::R`](R) reader structure"]
impl crate::Readable for NECC2_SPEC {}
#[doc = "`reset()` method sets NECC2 to value 0"]
impl crate::Resettable for NECC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
