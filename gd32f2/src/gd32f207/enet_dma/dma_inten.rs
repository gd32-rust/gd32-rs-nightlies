#[doc = "Register `DMA_INTEN` reader"]
pub type R = crate::R<DMA_INTEN_SPEC>;
#[doc = "Register `DMA_INTEN` writer"]
pub type W = crate::W<DMA_INTEN_SPEC>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPSIE` reader - Transmit process stopped interrupt enable"]
pub type TPSIE_R = crate::BitReader;
#[doc = "Field `TPSIE` writer - Transmit process stopped interrupt enable"]
pub type TPSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TBUIE` reader - Transmit buffer unavailable interrupt enable"]
pub type TBUIE_R = crate::BitReader;
#[doc = "Field `TBUIE` writer - Transmit buffer unavailable interrupt enable"]
pub type TBUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TJTIE` reader - Transmit jabber timeout interrupt enable"]
pub type TJTIE_R = crate::BitReader;
#[doc = "Field `TJTIE` writer - Transmit jabber timeout interrupt enable"]
pub type TJTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROIE` reader - Receive overflow interrupt enable"]
pub type ROIE_R = crate::BitReader;
#[doc = "Field `ROIE` writer - Receive overflow interrupt enable"]
pub type ROIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TUIE` reader - Transmit underflow interrupt enable"]
pub type TUIE_R = crate::BitReader;
#[doc = "Field `TUIE` writer - Transmit underflow interrupt enable"]
pub type TUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBUIE` reader - Receive buffer unavailable interrupt enable"]
pub type RBUIE_R = crate::BitReader;
#[doc = "Field `RBUIE` writer - Receive buffer unavailable interrupt enable"]
pub type RBUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RPSIE` reader - Receive process stopped interrupt enable"]
pub type RPSIE_R = crate::BitReader;
#[doc = "Field `RPSIE` writer - Receive process stopped interrupt enable"]
pub type RPSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWTIE` reader - receive watchdog timeout interrupt enable"]
pub type RWTIE_R = crate::BitReader;
#[doc = "Field `RWTIE` writer - receive watchdog timeout interrupt enable"]
pub type RWTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETIE` reader - Early transmit interrupt enable"]
pub type ETIE_R = crate::BitReader;
#[doc = "Field `ETIE` writer - Early transmit interrupt enable"]
pub type ETIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBEIE` reader - Fatal bus error interrupt enable"]
pub type FBEIE_R = crate::BitReader;
#[doc = "Field `FBEIE` writer - Fatal bus error interrupt enable"]
pub type FBEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERIE` reader - Early receive interrupt enable"]
pub type ERIE_R = crate::BitReader;
#[doc = "Field `ERIE` writer - Early receive interrupt enable"]
pub type ERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AIE` reader - Abnormal interrupt summary enable"]
pub type AIE_R = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal interrupt summary enable"]
pub type AIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NIE` reader - Normal interrupt summary enable"]
pub type NIE_R = crate::BitReader;
#[doc = "Field `NIE` writer - Normal interrupt summary enable"]
pub type NIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped interrupt enable"]
    #[inline(always)]
    pub fn tpsie(&self) -> TPSIE_R {
        TPSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn tbuie(&self) -> TBUIE_R {
        TBUIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout interrupt enable"]
    #[inline(always)]
    pub fn tjtie(&self) -> TJTIE_R {
        TJTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive overflow interrupt enable"]
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow interrupt enable"]
    #[inline(always)]
    pub fn tuie(&self) -> TUIE_R {
        TUIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn rbuie(&self) -> RBUIE_R {
        RBUIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped interrupt enable"]
    #[inline(always)]
    pub fn rpsie(&self) -> RPSIE_R {
        RPSIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - receive watchdog timeout interrupt enable"]
    #[inline(always)]
    pub fn rwtie(&self) -> RWTIE_R {
        RWTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt enable"]
    #[inline(always)]
    pub fn fbeie(&self) -> FBEIE_R {
        FBEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary enable"]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<DMA_INTEN_SPEC, 0> {
        TIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit process stopped interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpsie(&mut self) -> TPSIE_W<DMA_INTEN_SPEC, 1> {
        TPSIE_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbuie(&mut self) -> TBUIE_W<DMA_INTEN_SPEC, 2> {
        TBUIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit jabber timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tjtie(&mut self) -> TJTIE_W<DMA_INTEN_SPEC, 3> {
        TJTIE_W::new(self)
    }
    #[doc = "Bit 4 - Receive overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn roie(&mut self) -> ROIE_W<DMA_INTEN_SPEC, 4> {
        ROIE_W::new(self)
    }
    #[doc = "Bit 5 - Transmit underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuie(&mut self) -> TUIE_W<DMA_INTEN_SPEC, 5> {
        TUIE_W::new(self)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<DMA_INTEN_SPEC, 6> {
        RIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive buffer unavailable interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbuie(&mut self) -> RBUIE_W<DMA_INTEN_SPEC, 7> {
        RBUIE_W::new(self)
    }
    #[doc = "Bit 8 - Receive process stopped interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpsie(&mut self) -> RPSIE_W<DMA_INTEN_SPEC, 8> {
        RPSIE_W::new(self)
    }
    #[doc = "Bit 9 - receive watchdog timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwtie(&mut self) -> RWTIE_W<DMA_INTEN_SPEC, 9> {
        RWTIE_W::new(self)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn etie(&mut self) -> ETIE_W<DMA_INTEN_SPEC, 10> {
        ETIE_W::new(self)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbeie(&mut self) -> FBEIE_W<DMA_INTEN_SPEC, 13> {
        FBEIE_W::new(self)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<DMA_INTEN_SPEC, 14> {
        ERIE_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AIE_W<DMA_INTEN_SPEC, 15> {
        AIE_W::new(self)
    }
    #[doc = "Bit 16 - Normal interrupt summary enable"]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NIE_W<DMA_INTEN_SPEC, 16> {
        NIE_W::new(self)
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
#[doc = "Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INTEN_SPEC;
impl crate::RegisterSpec for DMA_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_inten::R`](R) reader structure"]
impl crate::Readable for DMA_INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_inten::W`](W) writer structure"]
impl crate::Writable for DMA_INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_INTEN to value 0"]
impl crate::Resettable for DMA_INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
