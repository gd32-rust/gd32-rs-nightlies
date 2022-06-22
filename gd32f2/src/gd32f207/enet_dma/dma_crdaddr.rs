#[doc = "Register `DMA_CRDADDR` reader"]
pub struct R(crate::R<DMA_CRDADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CRDADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CRDADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CRDADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDAP` reader - Receive descriptor address pointer"]
pub type RDAP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive descriptor address pointer"]
    #[inline(always)]
    pub fn rdap(&self) -> RDAP_R {
        RDAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current receive descriptor address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_crdaddr](index.html) module"]
pub struct DMA_CRDADDR_SPEC;
impl crate::RegisterSpec for DMA_CRDADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_crdaddr::R](R) reader structure"]
impl crate::Readable for DMA_CRDADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_CRDADDR to value 0"]
impl crate::Resettable for DMA_CRDADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
