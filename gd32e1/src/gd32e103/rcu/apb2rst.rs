#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Field `AFRST` reader - Alternate function I/O reset"]
pub type AFRST_R = crate::BitReader<AFRST_A>;
#[doc = "Alternate function I/O reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<AFRST_A> for bool {
    #[inline(always)]
    fn from(variant: AFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl AFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AFRST_A> {
        match self.bits {
            true => Some(AFRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AFRST_A::RESET
    }
}
#[doc = "Field `AFRST` writer - Alternate function I/O reset"]
pub type AFRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AFRST_A>;
impl<'a, REG, const O: u8> AFRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(AFRST_A::RESET)
    }
}
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub use AFRST_R as PARST_R;
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub use AFRST_R as PBRST_R;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub use AFRST_R as PCRST_R;
#[doc = "Field `PDRST` reader - GPIO port D reset"]
pub use AFRST_R as PDRST_R;
#[doc = "Field `PERST` reader - GPIO port E reset"]
pub use AFRST_R as PERST_R;
#[doc = "Field `ADC0RST` reader - ADC0 reset"]
pub use AFRST_R as ADC0RST_R;
#[doc = "Field `ADC1RST` reader - ADC1 reset"]
pub use AFRST_R as ADC1RST_R;
#[doc = "Field `TIMER0RST` reader - Timer 0 reset"]
pub use AFRST_R as TIMER0RST_R;
#[doc = "Field `SPI0RST` reader - SPI0 reset"]
pub use AFRST_R as SPI0RST_R;
#[doc = "Field `TIMER7RST` reader - Timer 7 reset"]
pub use AFRST_R as TIMER7RST_R;
#[doc = "Field `USART0RST` reader - USART0 Reset"]
pub use AFRST_R as USART0RST_R;
#[doc = "Field `TIMER8RST` reader - Timer 8 reset"]
pub use AFRST_R as TIMER8RST_R;
#[doc = "Field `TIMER9RST` reader - Timer 9 reset"]
pub use AFRST_R as TIMER9RST_R;
#[doc = "Field `TIMER10RST` reader - Timer 10 reset"]
pub use AFRST_R as TIMER10RST_R;
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub use AFRST_W as PARST_W;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub use AFRST_W as PBRST_W;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub use AFRST_W as PCRST_W;
#[doc = "Field `PDRST` writer - GPIO port D reset"]
pub use AFRST_W as PDRST_W;
#[doc = "Field `PERST` writer - GPIO port E reset"]
pub use AFRST_W as PERST_W;
#[doc = "Field `ADC0RST` writer - ADC0 reset"]
pub use AFRST_W as ADC0RST_W;
#[doc = "Field `ADC1RST` writer - ADC1 reset"]
pub use AFRST_W as ADC1RST_W;
#[doc = "Field `TIMER0RST` writer - Timer 0 reset"]
pub use AFRST_W as TIMER0RST_W;
#[doc = "Field `SPI0RST` writer - SPI0 reset"]
pub use AFRST_W as SPI0RST_W;
#[doc = "Field `TIMER7RST` writer - Timer 7 reset"]
pub use AFRST_W as TIMER7RST_W;
#[doc = "Field `USART0RST` writer - USART0 Reset"]
pub use AFRST_W as USART0RST_W;
#[doc = "Field `TIMER8RST` writer - Timer 8 reset"]
pub use AFRST_W as TIMER8RST_W;
#[doc = "Field `TIMER9RST` writer - Timer 9 reset"]
pub use AFRST_W as TIMER9RST_W;
#[doc = "Field `TIMER10RST` writer - Timer 10 reset"]
pub use AFRST_W as TIMER10RST_W;
impl R {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    pub fn afrst(&self) -> AFRST_R {
        AFRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&self) -> PARST_R {
        PARST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&self) -> PBRST_R {
        PBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&self) -> PCRST_R {
        PCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO port D reset"]
    #[inline(always)]
    pub fn pdrst(&self) -> PDRST_R {
        PDRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO port E reset"]
    #[inline(always)]
    pub fn perst(&self) -> PERST_R {
        PERST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC0 reset"]
    #[inline(always)]
    pub fn adc0rst(&self) -> ADC0RST_R {
        ADC0RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 0 reset"]
    #[inline(always)]
    pub fn timer0rst(&self) -> TIMER0RST_R {
        TIMER0RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer 7 reset"]
    #[inline(always)]
    pub fn timer7rst(&self) -> TIMER7RST_R {
        TIMER7RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&self) -> USART0RST_R {
        USART0RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer 8 reset"]
    #[inline(always)]
    pub fn timer8rst(&self) -> TIMER8RST_R {
        TIMER8RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer 9 reset"]
    #[inline(always)]
    pub fn timer9rst(&self) -> TIMER9RST_R {
        TIMER9RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer 10 reset"]
    #[inline(always)]
    pub fn timer10rst(&self) -> TIMER10RST_R {
        TIMER10RST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    #[must_use]
    pub fn afrst(&mut self) -> AFRST_W<APB2RST_SPEC, 0> {
        AFRST_W::new(self)
    }
    #[doc = "Bit 2 - GPIO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> PARST_W<APB2RST_SPEC, 2> {
        PARST_W::new(self)
    }
    #[doc = "Bit 3 - GPIO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PBRST_W<APB2RST_SPEC, 3> {
        PBRST_W::new(self)
    }
    #[doc = "Bit 4 - GPIO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PCRST_W<APB2RST_SPEC, 4> {
        PCRST_W::new(self)
    }
    #[doc = "Bit 5 - GPIO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PDRST_W<APB2RST_SPEC, 5> {
        PDRST_W::new(self)
    }
    #[doc = "Bit 6 - GPIO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn perst(&mut self) -> PERST_W<APB2RST_SPEC, 6> {
        PERST_W::new(self)
    }
    #[doc = "Bit 9 - ADC0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc0rst(&mut self) -> ADC0RST_W<APB2RST_SPEC, 9> {
        ADC0RST_W::new(self)
    }
    #[doc = "Bit 10 - ADC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<APB2RST_SPEC, 10> {
        ADC1RST_W::new(self)
    }
    #[doc = "Bit 11 - Timer 0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer0rst(&mut self) -> TIMER0RST_W<APB2RST_SPEC, 11> {
        TIMER0RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi0rst(&mut self) -> SPI0RST_W<APB2RST_SPEC, 12> {
        SPI0RST_W::new(self)
    }
    #[doc = "Bit 13 - Timer 7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer7rst(&mut self) -> TIMER7RST_W<APB2RST_SPEC, 13> {
        TIMER7RST_W::new(self)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart0rst(&mut self) -> USART0RST_W<APB2RST_SPEC, 14> {
        USART0RST_W::new(self)
    }
    #[doc = "Bit 19 - Timer 8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer8rst(&mut self) -> TIMER8RST_W<APB2RST_SPEC, 19> {
        TIMER8RST_W::new(self)
    }
    #[doc = "Bit 20 - Timer 9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer9rst(&mut self) -> TIMER9RST_W<APB2RST_SPEC, 20> {
        TIMER9RST_W::new(self)
    }
    #[doc = "Bit 21 - Timer 10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer10rst(&mut self) -> TIMER10RST_W<APB2RST_SPEC, 21> {
        TIMER10RST_W::new(self)
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
#[doc = "APB2 reset register (RCU_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
