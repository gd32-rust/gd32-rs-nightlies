#[doc = "Register `CFG2` reader"]
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG2` writer"]
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
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
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCPSC` reader - Bit 2 of ADCPSC"]
pub type ADCPSC_R = crate::BitReader<bool>;
#[doc = "Field `ADCPSC` writer - Bit 2 of ADCPSC"]
pub type ADCPSC_W<'a> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, 31>;
#[doc = "Field `IRC28MDIV` reader - CK_IRC28M divider 2 or not"]
pub type IRC28MDIV_R = crate::BitReader<bool>;
#[doc = "Field `IRC28MDIV` writer - CK_IRC28M divider 2 or not"]
pub type IRC28MDIV_W<'a> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, 16>;
#[doc = "Field `ADCSEL` reader - CK_ADC clock source selection"]
pub type ADCSEL_R = crate::BitReader<bool>;
#[doc = "Field `ADCSEL` writer - CK_ADC clock source selection"]
pub type ADCSEL_W<'a> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, 8>;
#[doc = "Field `USART0SEL` reader - CK_USART0 clock source selection"]
pub type USART0SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART0SEL` writer - CK_USART0 clock source selection"]
pub type USART0SEL_W<'a> = crate::FieldWriter<'a, u32, CFG2_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bit 31 - Bit 2 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc(&self) -> ADCPSC_R {
        ADCPSC_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 16 - CK_IRC28M divider 2 or not"]
    #[inline(always)]
    pub fn irc28mdiv(&self) -> IRC28MDIV_R {
        IRC28MDIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 8 - CK_ADC clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:1 - CK_USART0 clock source selection"]
    #[inline(always)]
    pub fn usart0sel(&self) -> USART0SEL_R {
        USART0SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Bit 2 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc(&mut self) -> ADCPSC_W {
        ADCPSC_W::new(self)
    }
    #[doc = "Bit 16 - CK_IRC28M divider 2 or not"]
    #[inline(always)]
    pub fn irc28mdiv(&mut self) -> IRC28MDIV_W {
        IRC28MDIV_W::new(self)
    }
    #[doc = "Bit 8 - CK_ADC clock source selection"]
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W {
        ADCSEL_W::new(self)
    }
    #[doc = "Bits 0:1 - CK_USART0 clock source selection"]
    #[inline(always)]
    pub fn usart0sel(&mut self) -> USART0SEL_W {
        USART0SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](index.html) module"]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg2::R](R) reader structure"]
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg2::W](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
