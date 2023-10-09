#[doc = "Register `DMA_CRBADDR` reader"]
pub type R = crate::R<DMA_CRBADDR_SPEC>;
#[doc = "Field `RBAP` reader - receive buffer address pointer"]
pub type RBAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - receive buffer address pointer"]
    #[inline(always)]
    pub fn rbap(&self) -> RBAP_R {
        RBAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current receive buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_crbaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CRBADDR_SPEC;
impl crate::RegisterSpec for DMA_CRBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_crbaddr::R`](R) reader structure"]
impl crate::Readable for DMA_CRBADDR_SPEC {}
#[doc = "`reset()` method sets DMA_CRBADDR to value 0"]
impl crate::Resettable for DMA_CRBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
