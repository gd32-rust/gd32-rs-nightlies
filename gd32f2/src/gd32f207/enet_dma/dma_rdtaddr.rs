#[doc = "Register `DMA_RDTADDR` reader"]
pub struct R(crate::R<DMA_RDTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RDTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RDTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RDTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RDTADDR` writer"]
pub struct W(crate::W<DMA_RDTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RDTADDR_SPEC>;
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
impl From<crate::W<DMA_RDTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RDTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRT` reader - Start address of receive table"]
pub type SRT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRT` writer - Start address of receive table"]
pub type SRT_W<'a> = crate::FieldWriter<'a, u32, DMA_RDTADDR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Start address of receive table"]
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of receive table"]
    #[inline(always)]
    pub fn srt(&mut self) -> SRT_W {
        SRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA receive descriptor table address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rdtaddr](index.html) module"]
pub struct DMA_RDTADDR_SPEC;
impl crate::RegisterSpec for DMA_RDTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rdtaddr::R](R) reader structure"]
impl crate::Readable for DMA_RDTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_rdtaddr::W](W) writer structure"]
impl crate::Writable for DMA_RDTADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_RDTADDR to value 0"]
impl crate::Resettable for DMA_RDTADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
