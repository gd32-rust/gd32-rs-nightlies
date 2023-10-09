#[doc = "Register `IDH` reader"]
pub type R = crate::R<IDH_SPEC>;
#[doc = "Field `SQPI_IDH` reader - ID High Data saved for SQPI read ID command"]
pub type SQPI_IDH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID High Data saved for SQPI read ID command"]
    #[inline(always)]
    pub fn sqpi_idh(&self) -> SQPI_IDH_R {
        SQPI_IDH_R::new(self.bits)
    }
}
#[doc = "ID High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDH_SPEC;
impl crate::RegisterSpec for IDH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idh::R`](R) reader structure"]
impl crate::Readable for IDH_SPEC {}
#[doc = "`reset()` method sets IDH to value 0"]
impl crate::Resettable for IDH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
