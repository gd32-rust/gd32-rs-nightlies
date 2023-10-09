#[doc = "Register `APB1EN` reader"]
pub type R = crate::R<APB1EN_SPEC>;
#[doc = "Register `APB1EN` writer"]
pub type W = crate::W<APB1EN_SPEC>;
#[doc = "Field `TIMER2EN` reader - TIMER2 timer clock enable"]
pub type TIMER2EN_R = crate::BitReader<TIMER2EN_A>;
#[doc = "TIMER2 timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER2EN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<TIMER2EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER2EN_A {
        match self.bits {
            false => TIMER2EN_A::DISABLED,
            true => TIMER2EN_A::ENABLED,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMER2EN_A::DISABLED
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMER2EN_A::ENABLED
    }
}
#[doc = "Field `TIMER2EN` writer - TIMER2 timer clock enable"]
pub type TIMER2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER2EN_A>;
impl<'a, REG, const O: u8> TIMER2EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER2EN_A::ENABLED)
    }
}
#[doc = "Field `TIMER5EN` reader - TIMER5 timer clock enable"]
pub use TIMER2EN_R as TIMER5EN_R;
#[doc = "Field `TIMER13EN` reader - TIMER13 timer clock enable"]
pub use TIMER2EN_R as TIMER13EN_R;
#[doc = "Field `WWDGTEN` reader - Window watchdog timer clock enable"]
pub use TIMER2EN_R as WWDGTEN_R;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub use TIMER2EN_R as SPI1EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use TIMER2EN_R as USART1EN_R;
#[doc = "Field `I2C0EN` reader - I2C0 clock enable"]
pub use TIMER2EN_R as I2C0EN_R;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub use TIMER2EN_R as I2C1EN_R;
#[doc = "Field `PMUEN` reader - Power interface clock enable"]
pub use TIMER2EN_R as PMUEN_R;
#[doc = "Field `TIMER5EN` writer - TIMER5 timer clock enable"]
pub use TIMER2EN_W as TIMER5EN_W;
#[doc = "Field `TIMER13EN` writer - TIMER13 timer clock enable"]
pub use TIMER2EN_W as TIMER13EN_W;
#[doc = "Field `WWDGTEN` writer - Window watchdog timer clock enable"]
pub use TIMER2EN_W as WWDGTEN_W;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub use TIMER2EN_W as SPI1EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use TIMER2EN_W as USART1EN_W;
#[doc = "Field `I2C0EN` writer - I2C0 clock enable"]
pub use TIMER2EN_W as I2C0EN_W;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub use TIMER2EN_W as I2C1EN_W;
#[doc = "Field `PMUEN` writer - Power interface clock enable"]
pub use TIMER2EN_W as PMUEN_W;
impl R {
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&self) -> TIMER2EN_R {
        TIMER2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&self) -> TIMER5EN_R {
        TIMER5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&self) -> TIMER13EN_R {
        TIMER13EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdgten(&self) -> WWDGTEN_R {
        WWDGTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2C0EN_R {
        I2C0EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pmuen(&self) -> PMUEN_R {
        PMUEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer2en(&mut self) -> TIMER2EN_W<APB1EN_SPEC, 1> {
        TIMER2EN_W::new(self)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer5en(&mut self) -> TIMER5EN_W<APB1EN_SPEC, 4> {
        TIMER5EN_W::new(self)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer13en(&mut self) -> TIMER13EN_W<APB1EN_SPEC, 8> {
        TIMER13EN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgten(&mut self) -> WWDGTEN_W<APB1EN_SPEC, 11> {
        WWDGTEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB1EN_SPEC, 14> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB1EN_SPEC, 17> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0en(&mut self) -> I2C0EN_W<APB1EN_SPEC, 21> {
        I2C0EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1EN_SPEC, 22> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmuen(&mut self) -> PMUEN_W<APB1EN_SPEC, 28> {
        PMUEN_W::new(self)
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
#[doc = "APB1 enable register (RCU_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1EN_SPEC;
impl crate::RegisterSpec for APB1EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1en::R`](R) reader structure"]
impl crate::Readable for APB1EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1en::W`](W) writer structure"]
impl crate::Writable for APB1EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1EN to value 0"]
impl crate::Resettable for APB1EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
