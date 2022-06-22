#[doc = "Register `CFG3` reader"]
pub struct R(crate::R<CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG3` writer"]
pub struct W(crate::W<CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG3_SPEC>;
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
impl From<crate::W<CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CKOUT1 Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CKOUT1SEL` reader - CKOUT1 Clock Source Selection"]
pub type CKOUT1SEL_R = crate::FieldReader<u8, CKOUT1SEL_A>;
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
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CKOUT1SEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `IRC28M`"]
    #[inline(always)]
    pub fn is_irc28m(&self) -> bool {
        *self == CKOUT1SEL_A::IRC28M
    }
    #[doc = "Checks if the value of the field is `LSI40K`"]
    #[inline(always)]
    pub fn is_lsi40k(&self) -> bool {
        *self == CKOUT1SEL_A::LSI40K
    }
    #[doc = "Checks if the value of the field is `LXTAL`"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == CKOUT1SEL_A::LXTAL
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKOUT1SEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `IRC8M`"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == CKOUT1SEL_A::IRC8M
    }
    #[doc = "Checks if the value of the field is `HXTAL`"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == CKOUT1SEL_A::HXTAL
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CKOUT1SEL_A::PLL
    }
}
#[doc = "Field `CKOUT1SEL` writer - CKOUT1 Clock Source Selection"]
pub type CKOUT1SEL_W<'a> = crate::FieldWriterSafe<'a, u32, CFG3_SPEC, u8, CKOUT1SEL_A, 3, 0>;
impl<'a> CKOUT1SEL_W<'a> {
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::NONE)
    }
    #[doc = "Internal 28 MHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn irc28m(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::IRC28M)
    }
    #[doc = "Internal 40 kHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn lsi40k(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::LSI40K)
    }
    #[doc = "External low speed oscillator clock selected"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::LXTAL)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::SYSCLK)
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::IRC8M)
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::HXTAL)
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::PLL)
    }
}
#[doc = "Field `CKOUT1DIV` reader - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
pub type CKOUT1DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKOUT1DIV` writer - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
pub type CKOUT1DIV_W<'a> = crate::FieldWriter<'a, u32, CFG3_SPEC, u8, u8, 6, 8>;
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
    pub fn ckout1sel(&mut self) -> CKOUT1SEL_W {
        CKOUT1SEL_W::new(self)
    }
    #[doc = "Bits 8:13 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced see bits 2:0 of RCU_CFG3 for CK_OUT1"]
    #[inline(always)]
    pub fn ckout1div(&mut self) -> CKOUT1DIV_W {
        CKOUT1DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg3](index.html) module"]
pub struct CFG3_SPEC;
impl crate::RegisterSpec for CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg3::R](R) reader structure"]
impl crate::Readable for CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg3::W](W) writer structure"]
impl crate::Writable for CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG3 to value 0"]
impl crate::Resettable for CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
