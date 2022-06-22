#[doc = "Register `PCF0` reader"]
pub struct R(crate::R<PCF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCF0` writer"]
pub struct W(crate::W<PCF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCF0_SPEC>;
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
impl From<crate::W<PCF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWJ_CFG` reader - Serial wire JTAG configuration"]
pub type SWJ_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWJ_CFG` writer - Serial wire JTAG configuration"]
pub type SWJ_CFG_W<'a> = crate::FieldWriter<'a, u32, PCF0_SPEC, u8, u8, 3, 24>;
#[doc = "Field `ADC1_ETRGREG_REMAP` reader - ADC 1 external trigger regular conversion remapping"]
pub type ADC1_ETRGREG_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `ADC1_ETRGREG_REMAP` writer - ADC 1 external trigger regular conversion remapping"]
pub type ADC1_ETRGREG_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 20>;
#[doc = "Field `ADC1_ETRGINJ_REMAP` reader - ADC 1 external trigger injected conversion remapping"]
pub type ADC1_ETRGINJ_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `ADC1_ETRGINJ_REMAP` writer - ADC 1 external trigger injected conversion remapping"]
pub type ADC1_ETRGINJ_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 19>;
#[doc = "Field `ADC0_ETRGREG_REMAP` reader - ADC 0 external trigger regular conversion remapping"]
pub type ADC0_ETRGREG_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `ADC0_ETRGREG_REMAP` writer - ADC 0 external trigger regular conversion remapping"]
pub type ADC0_ETRGREG_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 18>;
#[doc = "Field `ADC0_ETRGINJ_REMAP` reader - ADC 0 external trigger injected conversion remapping"]
pub type ADC0_ETRGINJ_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `ADC0_ETRGINJ_REMAP` writer - ADC 0 external trigger injected conversion remapping"]
pub type ADC0_ETRGINJ_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 17>;
#[doc = "Field `TIMER4CH3_IREMAP` reader - TIMER4 channel3 internal remapping"]
pub type TIMER4CH3_IREMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIMER4CH3_IREMAP` writer - TIMER4 channel3 internal remapping"]
pub type TIMER4CH3_IREMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 16>;
#[doc = "Field `PD01_REMAP` reader - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
pub type PD01_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `PD01_REMAP` writer - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
pub type PD01_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 15>;
#[doc = "Field `CAN_REMAP` reader - CAN interface remapping"]
pub type CAN_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN_REMAP` writer - CAN interface remapping"]
pub type CAN_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF0_SPEC, u8, u8, 2, 13>;
#[doc = "Field `TIMER3_REMAP` reader - TIMER3 remapping"]
pub type TIMER3_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIMER3_REMAP` writer - TIMER3 remapping"]
pub type TIMER3_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 12>;
#[doc = "Field `TIMER2_REMAP` reader - TIMER2 remapping"]
pub type TIMER2_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER2_REMAP` writer - TIMER2 remapping"]
pub type TIMER2_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF0_SPEC, u8, u8, 2, 10>;
#[doc = "Field `TIMER1_REMAP` reader - TIMER1 remapping"]
pub type TIMER1_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER1_REMAP` writer - TIMER1 remapping"]
pub type TIMER1_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF0_SPEC, u8, u8, 2, 8>;
#[doc = "Field `TIMER0_REMAP` reader - TIMER0 remapping"]
pub type TIMER0_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER0_REMAP` writer - TIMER0 remapping"]
pub type TIMER0_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF0_SPEC, u8, u8, 2, 6>;
#[doc = "Field `USART2_REMAP` reader - USART2 remapping"]
pub type USART2_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART2_REMAP` writer - USART2 remapping"]
pub type USART2_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF0_SPEC, u8, u8, 2, 4>;
#[doc = "Field `USART1_REMAP` reader - USART1 remapping"]
pub type USART1_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `USART1_REMAP` writer - USART1 remapping"]
pub type USART1_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 3>;
#[doc = "Field `USART0_REMAP` reader - USART0 remapping"]
pub type USART0_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `USART0_REMAP` writer - USART0 remapping"]
pub type USART0_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 2>;
#[doc = "Field `I2C0_REMAP` reader - I2C0 remapping"]
pub type I2C0_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `I2C0_REMAP` writer - I2C0 remapping"]
pub type I2C0_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 1>;
#[doc = "Field `SPI0_REMAP` reader - SPI0 remapping"]
pub type SPI0_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `SPI0_REMAP` writer - SPI0 remapping"]
pub type SPI0_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF0_SPEC, bool, 0>;
impl R {
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    pub fn swj_cfg(&self) -> SWJ_CFG_R {
        SWJ_CFG_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 20 - ADC 1 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrgreg_remap(&self) -> ADC1_ETRGREG_REMAP_R {
        ADC1_ETRGREG_REMAP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC 1 external trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrginj_remap(&self) -> ADC1_ETRGINJ_REMAP_R {
        ADC1_ETRGINJ_REMAP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC 0 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc0_etrgreg_remap(&self) -> ADC0_ETRGREG_REMAP_R {
        ADC0_ETRGREG_REMAP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC 0 external trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc0_etrginj_remap(&self) -> ADC0_ETRGINJ_REMAP_R {
        ADC0_ETRGINJ_REMAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER4 channel3 internal remapping"]
    #[inline(always)]
    pub fn timer4ch3_iremap(&self) -> TIMER4CH3_IREMAP_R {
        TIMER4CH3_IREMAP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
    #[inline(always)]
    pub fn pd01_remap(&self) -> PD01_REMAP_R {
        PD01_REMAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 13:14 - CAN interface remapping"]
    #[inline(always)]
    pub fn can_remap(&self) -> CAN_REMAP_R {
        CAN_REMAP_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 12 - TIMER3 remapping"]
    #[inline(always)]
    pub fn timer3_remap(&self) -> TIMER3_REMAP_R {
        TIMER3_REMAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 10:11 - TIMER2 remapping"]
    #[inline(always)]
    pub fn timer2_remap(&self) -> TIMER2_REMAP_R {
        TIMER2_REMAP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIMER1 remapping"]
    #[inline(always)]
    pub fn timer1_remap(&self) -> TIMER1_REMAP_R {
        TIMER1_REMAP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TIMER0 remapping"]
    #[inline(always)]
    pub fn timer0_remap(&self) -> TIMER0_REMAP_R {
        TIMER0_REMAP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2_remap(&self) -> USART2_REMAP_R {
        USART2_REMAP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1_remap(&self) -> USART1_REMAP_R {
        USART1_REMAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - USART0 remapping"]
    #[inline(always)]
    pub fn usart0_remap(&self) -> USART0_REMAP_R {
        USART0_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - I2C0 remapping"]
    #[inline(always)]
    pub fn i2c0_remap(&self) -> I2C0_REMAP_R {
        I2C0_REMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - SPI0 remapping"]
    #[inline(always)]
    pub fn spi0_remap(&self) -> SPI0_REMAP_R {
        SPI0_REMAP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    pub fn swj_cfg(&mut self) -> SWJ_CFG_W {
        SWJ_CFG_W::new(self)
    }
    #[doc = "Bit 20 - ADC 1 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrgreg_remap(&mut self) -> ADC1_ETRGREG_REMAP_W {
        ADC1_ETRGREG_REMAP_W::new(self)
    }
    #[doc = "Bit 19 - ADC 1 external trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrginj_remap(&mut self) -> ADC1_ETRGINJ_REMAP_W {
        ADC1_ETRGINJ_REMAP_W::new(self)
    }
    #[doc = "Bit 18 - ADC 0 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc0_etrgreg_remap(&mut self) -> ADC0_ETRGREG_REMAP_W {
        ADC0_ETRGREG_REMAP_W::new(self)
    }
    #[doc = "Bit 17 - ADC 0 external trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc0_etrginj_remap(&mut self) -> ADC0_ETRGINJ_REMAP_W {
        ADC0_ETRGINJ_REMAP_W::new(self)
    }
    #[doc = "Bit 16 - TIMER4 channel3 internal remapping"]
    #[inline(always)]
    pub fn timer4ch3_iremap(&mut self) -> TIMER4CH3_IREMAP_W {
        TIMER4CH3_IREMAP_W::new(self)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
    #[inline(always)]
    pub fn pd01_remap(&mut self) -> PD01_REMAP_W {
        PD01_REMAP_W::new(self)
    }
    #[doc = "Bits 13:14 - CAN interface remapping"]
    #[inline(always)]
    pub fn can_remap(&mut self) -> CAN_REMAP_W {
        CAN_REMAP_W::new(self)
    }
    #[doc = "Bit 12 - TIMER3 remapping"]
    #[inline(always)]
    pub fn timer3_remap(&mut self) -> TIMER3_REMAP_W {
        TIMER3_REMAP_W::new(self)
    }
    #[doc = "Bits 10:11 - TIMER2 remapping"]
    #[inline(always)]
    pub fn timer2_remap(&mut self) -> TIMER2_REMAP_W {
        TIMER2_REMAP_W::new(self)
    }
    #[doc = "Bits 8:9 - TIMER1 remapping"]
    #[inline(always)]
    pub fn timer1_remap(&mut self) -> TIMER1_REMAP_W {
        TIMER1_REMAP_W::new(self)
    }
    #[doc = "Bits 6:7 - TIMER0 remapping"]
    #[inline(always)]
    pub fn timer0_remap(&mut self) -> TIMER0_REMAP_W {
        TIMER0_REMAP_W::new(self)
    }
    #[doc = "Bits 4:5 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2_remap(&mut self) -> USART2_REMAP_W {
        USART2_REMAP_W::new(self)
    }
    #[doc = "Bit 3 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1_remap(&mut self) -> USART1_REMAP_W {
        USART1_REMAP_W::new(self)
    }
    #[doc = "Bit 2 - USART0 remapping"]
    #[inline(always)]
    pub fn usart0_remap(&mut self) -> USART0_REMAP_W {
        USART0_REMAP_W::new(self)
    }
    #[doc = "Bit 1 - I2C0 remapping"]
    #[inline(always)]
    pub fn i2c0_remap(&mut self) -> I2C0_REMAP_W {
        I2C0_REMAP_W::new(self)
    }
    #[doc = "Bit 0 - SPI0 remapping"]
    #[inline(always)]
    pub fn spi0_remap(&mut self) -> SPI0_REMAP_W {
        SPI0_REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO port configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcf0](index.html) module"]
pub struct PCF0_SPEC;
impl crate::RegisterSpec for PCF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcf0::R](R) reader structure"]
impl crate::Readable for PCF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcf0::W](W) writer structure"]
impl crate::Writable for PCF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCF0 to value 0"]
impl crate::Resettable for PCF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
