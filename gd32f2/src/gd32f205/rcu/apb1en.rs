#[doc = "Register `APB1EN` reader"]
pub struct R(crate::R<APB1EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1EN` writer"]
pub struct W(crate::W<APB1EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1EN_SPEC>;
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
impl From<crate::W<APB1EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIMER1 timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TIMER1EN` reader - TIMER1 timer clock enable"]
pub type TIMER1EN_R = crate::BitReader<TIMER1EN_A>;
impl TIMER1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1EN_A {
        match self.bits {
            false => TIMER1EN_A::DISABLED,
            true => TIMER1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMER1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMER1EN_A::ENABLED
    }
}
#[doc = "Field `TIMER1EN` writer - TIMER1 timer clock enable"]
pub type TIMER1EN_W<'a> = crate::BitWriter<'a, u32, APB1EN_SPEC, TIMER1EN_A, 0>;
impl<'a> TIMER1EN_W<'a> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMER1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIMER1EN_A::ENABLED)
    }
}
#[doc = "TIMER2 timer clock enable"]
pub use TIMER1EN_A as TIMER2EN_A;
#[doc = "TIMER3 timer clock enable"]
pub use TIMER1EN_A as TIMER3EN_A;
#[doc = "TIMER4 timer clock enable"]
pub use TIMER1EN_A as TIMER4EN_A;
#[doc = "TIMER5 timer clock enable"]
pub use TIMER1EN_A as TIMER5EN_A;
#[doc = "TIMER6 timer clock enable"]
pub use TIMER1EN_A as TIMER6EN_A;
#[doc = "TIMER11 timer clock enable"]
pub use TIMER1EN_A as TIMER11EN_A;
#[doc = "TIMER12 timer clock enable"]
pub use TIMER1EN_A as TIMER12EN_A;
#[doc = "TIMER13 timer clock enable"]
pub use TIMER1EN_A as TIMER13EN_A;
#[doc = "Window watchdog timer clock enable"]
pub use TIMER1EN_A as WWDGTEN_A;
#[doc = "SPI1 clock enable"]
pub use TIMER1EN_A as SPI1EN_A;
#[doc = "SPI2 clock enable"]
pub use TIMER1EN_A as SPI2EN_A;
#[doc = "USART1 clock enable"]
pub use TIMER1EN_A as USART1EN_A;
#[doc = "USART2 clock enable"]
pub use TIMER1EN_A as USART2EN_A;
#[doc = "UART3 clock enable"]
pub use TIMER1EN_A as UART3EN_A;
#[doc = "UART4 clock enable"]
pub use TIMER1EN_A as UART4EN_A;
#[doc = "I2C0 clock enable"]
pub use TIMER1EN_A as I2C0EN_A;
#[doc = "I2C1 clock enable"]
pub use TIMER1EN_A as I2C1EN_A;
#[doc = "CAN0 clock enable"]
pub use TIMER1EN_A as CAN0EN_A;
#[doc = "CAN1 clock enable"]
pub use TIMER1EN_A as CAN1EN_A;
#[doc = "Backup interface clock enable"]
pub use TIMER1EN_A as BKPIEN_A;
#[doc = "Power control clock enable"]
pub use TIMER1EN_A as PMUEN_A;
#[doc = "DAC clock enable"]
pub use TIMER1EN_A as DACEN_A;
#[doc = "Field `TIMER2EN` reader - TIMER2 timer clock enable"]
pub use TIMER1EN_R as TIMER2EN_R;
#[doc = "Field `TIMER3EN` reader - TIMER3 timer clock enable"]
pub use TIMER1EN_R as TIMER3EN_R;
#[doc = "Field `TIMER4EN` reader - TIMER4 timer clock enable"]
pub use TIMER1EN_R as TIMER4EN_R;
#[doc = "Field `TIMER5EN` reader - TIMER5 timer clock enable"]
pub use TIMER1EN_R as TIMER5EN_R;
#[doc = "Field `TIMER6EN` reader - TIMER6 timer clock enable"]
pub use TIMER1EN_R as TIMER6EN_R;
#[doc = "Field `TIMER11EN` reader - TIMER11 timer clock enable"]
pub use TIMER1EN_R as TIMER11EN_R;
#[doc = "Field `TIMER12EN` reader - TIMER12 timer clock enable"]
pub use TIMER1EN_R as TIMER12EN_R;
#[doc = "Field `TIMER13EN` reader - TIMER13 timer clock enable"]
pub use TIMER1EN_R as TIMER13EN_R;
#[doc = "Field `WWDGTEN` reader - Window watchdog timer clock enable"]
pub use TIMER1EN_R as WWDGTEN_R;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub use TIMER1EN_R as SPI1EN_R;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub use TIMER1EN_R as SPI2EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use TIMER1EN_R as USART1EN_R;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub use TIMER1EN_R as USART2EN_R;
#[doc = "Field `UART3EN` reader - UART3 clock enable"]
pub use TIMER1EN_R as UART3EN_R;
#[doc = "Field `UART4EN` reader - UART4 clock enable"]
pub use TIMER1EN_R as UART4EN_R;
#[doc = "Field `I2C0EN` reader - I2C0 clock enable"]
pub use TIMER1EN_R as I2C0EN_R;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub use TIMER1EN_R as I2C1EN_R;
#[doc = "Field `CAN0EN` reader - CAN0 clock enable"]
pub use TIMER1EN_R as CAN0EN_R;
#[doc = "Field `CAN1EN` reader - CAN1 clock enable"]
pub use TIMER1EN_R as CAN1EN_R;
#[doc = "Field `BKPIEN` reader - Backup interface clock enable"]
pub use TIMER1EN_R as BKPIEN_R;
#[doc = "Field `PMUEN` reader - Power control clock enable"]
pub use TIMER1EN_R as PMUEN_R;
#[doc = "Field `DACEN` reader - DAC clock enable"]
pub use TIMER1EN_R as DACEN_R;
#[doc = "Field `TIMER2EN` writer - TIMER2 timer clock enable"]
pub use TIMER1EN_W as TIMER2EN_W;
#[doc = "Field `TIMER3EN` writer - TIMER3 timer clock enable"]
pub use TIMER1EN_W as TIMER3EN_W;
#[doc = "Field `TIMER4EN` writer - TIMER4 timer clock enable"]
pub use TIMER1EN_W as TIMER4EN_W;
#[doc = "Field `TIMER5EN` writer - TIMER5 timer clock enable"]
pub use TIMER1EN_W as TIMER5EN_W;
#[doc = "Field `TIMER6EN` writer - TIMER6 timer clock enable"]
pub use TIMER1EN_W as TIMER6EN_W;
#[doc = "Field `TIMER11EN` writer - TIMER11 timer clock enable"]
pub use TIMER1EN_W as TIMER11EN_W;
#[doc = "Field `TIMER12EN` writer - TIMER12 timer clock enable"]
pub use TIMER1EN_W as TIMER12EN_W;
#[doc = "Field `TIMER13EN` writer - TIMER13 timer clock enable"]
pub use TIMER1EN_W as TIMER13EN_W;
#[doc = "Field `WWDGTEN` writer - Window watchdog timer clock enable"]
pub use TIMER1EN_W as WWDGTEN_W;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub use TIMER1EN_W as SPI1EN_W;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub use TIMER1EN_W as SPI2EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use TIMER1EN_W as USART1EN_W;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub use TIMER1EN_W as USART2EN_W;
#[doc = "Field `UART3EN` writer - UART3 clock enable"]
pub use TIMER1EN_W as UART3EN_W;
#[doc = "Field `UART4EN` writer - UART4 clock enable"]
pub use TIMER1EN_W as UART4EN_W;
#[doc = "Field `I2C0EN` writer - I2C0 clock enable"]
pub use TIMER1EN_W as I2C0EN_W;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub use TIMER1EN_W as I2C1EN_W;
#[doc = "Field `CAN0EN` writer - CAN0 clock enable"]
pub use TIMER1EN_W as CAN0EN_W;
#[doc = "Field `CAN1EN` writer - CAN1 clock enable"]
pub use TIMER1EN_W as CAN1EN_W;
#[doc = "Field `BKPIEN` writer - Backup interface clock enable"]
pub use TIMER1EN_W as BKPIEN_W;
#[doc = "Field `PMUEN` writer - Power control clock enable"]
pub use TIMER1EN_W as PMUEN_W;
#[doc = "Field `DACEN` writer - DAC clock enable"]
pub use TIMER1EN_W as DACEN_W;
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
    #[doc = "Bit 2 - TIMER3 timer clock enable"]
    #[inline(always)]
    pub fn timer3en(&self) -> TIMER3EN_R {
        TIMER3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIMER4 timer clock enable"]
    #[inline(always)]
    pub fn timer4en(&self) -> TIMER4EN_R {
        TIMER4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&self) -> TIMER5EN_R {
        TIMER5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIMER6 timer clock enable"]
    #[inline(always)]
    pub fn timer6en(&self) -> TIMER6EN_R {
        TIMER6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIMER11 timer clock enable"]
    #[inline(always)]
    pub fn timer11en(&self) -> TIMER11EN_R {
        TIMER11EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIMER12 timer clock enable"]
    #[inline(always)]
    pub fn timer12en(&self) -> TIMER12EN_R {
        TIMER12EN_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 18 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART3 clock enable"]
    #[inline(always)]
    pub fn uart3en(&self) -> UART3EN_R {
        UART3EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 20) & 1) != 0)
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
    #[doc = "Bit 27 - Backup interface clock enable"]
    #[inline(always)]
    pub fn bkpien(&self) -> BKPIEN_R {
        BKPIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power control clock enable"]
    #[inline(always)]
    pub fn pmuen(&self) -> PMUEN_R {
        PMUEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER1 timer clock enable"]
    #[inline(always)]
    pub fn timer1en(&mut self) -> TIMER1EN_W {
        TIMER1EN_W::new(self)
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&mut self) -> TIMER2EN_W {
        TIMER2EN_W::new(self)
    }
    #[doc = "Bit 2 - TIMER3 timer clock enable"]
    #[inline(always)]
    pub fn timer3en(&mut self) -> TIMER3EN_W {
        TIMER3EN_W::new(self)
    }
    #[doc = "Bit 3 - TIMER4 timer clock enable"]
    #[inline(always)]
    pub fn timer4en(&mut self) -> TIMER4EN_W {
        TIMER4EN_W::new(self)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&mut self) -> TIMER5EN_W {
        TIMER5EN_W::new(self)
    }
    #[doc = "Bit 5 - TIMER6 timer clock enable"]
    #[inline(always)]
    pub fn timer6en(&mut self) -> TIMER6EN_W {
        TIMER6EN_W::new(self)
    }
    #[doc = "Bit 6 - TIMER11 timer clock enable"]
    #[inline(always)]
    pub fn timer11en(&mut self) -> TIMER11EN_W {
        TIMER11EN_W::new(self)
    }
    #[doc = "Bit 7 - TIMER12 timer clock enable"]
    #[inline(always)]
    pub fn timer12en(&mut self) -> TIMER12EN_W {
        TIMER12EN_W::new(self)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&mut self) -> TIMER13EN_W {
        TIMER13EN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdgten(&mut self) -> WWDGTEN_W {
        WWDGTEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 15 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 18 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 19 - UART3 clock enable"]
    #[inline(always)]
    pub fn uart3en(&mut self) -> UART3EN_W {
        UART3EN_W::new(self)
    }
    #[doc = "Bit 20 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&mut self) -> UART4EN_W {
        UART4EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&mut self) -> I2C0EN_W {
        I2C0EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 25 - CAN0 clock enable"]
    #[inline(always)]
    pub fn can0en(&mut self) -> CAN0EN_W {
        CAN0EN_W::new(self)
    }
    #[doc = "Bit 26 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&mut self) -> CAN1EN_W {
        CAN1EN_W::new(self)
    }
    #[doc = "Bit 27 - Backup interface clock enable"]
    #[inline(always)]
    pub fn bkpien(&mut self) -> BKPIEN_W {
        BKPIEN_W::new(self)
    }
    #[doc = "Bit 28 - Power control clock enable"]
    #[inline(always)]
    pub fn pmuen(&mut self) -> PMUEN_W {
        PMUEN_W::new(self)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 clock enable register (RCU_APB1EN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1en](index.html) module"]
pub struct APB1EN_SPEC;
impl crate::RegisterSpec for APB1EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1en::R](R) reader structure"]
impl crate::Readable for APB1EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1en::W](W) writer structure"]
impl crate::Writable for APB1EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1EN to value 0"]
impl crate::Resettable for APB1EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
