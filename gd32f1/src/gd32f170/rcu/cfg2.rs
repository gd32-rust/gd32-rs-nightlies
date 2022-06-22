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
#[doc = "CK_USART0 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART0SEL_A {
    #[doc = "0: APB2 selected as USART0 clock source"]
    APB2 = 0,
    #[doc = "1: SYS selected as USART0 clock source"]
    SYS = 1,
    #[doc = "2: LXTAL selected as USART0 clock source"]
    LXTAL = 2,
    #[doc = "3: IRC8M selected as USART0 clock source"]
    IRC8M = 3,
}
impl From<USART0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART0SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USART0SEL` reader - CK_USART0 clock source selection"]
pub type USART0SEL_R = crate::FieldReader<u8, USART0SEL_A>;
impl USART0SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART0SEL_A {
        match self.bits {
            0 => USART0SEL_A::APB2,
            1 => USART0SEL_A::SYS,
            2 => USART0SEL_A::LXTAL,
            3 => USART0SEL_A::IRC8M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APB2`"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == USART0SEL_A::APB2
    }
    #[doc = "Checks if the value of the field is `SYS`"]
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == USART0SEL_A::SYS
    }
    #[doc = "Checks if the value of the field is `LXTAL`"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == USART0SEL_A::LXTAL
    }
    #[doc = "Checks if the value of the field is `IRC8M`"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == USART0SEL_A::IRC8M
    }
}
#[doc = "Field `USART0SEL` writer - CK_USART0 clock source selection"]
pub type USART0SEL_W<'a> = crate::FieldWriterSafe<'a, u32, CFG2_SPEC, u8, USART0SEL_A, 2, 0>;
impl<'a> USART0SEL_W<'a> {
    #[doc = "APB2 selected as USART0 clock source"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(USART0SEL_A::APB2)
    }
    #[doc = "SYS selected as USART0 clock source"]
    #[inline(always)]
    pub fn sys(self) -> &'a mut W {
        self.variant(USART0SEL_A::SYS)
    }
    #[doc = "LXTAL selected as USART0 clock source"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut W {
        self.variant(USART0SEL_A::LXTAL)
    }
    #[doc = "IRC8M selected as USART0 clock source"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut W {
        self.variant(USART0SEL_A::IRC8M)
    }
}
#[doc = "CK_CEC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECSEL_A {
    #[doc = "0: IRC8M clock divided by 244 selected as CEC clock source"]
    IRC8M_DIV244 = 0,
    #[doc = "1: LXTAL clock selected as CEC clock source"]
    LXTAL = 1,
}
impl From<CECSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CECSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CECSEL` reader - CK_CEC clock source selection"]
pub type CECSEL_R = crate::BitReader<CECSEL_A>;
impl CECSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CECSEL_A {
        match self.bits {
            false => CECSEL_A::IRC8M_DIV244,
            true => CECSEL_A::LXTAL,
        }
    }
    #[doc = "Checks if the value of the field is `IRC8M_DIV244`"]
    #[inline(always)]
    pub fn is_irc8m_div244(&self) -> bool {
        *self == CECSEL_A::IRC8M_DIV244
    }
    #[doc = "Checks if the value of the field is `LXTAL`"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == CECSEL_A::LXTAL
    }
}
#[doc = "Field `CECSEL` writer - CK_CEC clock source selection"]
pub type CECSEL_W<'a> = crate::BitWriter<'a, u32, CFG2_SPEC, CECSEL_A, 6>;
impl<'a> CECSEL_W<'a> {
    #[doc = "IRC8M clock divided by 244 selected as CEC clock source"]
    #[inline(always)]
    pub fn irc8m_div244(self) -> &'a mut W {
        self.variant(CECSEL_A::IRC8M_DIV244)
    }
    #[doc = "LXTAL clock selected as CEC clock source"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut W {
        self.variant(CECSEL_A::LXTAL)
    }
}
#[doc = "CK_ADC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCSEL_A {
    #[doc = "0: IRC28M selected as ADC clock source, possibly divided by 2"]
    IRC28M = 0,
    #[doc = "1: APB2 divided by prescaler selected as ADC clock source"]
    APB2 = 1,
}
impl From<ADCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCSEL` reader - CK_ADC clock source selection"]
pub type ADCSEL_R = crate::BitReader<ADCSEL_A>;
impl ADCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSEL_A {
        match self.bits {
            false => ADCSEL_A::IRC28M,
            true => ADCSEL_A::APB2,
        }
    }
    #[doc = "Checks if the value of the field is `IRC28M`"]
    #[inline(always)]
    pub fn is_irc28m(&self) -> bool {
        *self == ADCSEL_A::IRC28M
    }
    #[doc = "Checks if the value of the field is `APB2`"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == ADCSEL_A::APB2
    }
}
#[doc = "Field `ADCSEL` writer - CK_ADC clock source selection"]
pub type ADCSEL_W<'a> = crate::BitWriter<'a, u32, CFG2_SPEC, ADCSEL_A, 8>;
impl<'a> ADCSEL_W<'a> {
    #[doc = "IRC28M selected as ADC clock source, possibly divided by 2"]
    #[inline(always)]
    pub fn irc28m(self) -> &'a mut W {
        self.variant(ADCSEL_A::IRC28M)
    }
    #[doc = "APB2 divided by prescaler selected as ADC clock source"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(ADCSEL_A::APB2)
    }
}
#[doc = "CK_IRC28M divider 2 or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC28MDIV_A {
    #[doc = "0: IRC28M is divided by 2 for ADC clock"]
    DIV2 = 0,
    #[doc = "1: IRC28M is not divided for ADC clock"]
    DIV1 = 1,
}
impl From<IRC28MDIV_A> for bool {
    #[inline(always)]
    fn from(variant: IRC28MDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC28MDIV` reader - CK_IRC28M divider 2 or not"]
pub type IRC28MDIV_R = crate::BitReader<IRC28MDIV_A>;
impl IRC28MDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC28MDIV_A {
        match self.bits {
            false => IRC28MDIV_A::DIV2,
            true => IRC28MDIV_A::DIV1,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == IRC28MDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == IRC28MDIV_A::DIV1
    }
}
#[doc = "Field `IRC28MDIV` writer - CK_IRC28M divider 2 or not"]
pub type IRC28MDIV_W<'a> = crate::BitWriter<'a, u32, CFG2_SPEC, IRC28MDIV_A, 16>;
impl<'a> IRC28MDIV_W<'a> {
    #[doc = "IRC28M is divided by 2 for ADC clock"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(IRC28MDIV_A::DIV2)
    }
    #[doc = "IRC28M is not divided for ADC clock"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(IRC28MDIV_A::DIV1)
    }
}
impl R {
    #[doc = "Bits 0:1 - CK_USART0 clock source selection"]
    #[inline(always)]
    pub fn usart0sel(&self) -> USART0SEL_R {
        USART0SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 6 - CK_CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CK_ADC clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - CK_IRC28M divider 2 or not"]
    #[inline(always)]
    pub fn irc28mdiv(&self) -> IRC28MDIV_R {
        IRC28MDIV_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CK_USART0 clock source selection"]
    #[inline(always)]
    pub fn usart0sel(&mut self) -> USART0SEL_W {
        USART0SEL_W::new(self)
    }
    #[doc = "Bit 6 - CK_CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W {
        CECSEL_W::new(self)
    }
    #[doc = "Bit 8 - CK_ADC clock source selection"]
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W {
        ADCSEL_W::new(self)
    }
    #[doc = "Bit 16 - CK_IRC28M divider 2 or not"]
    #[inline(always)]
    pub fn irc28mdiv(&mut self) -> IRC28MDIV_W {
        IRC28MDIV_W::new(self)
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
