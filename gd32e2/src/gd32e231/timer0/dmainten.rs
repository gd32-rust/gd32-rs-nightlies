#[doc = "Register `DMAINTEN` reader"]
pub struct R(crate::R<DMAINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAINTEN` writer"]
pub struct W(crate::W<DMAINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAINTEN_SPEC>;
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
impl From<crate::W<DMAINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGDEN` reader - Trigger DMA request enable"]
pub type TRGDEN_R = crate::BitReader<bool>;
#[doc = "Field `TRGDEN` writer - Trigger DMA request enable"]
pub type TRGDEN_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 14>;
#[doc = "Field `CMTDEN` reader - Reserved"]
pub type CMTDEN_R = crate::BitReader<bool>;
#[doc = "Field `CMTDEN` writer - Reserved"]
pub type CMTDEN_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 13>;
#[doc = "Field `CH3DEN` reader - Capture/Compare 3 DMA request enable"]
pub type CH3DEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3DEN` writer - Capture/Compare 3 DMA request enable"]
pub type CH3DEN_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 12>;
#[doc = "Field `CH2DEN` reader - Capture/Compare 2 DMA request enable"]
pub type CH2DEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2DEN` writer - Capture/Compare 2 DMA request enable"]
pub type CH2DEN_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 11>;
#[doc = "Field `CH1DEN` reader - Capture/Compare 1 DMA request enable"]
pub type CH1DEN_R = crate::BitReader<bool>;
#[doc = "Field `CH1DEN` writer - Capture/Compare 1 DMA request enable"]
pub type CH1DEN_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 10>;
#[doc = "Field `CH0DEN` reader - Capture/Compare 0 DMA request enable"]
pub type CH0DEN_R = crate::BitReader<bool>;
#[doc = "Field `CH0DEN` writer - Capture/Compare 0 DMA request enable"]
pub type CH0DEN_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 9>;
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UPDEN_R = crate::BitReader<bool>;
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UPDEN_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 8>;
#[doc = "Field `BRKIE` reader - Break interrupt enable"]
pub type BRKIE_R = crate::BitReader<bool>;
#[doc = "Field `BRKIE` writer - Break interrupt enable"]
pub type BRKIE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 7>;
#[doc = "Field `TRGIE` reader - Trigger interrupt enable"]
pub type TRGIE_R = crate::BitReader<bool>;
#[doc = "Field `TRGIE` writer - Trigger interrupt enable"]
pub type TRGIE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 6>;
#[doc = "Field `CMTIE` reader - COM interrupt enable"]
pub type CMTIE_R = crate::BitReader<bool>;
#[doc = "Field `CMTIE` writer - COM interrupt enable"]
pub type CMTIE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 5>;
#[doc = "Field `CH3IE` reader - Capture/Compare 3 interrupt enable"]
pub type CH3IE_R = crate::BitReader<bool>;
#[doc = "Field `CH3IE` writer - Capture/Compare 3 interrupt enable"]
pub type CH3IE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 4>;
#[doc = "Field `CH2IE` reader - Capture/Compare 2 interrupt enable"]
pub type CH2IE_R = crate::BitReader<bool>;
#[doc = "Field `CH2IE` writer - Capture/Compare 2 interrupt enable"]
pub type CH2IE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 3>;
#[doc = "Field `CH1IE` reader - Capture/Compare 1 interrupt enable"]
pub type CH1IE_R = crate::BitReader<bool>;
#[doc = "Field `CH1IE` writer - Capture/Compare 1 interrupt enable"]
pub type CH1IE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 2>;
#[doc = "Field `CH0IE` reader - Capture/Compare 0 interrupt enable"]
pub type CH0IE_R = crate::BitReader<bool>;
#[doc = "Field `CH0IE` writer - Capture/Compare 0 interrupt enable"]
pub type CH0IE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 1>;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub type UPIE_R = crate::BitReader<bool>;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UPIE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&self) -> TRGDEN_R {
        TRGDEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn cmtden(&self) -> CMTDEN_R {
        CMTDEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn ch3den(&self) -> CH3DEN_R {
        CH3DEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn ch2den(&self) -> CH2DEN_R {
        CH2DEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&self) -> CH1DEN_R {
        CH1DEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 0 DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&self) -> CH0DEN_R {
        CH0DEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UPDEN_R {
        UPDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkie(&self) -> BRKIE_R {
        BRKIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&self) -> TRGIE_R {
        TRGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&self) -> CMTIE_R {
        CMTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn ch3ie(&self) -> CH3IE_R {
        CH3IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn ch2ie(&self) -> CH2IE_R {
        CH2IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&self) -> CH1IE_R {
        CH1IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 0 interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> CH0IE_R {
        CH0IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&mut self) -> TRGDEN_W {
        TRGDEN_W::new(self)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn cmtden(&mut self) -> CMTDEN_W {
        CMTDEN_W::new(self)
    }
    #[doc = "Bit 12 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn ch3den(&mut self) -> CH3DEN_W {
        CH3DEN_W::new(self)
    }
    #[doc = "Bit 11 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn ch2den(&mut self) -> CH2DEN_W {
        CH2DEN_W::new(self)
    }
    #[doc = "Bit 10 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&mut self) -> CH1DEN_W {
        CH1DEN_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 0 DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&mut self) -> CH0DEN_W {
        CH0DEN_W::new(self)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&mut self) -> UPDEN_W {
        UPDEN_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkie(&mut self) -> BRKIE_W {
        BRKIE_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&mut self) -> TRGIE_W {
        TRGIE_W::new(self)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&mut self) -> CMTIE_W {
        CMTIE_W::new(self)
    }
    #[doc = "Bit 4 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn ch3ie(&mut self) -> CH3IE_W {
        CH3IE_W::new(self)
    }
    #[doc = "Bit 3 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn ch2ie(&mut self) -> CH2IE_W {
        CH2IE_W::new(self)
    }
    #[doc = "Bit 2 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&mut self) -> CH1IE_W {
        CH1IE_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 0 interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&mut self) -> CH0IE_W {
        CH0IE_W::new(self)
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W {
        UPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmainten](index.html) module"]
pub struct DMAINTEN_SPEC;
impl crate::RegisterSpec for DMAINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmainten::R](R) reader structure"]
impl crate::Readable for DMAINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmainten::W](W) writer structure"]
impl crate::Writable for DMAINTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DMAINTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
