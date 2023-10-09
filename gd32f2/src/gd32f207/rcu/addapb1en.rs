#[doc = "Register `ADDAPB1EN` reader"]
pub type R = crate::R<ADDAPB1EN_SPEC>;
#[doc = "Register `ADDAPB1EN` writer"]
pub type W = crate::W<ADDAPB1EN_SPEC>;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
pub type I2C2EN_R = crate::BitReader<I2C2EN_A>;
#[doc = "I2C2 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl I2C2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2EN_A {
        match self.bits {
            false => I2C2EN_A::DISABLED,
            true => I2C2EN_A::ENABLED,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C2EN_A::DISABLED
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C2EN_A::ENABLED
    }
}
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub type I2C2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2C2EN_A>;
impl<'a, REG, const O: u8> I2C2EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2EN_A::ENABLED)
    }
}
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
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<ADDAPB1EN_SPEC, 23> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 30 - UART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart6en(&mut self) -> UART6EN_W<ADDAPB1EN_SPEC, 30> {
        UART6EN_W::new(self)
    }
    #[doc = "Bit 31 - UART7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart7en(&mut self) -> UART7EN_W<ADDAPB1EN_SPEC, 31> {
        UART7EN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB1 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDAPB1EN_SPEC;
impl crate::RegisterSpec for ADDAPB1EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1en::R`](R) reader structure"]
impl crate::Readable for ADDAPB1EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addapb1en::W`](W) writer structure"]
impl crate::Writable for ADDAPB1EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDAPB1EN to value 0"]
impl crate::Resettable for ADDAPB1EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
