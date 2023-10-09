#[doc = "Register `RDATA` reader"]
pub type R = crate::R<RDATA_SPEC>;
#[doc = "Field `RXDATA` reader - CEC Rx Data Register"]
pub type RXDATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CEC Rx Data Register"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Rx Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDATA_SPEC;
impl crate::RegisterSpec for RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdata::R`](R) reader structure"]
impl crate::Readable for RDATA_SPEC {}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
