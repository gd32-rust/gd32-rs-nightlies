#[doc = "Register `CHCTL2` reader"]
pub struct R(crate::R<CHCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTL2` writer"]
pub struct W(crate::W<CHCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTL2_SPEC>;
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
impl From<crate::W<CHCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH3P` reader - Capture/Compare 3 output Polarity"]
pub type CH3P_R = crate::BitReader<bool>;
#[doc = "Field `CH3P` writer - Capture/Compare 3 output Polarity"]
pub type CH3P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 13>;
#[doc = "Field `CH3EN` reader - Capture/Compare 3 output enable"]
pub type CH3EN_R = crate::BitReader<bool>;
#[doc = "Field `CH3EN` writer - Capture/Compare 3 output enable"]
pub type CH3EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 12>;
#[doc = "Field `CH2NP` reader - Capture/Compare 2 output Polarity"]
pub type CH2NP_R = crate::BitReader<bool>;
#[doc = "Field `CH2NP` writer - Capture/Compare 2 output Polarity"]
pub type CH2NP_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 11>;
#[doc = "Field `CH2NEN` reader - Capture/Compare 2 complementary output enable"]
pub type CH2NEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2NEN` writer - Capture/Compare 2 complementary output enable"]
pub type CH2NEN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 10>;
#[doc = "Field `CH2P` reader - Capture/Compare 2 output Polarity"]
pub type CH2P_R = crate::BitReader<bool>;
#[doc = "Field `CH2P` writer - Capture/Compare 2 output Polarity"]
pub type CH2P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 9>;
#[doc = "Field `CH2EN` reader - Capture/Compare 2 output enable"]
pub type CH2EN_R = crate::BitReader<bool>;
#[doc = "Field `CH2EN` writer - Capture/Compare 2 output enable"]
pub type CH2EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 8>;
#[doc = "Field `CH1NP` reader - Capture/Compare 1 output Polarity"]
pub type CH1NP_R = crate::BitReader<bool>;
#[doc = "Field `CH1NP` writer - Capture/Compare 1 output Polarity"]
pub type CH1NP_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 7>;
#[doc = "Field `CH1NEN` reader - Capture/Compare 1 complementary output enable"]
pub type CH1NEN_R = crate::BitReader<bool>;
#[doc = "Field `CH1NEN` writer - Capture/Compare 1 complementary output enable"]
pub type CH1NEN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 6>;
#[doc = "Field `CH1P` reader - Capture/Compare 1 output Polarity"]
pub type CH1P_R = crate::BitReader<bool>;
#[doc = "Field `CH1P` writer - Capture/Compare 1 output Polarity"]
pub type CH1P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 5>;
#[doc = "Field `CH1EN` reader - Capture/Compare 1 output enable"]
pub type CH1EN_R = crate::BitReader<bool>;
#[doc = "Field `CH1EN` writer - Capture/Compare 1 output enable"]
pub type CH1EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 4>;
#[doc = "Field `CH0NP` reader - Capture/Compare 0 output Polarity"]
pub type CH0NP_R = crate::BitReader<bool>;
#[doc = "Field `CH0NP` writer - Capture/Compare 0 output Polarity"]
pub type CH0NP_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 3>;
#[doc = "Field `CH0NEN` reader - Capture/Compare 0 complementary output enable"]
pub type CH0NEN_R = crate::BitReader<bool>;
#[doc = "Field `CH0NEN` writer - Capture/Compare 0 complementary output enable"]
pub type CH0NEN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 2>;
#[doc = "Field `CH0P` reader - Capture/Compare 0 output Polarity"]
pub type CH0P_R = crate::BitReader<bool>;
#[doc = "Field `CH0P` writer - Capture/Compare 0 output Polarity"]
pub type CH0P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 1>;
#[doc = "Field `CH0EN` reader - Capture/Compare 1 output enable"]
pub type CH0EN_R = crate::BitReader<bool>;
#[doc = "Field `CH0EN` writer - Capture/Compare 1 output enable"]
pub type CH0EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn ch3p(&self) -> CH3P_R {
        CH3P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn ch3en(&self) -> CH3EN_R {
        CH3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn ch2np(&self) -> CH2NP_R {
        CH2NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn ch2nen(&self) -> CH2NEN_R {
        CH2NEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn ch2p(&self) -> CH2P_R {
        CH2P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn ch2en(&self) -> CH2EN_R {
        CH2EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn ch1np(&self) -> CH1NP_R {
        CH1NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn ch1nen(&self) -> CH1NEN_R {
        CH1NEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn ch1p(&self) -> CH1P_R {
        CH1P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn ch1en(&self) -> CH1EN_R {
        CH1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> CH0NP_R {
        CH0NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&self) -> CH0NEN_R {
        CH0NEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> CH0P_R {
        CH0P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn ch3p(&mut self) -> CH3P_W {
        CH3P_W::new(self)
    }
    #[doc = "Bit 12 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn ch3en(&mut self) -> CH3EN_W {
        CH3EN_W::new(self)
    }
    #[doc = "Bit 11 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn ch2np(&mut self) -> CH2NP_W {
        CH2NP_W::new(self)
    }
    #[doc = "Bit 10 - Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn ch2nen(&mut self) -> CH2NEN_W {
        CH2NEN_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn ch2p(&mut self) -> CH2P_W {
        CH2P_W::new(self)
    }
    #[doc = "Bit 8 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn ch2en(&mut self) -> CH2EN_W {
        CH2EN_W::new(self)
    }
    #[doc = "Bit 7 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn ch1np(&mut self) -> CH1NP_W {
        CH1NP_W::new(self)
    }
    #[doc = "Bit 6 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn ch1nen(&mut self) -> CH1NEN_W {
        CH1NEN_W::new(self)
    }
    #[doc = "Bit 5 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn ch1p(&mut self) -> CH1P_W {
        CH1P_W::new(self)
    }
    #[doc = "Bit 4 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> CH1EN_W {
        CH1EN_W::new(self)
    }
    #[doc = "Bit 3 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0np(&mut self) -> CH0NP_W {
        CH0NP_W::new(self)
    }
    #[doc = "Bit 2 - Capture/Compare 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&mut self) -> CH0NEN_W {
        CH0NEN_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0p(&mut self) -> CH0P_W {
        CH0P_W::new(self)
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> CH0EN_W {
        CH0EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl2](index.html) module"]
pub struct CHCTL2_SPEC;
impl crate::RegisterSpec for CHCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctl2::R](R) reader structure"]
impl crate::Readable for CHCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctl2::W](W) writer structure"]
impl crate::Writable for CHCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTL2 to value 0"]
impl crate::Resettable for CHCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
