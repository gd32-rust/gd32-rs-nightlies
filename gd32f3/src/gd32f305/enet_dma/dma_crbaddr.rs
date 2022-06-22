#[doc = "Register `DMA_CRBADDR` reader"]
pub struct R(crate::R<DMA_CRBADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CRBADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CRBADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CRBADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RBAP` reader - receive buffer address pointer"]
pub type RBAP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - receive buffer address pointer"]
    #[inline(always)]
    pub fn rbap(&self) -> RBAP_R {
        RBAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current receive buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_crbaddr](index.html) module"]
pub struct DMA_CRBADDR_SPEC;
impl crate::RegisterSpec for DMA_CRBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_crbaddr::R](R) reader structure"]
impl crate::Readable for DMA_CRBADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_CRBADDR to value 0"]
impl crate::Resettable for DMA_CRBADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
