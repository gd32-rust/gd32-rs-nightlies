#[doc = "Register `ADDAPB1EN` reader"]
pub struct R(crate::R<ADDAPB1EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDAPB1EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDAPB1EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDAPB1EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDAPB1EN` writer"]
pub struct W(crate::W<ADDAPB1EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDAPB1EN_SPEC>;
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
impl From<crate::W<ADDAPB1EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDAPB1EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C2 clock enable\n\nValue on reset: 0"]
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
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
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
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub type I2C2EN_W<'a> = crate::BitWriter<'a, u32, ADDAPB1EN_SPEC, I2C2EN_A, 23>;
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
#[doc = "UART6 clock enable"]
pub use I2C2EN_A as UART6EN_A;
#[doc = "UART7 clock enable"]
pub use I2C2EN_A as UART7EN_A;
#[doc = "Field `UART6EN` reader - UART6 clock enable"]
pub use I2C2EN_R as UART6EN_R;
#[doc = "Field `UART7EN` reader - UART7 clock enable"]
pub use I2C2EN_R as UART7EN_R;
#[doc = "Field `UART6EN` writer - UART6 clock enable"]
pub use I2C2EN_W as UART6EN_W;
#[doc = "Field `UART7EN` writer - UART7 clock enable"]
pub use I2C2EN_W as UART7EN_W;
impl R {
    #[doc = "Bit 23 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - UART6 clock enable"]
    #[inline(always)]
    pub fn uart6en(&self) -> UART6EN_R {
        UART6EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART7 clock enable"]
    #[inline(always)]
    pub fn uart7en(&self) -> UART7EN_R {
        UART7EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 30 - UART6 clock enable"]
    #[inline(always)]
    pub fn uart6en(&mut self) -> UART6EN_W {
        UART6EN_W::new(self)
    }
    #[doc = "Bit 31 - UART7 clock enable"]
    #[inline(always)]
    pub fn uart7en(&mut self) -> UART7EN_W {
        UART7EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 additional enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addapb1en](index.html) module"]
pub struct ADDAPB1EN_SPEC;
impl crate::RegisterSpec for ADDAPB1EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addapb1en::R](R) reader structure"]
impl crate::Readable for ADDAPB1EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addapb1en::W](W) writer structure"]
impl crate::Writable for ADDAPB1EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDAPB1EN to value 0"]
impl crate::Resettable for ADDAPB1EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
