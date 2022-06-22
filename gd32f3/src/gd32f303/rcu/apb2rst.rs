#[doc = "Register `APB2RST` reader"]
pub struct R(crate::R<APB2RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2RST` writer"]
pub struct W(crate::W<APB2RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RST_SPEC>;
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
impl From<crate::W<APB2RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFRST` reader - Alternate function I/O reset"]
pub type AFRST_R = crate::BitReader<bool>;
#[doc = "Field `AFRST` writer - Alternate function I/O reset"]
pub type AFRST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 0>;
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub type PARST_R = crate::BitReader<bool>;
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub type PARST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 2>;
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub type PBRST_R = crate::BitReader<bool>;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub type PBRST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 3>;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub type PCRST_R = crate::BitReader<bool>;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub type PCRST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 4>;
#[doc = "Field `PDRST` reader - GPIO port D reset"]
pub type PDRST_R = crate::BitReader<bool>;
#[doc = "Field `PDRST` writer - GPIO port D reset"]
pub type PDRST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 5>;
#[doc = "Field `PERST` reader - GPIO port E reset"]
pub type PERST_R = crate::BitReader<bool>;
#[doc = "Field `PERST` writer - GPIO port E reset"]
pub type PERST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 6>;
#[doc = "Field `PFRST` reader - GPIO portF reset"]
pub type PFRST_R = crate::BitReader<bool>;
#[doc = "Field `PFRST` writer - GPIO portF reset"]
pub type PFRST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 7>;
#[doc = "Field `PGRST` reader - GPIO port G reset"]
pub type PGRST_R = crate::BitReader<bool>;
#[doc = "Field `PGRST` writer - GPIO port G reset"]
pub type PGRST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 8>;
#[doc = "Field `ADC0RST` reader - ADC0 reset"]
pub type ADC0RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC0RST` writer - ADC0 reset"]
pub type ADC0RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 9>;
#[doc = "Field `ADC1RST` reader - ADC1 reset"]
pub type ADC1RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC1RST` writer - ADC1 reset"]
pub type ADC1RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 10>;
#[doc = "Field `TIMER0RST` reader - Timer 0 reset"]
pub type TIMER0RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0RST` writer - Timer 0 reset"]
pub type TIMER0RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 11>;
#[doc = "Field `SPI0RST` reader - SPI0 reset"]
pub type SPI0RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI0RST` writer - SPI0 reset"]
pub type SPI0RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 12>;
#[doc = "Field `TIMER7RST` reader - Timer 7 reset"]
pub type TIMER7RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER7RST` writer - Timer 7 reset"]
pub type TIMER7RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 13>;
#[doc = "Field `USART0RST` reader - USART0 Reset"]
pub type USART0RST_R = crate::BitReader<bool>;
#[doc = "Field `USART0RST` writer - USART0 Reset"]
pub type USART0RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 14>;
#[doc = "Field `ADC2RST` reader - ADC2 reset"]
pub type ADC2RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC2RST` writer - ADC2 reset"]
pub type ADC2RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 15>;
#[doc = "Field `TIMER8RST` reader - Timer 8 reset"]
pub type TIMER8RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER8RST` writer - Timer 8 reset"]
pub type TIMER8RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 19>;
#[doc = "Field `TIMER9RST` reader - Timer 9 reset"]
pub type TIMER9RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER9RST` writer - Timer 9 reset"]
pub type TIMER9RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 20>;
#[doc = "Field `TIMER10RST` reader - Timer 10 reset"]
pub type TIMER10RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER10RST` writer - Timer 10 reset"]
pub type TIMER10RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 21>;
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
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    pub fn afrst(&mut self) -> AFRST_W {
        AFRST_W::new(self)
    }
    #[doc = "Bit 2 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&mut self) -> PARST_W {
        PARST_W::new(self)
    }
    #[doc = "Bit 3 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&mut self) -> PBRST_W {
        PBRST_W::new(self)
    }
    #[doc = "Bit 4 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&mut self) -> PCRST_W {
        PCRST_W::new(self)
    }
    #[doc = "Bit 5 - GPIO port D reset"]
    #[inline(always)]
    pub fn pdrst(&mut self) -> PDRST_W {
        PDRST_W::new(self)
    }
    #[doc = "Bit 6 - GPIO port E reset"]
    #[inline(always)]
    pub fn perst(&mut self) -> PERST_W {
        PERST_W::new(self)
    }
    #[doc = "Bit 7 - GPIO portF reset"]
    #[inline(always)]
    pub fn pfrst(&mut self) -> PFRST_W {
        PFRST_W::new(self)
    }
    #[doc = "Bit 8 - GPIO port G reset"]
    #[inline(always)]
    pub fn pgrst(&mut self) -> PGRST_W {
        PGRST_W::new(self)
    }
    #[doc = "Bit 9 - ADC0 reset"]
    #[inline(always)]
    pub fn adc0rst(&mut self) -> ADC0RST_W {
        ADC0RST_W::new(self)
    }
    #[doc = "Bit 10 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1rst(&mut self) -> ADC1RST_W {
        ADC1RST_W::new(self)
    }
    #[doc = "Bit 11 - Timer 0 reset"]
    #[inline(always)]
    pub fn timer0rst(&mut self) -> TIMER0RST_W {
        TIMER0RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 reset"]
    #[inline(always)]
    pub fn spi0rst(&mut self) -> SPI0RST_W {
        SPI0RST_W::new(self)
    }
    #[doc = "Bit 13 - Timer 7 reset"]
    #[inline(always)]
    pub fn timer7rst(&mut self) -> TIMER7RST_W {
        TIMER7RST_W::new(self)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&mut self) -> USART0RST_W {
        USART0RST_W::new(self)
    }
    #[doc = "Bit 15 - ADC2 reset"]
    #[inline(always)]
    pub fn adc2rst(&mut self) -> ADC2RST_W {
        ADC2RST_W::new(self)
    }
    #[doc = "Bit 19 - Timer 8 reset"]
    #[inline(always)]
    pub fn timer8rst(&mut self) -> TIMER8RST_W {
        TIMER8RST_W::new(self)
    }
    #[doc = "Bit 20 - Timer 9 reset"]
    #[inline(always)]
    pub fn timer9rst(&mut self) -> TIMER9RST_W {
        TIMER9RST_W::new(self)
    }
    #[doc = "Bit 21 - Timer 10 reset"]
    #[inline(always)]
    pub fn timer10rst(&mut self) -> TIMER10RST_W {
        TIMER10RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 reset register (RCU_APB2RST)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rst](index.html) module"]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2rst::R](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2rst::W](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
