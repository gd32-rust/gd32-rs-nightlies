#[doc = "Register `MTDMAINTEN` reader"]
pub type R = crate::R<MTDMAINTEN_SPEC>;
#[doc = "Register `MTDMAINTEN` writer"]
pub type W = crate::W<MTDMAINTEN_SPEC>;
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
#[doc = "Field `SYNIIE` reader - Synchronization input interrupt enable"]
pub type SYNIIE_R = crate::BitReader;
#[doc = "Field `SYNIIE` writer - Synchronization input interrupt enable"]
pub type SYNIIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub type UPIE_R = crate::BitReader;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
#[doc = "Field `SYNIDEN` reader - Synchronization input DMA request enable"]
pub type SYNIDEN_R = crate::BitReader;
#[doc = "Field `SYNIDEN` writer - Synchronization input DMA request enable"]
pub type SYNIDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UPDEN_R = crate::BitReader;
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UPDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 5 - Synchronization input interrupt enable"]
    #[inline(always)]
    pub fn syniie(&self) -> SYNIIE_R {
        SYNIIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 21 - Synchronization input DMA request enable"]
    #[inline(always)]
    pub fn syniden(&self) -> SYNIDEN_R {
        SYNIDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UPDEN_R {
        UPDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0ie(&mut self) -> CMP0IE_W<MTDMAINTEN_SPEC, 0> {
        CMP0IE_W::new(self)
    }
    #[doc = "Bit 1 - Compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ie(&mut self) -> CMP1IE_W<MTDMAINTEN_SPEC, 1> {
        CMP1IE_W::new(self)
    }
    #[doc = "Bit 2 - Compare 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ie(&mut self) -> CMP2IE_W<MTDMAINTEN_SPEC, 2> {
        CMP2IE_W::new(self)
    }
    #[doc = "Bit 3 - Compare 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3ie(&mut self) -> CMP3IE_W<MTDMAINTEN_SPEC, 3> {
        CMP3IE_W::new(self)
    }
    #[doc = "Bit 4 - Repetition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn repie(&mut self) -> REPIE_W<MTDMAINTEN_SPEC, 4> {
        REPIE_W::new(self)
    }
    #[doc = "Bit 5 - Synchronization input interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syniie(&mut self) -> SYNIIE_W<MTDMAINTEN_SPEC, 5> {
        SYNIIE_W::new(self)
    }
    #[doc = "Bit 6 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<MTDMAINTEN_SPEC, 6> {
        UPIE_W::new(self)
    }
    #[doc = "Bit 16 - Compare 0 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0den(&mut self) -> CMP0DEN_W<MTDMAINTEN_SPEC, 16> {
        CMP0DEN_W::new(self)
    }
    #[doc = "Bit 17 - Compare 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1den(&mut self) -> CMP1DEN_W<MTDMAINTEN_SPEC, 17> {
        CMP1DEN_W::new(self)
    }
    #[doc = "Bit 18 - Compare 2 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2den(&mut self) -> CMP2DEN_W<MTDMAINTEN_SPEC, 18> {
        CMP2DEN_W::new(self)
    }
    #[doc = "Bit 19 - Compare 3 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3den(&mut self) -> CMP3DEN_W<MTDMAINTEN_SPEC, 19> {
        CMP3DEN_W::new(self)
    }
    #[doc = "Bit 20 - Repetition DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn repden(&mut self) -> REPDEN_W<MTDMAINTEN_SPEC, 20> {
        REPDEN_W::new(self)
    }
    #[doc = "Bit 21 - Synchronization input DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn syniden(&mut self) -> SYNIDEN_W<MTDMAINTEN_SPEC, 21> {
        SYNIDEN_W::new(self)
    }
    #[doc = "Bit 22 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn upden(&mut self) -> UPDEN_W<MTDMAINTEN_SPEC, 22> {
        UPDEN_W::new(self)
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
#[doc = "SHRTIMER Master_TIMER DMA and interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtdmainten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtdmainten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTDMAINTEN_SPEC;
impl crate::RegisterSpec for MTDMAINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtdmainten::R`](R) reader structure"]
impl crate::Readable for MTDMAINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtdmainten::W`](W) writer structure"]
impl crate::Writable for MTDMAINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTDMAINTEN to value 0"]
impl crate::Resettable for MTDMAINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
