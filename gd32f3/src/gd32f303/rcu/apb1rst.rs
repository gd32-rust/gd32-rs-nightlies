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
#[doc = "Field `TIMER1RST` reader - TIMER1 timer reset"]
pub type TIMER1RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1RST` writer - TIMER1 timer reset"]
pub type TIMER1RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 0>;
#[doc = "Field `TIMER2RST` reader - TIMER2 timer reset"]
pub type TIMER2RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2RST` writer - TIMER2 timer reset"]
pub type TIMER2RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 1>;
#[doc = "Field `TIMER3RST` reader - TIMER3 timer reset"]
pub type TIMER3RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER3RST` writer - TIMER3 timer reset"]
pub type TIMER3RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 2>;
#[doc = "Field `TIMER4RST` reader - TIMER4 timer reset"]
pub type TIMER4RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER4RST` writer - TIMER4 timer reset"]
pub type TIMER4RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 3>;
#[doc = "Field `TIMER5RST` reader - TIMER5 timer reset"]
pub type TIMER5RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER5RST` writer - TIMER5 timer reset"]
pub type TIMER5RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 4>;
#[doc = "Field `TIMER6RST` reader - TIMER6 timer reset"]
pub type TIMER6RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER6RST` writer - TIMER6 timer reset"]
pub type TIMER6RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 5>;
#[doc = "Field `TIMER11RST` reader - TIMER11 timer reset"]
pub type TIMER11RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER11RST` writer - TIMER11 timer reset"]
pub type TIMER11RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 6>;
#[doc = "Field `TIMER12RST` reader - TIMER12 timer reset"]
pub type TIMER12RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER12RST` writer - TIMER12 timer reset"]
pub type TIMER12RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 7>;
#[doc = "Field `TIMER13RST` reader - TIMER13 timer reset"]
pub type TIMER13RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER13RST` writer - TIMER13 timer reset"]
pub type TIMER13RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 8>;
#[doc = "Field `WWDGTRST` reader - Window watchdog timer reset"]
pub type WWDGTRST_R = crate::BitReader<bool>;
#[doc = "Field `WWDGTRST` writer - Window watchdog timer reset"]
pub type WWDGTRST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 11>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type SPI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type SPI1RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 14>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type SPI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type SPI2RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 15>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader<bool>;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 17>;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub type USART2RST_R = crate::BitReader<bool>;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub type USART2RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 18>;
#[doc = "Field `UART3RST` reader - UART3 reset"]
pub type UART3RST_R = crate::BitReader<bool>;
#[doc = "Field `UART3RST` writer - UART3 reset"]
pub type UART3RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 19>;
#[doc = "Field `UART4RST` reader - UART4 reset"]
pub type UART4RST_R = crate::BitReader<bool>;
#[doc = "Field `UART4RST` writer - UART4 reset"]
pub type UART4RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 20>;
#[doc = "Field `I2C0RST` reader - I2C0 reset"]
pub type I2C0RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C0RST` writer - I2C0 reset"]
pub type I2C0RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 21>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2C1RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 22>;
#[doc = "Field `USBDRST` reader - USBD reset"]
pub type USBDRST_R = crate::BitReader<bool>;
#[doc = "Field `USBDRST` writer - USBD reset"]
pub type USBDRST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 23>;
#[doc = "Field `CAN0RST` reader - CAN0 reset"]
pub type CAN0RST_R = crate::BitReader<bool>;
#[doc = "Field `CAN0RST` writer - CAN0 reset"]
pub type CAN0RST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 25>;
#[doc = "Field `BKPIRST` reader - Backup interface reset"]
pub type BKPIRST_R = crate::BitReader<bool>;
#[doc = "Field `BKPIRST` writer - Backup interface reset"]
pub type BKPIRST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 27>;
#[doc = "Field `PMURST` reader - Power control reset"]
pub type PMURST_R = crate::BitReader<bool>;
#[doc = "Field `PMURST` writer - Power control reset"]
pub type PMURST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 28>;
#[doc = "Field `DACRST` reader - DAC reset"]
pub type DACRST_R = crate::BitReader<bool>;
#[doc = "Field `DACRST` writer - DAC reset"]
pub type DACRST_W<'a> = crate::BitWriter<'a, u32, APB1RST_SPEC, bool, 29>;
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
    #[doc = "Bit 2 - TIMER3 timer reset"]
    #[inline(always)]
    pub fn timer3rst(&self) -> TIMER3RST_R {
        TIMER3RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIMER4 timer reset"]
    #[inline(always)]
    pub fn timer4rst(&self) -> TIMER4RST_R {
        TIMER4RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer reset"]
    #[inline(always)]
    pub fn timer5rst(&self) -> TIMER5RST_R {
        TIMER5RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIMER6 timer reset"]
    #[inline(always)]
    pub fn timer6rst(&self) -> TIMER6RST_R {
        TIMER6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIMER11 timer reset"]
    #[inline(always)]
    pub fn timer11rst(&self) -> TIMER11RST_R {
        TIMER11RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIMER12 timer reset"]
    #[inline(always)]
    pub fn timer12rst(&self) -> TIMER12RST_R {
        TIMER12RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER13 timer reset"]
    #[inline(always)]
    pub fn timer13rst(&self) -> TIMER13RST_R {
        TIMER13RST_R::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bit 18 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART3 reset"]
    #[inline(always)]
    pub fn uart3rst(&self) -> UART3RST_R {
        UART3RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART4 reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 20) & 1) != 0)
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
    #[doc = "Bit 23 - USBD reset"]
    #[inline(always)]
    pub fn usbdrst(&self) -> USBDRST_R {
        USBDRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN0 reset"]
    #[inline(always)]
    pub fn can0rst(&self) -> CAN0RST_R {
        CAN0RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline(always)]
    pub fn bkpirst(&self) -> BKPIRST_R {
        BKPIRST_R::new(((self.bits >> 27) & 1) != 0)
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
    #[doc = "Bit 2 - TIMER3 timer reset"]
    #[inline(always)]
    pub fn timer3rst(&mut self) -> TIMER3RST_W {
        TIMER3RST_W::new(self)
    }
    #[doc = "Bit 3 - TIMER4 timer reset"]
    #[inline(always)]
    pub fn timer4rst(&mut self) -> TIMER4RST_W {
        TIMER4RST_W::new(self)
    }
    #[doc = "Bit 4 - TIMER5 timer reset"]
    #[inline(always)]
    pub fn timer5rst(&mut self) -> TIMER5RST_W {
        TIMER5RST_W::new(self)
    }
    #[doc = "Bit 5 - TIMER6 timer reset"]
    #[inline(always)]
    pub fn timer6rst(&mut self) -> TIMER6RST_W {
        TIMER6RST_W::new(self)
    }
    #[doc = "Bit 6 - TIMER11 timer reset"]
    #[inline(always)]
    pub fn timer11rst(&mut self) -> TIMER11RST_W {
        TIMER11RST_W::new(self)
    }
    #[doc = "Bit 7 - TIMER12 timer reset"]
    #[inline(always)]
    pub fn timer12rst(&mut self) -> TIMER12RST_W {
        TIMER12RST_W::new(self)
    }
    #[doc = "Bit 8 - TIMER13 timer reset"]
    #[inline(always)]
    pub fn timer13rst(&mut self) -> TIMER13RST_W {
        TIMER13RST_W::new(self)
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
    #[doc = "Bit 18 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 19 - UART3 reset"]
    #[inline(always)]
    pub fn uart3rst(&mut self) -> UART3RST_W {
        UART3RST_W::new(self)
    }
    #[doc = "Bit 20 - UART4 reset"]
    #[inline(always)]
    pub fn uart4rst(&mut self) -> UART4RST_W {
        UART4RST_W::new(self)
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
    #[doc = "Bit 23 - USBD reset"]
    #[inline(always)]
    pub fn usbdrst(&mut self) -> USBDRST_W {
        USBDRST_W::new(self)
    }
    #[doc = "Bit 25 - CAN0 reset"]
    #[inline(always)]
    pub fn can0rst(&mut self) -> CAN0RST_W {
        CAN0RST_W::new(self)
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline(always)]
    pub fn bkpirst(&mut self) -> BKPIRST_W {
        BKPIRST_W::new(self)
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
