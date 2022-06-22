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
#[doc = "Channel 3 capture/compare function polarity"]
pub use crate::gd32f207::timer0::chctl2::CH0P_A as CH3P_A;
#[doc = "Field `CH3P` reader - Channel 3 capture/compare function polarity"]
pub use crate::gd32f207::timer0::chctl2::CH0P_R as CH3P_R;
#[doc = "Field `CH3P` writer - Channel 3 capture/compare function polarity"]
pub type CH3P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, CH3P_A, 13>;
impl<'a> CH3P_W<'a> {
    #[doc = "Active high"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(CH3P_A::NOTINVERTED)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CH3P_A::INVERTED)
    }
}
#[doc = "Channel 3 capture/compare function enable"]
pub use crate::gd32f207::timer0::chctl2::CH0EN_A as CH3EN_A;
#[doc = "Field `CH3EN` reader - Channel 3 capture/compare function enable"]
pub use crate::gd32f207::timer0::chctl2::CH0EN_R as CH3EN_R;
#[doc = "Field `CH3EN` writer - Channel 3 capture/compare function enable"]
pub type CH3EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, CH3EN_A, 12>;
impl<'a> CH3EN_W<'a> {
    #[doc = "Channel output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH3EN_A::DISABLED)
    }
    #[doc = "Channel output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH3EN_A::ENABLED)
    }
}
#[doc = "Channel 2 capture/compare function polarity"]
pub use crate::gd32f207::timer0::chctl2::CH0P_A as CH2P_A;
#[doc = "Field `CH2P` reader - Channel 2 capture/compare function polarity"]
pub use crate::gd32f207::timer0::chctl2::CH0P_R as CH2P_R;
#[doc = "Field `CH2P` writer - Channel 2 capture/compare function polarity"]
pub type CH2P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, CH2P_A, 9>;
impl<'a> CH2P_W<'a> {
    #[doc = "Active high"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(CH2P_A::NOTINVERTED)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CH2P_A::INVERTED)
    }
}
#[doc = "Channel 2 capture/compare function enable"]
pub use crate::gd32f207::timer0::chctl2::CH0EN_A as CH2EN_A;
#[doc = "Field `CH2EN` reader - Channel 2 capture/compare function enable"]
pub use crate::gd32f207::timer0::chctl2::CH0EN_R as CH2EN_R;
#[doc = "Field `CH2EN` writer - Channel 2 capture/compare function enable"]
pub type CH2EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, CH2EN_A, 8>;
impl<'a> CH2EN_W<'a> {
    #[doc = "Channel output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH2EN_A::DISABLED)
    }
    #[doc = "Channel output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH2EN_A::ENABLED)
    }
}
#[doc = "Channel 1 capture/compare function polarity"]
pub use crate::gd32f207::timer0::chctl2::CH0P_A as CH1P_A;
#[doc = "Field `CH1P` reader - Channel 1 capture/compare function polarity"]
pub use crate::gd32f207::timer0::chctl2::CH0P_R as CH1P_R;
#[doc = "Field `CH1P` writer - Channel 1 capture/compare function polarity"]
pub type CH1P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, CH1P_A, 5>;
impl<'a> CH1P_W<'a> {
    #[doc = "Active high"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(CH1P_A::NOTINVERTED)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CH1P_A::INVERTED)
    }
}
#[doc = "Channel 1 capture/compare function enable"]
pub use crate::gd32f207::timer0::chctl2::CH0EN_A as CH1EN_A;
#[doc = "Field `CH1EN` reader - Channel 1 capture/compare function enable"]
pub use crate::gd32f207::timer0::chctl2::CH0EN_R as CH1EN_R;
#[doc = "Field `CH1EN` writer - Channel 1 capture/compare function enable"]
pub type CH1EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, CH1EN_A, 4>;
impl<'a> CH1EN_W<'a> {
    #[doc = "Channel output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH1EN_A::DISABLED)
    }
    #[doc = "Channel output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH1EN_A::ENABLED)
    }
}
#[doc = "Channel 0 capture/compare function polarity"]
pub use crate::gd32f207::timer0::chctl2::CH0P_A;
#[doc = "Field `CH0P` reader - Channel 0 capture/compare function polarity"]
pub use crate::gd32f207::timer0::chctl2::CH0P_R;
#[doc = "Field `CH0P` writer - Channel 0 capture/compare function polarity"]
pub type CH0P_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, CH0P_A, 1>;
impl<'a> CH0P_W<'a> {
    #[doc = "Active high"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(CH0P_A::NOTINVERTED)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CH0P_A::INVERTED)
    }
}
#[doc = "Channel 0 capture/compare function enable"]
pub use crate::gd32f207::timer0::chctl2::CH0EN_A;
#[doc = "Field `CH0EN` reader - Channel 0 capture/compare function enable"]
pub use crate::gd32f207::timer0::chctl2::CH0EN_R;
#[doc = "Field `CH0EN` writer - Channel 0 capture/compare function enable"]
pub type CH0EN_W<'a> = crate::BitWriter<'a, u32, CHCTL2_SPEC, CH0EN_A, 0>;
impl<'a> CH0EN_W<'a> {
    #[doc = "Channel output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH0EN_A::DISABLED)
    }
    #[doc = "Channel output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH0EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 13 - Channel 3 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch3p(&self) -> CH3P_R {
        CH3P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare function enable"]
    #[inline(always)]
    pub fn ch3en(&self) -> CH3EN_R {
        CH3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 2 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch2p(&self) -> CH2P_R {
        CH2P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 2 capture/compare function enable"]
    #[inline(always)]
    pub fn ch2en(&self) -> CH2EN_R {
        CH2EN_R::new(((self.bits >> 8) & 1) != 0)
    }
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
    #[doc = "Bit 13 - Channel 3 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch3p(&mut self) -> CH3P_W {
        CH3P_W::new(self)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare function enable"]
    #[inline(always)]
    pub fn ch3en(&mut self) -> CH3EN_W {
        CH3EN_W::new(self)
    }
    #[doc = "Bit 9 - Channel 2 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch2p(&mut self) -> CH2P_W {
        CH2P_W::new(self)
    }
    #[doc = "Bit 8 - Channel 2 capture/compare function enable"]
    #[inline(always)]
    pub fn ch2en(&mut self) -> CH2EN_W {
        CH2EN_W::new(self)
    }
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
