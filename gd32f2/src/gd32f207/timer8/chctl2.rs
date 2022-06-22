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
#[doc = "Field `CH1P` reader - Channel 1 capture/compare function polarity"]
pub type CH1P_R = crate::BitReader<bool>;
#[doc = "Field `CH1P` writer - Channel 1 capture/compare function polarity"]
pub type CH1P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 5>;
#[doc = "Field `CH1EN` reader - Channel 1 capture/compare function enable"]
pub type CH1EN_R = crate::BitReader<bool>;
#[doc = "Field `CH1EN` writer - Channel 1 capture/compare function enable"]
pub type CH1EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 4>;
#[doc = "Field `CH0NP` reader - Channel 0 complementary output polarity"]
pub type CH0NP_R = crate::BitReader<bool>;
#[doc = "Field `CH0NP` writer - Channel 0 complementary output polarity"]
pub type CH0NP_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 3>;
#[doc = "Field `CH0P` reader - Channel 0 capture/compare function polarity"]
pub type CH0P_R = crate::BitReader<bool>;
#[doc = "Field `CH0P` writer - Channel 0 capture/compare function polarity"]
pub type CH0P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 1>;
#[doc = "Field `CH0EN` reader - Channel 0 capture/compare function enable"]
pub type CH0EN_R = crate::BitReader<bool>;
#[doc = "Field `CH0EN` writer - Channel 0 capture/compare function enable"]
pub type CH0EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 5 - Channel 1 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch1p(&self) -> CH1P_R {
        CH1P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 capture/compare function enable"]
    #[inline(always)]
    pub fn ch1en(&self) -> CH1EN_R {
        CH1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> CH0NP_R {
        CH0NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> CH0P_R {
        CH0P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel 0 capture/compare function enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Channel 1 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch1p(&mut self) -> CH1P_W {
        CH1P_W::new(self)
    }
    #[doc = "Bit 4 - Channel 1 capture/compare function enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> CH1EN_W {
        CH1EN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&mut self) -> CH0NP_W {
        CH0NP_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch0p(&mut self) -> CH0P_W {
        CH0P_W::new(self)
    }
    #[doc = "Bit 0 - Channel 0 capture/compare function enable"]
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
#[doc = "Channel control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl2](index.html) module"]
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
