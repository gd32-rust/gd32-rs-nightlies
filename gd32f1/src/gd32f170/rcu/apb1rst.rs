#[doc = "Register `APB1RST` reader"]
pub struct R(crate::R<APB1RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1RST` writer"]
pub struct W(crate::W<APB1RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1RST_SPEC>;
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
impl From<crate::W<APB1RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIMER1 timer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1RST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<TIMER1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1RST` reader - TIMER1 timer reset"]
pub type TIMER1RST_R = crate::BitReader<TIMER1RST_A>;
impl TIMER1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMER1RST_A> {
        match self.bits {
            true => Some(TIMER1RST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIMER1RST_A::RESET
    }
}
#[doc = "Field `TIMER1RST` writer - TIMER1 timer reset"]
pub type TIMER1RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, TIMER1RST_A, 0>;
impl<'a> TIMER1RST_W<'a> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER1RST_A::RESET)
    }
}
#[doc = "TIMER2 timer reset"]
pub use TIMER1RST_A as TIMER2RST_A;
#[doc = "TIMER5 timer reset"]
pub use TIMER1RST_A as TIMER5RST_A;
#[doc = "TIMER13 timer reset"]
pub use TIMER1RST_A as TIMER13RST_A;
#[doc = "SLCD reset"]
pub use TIMER1RST_A as SLCDRST_A;
#[doc = "Window watchdog timer reset"]
pub use TIMER1RST_A as WWDGTRST_A;
#[doc = "SPI1 reset"]
pub use TIMER1RST_A as SPI1RST_A;
#[doc = "SPI2 reset"]
pub use TIMER1RST_A as SPI2RST_A;
#[doc = "USART1 reset"]
pub use TIMER1RST_A as USART1RST_A;
#[doc = "I2C0 reset"]
pub use TIMER1RST_A as I2C0RST_A;
#[doc = "I2C1 reset"]
pub use TIMER1RST_A as I2C1RST_A;
#[doc = "CAN0 reset"]
pub use TIMER1RST_A as CAN0RST_A;
#[doc = "CAN1 reset"]
pub use TIMER1RST_A as CAN1RST_A;
#[doc = "Power control reset"]
pub use TIMER1RST_A as PMURST_A;
#[doc = "DAC reset"]
pub use TIMER1RST_A as DACRST_A;
#[doc = "HDMI CEC reset"]
pub use TIMER1RST_A as CECRST_A;
#[doc = "OPA and IVREF reset"]
pub use TIMER1RST_A as OPAIVREFRST_A;
#[doc = "Field `TIMER2RST` reader - TIMER2 timer reset"]
pub use TIMER1RST_R as TIMER2RST_R;
#[doc = "Field `TIMER5RST` reader - TIMER5 timer reset"]
pub use TIMER1RST_R as TIMER5RST_R;
#[doc = "Field `TIMER13RST` reader - TIMER13 timer reset"]
pub use TIMER1RST_R as TIMER13RST_R;
#[doc = "Field `SLCDRST` reader - SLCD reset"]
pub use TIMER1RST_R as SLCDRST_R;
#[doc = "Field `WWDGTRST` reader - Window watchdog timer reset"]
pub use TIMER1RST_R as WWDGTRST_R;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub use TIMER1RST_R as SPI1RST_R;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub use TIMER1RST_R as SPI2RST_R;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use TIMER1RST_R as USART1RST_R;
#[doc = "Field `I2C0RST` reader - I2C0 reset"]
pub use TIMER1RST_R as I2C0RST_R;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub use TIMER1RST_R as I2C1RST_R;
#[doc = "Field `CAN0RST` reader - CAN0 reset"]
pub use TIMER1RST_R as CAN0RST_R;
#[doc = "Field `CAN1RST` reader - CAN1 reset"]
pub use TIMER1RST_R as CAN1RST_R;
#[doc = "Field `PMURST` reader - Power control reset"]
pub use TIMER1RST_R as PMURST_R;
#[doc = "Field `DACRST` reader - DAC reset"]
pub use TIMER1RST_R as DACRST_R;
#[doc = "Field `CECRST` reader - HDMI CEC reset"]
pub use TIMER1RST_R as CECRST_R;
#[doc = "Field `OPAIVREFRST` reader - OPA and IVREF reset"]
pub use TIMER1RST_R as OPAIVREFRST_R;
#[doc = "Field `TIMER2RST` writer - TIMER2 timer reset"]
pub use TIMER1RST_W as TIMER2RST_W;
#[doc = "Field `TIMER5RST` writer - TIMER5 timer reset"]
pub use TIMER1RST_W as TIMER5RST_W;
#[doc = "Field `TIMER13RST` writer - TIMER13 timer reset"]
pub use TIMER1RST_W as TIMER13RST_W;
#[doc = "Field `SLCDRST` writer - SLCD reset"]
pub use TIMER1RST_W as SLCDRST_W;
#[doc = "Field `WWDGTRST` writer - Window watchdog timer reset"]
pub use TIMER1RST_W as WWDGTRST_W;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub use TIMER1RST_W as SPI1RST_W;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub use TIMER1RST_W as SPI2RST_W;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use TIMER1RST_W as USART1RST_W;
#[doc = "Field `I2C0RST` writer - I2C0 reset"]
pub use TIMER1RST_W as I2C0RST_W;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub use TIMER1RST_W as I2C1RST_W;
#[doc = "Field `CAN0RST` writer - CAN0 reset"]
pub use TIMER1RST_W as CAN0RST_W;
#[doc = "Field `CAN1RST` writer - CAN1 reset"]
pub use TIMER1RST_W as CAN1RST_W;
#[doc = "Field `PMURST` writer - Power control reset"]
pub use TIMER1RST_W as PMURST_W;
#[doc = "Field `DACRST` writer - DAC reset"]
pub use TIMER1RST_W as DACRST_W;
#[doc = "Field `CECRST` writer - HDMI CEC reset"]
pub use TIMER1RST_W as CECRST_W;
#[doc = "Field `OPAIVREFRST` writer - OPA and IVREF reset"]
pub use TIMER1RST_W as OPAIVREFRST_W;
impl R {
    #[doc = "Bit 0 - TIMER1 timer reset"]
    #[inline(always)]
    pub fn timer1rst(&self) -> TIMER1RST_R {
        TIMER1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER2 timer reset"]
    #[inline(always)]
    pub fn timer2rst(&self) -> TIMER2RST_R {
        TIMER2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer reset"]
    #[inline(always)]
    pub fn timer5rst(&self) -> TIMER5RST_R {
        TIMER5RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER13 timer reset"]
    #[inline(always)]
    pub fn timer13rst(&self) -> TIMER13RST_R {
        TIMER13RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SLCD reset"]
    #[inline(always)]
    pub fn slcdrst(&self) -> SLCDRST_R {
        SLCDRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    pub fn wwdgtrst(&self) -> WWDGTRST_R {
        WWDGTRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 reset"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2C0RST_R {
        I2C0RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN0 reset"]
    #[inline(always)]
    pub fn can0rst(&self) -> CAN0RST_R {
        CAN0RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&self) -> CAN1RST_R {
        CAN1RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power control reset"]
    #[inline(always)]
    pub fn pmurst(&self) -> PMURST_R {
        PMURST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - HDMI CEC reset"]
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA and IVREF reset"]
    #[inline(always)]
    pub fn opaivrefrst(&self) -> OPAIVREFRST_R {
        OPAIVREFRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER1 timer reset"]
    #[inline(always)]
    pub fn timer1rst(&mut self) -> TIMER1RST_W {
        TIMER1RST_W::new(self)
    }
    #[doc = "Bit 1 - TIMER2 timer reset"]
    #[inline(always)]
    pub fn timer2rst(&mut self) -> TIMER2RST_W {
        TIMER2RST_W::new(self)
    }
    #[doc = "Bit 4 - TIMER5 timer reset"]
    #[inline(always)]
    pub fn timer5rst(&mut self) -> TIMER5RST_W {
        TIMER5RST_W::new(self)
    }
    #[doc = "Bit 8 - TIMER13 timer reset"]
    #[inline(always)]
    pub fn timer13rst(&mut self) -> TIMER13RST_W {
        TIMER13RST_W::new(self)
    }
    #[doc = "Bit 9 - SLCD reset"]
    #[inline(always)]
    pub fn slcdrst(&mut self) -> SLCDRST_W {
        SLCDRST_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    pub fn wwdgtrst(&mut self) -> WWDGTRST_W {
        WWDGTRST_W::new(self)
    }
    #[doc = "Bit 14 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 17 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C0 reset"]
    #[inline(always)]
    pub fn i2c0rst(&mut self) -> I2C0RST_W {
        I2C0RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 25 - CAN0 reset"]
    #[inline(always)]
    pub fn can0rst(&mut self) -> CAN0RST_W {
        CAN0RST_W::new(self)
    }
    #[doc = "Bit 26 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&mut self) -> CAN1RST_W {
        CAN1RST_W::new(self)
    }
    #[doc = "Bit 28 - Power control reset"]
    #[inline(always)]
    pub fn pmurst(&mut self) -> PMURST_W {
        PMURST_W::new(self)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W {
        DACRST_W::new(self)
    }
    #[doc = "Bit 30 - HDMI CEC reset"]
    #[inline(always)]
    pub fn cecrst(&mut self) -> CECRST_W {
        CECRST_W::new(self)
    }
    #[doc = "Bit 31 - OPA and IVREF reset"]
    #[inline(always)]
    pub fn opaivrefrst(&mut self) -> OPAIVREFRST_W {
        OPAIVREFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 reset register (RCU_APB1RST)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rst](index.html) module"]
pub struct APB1RST_SPEC;
impl crate::RegisterSpec for APB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1rst::R](R) reader structure"]
impl crate::Readable for APB1RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1rst::W](W) writer structure"]
impl crate::Writable for APB1RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1RST to value 0"]
impl crate::Resettable for APB1RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
