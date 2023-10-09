#[doc = "Register `MSC_RFAECNT` reader"]
pub type R = crate::R<MSC_RFAECNT_SPEC>;
#[doc = "Field `RFAER` reader - Received frames with alignment error counter"]
pub type RFAER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames with alignment error counter"]
    #[inline(always)]
    pub fn rfaer(&self) -> RFAER_R {
        RFAER_R::new(self.bits)
    }
}
#[doc = "Ethernet MSC received frames with alignment error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_rfaecnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_RFAECNT_SPEC;
impl crate::RegisterSpec for MSC_RFAECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_rfaecnt::R`](R) reader structure"]
impl crate::Readable for MSC_RFAECNT_SPEC {}
#[doc = "`reset()` method sets MSC_RFAECNT to value 0"]
impl crate::Resettable for MSC_RFAECNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
