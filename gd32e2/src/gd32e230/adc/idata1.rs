#[doc = "Register `IDATA1` reader"]
pub type R = crate::R<Idata1Spec>;
#[doc = "Field `IDATAn` reader - Injected data"]
pub type IdatanR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn idatan(&self) -> IdatanR {
        IdatanR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idata1Spec;
impl crate::RegisterSpec for Idata1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idata1::R`](R) reader structure"]
impl crate::Readable for Idata1Spec {}
#[doc = "`reset()` method sets IDATA1 to value 0"]
impl crate::Resettable for Idata1Spec {
    const RESET_VALUE: u32 = 0;
}
