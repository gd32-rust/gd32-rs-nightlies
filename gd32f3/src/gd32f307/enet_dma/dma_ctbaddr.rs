#[doc = "Register `DMA_CTBADDR` reader"]
pub struct R(crate::R<DMA_CTBADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CTBADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CTBADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CTBADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TBAP` reader - Transmit buffer address pointer"]
pub type TBAP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit buffer address pointer"]
    #[inline(always)]
    pub fn tbap(&self) -> TBAP_R {
        TBAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current transmit buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ctbaddr](index.html) module"]
pub struct DMA_CTBADDR_SPEC;
impl crate::RegisterSpec for DMA_CTBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_ctbaddr::R](R) reader structure"]
impl crate::Readable for DMA_CTBADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_CTBADDR to value 0"]
impl crate::Resettable for DMA_CTBADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
