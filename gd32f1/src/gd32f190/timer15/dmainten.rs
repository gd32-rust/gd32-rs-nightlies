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
#[doc = "Channel 0 Capture/Compare DMA request enable\n\nValue on reset: 0"]
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
#[doc = "Field `CH0DEN` reader - Channel 0 Capture/Compare DMA request enable"]
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
#[doc = "Field `CH0DEN` writer - Channel 0 Capture/Compare DMA request enable"]
pub type CH0DEN_W<'a> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, CH0DEN_A, 9>;
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
pub type UPDEN_W<'a> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, UPDEN_A, 8>;
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
#[doc = "Break interrupt enable"]
pub use crate::gd32f190::timer0::dmainten::BRKIE_A;
#[doc = "Field `BRKIE` reader - Break interrupt enable"]
pub use crate::gd32f190::timer0::dmainten::BRKIE_R;
#[doc = "Field `BRKIE` writer - Break interrupt enable"]
pub type BRKIE_W<'a> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, BRKIE_A, 7>;
impl<'a> BRKIE_W<'a> {
    #[doc = "Break interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BRKIE_A::DISABLED)
    }
    #[doc = "Break interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BRKIE_A::ENABLED)
    }
}
#[doc = "CMT interrupt enable"]
pub use crate::gd32f190::timer0::dmainten::CMTIE_A;
#[doc = "Field `CMTIE` reader - CMT interrupt enable"]
pub use crate::gd32f190::timer0::dmainten::CMTIE_R;
#[doc = "Field `CMTIE` writer - CMT interrupt enable"]
pub type CMTIE_W<'a> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, CMTIE_A, 5>;
impl<'a> CMTIE_W<'a> {
    #[doc = "Commutation interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMTIE_A::DISABLED)
    }
    #[doc = "Commutation interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMTIE_A::ENABLED)
    }
}
#[doc = "Channel 0 Capture/Compare interrupt enable"]
pub use crate::gd32f190::timer0::dmainten::CH0IE_A;
#[doc = "Field `CH0IE` reader - Channel 0 Capture/Compare interrupt enable"]
pub use crate::gd32f190::timer0::dmainten::CH0IE_R;
#[doc = "Field `CH0IE` writer - Channel 0 Capture/Compare interrupt enable"]
pub type CH0IE_W<'a> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, CH0IE_A, 1>;
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
pub use crate::gd32f190::timer0::dmainten::UPIE_A;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub use crate::gd32f190::timer0::dmainten::UPIE_R;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UPIE_W<'a> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, UPIE_A, 0>;
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
    #[doc = "Bit 9 - Channel 0 Capture/Compare DMA request enable"]
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
    #[doc = "Bit 5 - CMT interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&self) -> CMTIE_R {
        CMTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 Capture/Compare interrupt enable"]
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
    #[doc = "Bit 9 - Channel 0 Capture/Compare DMA request enable"]
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
    #[doc = "Bit 5 - CMT interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&mut self) -> CMTIE_W {
        CMTIE_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 Capture/Compare interrupt enable"]
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA and interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmainten](index.html) module"]
pub struct DMAINTEN_SPEC;
impl crate::RegisterSpec for DMAINTEN_SPEC {
    type Ux = u16;
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
