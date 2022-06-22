#[doc = "Register `DMA_TPEN` reader"]
pub struct R(crate::R<DMA_TPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_TPEN` writer"]
pub struct W(crate::W<DMA_TPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_TPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_TPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_TPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPE` reader - Transmit poll enable"]
pub type TPE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TPE` writer - Transmit poll enable"]
pub type TPE_W<'a> = crate::FieldWriter<'a, u32, DMA_TPEN_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Transmit poll enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll enable"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TPE_W {
        TPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA transmit poll enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_tpen](index.html) module"]
pub struct DMA_TPEN_SPEC;
impl crate::RegisterSpec for DMA_TPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_tpen::R](R) reader structure"]
impl crate::Readable for DMA_TPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_tpen::W](W) writer structure"]
impl crate::Writable for DMA_TPEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_TPEN to value 0"]
impl crate::Resettable for DMA_TPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
