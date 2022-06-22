#[doc = "Register `ADDEN` reader"]
pub struct R(crate::R<ADDEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDEN` writer"]
pub struct W(crate::W<ADDEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDEN_SPEC>;
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
impl From<crate::W<ADDEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C2 unit clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2EN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<I2C2EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2EN` reader - I2C2 unit clock enable"]
pub type I2C2EN_R = crate::BitReader<I2C2EN_A>;
impl I2C2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2EN_A {
        match self.bits {
            false => I2C2EN_A::DISABLED,
            true => I2C2EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C2EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C2EN_A::ENABLED
    }
}
#[doc = "Field `I2C2EN` writer - I2C2 unit clock enable"]
pub type I2C2EN_W<'a> = crate::BitWriter<'a, u32, ADDEN_SPEC, I2C2EN_A, 0>;
impl<'a> I2C2EN_W<'a> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C2EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - I2C2 unit clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C2 unit clock enable"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W {
        I2C2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Additional enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adden](index.html) module"]
pub struct ADDEN_SPEC;
impl crate::RegisterSpec for ADDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adden::R](R) reader structure"]
impl crate::Readable for ADDEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adden::W](W) writer structure"]
impl crate::Writable for ADDEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDEN to value 0"]
impl crate::Resettable for ADDEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
