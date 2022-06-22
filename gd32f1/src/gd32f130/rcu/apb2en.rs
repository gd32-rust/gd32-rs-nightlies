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
#[doc = "System configuration and comparator clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGCMPEN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<CFGCMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CFGCMPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGCMPEN` reader - System configuration and comparator clock enable"]
pub type CFGCMPEN_R = crate::BitReader<CFGCMPEN_A>;
impl CFGCMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGCMPEN_A {
        match self.bits {
            false => CFGCMPEN_A::DISABLED,
            true => CFGCMPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CFGCMPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CFGCMPEN_A::ENABLED
    }
}
#[doc = "Field `CFGCMPEN` writer - System configuration and comparator clock enable"]
pub type CFGCMPEN_W<'a> = crate::BitWriter<'a, u32, APB2EN_SPEC, CFGCMPEN_A, 0>;
impl<'a> CFGCMPEN_W<'a> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CFGCMPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CFGCMPEN_A::ENABLED)
    }
}
#[doc = "ADC interface clock enable"]
pub use CFGCMPEN_A as ADCEN_A;
#[doc = "TIMER0 timer clock enable"]
pub use CFGCMPEN_A as TIMER0EN_A;
#[doc = "SPI0 clock enable"]
pub use CFGCMPEN_A as SPI0EN_A;
#[doc = "USART0 clock enable"]
pub use CFGCMPEN_A as USART0EN_A;
#[doc = "TIMER14 timer clock enable"]
pub use CFGCMPEN_A as TIMER14EN_A;
#[doc = "TIMER15 timer clock enable"]
pub use CFGCMPEN_A as TIMER15EN_A;
#[doc = "TIMER16 timer clock enable"]
pub use CFGCMPEN_A as TIMER16EN_A;
#[doc = "Field `ADCEN` reader - ADC interface clock enable"]
pub use CFGCMPEN_R as ADCEN_R;
#[doc = "Field `TIMER0EN` reader - TIMER0 timer clock enable"]
pub use CFGCMPEN_R as TIMER0EN_R;
#[doc = "Field `SPI0EN` reader - SPI0 clock enable"]
pub use CFGCMPEN_R as SPI0EN_R;
#[doc = "Field `USART0EN` reader - USART0 clock enable"]
pub use CFGCMPEN_R as USART0EN_R;
#[doc = "Field `TIMER14EN` reader - TIMER14 timer clock enable"]
pub use CFGCMPEN_R as TIMER14EN_R;
#[doc = "Field `TIMER15EN` reader - TIMER15 timer clock enable"]
pub use CFGCMPEN_R as TIMER15EN_R;
#[doc = "Field `TIMER16EN` reader - TIMER16 timer clock enable"]
pub use CFGCMPEN_R as TIMER16EN_R;
#[doc = "Field `ADCEN` writer - ADC interface clock enable"]
pub use CFGCMPEN_W as ADCEN_W;
#[doc = "Field `TIMER0EN` writer - TIMER0 timer clock enable"]
pub use CFGCMPEN_W as TIMER0EN_W;
#[doc = "Field `SPI0EN` writer - SPI0 clock enable"]
pub use CFGCMPEN_W as SPI0EN_W;
#[doc = "Field `USART0EN` writer - USART0 clock enable"]
pub use CFGCMPEN_W as USART0EN_W;
#[doc = "Field `TIMER14EN` writer - TIMER14 timer clock enable"]
pub use CFGCMPEN_W as TIMER14EN_W;
#[doc = "Field `TIMER15EN` writer - TIMER15 timer clock enable"]
pub use CFGCMPEN_W as TIMER15EN_W;
#[doc = "Field `TIMER16EN` writer - TIMER16 timer clock enable"]
pub use CFGCMPEN_W as TIMER16EN_W;
impl R {
    #[doc = "Bit 0 - System configuration and comparator clock enable"]
    #[inline(always)]
    pub fn cfgcmpen(&self) -> CFGCMPEN_R {
        CFGCMPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER0 timer clock enable"]
    #[inline(always)]
    pub fn timer0en(&self) -> TIMER0EN_R {
        TIMER0EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&self) -> SPI0EN_R {
        SPI0EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&self) -> USART0EN_R {
        USART0EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER14 timer clock enable"]
    #[inline(always)]
    pub fn timer14en(&self) -> TIMER14EN_R {
        TIMER14EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER15 timer clock enable"]
    #[inline(always)]
    pub fn timer15en(&self) -> TIMER15EN_R {
        TIMER15EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER16 timer clock enable"]
    #[inline(always)]
    pub fn timer16en(&self) -> TIMER16EN_R {
        TIMER16EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration and comparator clock enable"]
    #[inline(always)]
    pub fn cfgcmpen(&mut self) -> CFGCMPEN_W {
        CFGCMPEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 11 - TIMER0 timer clock enable"]
    #[inline(always)]
    pub fn timer0en(&mut self) -> TIMER0EN_W {
        TIMER0EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&mut self) -> SPI0EN_W {
        SPI0EN_W::new(self)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&mut self) -> USART0EN_W {
        USART0EN_W::new(self)
    }
    #[doc = "Bit 16 - TIMER14 timer clock enable"]
    #[inline(always)]
    pub fn timer14en(&mut self) -> TIMER14EN_W {
        TIMER14EN_W::new(self)
    }
    #[doc = "Bit 17 - TIMER15 timer clock enable"]
    #[inline(always)]
    pub fn timer15en(&mut self) -> TIMER15EN_W {
        TIMER15EN_W::new(self)
    }
    #[doc = "Bit 18 - TIMER16 timer clock enable"]
    #[inline(always)]
    pub fn timer16en(&mut self) -> TIMER16EN_W {
        TIMER16EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 enable register (RCU_APB2EN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2en](index.html) module"]
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
