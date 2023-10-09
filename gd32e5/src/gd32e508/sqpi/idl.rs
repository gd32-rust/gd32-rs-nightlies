#[doc = "Register `IDL` reader"]
pub type R = crate::R<IDL_SPEC>;
#[doc = "Field `SQPI_IDL` reader - ID Low Data saved for SQPI Read ID Command"]
pub type SQPI_IDL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID Low Data saved for SQPI Read ID Command"]
    #[inline(always)]
    pub fn sqpi_idl(&self) -> SQPI_IDL_R {
        SQPI_IDL_R::new(self.bits)
    }
}
#[doc = "ID Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDL_SPEC;
impl crate::RegisterSpec for IDL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idl::R`](R) reader structure"]
impl crate::Readable for IDL_SPEC {}
#[doc = "`reset()` method sets IDL to value 0"]
impl crate::Resettable for IDL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
