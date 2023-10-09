#[doc = "Register `DMA_RSWDC` reader"]
pub type R = crate::R<DMA_RSWDC_SPEC>;
#[doc = "Register `DMA_RSWDC` writer"]
pub type W = crate::W<DMA_RSWDC_SPEC>;
#[doc = "Field `WDCFRS` reader - Watchdog counter for receive status (RS)"]
pub type WDCFRS_R = crate::FieldReader;
#[doc = "Field `WDCFRS` writer - Watchdog counter for receive status (RS)"]
pub type WDCFRS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Watchdog counter for receive status (RS)"]
    #[inline(always)]
    pub fn wdcfrs(&self) -> WDCFRS_R {
        WDCFRS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog counter for receive status (RS)"]
    #[inline(always)]
    #[must_use]
    pub fn wdcfrs(&mut self) -> WDCFRS_W<DMA_RSWDC_SPEC, 0> {
        WDCFRS_W::new(self)
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
#[doc = "Ethernet DMA receive state watchdog counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rswdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_rswdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_RSWDC_SPEC;
impl crate::RegisterSpec for DMA_RSWDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rswdc::R`](R) reader structure"]
impl crate::Readable for DMA_RSWDC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_rswdc::W`](W) writer structure"]
impl crate::Writable for DMA_RSWDC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_RSWDC to value 0"]
impl crate::Resettable for DMA_RSWDC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
