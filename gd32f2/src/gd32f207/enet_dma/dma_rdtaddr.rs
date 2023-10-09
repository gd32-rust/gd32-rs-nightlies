#[doc = "Register `DMA_RDTADDR` reader"]
pub type R = crate::R<DMA_RDTADDR_SPEC>;
#[doc = "Register `DMA_RDTADDR` writer"]
pub type W = crate::W<DMA_RDTADDR_SPEC>;
#[doc = "Field `SRT` reader - Start address of receive table"]
pub type SRT_R = crate::FieldReader<u32>;
#[doc = "Field `SRT` writer - Start address of receive table"]
pub type SRT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
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
    #[must_use]
    pub fn srt(&mut self) -> SRT_W<DMA_RDTADDR_SPEC, 0> {
        SRT_W::new(self)
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
#[doc = "Ethernet DMA receive descriptor table address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rdtaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_rdtaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_RDTADDR_SPEC;
impl crate::RegisterSpec for DMA_RDTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rdtaddr::R`](R) reader structure"]
impl crate::Readable for DMA_RDTADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_rdtaddr::W`](W) writer structure"]
impl crate::Writable for DMA_RDTADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_RDTADDR to value 0"]
impl crate::Resettable for DMA_RDTADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
