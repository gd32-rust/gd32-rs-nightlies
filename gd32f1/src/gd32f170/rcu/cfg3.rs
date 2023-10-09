#[doc = "Register `CFG3` reader"]
pub type R = crate::R<CFG3_SPEC>;
#[doc = "Register `CFG3` writer"]
pub type W = crate::W<CFG3_SPEC>;
#[doc = "Field `CKOUT1SEL` reader - CKOUT1 Clock Source Selection"]
pub type CKOUT1SEL_R = crate::FieldReader<CKOUT1SEL_A>;
#[doc = "CKOUT1 Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKOUT1SEL_A {
    #[doc = "0: No clock selected"]
    NONE = 0,
    #[doc = "1: Internal 28 MHz RC oscillator clock selected"]
    IRC28M = 1,
    #[doc = "2: Internal 40 kHz RC oscillator clock selected"]
    LSI40K = 2,
    #[doc = "3: External low speed oscillator clock selected"]
    LXTAL = 3,
    #[doc = "4: System clock selected"]
    SYSCLK = 4,
    #[doc = "5: Internal RC 8 MHz (HSI) oscillator clock selected"]
    IRC8M = 5,
    #[doc = "6: External 4-32 MHz (HSE) oscillator clock selected"]
    HXTAL = 6,
    #[doc = "7: PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    PLL = 7,
}
impl From<CKOUT1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKOUT1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKOUT1SEL_A {
    type Ux = u8;
}
impl CKOUT1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKOUT1SEL_A {
        match self.bits {
            0 => CKOUT1SEL_A::NONE,
            1 => CKOUT1SEL_A::IRC28M,
            2 => CKOUT1SEL_A::LSI40K,
            3 => CKOUT1SEL_A::LXTAL,
            4 => CKOUT1SEL_A::SYSCLK,
            5 => CKOUT1SEL_A::IRC8M,
            6 => CKOUT1SEL_A::HXTAL,
            7 => CKOUT1SEL_A::PLL,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CKOUT1SEL_A::NONE
    }
    #[doc = "Internal 28 MHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn is_irc28m(&self) -> bool {
        *self == CKOUT1SEL_A::IRC28M
    }
    #[doc = "Internal 40 kHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn is_lsi40k(&self) -> bool {
        *self == CKOUT1SEL_A::LSI40K
    }
    #[doc = "External low speed oscillator clock selected"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == CKOUT1SEL_A::LXTAL
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKOUT1SEL_A::SYSCLK
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == CKOUT1SEL_A::IRC8M
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == CKOUT1SEL_A::HXTAL
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CKOUT1SEL_A::PLL
    }
}
#[doc = "Field `CKOUT1SEL` writer - CKOUT1 Clock Source Selection"]
pub type CKOUT1SEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CKOUT1SEL_A>;
impl<'a, REG, const O: u8> CKOUT1SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::NONE)
    }
    #[doc = "Internal 28 MHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn irc28m(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::IRC28M)
    }
    #[doc = "Internal 40 kHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn lsi40k(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::LSI40K)
    }
    #[doc = "External low speed oscillator clock selected"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::LXTAL)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::SYSCLK)
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::IRC8M)
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::HXTAL)
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::PLL)
    }
}
#[doc = "Field `CKOUT1DIV` reader - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
pub type CKOUT1DIV_R = crate::FieldReader;
#[doc = "Field `CKOUT1DIV` writer - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
pub type CKOUT1DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:2 - CKOUT1 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout1sel(&self) -> CKOUT1SEL_R {
        CKOUT1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:13 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
    #[inline(always)]
    pub fn ckout1div(&self) -> CKOUT1DIV_R {
        CKOUT1DIV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CKOUT1 Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckout1sel(&mut self) -> CKOUT1SEL_W<CFG3_SPEC, 0> {
        CKOUT1SEL_W::new(self)
    }
    #[doc = "Bits 8:13 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn ckout1div(&mut self) -> CKOUT1DIV_W<CFG3_SPEC, 8> {
        CKOUT1DIV_W::new(self)
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
#[doc = "Configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG3_SPEC;
impl crate::RegisterSpec for CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg3::R`](R) reader structure"]
impl crate::Readable for CFG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg3::W`](W) writer structure"]
impl crate::Writable for CFG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG3 to value 0"]
impl crate::Resettable for CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
