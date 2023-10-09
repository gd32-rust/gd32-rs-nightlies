#[doc = "Register `DMA_TPEN` reader"]
pub type R = crate::R<DMA_TPEN_SPEC>;
#[doc = "Register `DMA_TPEN` writer"]
pub type W = crate::W<DMA_TPEN_SPEC>;
#[doc = "Field `TPE` reader - Transmit poll enable"]
pub type TPE_R = crate::FieldReader<u32>;
#[doc = "Field `TPE` writer - Transmit poll enable"]
pub type TPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
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
    #[must_use]
    pub fn tpe(&mut self) -> TPE_W<DMA_TPEN_SPEC, 0> {
        TPE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet DMA transmit poll enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_tpen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_tpen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_TPEN_SPEC;
impl crate::RegisterSpec for DMA_TPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_tpen::R`](R) reader structure"]
impl crate::Readable for DMA_TPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_tpen::W`](W) writer structure"]
impl crate::Writable for DMA_TPEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_TPEN to value 0"]
impl crate::Resettable for DMA_TPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
