#[doc = "Register `RDATA` reader"]
pub type R = crate::R<RDATA_SPEC>;
#[doc = "Field `RDATA` reader - Receive data value"]
pub type RDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Receive data value"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
