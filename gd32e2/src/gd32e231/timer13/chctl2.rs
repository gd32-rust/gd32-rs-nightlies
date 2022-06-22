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
#[doc = "Field `CH0NP` reader - Capture/Compare 0 output Polarity"]
pub type CH0NP_R = crate::BitReader<bool>;
#[doc = "Field `CH0NP` writer - Capture/Compare 0 output Polarity"]
pub type CH0NP_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 3>;
#[doc = "Field `CH0P` reader - Capture/Compare 0 output Polarity"]
pub type CH0P_R = crate::BitReader<bool>;
#[doc = "Field `CH0P` writer - Capture/Compare 0 output Polarity"]
pub type CH0P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 1>;
#[doc = "Field `CH0EN` reader - Capture/Compare 1 output enable"]
pub type CH0EN_R = crate::BitReader<bool>;
#[doc = "Field `CH0EN` writer - Capture/Compare 1 output enable"]
pub type CH0EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 3 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> CH0NP_R {
        CH0NP_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 3 - Capture/Compare 0 output Polarity"]
    #[inline(always)]
    pub fn ch0np(&mut self) -> CH0NP_W {
        CH0NP_W::new(self)
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
