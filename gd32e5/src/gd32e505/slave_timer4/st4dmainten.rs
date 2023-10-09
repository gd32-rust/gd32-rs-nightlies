#[doc = "Register `ST4DMAINTEN` reader"]
pub type R = crate::R<ST4DMAINTEN_SPEC>;
#[doc = "Register `ST4DMAINTEN` writer"]
pub type W = crate::W<ST4DMAINTEN_SPEC>;
#[doc = "Field `CMP0IE` reader - Compare 0 interrupt enable"]
pub type CMP0IE_R = crate::BitReader;
#[doc = "Field `CMP0IE` writer - Compare 0 interrupt enable"]
pub type CMP0IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1IE` reader - Compare 1 interrupt enable"]
pub type CMP1IE_R = crate::BitReader;
#[doc = "Field `CMP1IE` writer - Compare 1 interrupt enable"]
pub type CMP1IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP2IE` reader - Compare 2 interrupt enable"]
pub type CMP2IE_R = crate::BitReader;
#[doc = "Field `CMP2IE` writer - Compare 2 interrupt enable"]
pub type CMP2IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP3IE` reader - Compare 3 interrupt enable"]
pub type CMP3IE_R = crate::BitReader;
#[doc = "Field `CMP3IE` writer - Compare 3 interrupt enable"]
pub type CMP3IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REPIE` reader - Repetition interrupt enable"]
pub type REPIE_R = crate::BitReader;
#[doc = "Field `REPIE` writer - Repetition interrupt enable"]
pub type REPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub type UPIE_R = crate::BitReader;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP0IE` reader - Capture 0 interrupt enable"]
pub type CAP0IE_R = crate::BitReader;
#[doc = "Field `CAP0IE` writer - Capture 0 interrupt enable"]
pub type CAP0IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP1IE` reader - Capture 1 interrupt enable"]
pub type CAP1IE_R = crate::BitReader;
#[doc = "Field `CAP1IE` writer - Capture 1 interrupt enable"]
pub type CAP1IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0OAIE` reader - Channel 0 output active interrupt enable"]
pub type CH0OAIE_R = crate::BitReader;
#[doc = "Field `CH0OAIE` writer - Channel 0 output active interrupt enable"]
pub type CH0OAIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0ONAIE` reader - Channel 0 output inactive interrupt enable"]
pub type CH0ONAIE_R = crate::BitReader;
#[doc = "Field `CH0ONAIE` writer - Channel 0 output inactive interrupt enable"]
pub type CH0ONAIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1OAIE` reader - Channel 1 output active interrupt enable"]
pub type CH1OAIE_R = crate::BitReader;
#[doc = "Field `CH1OAIE` writer - Channel 1 output active interrupt enable"]
pub type CH1OAIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1ONAIE` reader - Channel 1 output inactive interrupt enable"]
pub type CH1ONAIE_R = crate::BitReader;
#[doc = "Field `CH1ONAIE` writer - Channel 1 output inactive interrupt enable"]
pub type CH1ONAIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIE` reader - Counter reset interrupt enable"]
pub type RSTIE_R = crate::BitReader;
#[doc = "Field `RSTIE` writer - Counter reset interrupt enable"]
pub type RSTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLYIIE` reader - Delayed IDLE mode entry interrupt enable"]
pub type DLYIIE_R = crate::BitReader;
#[doc = "Field `DLYIIE` writer - Delayed IDLE mode entry interrupt enable"]
pub type DLYIIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP0DEN` reader - Compare 0 DMA request enable"]
pub type CMP0DEN_R = crate::BitReader;
#[doc = "Field `CMP0DEN` writer - Compare 0 DMA request enable"]
pub type CMP0DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1DEN` reader - Compare 1 DMA request enable"]
pub type CMP1DEN_R = crate::BitReader;
#[doc = "Field `CMP1DEN` writer - Compare 1 DMA request enable"]
pub type CMP1DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP2DEN` reader - Compare 2 DMA request enable"]
pub type CMP2DEN_R = crate::BitReader;
#[doc = "Field `CMP2DEN` writer - Compare 2 DMA request enable"]
pub type CMP2DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP3DEN` reader - Compare 3 DMA request enable"]
pub type CMP3DEN_R = crate::BitReader;
#[doc = "Field `CMP3DEN` writer - Compare 3 DMA request enable"]
pub type CMP3DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REPDEN` reader - Repetition DMA request enable"]
pub type REPDEN_R = crate::BitReader;
#[doc = "Field `REPDEN` writer - Repetition DMA request enable"]
pub type REPDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UPDEN_R = crate::BitReader;
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UPDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP0DEN` reader - Capture 0 DMA request enable"]
pub type CAP0DEN_R = crate::BitReader;
#[doc = "Field `CAP0DEN` writer - Capture 0 DMA request enable"]
pub type CAP0DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP1DEN` reader - Capture 1 DMA request enable"]
pub type CAP1DEN_R = crate::BitReader;
#[doc = "Field `CAP1DEN` writer - Capture 1 DMA request enable"]
pub type CAP1DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0ADEN` reader - Channel 0 output active DMA request enable"]
pub type CH0ADEN_R = crate::BitReader;
#[doc = "Field `CH0ADEN` writer - Channel 0 output active DMA request enable"]
pub type CH0ADEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0ONADEN` reader - Channel 0 output inactive DMA request enable"]
pub type CH0ONADEN_R = crate::BitReader;
#[doc = "Field `CH0ONADEN` writer - Channel 0 output inactive DMA request enable"]
pub type CH0ONADEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1OADEN` reader - Channel 1 output active DMA request enable"]
pub type CH1OADEN_R = crate::BitReader;
#[doc = "Field `CH1OADEN` writer - Channel 1 output active DMA request enable"]
pub type CH1OADEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1ONADEN` reader - Channel 1 output inactive DMA request enable"]
pub type CH1ONADEN_R = crate::BitReader;
#[doc = "Field `CH1ONADEN` writer - Channel 1 output inactive DMA request enable"]
pub type CH1ONADEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTDEN` reader - Counter reset DMA request enable"]
pub type RSTDEN_R = crate::BitReader;
#[doc = "Field `RSTDEN` writer - Counter reset DMA request enable"]
pub type RSTDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLYIDEN` reader - Delayed IDLE mode entry DMA request enable"]
pub type DLYIDEN_R = crate::BitReader;
#[doc = "Field `DLYIDEN` writer - Delayed IDLE mode entry DMA request enable"]
pub type DLYIDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Compare 0 interrupt enable"]
    #[inline(always)]
    pub fn cmp0ie(&self) -> CMP0IE_R {
        CMP0IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cmp1ie(&self) -> CMP1IE_R {
        CMP1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMP2IE_R {
        CMP2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn cmp3ie(&self) -> CMP3IE_R {
        CMP3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repetition interrupt enable"]
    #[inline(always)]
    pub fn repie(&self) -> REPIE_R {
        REPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture 0 interrupt enable"]
    #[inline(always)]
    pub fn cap0ie(&self) -> CAP0IE_R {
        CAP0IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture 1 interrupt enable"]
    #[inline(always)]
    pub fn cap1ie(&self) -> CAP1IE_R {
        CAP1IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 output active interrupt enable"]
    #[inline(always)]
    pub fn ch0oaie(&self) -> CH0OAIE_R {
        CH0OAIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 0 output inactive interrupt enable"]
    #[inline(always)]
    pub fn ch0onaie(&self) -> CH0ONAIE_R {
        CH0ONAIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 1 output active interrupt enable"]
    #[inline(always)]
    pub fn ch1oaie(&self) -> CH1OAIE_R {
        CH1OAIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 1 output inactive interrupt enable"]
    #[inline(always)]
    pub fn ch1onaie(&self) -> CH1ONAIE_R {
        CH1ONAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Counter reset interrupt enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Delayed IDLE mode entry interrupt enable"]
    #[inline(always)]
    pub fn dlyiie(&self) -> DLYIIE_R {
        DLYIIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Compare 0 DMA request enable"]
    #[inline(always)]
    pub fn cmp0den(&self) -> CMP0DEN_R {
        CMP0DEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cmp1den(&self) -> CMP1DEN_R {
        CMP1DEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cmp2den(&self) -> CMP2DEN_R {
        CMP2DEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn cmp3den(&self) -> CMP3DEN_R {
        CMP3DEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Repetition DMA request enable"]
    #[inline(always)]
    pub fn repden(&self) -> REPDEN_R {
        REPDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UPDEN_R {
        UPDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Capture 0 DMA request enable"]
    #[inline(always)]
    pub fn cap0den(&self) -> CAP0DEN_R {
        CAP0DEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Capture 1 DMA request enable"]
    #[inline(always)]
    pub fn cap1den(&self) -> CAP1DEN_R {
        CAP1DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 0 output active DMA request enable"]
    #[inline(always)]
    pub fn ch0aden(&self) -> CH0ADEN_R {
        CH0ADEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 0 output inactive DMA request enable"]
    #[inline(always)]
    pub fn ch0onaden(&self) -> CH0ONADEN_R {
        CH0ONADEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 1 output active DMA request enable"]
    #[inline(always)]
    pub fn ch1oaden(&self) -> CH1OADEN_R {
        CH1OADEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Channel 1 output inactive DMA request enable"]
    #[inline(always)]
    pub fn ch1onaden(&self) -> CH1ONADEN_R {
        CH1ONADEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Counter reset DMA request enable"]
    #[inline(always)]
    pub fn rstden(&self) -> RSTDEN_R {
        RSTDEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Delayed IDLE mode entry DMA request enable"]
    #[inline(always)]
    pub fn dlyiden(&self) -> DLYIDEN_R {
        DLYIDEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0ie(&mut self) -> CMP0IE_W<ST4DMAINTEN_SPEC, 0> {
        CMP0IE_W::new(self)
    }
    #[doc = "Bit 1 - Compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ie(&mut self) -> CMP1IE_W<ST4DMAINTEN_SPEC, 1> {
        CMP1IE_W::new(self)
    }
    #[doc = "Bit 2 - Compare 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ie(&mut self) -> CMP2IE_W<ST4DMAINTEN_SPEC, 2> {
        CMP2IE_W::new(self)
    }
    #[doc = "Bit 3 - Compare 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3ie(&mut self) -> CMP3IE_W<ST4DMAINTEN_SPEC, 3> {
        CMP3IE_W::new(self)
    }
    #[doc = "Bit 4 - Repetition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn repie(&mut self) -> REPIE_W<ST4DMAINTEN_SPEC, 4> {
        REPIE_W::new(self)
    }
    #[doc = "Bit 6 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<ST4DMAINTEN_SPEC, 6> {
        UPIE_W::new(self)
    }
    #[doc = "Bit 7 - Capture 0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap0ie(&mut self) -> CAP0IE_W<ST4DMAINTEN_SPEC, 7> {
        CAP0IE_W::new(self)
    }
    #[doc = "Bit 8 - Capture 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap1ie(&mut self) -> CAP1IE_W<ST4DMAINTEN_SPEC, 8> {
        CAP1IE_W::new(self)
    }
    #[doc = "Bit 9 - Channel 0 output active interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0oaie(&mut self) -> CH0OAIE_W<ST4DMAINTEN_SPEC, 9> {
        CH0OAIE_W::new(self)
    }
    #[doc = "Bit 10 - Channel 0 output inactive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0onaie(&mut self) -> CH0ONAIE_W<ST4DMAINTEN_SPEC, 10> {
        CH0ONAIE_W::new(self)
    }
    #[doc = "Bit 11 - Channel 1 output active interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1oaie(&mut self) -> CH1OAIE_W<ST4DMAINTEN_SPEC, 11> {
        CH1OAIE_W::new(self)
    }
    #[doc = "Bit 12 - Channel 1 output inactive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1onaie(&mut self) -> CH1ONAIE_W<ST4DMAINTEN_SPEC, 12> {
        CH1ONAIE_W::new(self)
    }
    #[doc = "Bit 13 - Counter reset interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstie(&mut self) -> RSTIE_W<ST4DMAINTEN_SPEC, 13> {
        RSTIE_W::new(self)
    }
    #[doc = "Bit 14 - Delayed IDLE mode entry interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlyiie(&mut self) -> DLYIIE_W<ST4DMAINTEN_SPEC, 14> {
        DLYIIE_W::new(self)
    }
    #[doc = "Bit 16 - Compare 0 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0den(&mut self) -> CMP0DEN_W<ST4DMAINTEN_SPEC, 16> {
        CMP0DEN_W::new(self)
    }
    #[doc = "Bit 17 - Compare 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1den(&mut self) -> CMP1DEN_W<ST4DMAINTEN_SPEC, 17> {
        CMP1DEN_W::new(self)
    }
    #[doc = "Bit 18 - Compare 2 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2den(&mut self) -> CMP2DEN_W<ST4DMAINTEN_SPEC, 18> {
        CMP2DEN_W::new(self)
    }
    #[doc = "Bit 19 - Compare 3 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3den(&mut self) -> CMP3DEN_W<ST4DMAINTEN_SPEC, 19> {
        CMP3DEN_W::new(self)
    }
    #[doc = "Bit 20 - Repetition DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn repden(&mut self) -> REPDEN_W<ST4DMAINTEN_SPEC, 20> {
        REPDEN_W::new(self)
    }
    #[doc = "Bit 22 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn upden(&mut self) -> UPDEN_W<ST4DMAINTEN_SPEC, 22> {
        UPDEN_W::new(self)
    }
    #[doc = "Bit 23 - Capture 0 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap0den(&mut self) -> CAP0DEN_W<ST4DMAINTEN_SPEC, 23> {
        CAP0DEN_W::new(self)
    }
    #[doc = "Bit 24 - Capture 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap1den(&mut self) -> CAP1DEN_W<ST4DMAINTEN_SPEC, 24> {
        CAP1DEN_W::new(self)
    }
    #[doc = "Bit 25 - Channel 0 output active DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0aden(&mut self) -> CH0ADEN_W<ST4DMAINTEN_SPEC, 25> {
        CH0ADEN_W::new(self)
    }
    #[doc = "Bit 26 - Channel 0 output inactive DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0onaden(&mut self) -> CH0ONADEN_W<ST4DMAINTEN_SPEC, 26> {
        CH0ONADEN_W::new(self)
    }
    #[doc = "Bit 27 - Channel 1 output active DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1oaden(&mut self) -> CH1OADEN_W<ST4DMAINTEN_SPEC, 27> {
        CH1OADEN_W::new(self)
    }
    #[doc = "Bit 28 - Channel 1 output inactive DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1onaden(&mut self) -> CH1ONADEN_W<ST4DMAINTEN_SPEC, 28> {
        CH1ONADEN_W::new(self)
    }
    #[doc = "Bit 29 - Counter reset DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstden(&mut self) -> RSTDEN_W<ST4DMAINTEN_SPEC, 29> {
        RSTDEN_W::new(self)
    }
    #[doc = "Bit 30 - Delayed IDLE mode entry DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlyiden(&mut self) -> DLYIDEN_W<ST4DMAINTEN_SPEC, 30> {
        DLYIDEN_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx DMA and interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4dmainten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4dmainten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST4DMAINTEN_SPEC;
impl crate::RegisterSpec for ST4DMAINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st4dmainten::R`](R) reader structure"]
impl crate::Readable for ST4DMAINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st4dmainten::W`](W) writer structure"]
impl crate::Writable for ST4DMAINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST4DMAINTEN to value 0"]
impl crate::Resettable for ST4DMAINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
