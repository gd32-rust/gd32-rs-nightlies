#[doc = "Register `CKCFG` reader"]
pub struct R(crate::R<CKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCFG` writer"]
pub struct W(crate::W<CKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCFG_SPEC>;
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
impl From<crate::W<CKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C speed selection in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_A {
    #[doc = "0: Standard mode I2C"]
    STANDARD = 0,
    #[doc = "1: Fast mode I2C"]
    FAST = 1,
}
impl From<FAST_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAST` reader - I2C speed selection in master mode"]
pub type FAST_R = crate::BitReader<FAST_A>;
impl FAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_A {
        match self.bits {
            false => FAST_A::STANDARD,
            true => FAST_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FAST_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == FAST_A::FAST
    }
}
#[doc = "Field `FAST` writer - I2C speed selection in master mode"]
pub type FAST_W<'a> = crate::BitWriter<'a, u16, CKCFG_SPEC, FAST_A, 15>;
impl<'a> FAST_W<'a> {
    #[doc = "Standard mode I2C"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(FAST_A::STANDARD)
    }
    #[doc = "Fast mode I2C"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(FAST_A::FAST)
    }
}
#[doc = "Duty cycle in fast mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCY_A {
    #[doc = "0: Duty cycle t_low/t_high = 2"]
    DUTY2 = 0,
    #[doc = "1: Duty cycle t_low/t_high = 16/9"]
    DUTY16_9 = 1,
}
impl From<DTCY_A> for bool {
    #[inline(always)]
    fn from(variant: DTCY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCY` reader - Duty cycle in fast mode"]
pub type DTCY_R = crate::BitReader<DTCY_A>;
impl DTCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCY_A {
        match self.bits {
            false => DTCY_A::DUTY2,
            true => DTCY_A::DUTY16_9,
        }
    }
    #[doc = "Checks if the value of the field is `DUTY2`"]
    #[inline(always)]
    pub fn is_duty2(&self) -> bool {
        *self == DTCY_A::DUTY2
    }
    #[doc = "Checks if the value of the field is `DUTY16_9`"]
    #[inline(always)]
    pub fn is_duty16_9(&self) -> bool {
        *self == DTCY_A::DUTY16_9
    }
}
#[doc = "Field `DTCY` writer - Duty cycle in fast mode"]
pub type DTCY_W<'a> = crate::BitWriter<'a, u16, CKCFG_SPEC, DTCY_A, 14>;
impl<'a> DTCY_W<'a> {
    #[doc = "Duty cycle t_low/t_high = 2"]
    #[inline(always)]
    pub fn duty2(self) -> &'a mut W {
        self.variant(DTCY_A::DUTY2)
    }
    #[doc = "Duty cycle t_low/t_high = 16/9"]
    #[inline(always)]
    pub fn duty16_9(self) -> &'a mut W {
        self.variant(DTCY_A::DUTY16_9)
    }
}
#[doc = "Field `CLKC` reader - I2C Clock control in master mode"]
pub type CLKC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLKC` writer - I2C Clock control in master mode"]
pub type CLKC_W<'a> = crate::FieldWriterSafe<'a, u16, CKCFG_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    pub fn dtcy(&self) -> DTCY_R {
        DTCY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 0:11 - I2C Clock control in master mode"]
    #[inline(always)]
    pub fn clkc(&self) -> CLKC_R {
        CLKC_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W {
        FAST_W::new(self)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    pub fn dtcy(&mut self) -> DTCY_W {
        DTCY_W::new(self)
    }
    #[doc = "Bits 0:11 - I2C Clock control in master mode"]
    #[inline(always)]
    pub fn clkc(&mut self) -> CLKC_W {
        CLKC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcfg](index.html) module"]
pub struct CKCFG_SPEC;
impl crate::RegisterSpec for CKCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ckcfg::R](R) reader structure"]
impl crate::Readable for CKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcfg::W](W) writer structure"]
impl crate::Writable for CKCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKCFG to value 0"]
impl crate::Resettable for CKCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
