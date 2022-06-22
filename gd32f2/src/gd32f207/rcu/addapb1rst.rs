#[doc = "Register `ADDAPB1RST` reader"]
pub struct R(crate::R<ADDAPB1RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDAPB1RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDAPB1RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDAPB1RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDAPB1RST` writer"]
pub struct W(crate::W<ADDAPB1RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDAPB1RST_SPEC>;
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
impl From<crate::W<ADDAPB1RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDAPB1RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C2 reset\n\nValue on reset: 0"]
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
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
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
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2C2RST_W<'a> = crate::BitWriter<'a, u32, ADDAPB1RST_SPEC, I2C2RST_A, 23>;
impl<'a> I2C2RST_W<'a> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C2RST_A::RESET)
    }
}
#[doc = "UART6 reset"]
pub use I2C2RST_A as UART6RST_A;
#[doc = "UART7 reset"]
pub use I2C2RST_A as UART7RST_A;
#[doc = "Field `UART6RST` reader - UART6 reset"]
pub use I2C2RST_R as UART6RST_R;
#[doc = "Field `UART7RST` reader - UART7 reset"]
pub use I2C2RST_R as UART7RST_R;
#[doc = "Field `UART6RST` writer - UART6 reset"]
pub use I2C2RST_W as UART6RST_W;
#[doc = "Field `UART7RST` writer - UART7 reset"]
pub use I2C2RST_W as UART7RST_W;
impl R {
    #[doc = "Bit 23 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - UART6 reset"]
    #[inline(always)]
    pub fn uart6rst(&self) -> UART6RST_R {
        UART6RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART7 reset"]
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 30 - UART6 reset"]
    #[inline(always)]
    pub fn uart6rst(&mut self) -> UART6RST_W {
        UART6RST_W::new(self)
    }
    #[doc = "Bit 31 - UART7 reset"]
    #[inline(always)]
    pub fn uart7rst(&mut self) -> UART7RST_W {
        UART7RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 additional enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addapb1rst](index.html) module"]
pub struct ADDAPB1RST_SPEC;
impl crate::RegisterSpec for ADDAPB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addapb1rst::R](R) reader structure"]
impl crate::Readable for ADDAPB1RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addapb1rst::W](W) writer structure"]
impl crate::Writable for ADDAPB1RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDAPB1RST to value 0"]
impl crate::Resettable for ADDAPB1RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
