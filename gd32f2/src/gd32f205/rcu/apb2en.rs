#[doc = "Register `APB2EN` reader"]
pub struct R(crate::R<APB2EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2EN` writer"]
pub struct W(crate::W<APB2EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2EN_SPEC>;
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
impl From<crate::W<APB2EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alternate function IO clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFEN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<AFEN_A> for bool {
    #[inline(always)]
    fn from(variant: AFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFEN` reader - Alternate function IO clock enable"]
pub type AFEN_R = crate::BitReader<AFEN_A>;
impl AFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFEN_A {
        match self.bits {
            false => AFEN_A::DISABLED,
            true => AFEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFEN_A::ENABLED
    }
}
#[doc = "Field `AFEN` writer - Alternate function IO clock enable"]
pub type AFEN_W<'a> = crate::BitWriter<'a, u32, APB2EN_SPEC, AFEN_A, 0>;
impl<'a> AFEN_W<'a> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFEN_A::ENABLED)
    }
}
#[doc = "GPIO port A clock enable"]
pub use AFEN_A as PAEN_A;
#[doc = "GPIO port B clock enable"]
pub use AFEN_A as PBEN_A;
#[doc = "GPIO port C clock enable"]
pub use AFEN_A as PCEN_A;
#[doc = "GPIO port D clock enable"]
pub use AFEN_A as PDEN_A;
#[doc = "GPIO port E clock enable"]
pub use AFEN_A as PEEN_A;
#[doc = "GPIO port F clock enable"]
pub use AFEN_A as PFEN_A;
#[doc = "GPIO port G clock enable"]
pub use AFEN_A as PGEN_A;
#[doc = "ADC0 clock enable"]
pub use AFEN_A as ADC0EN_A;
#[doc = "ADC1 clock enable"]
pub use AFEN_A as ADC1EN_A;
#[doc = "TIMER0 clock enable"]
pub use AFEN_A as TIMER0EN_A;
#[doc = "SPI0 clock enable"]
pub use AFEN_A as SPI0EN_A;
#[doc = "TIMER7 clock enable"]
pub use AFEN_A as TIMER7EN_A;
#[doc = "USART0 clock enable"]
pub use AFEN_A as USART0EN_A;
#[doc = "ADC2 clock enable"]
pub use AFEN_A as ADC2EN_A;
#[doc = "TIMER8 clock enable"]
pub use AFEN_A as TIMER8EN_A;
#[doc = "TIMER9 clock enable"]
pub use AFEN_A as TIMER9EN_A;
#[doc = "TIMER10 clock enable"]
pub use AFEN_A as TIMER10EN_A;
#[doc = "Field `PAEN` reader - GPIO port A clock enable"]
pub use AFEN_R as PAEN_R;
#[doc = "Field `PBEN` reader - GPIO port B clock enable"]
pub use AFEN_R as PBEN_R;
#[doc = "Field `PCEN` reader - GPIO port C clock enable"]
pub use AFEN_R as PCEN_R;
#[doc = "Field `PDEN` reader - GPIO port D clock enable"]
pub use AFEN_R as PDEN_R;
#[doc = "Field `PEEN` reader - GPIO port E clock enable"]
pub use AFEN_R as PEEN_R;
#[doc = "Field `PFEN` reader - GPIO port F clock enable"]
pub use AFEN_R as PFEN_R;
#[doc = "Field `PGEN` reader - GPIO port G clock enable"]
pub use AFEN_R as PGEN_R;
#[doc = "Field `ADC0EN` reader - ADC0 clock enable"]
pub use AFEN_R as ADC0EN_R;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub use AFEN_R as ADC1EN_R;
#[doc = "Field `TIMER0EN` reader - TIMER0 clock enable"]
pub use AFEN_R as TIMER0EN_R;
#[doc = "Field `SPI0EN` reader - SPI0 clock enable"]
pub use AFEN_R as SPI0EN_R;
#[doc = "Field `TIMER7EN` reader - TIMER7 clock enable"]
pub use AFEN_R as TIMER7EN_R;
#[doc = "Field `USART0EN` reader - USART0 clock enable"]
pub use AFEN_R as USART0EN_R;
#[doc = "Field `ADC2EN` reader - ADC2 clock enable"]
pub use AFEN_R as ADC2EN_R;
#[doc = "Field `TIMER8EN` reader - TIMER8 clock enable"]
pub use AFEN_R as TIMER8EN_R;
#[doc = "Field `TIMER9EN` reader - TIMER9 clock enable"]
pub use AFEN_R as TIMER9EN_R;
#[doc = "Field `TIMER10EN` reader - TIMER10 clock enable"]
pub use AFEN_R as TIMER10EN_R;
#[doc = "Field `PAEN` writer - GPIO port A clock enable"]
pub use AFEN_W as PAEN_W;
#[doc = "Field `PBEN` writer - GPIO port B clock enable"]
pub use AFEN_W as PBEN_W;
#[doc = "Field `PCEN` writer - GPIO port C clock enable"]
pub use AFEN_W as PCEN_W;
#[doc = "Field `PDEN` writer - GPIO port D clock enable"]
pub use AFEN_W as PDEN_W;
#[doc = "Field `PEEN` writer - GPIO port E clock enable"]
pub use AFEN_W as PEEN_W;
#[doc = "Field `PFEN` writer - GPIO port F clock enable"]
pub use AFEN_W as PFEN_W;
#[doc = "Field `PGEN` writer - GPIO port G clock enable"]
pub use AFEN_W as PGEN_W;
#[doc = "Field `ADC0EN` writer - ADC0 clock enable"]
pub use AFEN_W as ADC0EN_W;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub use AFEN_W as ADC1EN_W;
#[doc = "Field `TIMER0EN` writer - TIMER0 clock enable"]
pub use AFEN_W as TIMER0EN_W;
#[doc = "Field `SPI0EN` writer - SPI0 clock enable"]
pub use AFEN_W as SPI0EN_W;
#[doc = "Field `TIMER7EN` writer - TIMER7 clock enable"]
pub use AFEN_W as TIMER7EN_W;
#[doc = "Field `USART0EN` writer - USART0 clock enable"]
pub use AFEN_W as USART0EN_W;
#[doc = "Field `ADC2EN` writer - ADC2 clock enable"]
pub use AFEN_W as ADC2EN_W;
#[doc = "Field `TIMER8EN` writer - TIMER8 clock enable"]
pub use AFEN_W as TIMER8EN_W;
#[doc = "Field `TIMER9EN` writer - TIMER9 clock enable"]
pub use AFEN_W as TIMER9EN_W;
#[doc = "Field `TIMER10EN` writer - TIMER10 clock enable"]
pub use AFEN_W as TIMER10EN_W;
impl R {
    #[doc = "Bit 0 - Alternate function IO clock enable"]
    #[inline(always)]
    pub fn afen(&self) -> AFEN_R {
        AFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&self) -> PAEN_R {
        PAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&self) -> PBEN_R {
        PBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO port E clock enable"]
    #[inline(always)]
    pub fn peen(&self) -> PEEN_R {
        PEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PFEN_R {
        PFEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO port G clock enable"]
    #[inline(always)]
    pub fn pgen(&self) -> PGEN_R {
        PGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC0 clock enable"]
    #[inline(always)]
    pub fn adc0en(&self) -> ADC0EN_R {
        ADC0EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER0 clock enable"]
    #[inline(always)]
    pub fn timer0en(&self) -> TIMER0EN_R {
        TIMER0EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&self) -> SPI0EN_R {
        SPI0EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIMER7 clock enable"]
    #[inline(always)]
    pub fn timer7en(&self) -> TIMER7EN_R {
        TIMER7EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&self) -> USART0EN_R {
        USART0EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - TIMER8 clock enable"]
    #[inline(always)]
    pub fn timer8en(&self) -> TIMER8EN_R {
        TIMER8EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIMER9 clock enable"]
    #[inline(always)]
    pub fn timer9en(&self) -> TIMER9EN_R {
        TIMER9EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TIMER10 clock enable"]
    #[inline(always)]
    pub fn timer10en(&self) -> TIMER10EN_R {
        TIMER10EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function IO clock enable"]
    #[inline(always)]
    pub fn afen(&mut self) -> AFEN_W {
        AFEN_W::new(self)
    }
    #[doc = "Bit 2 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&mut self) -> PAEN_W {
        PAEN_W::new(self)
    }
    #[doc = "Bit 3 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&mut self) -> PBEN_W {
        PBEN_W::new(self)
    }
    #[doc = "Bit 4 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W::new(self)
    }
    #[doc = "Bit 5 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W::new(self)
    }
    #[doc = "Bit 6 - GPIO port E clock enable"]
    #[inline(always)]
    pub fn peen(&mut self) -> PEEN_W {
        PEEN_W::new(self)
    }
    #[doc = "Bit 7 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&mut self) -> PFEN_W {
        PFEN_W::new(self)
    }
    #[doc = "Bit 8 - GPIO port G clock enable"]
    #[inline(always)]
    pub fn pgen(&mut self) -> PGEN_W {
        PGEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC0 clock enable"]
    #[inline(always)]
    pub fn adc0en(&mut self) -> ADC0EN_W {
        ADC0EN_W::new(self)
    }
    #[doc = "Bit 10 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W {
        ADC1EN_W::new(self)
    }
    #[doc = "Bit 11 - TIMER0 clock enable"]
    #[inline(always)]
    pub fn timer0en(&mut self) -> TIMER0EN_W {
        TIMER0EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&mut self) -> SPI0EN_W {
        SPI0EN_W::new(self)
    }
    #[doc = "Bit 13 - TIMER7 clock enable"]
    #[inline(always)]
    pub fn timer7en(&mut self) -> TIMER7EN_W {
        TIMER7EN_W::new(self)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&mut self) -> USART0EN_W {
        USART0EN_W::new(self)
    }
    #[doc = "Bit 15 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> ADC2EN_W {
        ADC2EN_W::new(self)
    }
    #[doc = "Bit 19 - TIMER8 clock enable"]
    #[inline(always)]
    pub fn timer8en(&mut self) -> TIMER8EN_W {
        TIMER8EN_W::new(self)
    }
    #[doc = "Bit 20 - TIMER9 clock enable"]
    #[inline(always)]
    pub fn timer9en(&mut self) -> TIMER9EN_W {
        TIMER9EN_W::new(self)
    }
    #[doc = "Bit 21 - TIMER10 clock enable"]
    #[inline(always)]
    pub fn timer10en(&mut self) -> TIMER10EN_W {
        TIMER10EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 clock enable register (RCU_APB2EN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2en](index.html) module"]
pub struct APB2EN_SPEC;
impl crate::RegisterSpec for APB2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2en::R](R) reader structure"]
impl crate::Readable for APB2EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2en::W](W) writer structure"]
impl crate::Writable for APB2EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2EN to value 0"]
impl crate::Resettable for APB2EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
