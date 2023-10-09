#[doc = "Register `DIEP0DMAADDR` reader"]
pub type R = crate::R<DIEP0DMAADDR_SPEC>;
#[doc = "Register `DIEP0DMAADDR` writer"]
pub type W = crate::W<DIEP0DMAADDR_SPEC>;
#[doc = "Field `DMAADDR` reader - DMA address"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA address"]
pub type DMAADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DIEP0DMAADDR_SPEC, 0> {
        DMAADDR_W::new(self)
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
#[doc = "Device IN endpoint 0 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0dmaaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0dmaaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP0DMAADDR_SPEC;
impl crate::RegisterSpec for DIEP0DMAADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0dmaaddr::R`](R) reader structure"]
impl crate::Readable for DIEP0DMAADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep0dmaaddr::W`](W) writer structure"]
impl crate::Writable for DIEP0DMAADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP0DMAADDR to value 0"]
impl crate::Resettable for DIEP0DMAADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
