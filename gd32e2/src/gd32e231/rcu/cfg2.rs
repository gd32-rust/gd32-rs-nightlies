#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "Field `USART0SEL` reader - CK_USART0 clock source selection"]
pub type USART0SEL_R = crate::FieldReader<USART0SEL_A>;
#[doc = "CK_USART0 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for USART0SEL_A {
    type Ux = u8;
}
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
    #[doc = "APB2 selected as USART0 clock source"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == USART0SEL_A::APB2
    }
    #[doc = "SYS selected as USART0 clock source"]
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == USART0SEL_A::SYS
    }
    #[doc = "LXTAL selected as USART0 clock source"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == USART0SEL_A::LXTAL
    }
    #[doc = "IRC8M selected as USART0 clock source"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == USART0SEL_A::IRC8M
    }
}
#[doc = "Field `USART0SEL` writer - CK_USART0 clock source selection"]
pub type USART0SEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, USART0SEL_A>;
impl<'a, REG, const O: u8> USART0SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB2 selected as USART0 clock source"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut crate::W<REG> {
        self.variant(USART0SEL_A::APB2)
    }
    #[doc = "SYS selected as USART0 clock source"]
    #[inline(always)]
    pub fn sys(self) -> &'a mut crate::W<REG> {
        self.variant(USART0SEL_A::SYS)
    }
    #[doc = "LXTAL selected as USART0 clock source"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut crate::W<REG> {
        self.variant(USART0SEL_A::LXTAL)
    }
    #[doc = "IRC8M selected as USART0 clock source"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut crate::W<REG> {
        self.variant(USART0SEL_A::IRC8M)
    }
}
#[doc = "Field `ADCSEL` reader - CK_ADC clock source selection"]
pub type ADCSEL_R = crate::BitReader;
#[doc = "Field `ADCSEL` writer - CK_ADC clock source selection"]
pub type ADCSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRC28MDIV` reader - CK_IRC28M divider 2 or not"]
pub type IRC28MDIV_R = crate::BitReader;
#[doc = "Field `IRC28MDIV` writer - CK_IRC28M divider 2 or not"]
pub type IRC28MDIV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADCPSC` reader - Bit 2 of ADCPSC"]
pub type ADCPSC_R = crate::BitReader;
#[doc = "Field `ADCPSC` writer - Bit 2 of ADCPSC"]
pub type ADCPSC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - CK_USART0 clock source selection"]
    #[inline(always)]
    pub fn usart0sel(&self) -> USART0SEL_R {
        USART0SEL_R::new((self.bits & 3) as u8)
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
    #[doc = "Bit 31 - Bit 2 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc(&self) -> ADCPSC_R {
        ADCPSC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CK_USART0 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart0sel(&mut self) -> USART0SEL_W<CFG2_SPEC, 0> {
        USART0SEL_W::new(self)
    }
    #[doc = "Bit 8 - CK_ADC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<CFG2_SPEC, 8> {
        ADCSEL_W::new(self)
    }
    #[doc = "Bit 16 - CK_IRC28M divider 2 or not"]
    #[inline(always)]
    #[must_use]
    pub fn irc28mdiv(&mut self) -> IRC28MDIV_W<CFG2_SPEC, 16> {
        IRC28MDIV_W::new(self)
    }
    #[doc = "Bit 31 - Bit 2 of ADCPSC"]
    #[inline(always)]
    #[must_use]
    pub fn adcpsc(&mut self) -> ADCPSC_W<CFG2_SPEC, 31> {
        ADCPSC_W::new(self)
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
#[doc = "Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
