#[doc = "Register `EXTISS2` reader"]
pub type R = crate::R<EXTISS2_SPEC>;
#[doc = "Register `EXTISS2` writer"]
pub type W = crate::W<EXTISS2_SPEC>;
#[doc = "Field `EXTI8_SS` reader - EXTI 8 sources selection"]
pub type EXTI8_SS_R = crate::FieldReader<EXTI8_SS_A>;
#[doc = "EXTI 8 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI8_SS_A {
    #[doc = "0: PA8 pin"]
    PA8 = 0,
    #[doc = "1: PB8 pin"]
    PB8 = 1,
    #[doc = "2: PC8 pin"]
    PC8 = 2,
}
impl From<EXTI8_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI8_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI8_SS_A {
    type Ux = u8;
}
impl EXTI8_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI8_SS_A> {
        match self.bits {
            0 => Some(EXTI8_SS_A::PA8),
            1 => Some(EXTI8_SS_A::PB8),
            2 => Some(EXTI8_SS_A::PC8),
            _ => None,
        }
    }
    #[doc = "PA8 pin"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == EXTI8_SS_A::PA8
    }
    #[doc = "PB8 pin"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == EXTI8_SS_A::PB8
    }
    #[doc = "PC8 pin"]
    #[inline(always)]
    pub fn is_pc8(&self) -> bool {
        *self == EXTI8_SS_A::PC8
    }
}
#[doc = "Field `EXTI8_SS` writer - EXTI 8 sources selection"]
pub type EXTI8_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI8_SS_A>;
impl<'a, REG, const O: u8> EXTI8_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA8 pin"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8_SS_A::PA8)
    }
    #[doc = "PB8 pin"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8_SS_A::PB8)
    }
    #[doc = "PC8 pin"]
    #[inline(always)]
    pub fn pc8(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8_SS_A::PC8)
    }
}
#[doc = "Field `EXTI9_SS` reader - EXTI 9 sources selection"]
pub type EXTI9_SS_R = crate::FieldReader<EXTI9_SS_A>;
#[doc = "EXTI 9 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI9_SS_A {
    #[doc = "0: PA9 pin"]
    PA9 = 0,
    #[doc = "1: PB9 pin"]
    PB9 = 1,
    #[doc = "2: PC9 pin"]
    PC9 = 2,
}
impl From<EXTI9_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI9_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI9_SS_A {
    type Ux = u8;
}
impl EXTI9_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI9_SS_A> {
        match self.bits {
            0 => Some(EXTI9_SS_A::PA9),
            1 => Some(EXTI9_SS_A::PB9),
            2 => Some(EXTI9_SS_A::PC9),
            _ => None,
        }
    }
    #[doc = "PA9 pin"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == EXTI9_SS_A::PA9
    }
    #[doc = "PB9 pin"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == EXTI9_SS_A::PB9
    }
    #[doc = "PC9 pin"]
    #[inline(always)]
    pub fn is_pc9(&self) -> bool {
        *self == EXTI9_SS_A::PC9
    }
}
#[doc = "Field `EXTI9_SS` writer - EXTI 9 sources selection"]
pub type EXTI9_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI9_SS_A>;
impl<'a, REG, const O: u8> EXTI9_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA9 pin"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9_SS_A::PA9)
    }
    #[doc = "PB9 pin"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9_SS_A::PB9)
    }
    #[doc = "PC9 pin"]
    #[inline(always)]
    pub fn pc9(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9_SS_A::PC9)
    }
}
#[doc = "Field `EXTI10_SS` reader - EXTI 10 sources selection"]
pub type EXTI10_SS_R = crate::FieldReader<EXTI10_SS_A>;
#[doc = "EXTI 10 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI10_SS_A {
    #[doc = "0: PA10 pin"]
    PA10 = 0,
    #[doc = "1: PB10 pin"]
    PB10 = 1,
    #[doc = "2: PC10 pin"]
    PC10 = 2,
}
impl From<EXTI10_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI10_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI10_SS_A {
    type Ux = u8;
}
impl EXTI10_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI10_SS_A> {
        match self.bits {
            0 => Some(EXTI10_SS_A::PA10),
            1 => Some(EXTI10_SS_A::PB10),
            2 => Some(EXTI10_SS_A::PC10),
            _ => None,
        }
    }
    #[doc = "PA10 pin"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == EXTI10_SS_A::PA10
    }
    #[doc = "PB10 pin"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == EXTI10_SS_A::PB10
    }
    #[doc = "PC10 pin"]
    #[inline(always)]
    pub fn is_pc10(&self) -> bool {
        *self == EXTI10_SS_A::PC10
    }
}
#[doc = "Field `EXTI10_SS` writer - EXTI 10 sources selection"]
pub type EXTI10_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI10_SS_A>;
impl<'a, REG, const O: u8> EXTI10_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA10 pin"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10_SS_A::PA10)
    }
    #[doc = "PB10 pin"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10_SS_A::PB10)
    }
    #[doc = "PC10 pin"]
    #[inline(always)]
    pub fn pc10(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10_SS_A::PC10)
    }
}
#[doc = "Field `EXTI11_SS` reader - EXTI 11 sources selection"]
pub type EXTI11_SS_R = crate::FieldReader<EXTI11_SS_A>;
#[doc = "EXTI 11 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI11_SS_A {
    #[doc = "0: PA11 pin"]
    PA11 = 0,
    #[doc = "1: PB11 pin"]
    PB11 = 1,
    #[doc = "2: PC11 pin"]
    PC11 = 2,
}
impl From<EXTI11_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI11_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI11_SS_A {
    type Ux = u8;
}
impl EXTI11_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI11_SS_A> {
        match self.bits {
            0 => Some(EXTI11_SS_A::PA11),
            1 => Some(EXTI11_SS_A::PB11),
            2 => Some(EXTI11_SS_A::PC11),
            _ => None,
        }
    }
    #[doc = "PA11 pin"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == EXTI11_SS_A::PA11
    }
    #[doc = "PB11 pin"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == EXTI11_SS_A::PB11
    }
    #[doc = "PC11 pin"]
    #[inline(always)]
    pub fn is_pc11(&self) -> bool {
        *self == EXTI11_SS_A::PC11
    }
}
#[doc = "Field `EXTI11_SS` writer - EXTI 11 sources selection"]
pub type EXTI11_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI11_SS_A>;
impl<'a, REG, const O: u8> EXTI11_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA11 pin"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11_SS_A::PA11)
    }
    #[doc = "PB11 pin"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11_SS_A::PB11)
    }
    #[doc = "PC11 pin"]
    #[inline(always)]
    pub fn pc11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11_SS_A::PC11)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline(always)]
    pub fn exti8_ss(&self) -> EXTI8_SS_R {
        EXTI8_SS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline(always)]
    pub fn exti9_ss(&self) -> EXTI9_SS_R {
        EXTI9_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline(always)]
    pub fn exti10_ss(&self) -> EXTI10_SS_R {
        EXTI10_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline(always)]
    pub fn exti11_ss(&self) -> EXTI11_SS_R {
        EXTI11_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti8_ss(&mut self) -> EXTI8_SS_W<EXTISS2_SPEC, 0> {
        EXTI8_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti9_ss(&mut self) -> EXTI9_SS_W<EXTISS2_SPEC, 4> {
        EXTI9_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti10_ss(&mut self) -> EXTI10_SS_W<EXTISS2_SPEC, 8> {
        EXTI10_SS_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti11_ss(&mut self) -> EXTI11_SS_W<EXTISS2_SPEC, 12> {
        EXTI11_SS_W::new(self)
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
#[doc = "EXTI sources selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTISS2_SPEC;
impl crate::RegisterSpec for EXTISS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss2::R`](R) reader structure"]
impl crate::Readable for EXTISS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extiss2::W`](W) writer structure"]
impl crate::Writable for EXTISS2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTISS2 to value 0"]
impl crate::Resettable for EXTISS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
