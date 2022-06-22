#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flag indicating DMA last transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMALST_A {
    #[doc = "0: Next DMA EOT is not the last transfer"]
    NOTLAST = 0,
    #[doc = "1: Next DMA EOT is the last transfer"]
    LAST = 1,
}
impl From<DMALST_A> for bool {
    #[inline(always)]
    fn from(variant: DMALST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMALST` reader - Flag indicating DMA last transfer"]
pub type DMALST_R = crate::BitReader<DMALST_A>;
impl DMALST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMALST_A {
        match self.bits {
            false => DMALST_A::NOTLAST,
            true => DMALST_A::LAST,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLAST`"]
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == DMALST_A::NOTLAST
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == DMALST_A::LAST
    }
}
#[doc = "Field `DMALST` writer - Flag indicating DMA last transfer"]
pub type DMALST_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, DMALST_A, 12>;
impl<'a> DMALST_W<'a> {
    #[doc = "Next DMA EOT is not the last transfer"]
    #[inline(always)]
    pub fn not_last(self) -> &'a mut W {
        self.variant(DMALST_A::NOTLAST)
    }
    #[doc = "Next DMA EOT is the last transfer"]
    #[inline(always)]
    pub fn last(self) -> &'a mut W {
        self.variant(DMALST_A::LAST)
    }
}
#[doc = "DMA mode switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAON_A {
    #[doc = "0: DMA requests disabled"]
    DISABLED = 0,
    #[doc = "1: DMA requests enabled"]
    ENABLED = 1,
}
impl From<DMAON_A> for bool {
    #[inline(always)]
    fn from(variant: DMAON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAON` reader - DMA mode switch"]
pub type DMAON_R = crate::BitReader<DMAON_A>;
impl DMAON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAON_A {
        match self.bits {
            false => DMAON_A::DISABLED,
            true => DMAON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAON_A::ENABLED
    }
}
#[doc = "Field `DMAON` writer - DMA mode switch"]
pub type DMAON_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, DMAON_A, 11>;
impl<'a> DMAON_W<'a> {
    #[doc = "DMA requests disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAON_A::DISABLED)
    }
    #[doc = "DMA requests enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAON_A::ENABLED)
    }
}
#[doc = "Buffer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFIE_A {
    #[doc = "0: TBE=1 or RBNE=1 does not generate any interrupt"]
    DISABLED = 0,
    #[doc = "1: TBE=1 or RBNE=1 generates Event interrupt"]
    ENABLED = 1,
}
impl From<BUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: BUFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFIE` reader - Buffer interrupt enable"]
pub type BUFIE_R = crate::BitReader<BUFIE_A>;
impl BUFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFIE_A {
        match self.bits {
            false => BUFIE_A::DISABLED,
            true => BUFIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BUFIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BUFIE_A::ENABLED
    }
}
#[doc = "Field `BUFIE` writer - Buffer interrupt enable"]
pub type BUFIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, BUFIE_A, 10>;
impl<'a> BUFIE_W<'a> {
    #[doc = "TBE=1 or RBNE=1 does not generate any interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BUFIE_A::DISABLED)
    }
    #[doc = "TBE=1 or RBNE=1 generates Event interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BUFIE_A::ENABLED)
    }
}
#[doc = "Event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVIE_A {
    #[doc = "0: Event interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Event interrupt enabled"]
    ENABLED = 1,
}
impl From<EVIE_A> for bool {
    #[inline(always)]
    fn from(variant: EVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVIE` reader - Event interrupt enable"]
pub type EVIE_R = crate::BitReader<EVIE_A>;
impl EVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVIE_A {
        match self.bits {
            false => EVIE_A::DISABLED,
            true => EVIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EVIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EVIE_A::ENABLED
    }
}
#[doc = "Field `EVIE` writer - Event interrupt enable"]
pub type EVIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, EVIE_A, 9>;
impl<'a> EVIE_W<'a> {
    #[doc = "Event interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EVIE_A::DISABLED)
    }
    #[doc = "Event interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EVIE_A::ENABLED)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Error interrupt enabled"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, ERRIE_A, 8>;
impl<'a> ERRIE_W<'a> {
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
    }
}
#[doc = "Field `I2CCLK` reader - I2C Peripheral clock frequency"]
pub type I2CCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2CCLK` writer - I2C Peripheral clock frequency"]
pub type I2CCLK_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    pub fn dmalst(&self) -> DMALST_R {
        DMALST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    pub fn dmaon(&self) -> DMAON_R {
        DMAON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn bufie(&self) -> BUFIE_R {
        BUFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evie(&self) -> EVIE_R {
        EVIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:5 - I2C Peripheral clock frequency"]
    #[inline(always)]
    pub fn i2cclk(&self) -> I2CCLK_R {
        I2CCLK_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    pub fn dmalst(&mut self) -> DMALST_W {
        DMALST_W::new(self)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    pub fn dmaon(&mut self) -> DMAON_W {
        DMAON_W::new(self)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn bufie(&mut self) -> BUFIE_W {
        BUFIE_W::new(self)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evie(&mut self) -> EVIE_W {
        EVIE_W::new(self)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Bits 0:5 - I2C Peripheral clock frequency"]
    #[inline(always)]
    pub fn i2cclk(&mut self) -> I2CCLK_W {
        I2CCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
