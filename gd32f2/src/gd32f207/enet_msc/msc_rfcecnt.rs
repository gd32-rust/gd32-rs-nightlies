#[doc = "Register `MSC_RFCECNT` reader"]
pub type R = crate::R<MSC_RFCECNT_SPEC>;
#[doc = "Field `RFCER` reader - Received frames with CRC error counter"]
pub type RFCER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames with CRC error counter"]
    #[inline(always)]
    pub fn rfcer(&self) -> RFCER_R {
        RFCER_R::new(self.bits)
    }
}
#[doc = "Ethernet MSC received frames with CRC error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_rfcecnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_RFCECNT_SPEC;
impl crate::RegisterSpec for MSC_RFCECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_rfcecnt::R`](R) reader structure"]
impl crate::Readable for MSC_RFCECNT_SPEC {}
#[doc = "`reset()` method sets MSC_RFCECNT to value 0"]
impl crate::Resettable for MSC_RFCECNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
