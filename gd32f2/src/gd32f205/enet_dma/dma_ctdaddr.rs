#[doc = "Register `DMA_CTDADDR` reader"]
pub type R = crate::R<DMA_CTDADDR_SPEC>;
#[doc = "Field `TDAP` reader - transmit descriptor address pointer"]
pub type TDAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - transmit descriptor address pointer"]
    #[inline(always)]
    pub fn tdap(&self) -> TDAP_R {
        TDAP_R::new(self.bits)
    }
}
#[doc = "DMA current transmit descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ctdaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CTDADDR_SPEC;
impl crate::RegisterSpec for DMA_CTDADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ctdaddr::R`](R) reader structure"]
impl crate::Readable for DMA_CTDADDR_SPEC {}
#[doc = "`reset()` method sets DMA_CTDADDR to value 0"]
impl crate::Resettable for DMA_CTDADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
