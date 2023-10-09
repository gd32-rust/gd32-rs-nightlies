#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DMAINTEN_SPEC>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DMAINTEN_SPEC>;
#[doc = "Channel 0 capture/compare interrupt enable"]
pub use crate::gd32e503::timer0::dmainten::CH0IE_A;
#[doc = "Field `CH0IE` reader - Channel 0 capture/compare interrupt enable"]
pub use crate::gd32e503::timer0::dmainten::CH0IE_R;
#[doc = "Field `CH0IE` writer - Channel 0 capture/compare interrupt enable"]
pub use crate::gd32e503::timer0::dmainten::CH0IE_W;
#[doc = "Update interrupt enable"]
pub use crate::gd32e503::timer0::dmainten::UPIE_A;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub use crate::gd32e503::timer0::dmainten::UPIE_R;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub use crate::gd32e503::timer0::dmainten::UPIE_W;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> CH0IE_R {
        CH0IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<DMAINTEN_SPEC, 0> {
        UPIE_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ie(&mut self) -> CH0IE_W<DMAINTEN_SPEC, 1> {
        CH0IE_W::new(self)
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
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmainten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAINTEN_SPEC;
impl crate::RegisterSpec for DMAINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DMAINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DMAINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DMAINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
