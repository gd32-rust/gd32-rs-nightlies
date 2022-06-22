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
#[doc = "Field `PMUEN` reader - Power interface clock enable"]
pub type PMUEN_R = crate::BitReader<bool>;
#[doc = "Field `PMUEN` writer - Power interface clock enable"]
pub type PMUEN_W<'a> = crate::BitWriter<'a, u32, APB1EN_SPEC, bool, 28>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub type I2C1EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub type I2C1EN_W<'a> = crate::BitWriter<'a, u32, APB1EN_SPEC, bool, 22>;
#[doc = "Field `I2C0EN` reader - I2C0 clock enable"]
pub type I2C0EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C0EN` writer - I2C0 clock enable"]
pub type I2C0EN_W<'a> = crate::BitWriter<'a, u32, APB1EN_SPEC, bool, 21>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type USART1EN_R = crate::BitReader<bool>;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type USART1EN_W<'a> = crate::BitWriter<'a, u32, APB1EN_SPEC, bool, 17>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type SPI1EN_W<'a> = crate::BitWriter<'a, u32, APB1EN_SPEC, bool, 14>;
#[doc = "Field `WWDGTEN` reader - Window watchdog timer clock enable"]
pub type WWDGTEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGTEN` writer - Window watchdog timer clock enable"]
pub type WWDGTEN_W<'a> = crate::BitWriter<'a, u32, APB1EN_SPEC, bool, 11>;
#[doc = "Field `TIMER13EN` reader - TIMER13 timer clock enable"]
pub type TIMER13EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER13EN` writer - TIMER13 timer clock enable"]
pub type TIMER13EN_W<'a> = crate::BitWriter<'a, u32, APB1EN_SPEC, bool, 8>;
#[doc = "Field `TIMER5EN` reader - TIMER5 timer clock enable"]
pub type TIMER5EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER5EN` writer - TIMER5 timer clock enable"]
pub type TIMER5EN_W<'a> = crate::BitWriter<'a, u32, APB1EN_SPEC, bool, 4>;
#[doc = "Field `TIMER2EN` reader - TIMER2 timer clock enable"]
pub type TIMER2EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2EN` writer - TIMER2 timer clock enable"]
pub type TIMER2EN_W<'a> = crate::BitWriter<'a, u32, APB1EN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pmuen(&self) -> PMUEN_R {
        PMUEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2C0EN_R {
        I2C0EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdgten(&self) -> WWDGTEN_R {
        WWDGTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&self) -> TIMER13EN_R {
        TIMER13EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&self) -> TIMER5EN_R {
        TIMER5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&self) -> TIMER2EN_R {
        TIMER2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pmuen(&mut self) -> PMUEN_W {
        PMUEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&mut self) -> I2C0EN_W {
        I2C0EN_W::new(self)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdgten(&mut self) -> WWDGTEN_W {
        WWDGTEN_W::new(self)
    }
    #[doc = "Bit 8 - TIMER13 timer clock enable"]
    #[inline(always)]
    pub fn timer13en(&mut self) -> TIMER13EN_W {
        TIMER13EN_W::new(self)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&mut self) -> TIMER5EN_W {
        TIMER5EN_W::new(self)
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&mut self) -> TIMER2EN_W {
        TIMER2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 enable register (RCU_APB1EN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1en](index.html) module"]
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
