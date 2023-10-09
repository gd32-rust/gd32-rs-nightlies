#[doc = "Register `APB1EN` reader"]
pub type R = crate::R<APB1EN_SPEC>;
#[doc = "Register `APB1EN` writer"]
pub type W = crate::W<APB1EN_SPEC>;
#[doc = "Field `TIMER1EN` reader - TIMER1 timer clock enable"]
pub type TIMER1EN_R = crate::BitReader<TIMER1EN_A>;
#[doc = "TIMER1 timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER1EN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<TIMER1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1EN_A {
        match self.bits {
            false => TIMER1EN_A::DISABLED,
            true => TIMER1EN_A::ENABLED,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMER1EN_A::DISABLED
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMER1EN_A::ENABLED
    }
}
#[doc = "Field `TIMER1EN` writer - TIMER1 timer clock enable"]
pub type TIMER1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER1EN_A>;
impl<'a, REG, const O: u8> TIMER1EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER1EN_A::ENABLED)
    }
}
#[doc = "Field `TIMER2EN` reader - TIMER2 timer clock enable"]
pub use TIMER1EN_R as TIMER2EN_R;
#[doc = "Field `TIMER5EN` reader - TIMER5 timer clock enable"]
pub use TIMER1EN_R as TIMER5EN_R;
#[doc = "Field `TIMER13EN` reader - TIMER13 timer clock enable"]
pub use TIMER1EN_R as TIMER13EN_R;
#[doc = "Field `SLCDEN` reader - SLCD clock enable"]
pub use TIMER1EN_R as SLCDEN_R;
#[doc = "Field `WWDGTEN` reader - Window watchdog timer clock enable"]
pub use TIMER1EN_R as WWDGTEN_R;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub use TIMER1EN_R as SPI1EN_R;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub use TIMER1EN_R as SPI2EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use TIMER1EN_R as USART1EN_R;
#[doc = "Field `I2C0EN` reader - I2C0 clock enable"]
pub use TIMER1EN_R as I2C0EN_R;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub use TIMER1EN_R as I2C1EN_R;
#[doc = "Field `CAN0EN` reader - CAN0 clock enable"]
pub use TIMER1EN_R as CAN0EN_R;
#[doc = "Field `CAN1EN` reader - CAN1 clock enable"]
pub use TIMER1EN_R as CAN1EN_R;
#[doc = "Field `PMUEN` reader - Power interface clock enable"]
pub use TIMER1EN_R as PMUEN_R;
#[doc = "Field `DACEN` reader - DAC interface clock enable"]
pub use TIMER1EN_R as DACEN_R;
#[doc = "Field `CECEN` reader - HDMI CEC interface clock enable"]
pub use TIMER1EN_R as CECEN_R;
#[doc = "Field `OPAIVREFEN` reader - OPA and IVREF clock enable"]
pub use TIMER1EN_R as OPAIVREFEN_R;
#[doc = "Field `TIMER2EN` writer - TIMER2 timer clock enable"]
pub use TIMER1EN_W as TIMER2EN_W;
#[doc = "Field `TIMER5EN` writer - TIMER5 timer clock enable"]
pub use TIMER1EN_W as TIMER5EN_W;
#[doc = "Field `TIMER13EN` writer - TIMER13 timer clock enable"]
pub use TIMER1EN_W as TIMER13EN_W;
#[doc = "Field `SLCDEN` writer - SLCD clock enable"]
pub use TIMER1EN_W as SLCDEN_W;
#[doc = "Field `WWDGTEN` writer - Window watchdog timer clock enable"]
pub use TIMER1EN_W as WWDGTEN_W;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub use TIMER1EN_W as SPI1EN_W;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub use TIMER1EN_W as SPI2EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use TIMER1EN_W as USART1EN_W;
#[doc = "Field `I2C0EN` writer - I2C0 clock enable"]
pub use TIMER1EN_W as I2C0EN_W;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub use TIMER1EN_W as I2C1EN_W;
#[doc = "Field `CAN0EN` writer - CAN0 clock enable"]
pub use TIMER1EN_W as CAN0EN_W;
#[doc = "Field `CAN1EN` writer - CAN1 clock enable"]
pub use TIMER1EN_W as CAN1EN_W;
#[doc = "Field `PMUEN` writer - Power interface clock enable"]
pub use TIMER1EN_W as PMUEN_W;
#[doc = "Field `DACEN` writer - DAC interface clock enable"]
pub use TIMER1EN_W as DACEN_W;
#[doc = "Field `CECEN` writer - HDMI CEC interface clock enable"]
pub use TIMER1EN_W as CECEN_W;
#[doc = "Field `OPAIVREFEN` writer - OPA and IVREF clock enable"]
pub use TIMER1EN_W as OPAIVREFEN_W;
impl R {
    #[doc = "Bit 0 - TIMER1 timer clock enable"]
    #[inline(always)]
    pub fn timer1en(&self) -> TIMER1EN_R {
        TIMER1EN_R::new((self.bits & 1) != 0)
    }
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
    #[doc = "Bit 9 - SLCD clock enable"]
    #[inline(always)]
    pub fn slcden(&self) -> SLCDEN_R {
        SLCDEN_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 15 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 25 - CAN0 clock enable"]
    #[inline(always)]
    pub fn can0en(&self) -> CAN0EN_R {
        CAN0EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&self) -> CAN1EN_R {
        CAN1EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pmuen(&self) -> PMUEN_R {
        PMUEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - HDMI CEC interface clock enable"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA and IVREF clock enable"]
    #[inline(always)]
    pub fn opaivrefen(&self) -> OPAIVREFEN_R {
        OPAIVREFEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER1 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer1en(&mut self) -> TIMER1EN_W<APB1EN_SPEC, 0> {
        TIMER1EN_W::new(self)
    }
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
    #[doc = "Bit 9 - SLCD clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn slcden(&mut self) -> SLCDEN_W<APB1EN_SPEC, 9> {
        SLCDEN_W::new(self)
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
    #[doc = "Bit 15 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1EN_SPEC, 15> {
        SPI2EN_W::new(self)
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
    #[doc = "Bit 25 - CAN0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can0en(&mut self) -> CAN0EN_W<APB1EN_SPEC, 25> {
        CAN0EN_W::new(self)
    }
    #[doc = "Bit 26 - CAN1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can1en(&mut self) -> CAN1EN_W<APB1EN_SPEC, 26> {
        CAN1EN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmuen(&mut self) -> PMUEN_W<APB1EN_SPEC, 28> {
        PMUEN_W::new(self)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<APB1EN_SPEC, 29> {
        DACEN_W::new(self)
    }
    #[doc = "Bit 30 - HDMI CEC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<APB1EN_SPEC, 30> {
        CECEN_W::new(self)
    }
    #[doc = "Bit 31 - OPA and IVREF clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn opaivrefen(&mut self) -> OPAIVREFEN_W<APB1EN_SPEC, 31> {
        OPAIVREFEN_W::new(self)
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
