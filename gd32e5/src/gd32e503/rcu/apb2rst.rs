#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Field `AFRST` reader - Alternate function I/O reset"]
pub type AFRST_R = crate::BitReader;
#[doc = "Field `AFRST` writer - Alternate function I/O reset"]
pub type AFRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub type PARST_R = crate::BitReader;
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub type PARST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub type PBRST_R = crate::BitReader;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub type PBRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub type PCRST_R = crate::BitReader;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub type PCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDRST` reader - GPIO port D reset"]
pub type PDRST_R = crate::BitReader;
#[doc = "Field `PDRST` writer - GPIO port D reset"]
pub type PDRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERST` reader - GPIO port E reset"]
pub type PERST_R = crate::BitReader;
#[doc = "Field `PERST` writer - GPIO port E reset"]
pub type PERST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFRST` reader - GPIO portF reset"]
pub type PFRST_R = crate::BitReader;
#[doc = "Field `PFRST` writer - GPIO portF reset"]
pub type PFRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGRST` reader - GPIO port G reset"]
pub type PGRST_R = crate::BitReader;
#[doc = "Field `PGRST` writer - GPIO port G reset"]
pub type PGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC0RST` reader - ADC0 reset"]
pub type ADC0RST_R = crate::BitReader;
#[doc = "Field `ADC0RST` writer - ADC0 reset"]
pub type ADC0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1RST` reader - ADC1 reset"]
pub type ADC1RST_R = crate::BitReader;
#[doc = "Field `ADC1RST` writer - ADC1 reset"]
pub type ADC1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER0RST` reader - Timer 0 reset"]
pub type TIMER0RST_R = crate::BitReader;
#[doc = "Field `TIMER0RST` writer - Timer 0 reset"]
pub type TIMER0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI0RST` reader - SPI0 reset"]
pub type SPI0RST_R = crate::BitReader;
#[doc = "Field `SPI0RST` writer - SPI0 reset"]
pub type SPI0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER7RST` reader - Timer 7 reset"]
pub type TIMER7RST_R = crate::BitReader;
#[doc = "Field `TIMER7RST` writer - Timer 7 reset"]
pub type TIMER7RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART0RST` reader - USART0 Reset"]
pub type USART0RST_R = crate::BitReader;
#[doc = "Field `USART0RST` writer - USART0 Reset"]
pub type USART0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2RST` reader - ADC2 reset"]
pub type ADC2RST_R = crate::BitReader;
#[doc = "Field `ADC2RST` writer - ADC2 reset"]
pub type ADC2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER8RST` reader - Timer 8 reset"]
pub type TIMER8RST_R = crate::BitReader;
#[doc = "Field `TIMER8RST` writer - Timer 8 reset"]
pub type TIMER8RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER9RST` reader - Timer 9 reset"]
pub type TIMER9RST_R = crate::BitReader;
#[doc = "Field `TIMER9RST` writer - Timer 9 reset"]
pub type TIMER9RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER10RST` reader - Timer 10 reset"]
pub type TIMER10RST_R = crate::BitReader;
#[doc = "Field `TIMER10RST` writer - Timer 10 reset"]
pub type TIMER10RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART5RST` reader - USART5 reset"]
pub type USART5RST_R = crate::BitReader;
#[doc = "Field `USART5RST` writer - USART5 reset"]
pub type USART5RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHRTIMERRST` reader - SHRTIMER reset"]
pub type SHRTIMERRST_R = crate::BitReader;
#[doc = "Field `SHRTIMERRST` writer - SHRTIMER reset"]
pub type SHRTIMERRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 7 - GPIO portF reset"]
    #[inline(always)]
    pub fn pfrst(&self) -> PFRST_R {
        PFRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO port G reset"]
    #[inline(always)]
    pub fn pgrst(&self) -> PGRST_R {
        PGRST_R::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bit 15 - ADC2 reset"]
    #[inline(always)]
    pub fn adc2rst(&self) -> ADC2RST_R {
        ADC2RST_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 28 - USART5 reset"]
    #[inline(always)]
    pub fn usart5rst(&self) -> USART5RST_R {
        USART5RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SHRTIMER reset"]
    #[inline(always)]
    pub fn shrtimerrst(&self) -> SHRTIMERRST_R {
        SHRTIMERRST_R::new(((self.bits >> 29) & 1) != 0)
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
    #[doc = "Bit 7 - GPIO portF reset"]
    #[inline(always)]
    #[must_use]
    pub fn pfrst(&mut self) -> PFRST_W<APB2RST_SPEC, 7> {
        PFRST_W::new(self)
    }
    #[doc = "Bit 8 - GPIO port G reset"]
    #[inline(always)]
    #[must_use]
    pub fn pgrst(&mut self) -> PGRST_W<APB2RST_SPEC, 8> {
        PGRST_W::new(self)
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
    #[doc = "Bit 15 - ADC2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc2rst(&mut self) -> ADC2RST_W<APB2RST_SPEC, 15> {
        ADC2RST_W::new(self)
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
    #[doc = "Bit 28 - USART5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart5rst(&mut self) -> USART5RST_W<APB2RST_SPEC, 28> {
        USART5RST_W::new(self)
    }
    #[doc = "Bit 29 - SHRTIMER reset"]
    #[inline(always)]
    #[must_use]
    pub fn shrtimerrst(&mut self) -> SHRTIMERRST_W<APB2RST_SPEC, 29> {
        SHRTIMERRST_W::new(self)
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
