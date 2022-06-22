#[doc = "Register `ADDRST` reader"]
pub struct R(crate::R<ADDRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRST` writer"]
pub struct W(crate::W<ADDRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRST_SPEC>;
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
impl From<crate::W<ADDRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C2 unit reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2RST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<I2C2RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2RST` reader - I2C2 unit reset"]
pub type I2C2RST_R = crate::BitReader<I2C2RST_A>;
impl I2C2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C2RST_A> {
        match self.bits {
            true => Some(I2C2RST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == I2C2RST_A::RESET
    }
}
#[doc = "Field `I2C2RST` writer - I2C2 unit reset"]
pub type I2C2RST_W<'a> = crate::BitWriter<'a, u32, ADDRST_SPEC, I2C2RST_A, 0>;
impl<'a> I2C2RST_W<'a> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C2RST_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - I2C2 unit reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C2 unit reset"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W {
        I2C2RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Additional reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrst](index.html) module"]
pub struct ADDRST_SPEC;
impl crate::RegisterSpec for ADDRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addrst::R](R) reader structure"]
impl crate::Readable for ADDRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addrst::W](W) writer structure"]
impl crate::Writable for ADDRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDRST to value 0"]
impl crate::Resettable for ADDRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
