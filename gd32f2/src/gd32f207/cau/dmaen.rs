#[doc = "Register `DMAEN` reader"]
pub type R = crate::R<DMAEN_SPEC>;
#[doc = "Register `DMAEN` writer"]
pub type W = crate::W<DMAEN_SPEC>;
#[doc = "Field `DMAIEN` reader - In FIFO DMA enable"]
pub type DMAIEN_R = crate::BitReader;
#[doc = "Field `DMAIEN` writer - In FIFO DMA enable"]
pub type DMAIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAOEN` reader - Out FIFO DMA enable"]
pub type DMAOEN_R = crate::BitReader;
#[doc = "Field `DMAOEN` writer - Out FIFO DMA enable"]
pub type DMAOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - In FIFO DMA enable"]
    #[inline(always)]
    pub fn dmaien(&self) -> DMAIEN_R {
        DMAIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Out FIFO DMA enable"]
    #[inline(always)]
    pub fn dmaoen(&self) -> DMAOEN_R {
        DMAOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - In FIFO DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaien(&mut self) -> DMAIEN_W<DMAEN_SPEC, 0> {
        DMAIEN_W::new(self)
    }
    #[doc = "Bit 1 - Out FIFO DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaoen(&mut self) -> DMAOEN_W<DMAEN_SPEC, 1> {
        DMAOEN_W::new(self)
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
#[doc = "CAU DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAEN_SPEC;
impl crate::RegisterSpec for DMAEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaen::R`](R) reader structure"]
impl crate::Readable for DMAEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaen::W`](W) writer structure"]
impl crate::Writable for DMAEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAEN to value 0"]
impl crate::Resettable for DMAEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
