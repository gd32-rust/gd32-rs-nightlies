#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<APB2EN_SPEC>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<APB2EN_SPEC>;
#[doc = "Field `CFGCMPEN` reader - System configuration and comparator clock enable"]
pub type CFGCMPEN_R = crate::BitReader<CFGCMPEN_A>;
#[doc = "System configuration and comparator clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CFGCMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGCMPEN_A {
        match self.bits {
            false => CFGCMPEN_A::DISABLED,
            true => CFGCMPEN_A::ENABLED,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CFGCMPEN_A::DISABLED
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CFGCMPEN_A::ENABLED
    }
}
#[doc = "Field `CFGCMPEN` writer - System configuration and comparator clock enable"]
pub type CFGCMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CFGCMPEN_A>;
impl<'a, REG, const O: u8> CFGCMPEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CFGCMPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CFGCMPEN_A::ENABLED)
    }
}
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
#[doc = "Field `DBGMCUEN` reader - DBGMCU clock enable"]
pub use CFGCMPEN_R as DBGMCUEN_R;
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
#[doc = "Field `DBGMCUEN` writer - DBGMCU clock enable"]
pub use CFGCMPEN_W as DBGMCUEN_W;
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
    #[doc = "Bit 22 - DBGMCU clock enable"]
    #[inline(always)]
    pub fn dbgmcuen(&self) -> DBGMCUEN_R {
        DBGMCUEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration and comparator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfgcmpen(&mut self) -> CFGCMPEN_W<APB2EN_SPEC, 0> {
        CFGCMPEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<APB2EN_SPEC, 9> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 11 - TIMER0 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer0en(&mut self) -> TIMER0EN_W<APB2EN_SPEC, 11> {
        TIMER0EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi0en(&mut self) -> SPI0EN_W<APB2EN_SPEC, 12> {
        SPI0EN_W::new(self)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0en(&mut self) -> USART0EN_W<APB2EN_SPEC, 14> {
        USART0EN_W::new(self)
    }
    #[doc = "Bit 16 - TIMER14 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer14en(&mut self) -> TIMER14EN_W<APB2EN_SPEC, 16> {
        TIMER14EN_W::new(self)
    }
    #[doc = "Bit 17 - TIMER15 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer15en(&mut self) -> TIMER15EN_W<APB2EN_SPEC, 17> {
        TIMER15EN_W::new(self)
    }
    #[doc = "Bit 18 - TIMER16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer16en(&mut self) -> TIMER16EN_W<APB2EN_SPEC, 18> {
        TIMER16EN_W::new(self)
    }
    #[doc = "Bit 22 - DBGMCU clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgmcuen(&mut self) -> DBGMCUEN_W<APB2EN_SPEC, 22> {
        DBGMCUEN_W::new(self)
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
#[doc = "APB2 enable register (RCU_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2EN_SPEC;
impl crate::RegisterSpec for APB2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2en::R`](R) reader structure"]
impl crate::Readable for APB2EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2en::W`](W) writer structure"]
impl crate::Writable for APB2EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2EN to value 0"]
impl crate::Resettable for APB2EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
