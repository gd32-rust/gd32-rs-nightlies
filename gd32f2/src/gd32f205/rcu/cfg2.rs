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
#[doc = "Field `CKOUT0DIV` reader - the CK_OUT0 divider"]
pub type CKOUT0DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKOUT0DIV` writer - the CK_OUT0 divider"]
pub type CKOUT0DIV_W<'a> = crate::FieldWriter<'a, u32, CFG2_SPEC, u8, u8, 6, 0>;
#[doc = "Field `CKOUT1DIV` reader - the CK_OUT1 divider"]
pub type CKOUT1DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKOUT1DIV` writer - the CK_OUT1 divider"]
pub type CKOUT1DIV_W<'a> = crate::FieldWriter<'a, u32, CFG2_SPEC, u8, u8, 6, 8>;
#[doc = "CK_OUT1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKOUT1SEL_A {
    #[doc = "0: No clock selected"]
    NOCLK = 0,
    #[doc = "1: System clock selected"]
    SYSCLK = 1,
    #[doc = "2: High Speed 8M Internal Oscillator clock (IRC8M) selected"]
    IRC8M = 2,
    #[doc = "3: External High Speed oscillator clock (HXTAL) selected"]
    HXTAL = 3,
}
impl From<CKOUT1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKOUT1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKOUT1SEL` reader - CK_OUT1 clock source selection"]
pub type CKOUT1SEL_R = crate::FieldReader<u8, CKOUT1SEL_A>;
impl CKOUT1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKOUT1SEL_A> {
        match self.bits {
            0 => Some(CKOUT1SEL_A::NOCLK),
            1 => Some(CKOUT1SEL_A::SYSCLK),
            2 => Some(CKOUT1SEL_A::IRC8M),
            3 => Some(CKOUT1SEL_A::HXTAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOCLK`"]
    #[inline(always)]
    pub fn is_noclk(&self) -> bool {
        *self == CKOUT1SEL_A::NOCLK
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
}
#[doc = "Field `CKOUT1SEL` writer - CK_OUT1 clock source selection"]
pub type CKOUT1SEL_W<'a> = crate::FieldWriter<'a, u32, CFG2_SPEC, u8, CKOUT1SEL_A, 4, 16>;
impl<'a> CKOUT1SEL_W<'a> {
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn noclk(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::NOCLK)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::SYSCLK)
    }
    #[doc = "High Speed 8M Internal Oscillator clock (IRC8M) selected"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::IRC8M)
    }
    #[doc = "External High Speed oscillator clock (HXTAL) selected"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut W {
        self.variant(CKOUT1SEL_A::HXTAL)
    }
}
impl R {
    #[doc = "Bits 0:5 - the CK_OUT0 divider"]
    #[inline(always)]
    pub fn ckout0div(&self) -> CKOUT0DIV_R {
        CKOUT0DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - the CK_OUT1 divider"]
    #[inline(always)]
    pub fn ckout1div(&self) -> CKOUT1DIV_R {
        CKOUT1DIV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - CK_OUT1 clock source selection"]
    #[inline(always)]
    pub fn ckout1sel(&self) -> CKOUT1SEL_R {
        CKOUT1SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - the CK_OUT0 divider"]
    #[inline(always)]
    pub fn ckout0div(&mut self) -> CKOUT0DIV_W {
        CKOUT0DIV_W::new(self)
    }
    #[doc = "Bits 8:13 - the CK_OUT1 divider"]
    #[inline(always)]
    pub fn ckout1div(&mut self) -> CKOUT1DIV_W {
        CKOUT1DIV_W::new(self)
    }
    #[doc = "Bits 16:19 - CK_OUT1 clock source selection"]
    #[inline(always)]
    pub fn ckout1sel(&mut self) -> CKOUT1SEL_W {
        CKOUT1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](index.html) module"]
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
