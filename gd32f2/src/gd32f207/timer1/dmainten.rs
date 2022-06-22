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
#[doc = "Trigger DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGDEN_A {
    #[doc = "0: Trigger DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger DMA request enabled"]
    ENABLED = 1,
}
impl From<TRGDEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRGDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGDEN` reader - Trigger DMA request enable"]
pub type TRGDEN_R = crate::BitReader<TRGDEN_A>;
impl TRGDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGDEN_A {
        match self.bits {
            false => TRGDEN_A::DISABLED,
            true => TRGDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRGDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRGDEN_A::ENABLED
    }
}
#[doc = "Field `TRGDEN` writer - Trigger DMA request enable"]
pub type TRGDEN_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, TRGDEN_A, 14>;
impl<'a> TRGDEN_W<'a> {
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRGDEN_A::DISABLED)
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRGDEN_A::ENABLED)
    }
}
#[doc = "Channel 3 capture/compare DMA request enable"]
pub use CH0DEN_A as CH3DEN_A;
#[doc = "Channel 2 capture/compare DMA request enable"]
pub use CH0DEN_A as CH2DEN_A;
#[doc = "Channel 1 capture/compare DMA request enable"]
pub use CH0DEN_A as CH1DEN_A;
#[doc = "Field `CH3DEN` reader - Channel 3 capture/compare DMA request enable"]
pub use CH0DEN_R as CH3DEN_R;
#[doc = "Field `CH2DEN` reader - Channel 2 capture/compare DMA request enable"]
pub use CH0DEN_R as CH2DEN_R;
#[doc = "Field `CH1DEN` reader - Channel 1 capture/compare DMA request enable"]
pub use CH0DEN_R as CH1DEN_R;
#[doc = "Field `CH3DEN` writer - Channel 3 capture/compare DMA request enable"]
pub use CH0DEN_W as CH3DEN_W;
#[doc = "Field `CH2DEN` writer - Channel 2 capture/compare DMA request enable"]
pub use CH0DEN_W as CH2DEN_W;
#[doc = "Field `CH1DEN` writer - Channel 1 capture/compare DMA request enable"]
pub use CH0DEN_W as CH1DEN_W;
#[doc = "Channel 0 capture/compare DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0DEN_A {
    #[doc = "0: Capture/compare DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Capture/compare DMA request enabled"]
    ENABLED = 1,
}
impl From<CH0DEN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0DEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0DEN` reader - Channel 0 capture/compare DMA request enable"]
pub type CH0DEN_R = crate::BitReader<CH0DEN_A>;
impl CH0DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0DEN_A {
        match self.bits {
            false => CH0DEN_A::DISABLED,
            true => CH0DEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0DEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0DEN_A::ENABLED
    }
}
#[doc = "Field `CH0DEN` writer - Channel 0 capture/compare DMA request enable"]
pub type CH0DEN_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, CH0DEN_A, 9>;
impl<'a> CH0DEN_W<'a> {
    #[doc = "Capture/compare DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH0DEN_A::DISABLED)
    }
    #[doc = "Capture/compare DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH0DEN_A::ENABLED)
    }
}
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDEN_A {
    #[doc = "0: Update DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Update DMA request enabled"]
    ENABLED = 1,
}
impl From<UPDEN_A> for bool {
    #[inline(always)]
    fn from(variant: UPDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UPDEN_R = crate::BitReader<UPDEN_A>;
impl UPDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDEN_A {
        match self.bits {
            false => UPDEN_A::DISABLED,
            true => UPDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDEN_A::ENABLED
    }
}
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UPDEN_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, UPDEN_A, 8>;
impl<'a> UPDEN_W<'a> {
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPDEN_A::DISABLED)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPDEN_A::ENABLED)
    }
}
#[doc = "Trigger interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGIE_A {
    #[doc = "0: Trigger interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger interrupt enabled"]
    ENABLED = 1,
}
impl From<TRGIE_A> for bool {
    #[inline(always)]
    fn from(variant: TRGIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGIE` reader - Trigger interrupt enable"]
pub type TRGIE_R = crate::BitReader<TRGIE_A>;
impl TRGIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGIE_A {
        match self.bits {
            false => TRGIE_A::DISABLED,
            true => TRGIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRGIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRGIE_A::ENABLED
    }
}
#[doc = "Field `TRGIE` writer - Trigger interrupt enable"]
pub type TRGIE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, TRGIE_A, 6>;
impl<'a> TRGIE_W<'a> {
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRGIE_A::DISABLED)
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRGIE_A::ENABLED)
    }
}
#[doc = "Channel 3 capture/compare interrupt enable"]
pub use crate::gd32f207::timer0::dmainten::CH0IE_A as CH3IE_A;
#[doc = "Field `CH3IE` reader - Channel 3 capture/compare interrupt enable"]
pub use crate::gd32f207::timer0::dmainten::CH0IE_R as CH3IE_R;
#[doc = "Field `CH3IE` writer - Channel 3 capture/compare interrupt enable"]
pub type CH3IE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, CH3IE_A, 4>;
impl<'a> CH3IE_W<'a> {
    #[doc = "Capture/compare interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH3IE_A::DISABLED)
    }
    #[doc = "Capture/compare interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH3IE_A::ENABLED)
    }
}
#[doc = "Channel 2 capture/compare interrupt enable"]
pub use crate::gd32f207::timer0::dmainten::CH0IE_A as CH2IE_A;
#[doc = "Field `CH2IE` reader - Channel 2 capture/compare interrupt enable"]
pub use crate::gd32f207::timer0::dmainten::CH0IE_R as CH2IE_R;
#[doc = "Field `CH2IE` writer - Channel 2 capture/compare interrupt enable"]
pub type CH2IE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, CH2IE_A, 3>;
impl<'a> CH2IE_W<'a> {
    #[doc = "Capture/compare interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH2IE_A::DISABLED)
    }
    #[doc = "Capture/compare interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH2IE_A::ENABLED)
    }
}
#[doc = "Channel 1 capture/compare interrupt enable"]
pub use crate::gd32f207::timer0::dmainten::CH0IE_A as CH1IE_A;
#[doc = "Field `CH1IE` reader - Channel 1 capture/compare interrupt enable"]
pub use crate::gd32f207::timer0::dmainten::CH0IE_R as CH1IE_R;
#[doc = "Field `CH1IE` writer - Channel 1 capture/compare interrupt enable"]
pub type CH1IE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, CH1IE_A, 2>;
impl<'a> CH1IE_W<'a> {
    #[doc = "Capture/compare interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH1IE_A::DISABLED)
    }
    #[doc = "Capture/compare interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH1IE_A::ENABLED)
    }
}
#[doc = "Channel 0 capture/compare interrupt enable"]
pub use crate::gd32f207::timer0::dmainten::CH0IE_A;
#[doc = "Field `CH0IE` reader - Channel 0 capture/compare interrupt enable"]
pub use crate::gd32f207::timer0::dmainten::CH0IE_R;
#[doc = "Field `CH0IE` writer - Channel 0 capture/compare interrupt enable"]
pub type CH0IE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, CH0IE_A, 1>;
impl<'a> CH0IE_W<'a> {
    #[doc = "Capture/compare interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH0IE_A::DISABLED)
    }
    #[doc = "Capture/compare interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH0IE_A::ENABLED)
    }
}
#[doc = "Update interrupt enable"]
pub use crate::gd32f207::timer0::dmainten::UPIE_A;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub use crate::gd32f207::timer0::dmainten::UPIE_R;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UPIE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, UPIE_A, 0>;
impl<'a> UPIE_W<'a> {
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPIE_A::DISABLED)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&self) -> TRGDEN_R {
        TRGDEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch3den(&self) -> CH3DEN_R {
        CH3DEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch2den(&self) -> CH2DEN_R {
        CH2DEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&self) -> CH1DEN_R {
        CH1DEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&self) -> CH0DEN_R {
        CH0DEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UPDEN_R {
        UPDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&self) -> TRGIE_R {
        TRGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch3ie(&self) -> CH3IE_R {
        CH3IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch2ie(&self) -> CH2IE_R {
        CH2IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&self) -> CH1IE_R {
        CH1IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
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
    #[doc = "Bit 12 - Channel 3 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch3den(&mut self) -> CH3DEN_W {
        CH3DEN_W::new(self)
    }
    #[doc = "Bit 11 - Channel 2 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch2den(&mut self) -> CH2DEN_W {
        CH2DEN_W::new(self)
    }
    #[doc = "Bit 10 - Channel 1 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&mut self) -> CH1DEN_W {
        CH1DEN_W::new(self)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&mut self) -> CH0DEN_W {
        CH0DEN_W::new(self)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&mut self) -> UPDEN_W {
        UPDEN_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&mut self) -> TRGIE_W {
        TRGIE_W::new(self)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch3ie(&mut self) -> CH3IE_W {
        CH3IE_W::new(self)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch2ie(&mut self) -> CH2IE_W {
        CH2IE_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&mut self) -> CH1IE_W {
        CH1IE_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
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
