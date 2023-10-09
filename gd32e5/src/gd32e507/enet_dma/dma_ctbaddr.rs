#[doc = "Register `DMA_CTBADDR` reader"]
pub type R = crate::R<DMA_CTBADDR_SPEC>;
#[doc = "Field `TBAP` reader - Transmit buffer address pointer"]
pub type TBAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit buffer address pointer"]
    #[inline(always)]
    pub fn tbap(&self) -> TBAP_R {
        TBAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current transmit buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ctbaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CTBADDR_SPEC;
impl crate::RegisterSpec for DMA_CTBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ctbaddr::R`](R) reader structure"]
impl crate::Readable for DMA_CTBADDR_SPEC {}
#[doc = "`reset()` method sets DMA_CTBADDR to value 0"]
impl crate::Resettable for DMA_CTBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
