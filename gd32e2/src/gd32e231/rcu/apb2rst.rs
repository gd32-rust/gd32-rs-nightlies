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
#[doc = "Field `TIMER16RST` reader - TIMER16 reset"]
pub type TIMER16RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER16RST` writer - TIMER16 reset"]
pub type TIMER16RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 18>;
#[doc = "Field `TIMER15RST` reader - TIMER15 reset"]
pub type TIMER15RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER15RST` writer - TIMER15 reset"]
pub type TIMER15RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 17>;
#[doc = "Field `TIMER14RST` reader - TIMER14 reset"]
pub type TIMER14RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER14RST` writer - TIMER14 reset"]
pub type TIMER14RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 16>;
#[doc = "Field `USART0RST` reader - USART0 Reset"]
pub type USART0RST_R = crate::BitReader<bool>;
#[doc = "Field `USART0RST` writer - USART0 Reset"]
pub type USART0RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 14>;
#[doc = "Field `SPI0RST` reader - SPI0 Reset"]
pub type SPI0RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI0RST` writer - SPI0 Reset"]
pub type SPI0RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 12>;
#[doc = "Field `TIMER0RST` reader - TIMER0 reset"]
pub type TIMER0RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0RST` writer - TIMER0 reset"]
pub type TIMER0RST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 11>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type ADCRST_R = crate::BitReader<bool>;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type ADCRST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 9>;
#[doc = "Field `CFGCMPRST` reader - System configuration and comparator reset"]
pub type CFGCMPRST_R = crate::BitReader<bool>;
#[doc = "Field `CFGCMPRST` writer - System configuration and comparator reset"]
pub type CFGCMPRST_W<'a> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 18 - TIMER16 reset"]
    #[inline(always)]
    pub fn timer16rst(&self) -> TIMER16RST_R {
        TIMER16RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER15 reset"]
    #[inline(always)]
    pub fn timer15rst(&self) -> TIMER15RST_R {
        TIMER15RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER14 reset"]
    #[inline(always)]
    pub fn timer14rst(&self) -> TIMER14RST_R {
        TIMER14RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&self) -> USART0RST_R {
        USART0RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&self) -> TIMER0RST_R {
        TIMER0RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 0 - System configuration and comparator reset"]
    #[inline(always)]
    pub fn cfgcmprst(&self) -> CFGCMPRST_R {
        CFGCMPRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - TIMER16 reset"]
    #[inline(always)]
    pub fn timer16rst(&mut self) -> TIMER16RST_W {
        TIMER16RST_W::new(self)
    }
    #[doc = "Bit 17 - TIMER15 reset"]
    #[inline(always)]
    pub fn timer15rst(&mut self) -> TIMER15RST_W {
        TIMER15RST_W::new(self)
    }
    #[doc = "Bit 16 - TIMER14 reset"]
    #[inline(always)]
    pub fn timer14rst(&mut self) -> TIMER14RST_W {
        TIMER14RST_W::new(self)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&mut self) -> USART0RST_W {
        USART0RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&mut self) -> SPI0RST_W {
        SPI0RST_W::new(self)
    }
    #[doc = "Bit 11 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&mut self) -> TIMER0RST_W {
        TIMER0RST_W::new(self)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W {
        ADCRST_W::new(self)
    }
    #[doc = "Bit 0 - System configuration and comparator reset"]
    #[inline(always)]
    pub fn cfgcmprst(&mut self) -> CFGCMPRST_W {
        CFGCMPRST_W::new(self)
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
