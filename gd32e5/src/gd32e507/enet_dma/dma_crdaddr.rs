#[doc = "Register `DMA_CRDADDR` reader"]
pub type R = crate::R<DMA_CRDADDR_SPEC>;
#[doc = "Field `RDAP` reader - Receive descriptor address pointer"]
pub type RDAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive descriptor address pointer"]
    #[inline(always)]
    pub fn rdap(&self) -> RDAP_R {
        RDAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current receive descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_crdaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CRDADDR_SPEC;
impl crate::RegisterSpec for DMA_CRDADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_crdaddr::R`](R) reader structure"]
impl crate::Readable for DMA_CRDADDR_SPEC {}
#[doc = "`reset()` method sets DMA_CRDADDR to value 0"]
impl crate::Resettable for DMA_CRDADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
