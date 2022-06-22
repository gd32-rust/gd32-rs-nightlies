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
#[doc = "Channel 0 complementary output polarity"]
pub use crate::gd32f170::timer0::chctl2::CH0P_A as CH0NP_A;
#[doc = "Field `CH0NP` reader - Channel 0 complementary output polarity"]
pub use crate::gd32f170::timer0::chctl2::CH0P_R as CH0NP_R;
#[doc = "Field `CH0NP` writer - Channel 0 complementary output polarity"]
pub type CH0NP_W<'a> = crate::BitWriter<'a, u16, CHCTL2_SPEC, CH0NP_A, 3>;
impl<'a> CH0NP_W<'a> {
    #[doc = "Active high"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(CH0NP_A::NOTINVERTED)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CH0NP_A::INVERTED)
    }
}
#[doc = "Channel 0 complementary output enable"]
pub use crate::gd32f170::timer0::chctl2::CH0EN_A as CH0NEN_A;
#[doc = "Field `CH0NEN` reader - Channel 0 complementary output enable"]
pub use crate::gd32f170::timer0::chctl2::CH0EN_R as CH0NEN_R;
#[doc = "Field `CH0NEN` writer - Channel 0 complementary output enable"]
pub type CH0NEN_W<'a> = crate::BitWriter<'a, u16, CHCTL2_SPEC, CH0NEN_A, 2>;
impl<'a> CH0NEN_W<'a> {
    #[doc = "Channel output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH0NEN_A::DISABLED)
    }
    #[doc = "Channel output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH0NEN_A::ENABLED)
    }
}
#[doc = "Channel 0 polarity"]
pub use crate::gd32f170::timer0::chctl2::CH0P_A;
#[doc = "Field `CH0P` reader - Channel 0 polarity"]
pub use crate::gd32f170::timer0::chctl2::CH0P_R;
#[doc = "Field `CH0P` writer - Channel 0 polarity"]
pub type CH0P_W<'a> = crate::BitWriter<'a, u16, CHCTL2_SPEC, CH0P_A, 1>;
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
#[doc = "Channel 0 enable"]
pub use crate::gd32f170::timer0::chctl2::CH0EN_A;
#[doc = "Field `CH0EN` reader - Channel 0 enable"]
pub use crate::gd32f170::timer0::chctl2::CH0EN_R;
#[doc = "Field `CH0EN` writer - Channel 0 enable"]
pub type CH0EN_W<'a> = crate::BitWriter<'a, u16, CHCTL2_SPEC, CH0EN_A, 0>;
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
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> CH0NP_R {
        CH0NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&self) -> CH0NEN_R {
        CH0NEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> CH0P_R {
        CH0P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel 0 enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&mut self) -> CH0NP_W {
        CH0NP_W::new(self)
    }
    #[doc = "Bit 2 - Channel 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&mut self) -> CH0NEN_W {
        CH0NEN_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 polarity"]
    #[inline(always)]
    pub fn ch0p(&mut self) -> CH0P_W {
        CH0P_W::new(self)
    }
    #[doc = "Bit 0 - Channel 0 enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> CH0EN_W {
        CH0EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl2](index.html) module"]
pub struct CHCTL2_SPEC;
impl crate::RegisterSpec for CHCTL2_SPEC {
    type Ux = u16;
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
