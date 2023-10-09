#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "Field `CKOUT0DIV` reader - the CK_OUT0 divider"]
pub type CKOUT0DIV_R = crate::FieldReader;
#[doc = "Field `CKOUT0DIV` writer - the CK_OUT0 divider"]
pub type CKOUT0DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CKOUT1DIV` reader - the CK_OUT1 divider"]
pub type CKOUT1DIV_R = crate::FieldReader;
#[doc = "Field `CKOUT1DIV` writer - the CK_OUT1 divider"]
pub type CKOUT1DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CKOUT1SEL` reader - CK_OUT1 clock source selection"]
pub type CKOUT1SEL_R = crate::FieldReader<CKOUT1SEL_A>;
#[doc = "CK_OUT1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CKOUT1SEL_A {
    type Ux = u8;
}
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
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn is_noclk(&self) -> bool {
        *self == CKOUT1SEL_A::NOCLK
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKOUT1SEL_A::SYSCLK
    }
    #[doc = "High Speed 8M Internal Oscillator clock (IRC8M) selected"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == CKOUT1SEL_A::IRC8M
    }
    #[doc = "External High Speed oscillator clock (HXTAL) selected"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == CKOUT1SEL_A::HXTAL
    }
}
#[doc = "Field `CKOUT1SEL` writer - CK_OUT1 clock source selection"]
pub type CKOUT1SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, CKOUT1SEL_A>;
impl<'a, REG, const O: u8> CKOUT1SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn noclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::NOCLK)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::SYSCLK)
    }
    #[doc = "High Speed 8M Internal Oscillator clock (IRC8M) selected"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUT1SEL_A::IRC8M)
    }
    #[doc = "External High Speed oscillator clock (HXTAL) selected"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
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
    #[must_use]
    pub fn ckout0div(&mut self) -> CKOUT0DIV_W<CFG2_SPEC, 0> {
        CKOUT0DIV_W::new(self)
    }
    #[doc = "Bits 8:13 - the CK_OUT1 divider"]
    #[inline(always)]
    #[must_use]
    pub fn ckout1div(&mut self) -> CKOUT1DIV_W<CFG2_SPEC, 8> {
        CKOUT1DIV_W::new(self)
    }
    #[doc = "Bits 16:19 - CK_OUT1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckout1sel(&mut self) -> CKOUT1SEL_W<CFG2_SPEC, 16> {
        CKOUT1SEL_W::new(self)
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
#[doc = "configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
