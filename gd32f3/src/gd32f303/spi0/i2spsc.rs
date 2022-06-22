#[doc = "Register `I2SPSC` reader"]
pub struct R(crate::R<I2SPSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SPSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SPSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SPSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SPSC` writer"]
pub struct W(crate::W<I2SPSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SPSC_SPEC>;
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
impl From<crate::W<I2SPSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SPSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2S_MCK output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKOEN_A {
    #[doc = "0: Master clock output is disabled"]
    DISABLED = 0,
    #[doc = "1: Master clock output is enabled"]
    ENABLED = 1,
}
impl From<MCKOEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCKOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKOEN` reader - I2S_MCK output enable"]
pub type MCKOEN_R = crate::BitReader<MCKOEN_A>;
impl MCKOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKOEN_A {
        match self.bits {
            false => MCKOEN_A::DISABLED,
            true => MCKOEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKOEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKOEN_A::ENABLED
    }
}
#[doc = "Field `MCKOEN` writer - I2S_MCK output enable"]
pub type MCKOEN_W<'a> = crate::BitWriter<'a, u32, I2SPSC_SPEC, MCKOEN_A, 9>;
impl<'a> MCKOEN_W<'a> {
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCKOEN_A::DISABLED)
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCKOEN_A::ENABLED)
    }
}
#[doc = "Odd factor for the prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OF_A {
    #[doc = "0: Real divider value is DIV * 2"]
    EVEN = 0,
    #[doc = "1: Real divider value is (DIV * 2) + 1"]
    ODD = 1,
}
impl From<OF_A> for bool {
    #[inline(always)]
    fn from(variant: OF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OF` reader - Odd factor for the prescaler"]
pub type OF_R = crate::BitReader<OF_A>;
impl OF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OF_A {
        match self.bits {
            false => OF_A::EVEN,
            true => OF_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == OF_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == OF_A::ODD
    }
}
#[doc = "Field `OF` writer - Odd factor for the prescaler"]
pub type OF_W<'a> = crate::BitWriter<'a, u32, I2SPSC_SPEC, OF_A, 8>;
impl<'a> OF_W<'a> {
    #[doc = "Real divider value is DIV * 2"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(OF_A::EVEN)
    }
    #[doc = "Real divider value is (DIV * 2) + 1"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(OF_A::ODD)
    }
}
#[doc = "Field `DIV` reader - Dividing factor for the prescaler"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Dividing factor for the prescaler"]
pub type DIV_W<'a> = crate::FieldWriter<'a, u32, I2SPSC_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bit 9 - I2S_MCK output enable"]
    #[inline(always)]
    pub fn mckoen(&self) -> MCKOEN_R {
        MCKOEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:7 - Dividing factor for the prescaler"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 9 - I2S_MCK output enable"]
    #[inline(always)]
    pub fn mckoen(&mut self) -> MCKOEN_W {
        MCKOEN_W::new(self)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W::new(self)
    }
    #[doc = "Bits 0:7 - Dividing factor for the prescaler"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2spsc](index.html) module"]
pub struct I2SPSC_SPEC;
impl crate::RegisterSpec for I2SPSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2spsc::R](R) reader structure"]
impl crate::Readable for I2SPSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2spsc::W](W) writer structure"]
impl crate::Writable for I2SPSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SPSC to value 0x02"]
impl crate::Resettable for I2SPSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
