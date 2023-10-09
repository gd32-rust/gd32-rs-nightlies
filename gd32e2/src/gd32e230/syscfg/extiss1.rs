#[doc = "Register `EXTISS1` reader"]
pub type R = crate::R<EXTISS1_SPEC>;
#[doc = "Register `EXTISS1` writer"]
pub type W = crate::W<EXTISS1_SPEC>;
#[doc = "Field `EXTI4_SS` reader - EXTI 4 sources selection"]
pub type EXTI4_SS_R = crate::FieldReader<EXTI4_SS_A>;
#[doc = "EXTI 4 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI4_SS_A {
    #[doc = "0: PA4 pin"]
    PA4 = 0,
    #[doc = "1: PB4 pin"]
    PB4 = 1,
    #[doc = "2: PC4 pin"]
    PC4 = 2,
    #[doc = "5: PF4 pin"]
    PF4 = 5,
}
impl From<EXTI4_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI4_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI4_SS_A {
    type Ux = u8;
}
impl EXTI4_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI4_SS_A> {
        match self.bits {
            0 => Some(EXTI4_SS_A::PA4),
            1 => Some(EXTI4_SS_A::PB4),
            2 => Some(EXTI4_SS_A::PC4),
            5 => Some(EXTI4_SS_A::PF4),
            _ => None,
        }
    }
    #[doc = "PA4 pin"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == EXTI4_SS_A::PA4
    }
    #[doc = "PB4 pin"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == EXTI4_SS_A::PB4
    }
    #[doc = "PC4 pin"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == EXTI4_SS_A::PC4
    }
    #[doc = "PF4 pin"]
    #[inline(always)]
    pub fn is_pf4(&self) -> bool {
        *self == EXTI4_SS_A::PF4
    }
}
#[doc = "Field `EXTI4_SS` writer - EXTI 4 sources selection"]
pub type EXTI4_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI4_SS_A>;
impl<'a, REG, const O: u8> EXTI4_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA4 pin"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4_SS_A::PA4)
    }
    #[doc = "PB4 pin"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4_SS_A::PB4)
    }
    #[doc = "PC4 pin"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4_SS_A::PC4)
    }
    #[doc = "PF4 pin"]
    #[inline(always)]
    pub fn pf4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4_SS_A::PF4)
    }
}
#[doc = "Field `EXTI5_SS` reader - EXTI 5 sources selection"]
pub type EXTI5_SS_R = crate::FieldReader<EXTI5_SS_A>;
#[doc = "EXTI 5 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI5_SS_A {
    #[doc = "0: PA5 pin"]
    PA5 = 0,
    #[doc = "1: PB5 pin"]
    PB5 = 1,
    #[doc = "2: PC5 pin"]
    PC5 = 2,
    #[doc = "5: PF5 pin"]
    PF5 = 5,
}
impl From<EXTI5_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI5_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI5_SS_A {
    type Ux = u8;
}
impl EXTI5_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI5_SS_A> {
        match self.bits {
            0 => Some(EXTI5_SS_A::PA5),
            1 => Some(EXTI5_SS_A::PB5),
            2 => Some(EXTI5_SS_A::PC5),
            5 => Some(EXTI5_SS_A::PF5),
            _ => None,
        }
    }
    #[doc = "PA5 pin"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == EXTI5_SS_A::PA5
    }
    #[doc = "PB5 pin"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == EXTI5_SS_A::PB5
    }
    #[doc = "PC5 pin"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == EXTI5_SS_A::PC5
    }
    #[doc = "PF5 pin"]
    #[inline(always)]
    pub fn is_pf5(&self) -> bool {
        *self == EXTI5_SS_A::PF5
    }
}
#[doc = "Field `EXTI5_SS` writer - EXTI 5 sources selection"]
pub type EXTI5_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI5_SS_A>;
impl<'a, REG, const O: u8> EXTI5_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA5 pin"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5_SS_A::PA5)
    }
    #[doc = "PB5 pin"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5_SS_A::PB5)
    }
    #[doc = "PC5 pin"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5_SS_A::PC5)
    }
    #[doc = "PF5 pin"]
    #[inline(always)]
    pub fn pf5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5_SS_A::PF5)
    }
}
#[doc = "Field `EXTI6_SS` reader - EXTI 6 sources selection"]
pub type EXTI6_SS_R = crate::FieldReader<EXTI6_SS_A>;
#[doc = "EXTI 6 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI6_SS_A {
    #[doc = "0: PA6 pin"]
    PA6 = 0,
    #[doc = "1: PB6 pin"]
    PB6 = 1,
    #[doc = "2: PC6 pin"]
    PC6 = 2,
    #[doc = "5: PF6 pin"]
    PF6 = 5,
}
impl From<EXTI6_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI6_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI6_SS_A {
    type Ux = u8;
}
impl EXTI6_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI6_SS_A> {
        match self.bits {
            0 => Some(EXTI6_SS_A::PA6),
            1 => Some(EXTI6_SS_A::PB6),
            2 => Some(EXTI6_SS_A::PC6),
            5 => Some(EXTI6_SS_A::PF6),
            _ => None,
        }
    }
    #[doc = "PA6 pin"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == EXTI6_SS_A::PA6
    }
    #[doc = "PB6 pin"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == EXTI6_SS_A::PB6
    }
    #[doc = "PC6 pin"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == EXTI6_SS_A::PC6
    }
    #[doc = "PF6 pin"]
    #[inline(always)]
    pub fn is_pf6(&self) -> bool {
        *self == EXTI6_SS_A::PF6
    }
}
#[doc = "Field `EXTI6_SS` writer - EXTI 6 sources selection"]
pub type EXTI6_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI6_SS_A>;
impl<'a, REG, const O: u8> EXTI6_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA6 pin"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6_SS_A::PA6)
    }
    #[doc = "PB6 pin"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6_SS_A::PB6)
    }
    #[doc = "PC6 pin"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6_SS_A::PC6)
    }
    #[doc = "PF6 pin"]
    #[inline(always)]
    pub fn pf6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6_SS_A::PF6)
    }
}
#[doc = "Field `EXTI7_SS` reader - EXTI 7 sources selection"]
pub type EXTI7_SS_R = crate::FieldReader<EXTI7_SS_A>;
#[doc = "EXTI 7 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI7_SS_A {
    #[doc = "0: PA7 pin"]
    PA7 = 0,
    #[doc = "1: PB7 pin"]
    PB7 = 1,
    #[doc = "2: PC7 pin"]
    PC7 = 2,
    #[doc = "5: PF7 pin"]
    PF7 = 5,
}
impl From<EXTI7_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI7_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI7_SS_A {
    type Ux = u8;
}
impl EXTI7_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI7_SS_A> {
        match self.bits {
            0 => Some(EXTI7_SS_A::PA7),
            1 => Some(EXTI7_SS_A::PB7),
            2 => Some(EXTI7_SS_A::PC7),
            5 => Some(EXTI7_SS_A::PF7),
            _ => None,
        }
    }
    #[doc = "PA7 pin"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == EXTI7_SS_A::PA7
    }
    #[doc = "PB7 pin"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == EXTI7_SS_A::PB7
    }
    #[doc = "PC7 pin"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == EXTI7_SS_A::PC7
    }
    #[doc = "PF7 pin"]
    #[inline(always)]
    pub fn is_pf7(&self) -> bool {
        *self == EXTI7_SS_A::PF7
    }
}
#[doc = "Field `EXTI7_SS` writer - EXTI 7 sources selection"]
pub type EXTI7_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI7_SS_A>;
impl<'a, REG, const O: u8> EXTI7_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA7 pin"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7_SS_A::PA7)
    }
    #[doc = "PB7 pin"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7_SS_A::PB7)
    }
    #[doc = "PC7 pin"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7_SS_A::PC7)
    }
    #[doc = "PF7 pin"]
    #[inline(always)]
    pub fn pf7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7_SS_A::PF7)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    pub fn exti4_ss(&self) -> EXTI4_SS_R {
        EXTI4_SS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    pub fn exti5_ss(&self) -> EXTI5_SS_R {
        EXTI5_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    pub fn exti6_ss(&self) -> EXTI6_SS_R {
        EXTI6_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    pub fn exti7_ss(&self) -> EXTI7_SS_R {
        EXTI7_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti4_ss(&mut self) -> EXTI4_SS_W<EXTISS1_SPEC, 0> {
        EXTI4_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti5_ss(&mut self) -> EXTI5_SS_W<EXTISS1_SPEC, 4> {
        EXTI5_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti6_ss(&mut self) -> EXTI6_SS_W<EXTISS1_SPEC, 8> {
        EXTI6_SS_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti7_ss(&mut self) -> EXTI7_SS_W<EXTISS1_SPEC, 12> {
        EXTI7_SS_W::new(self)
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
#[doc = "EXTI sources selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTISS1_SPEC;
impl crate::RegisterSpec for EXTISS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss1::R`](R) reader structure"]
impl crate::Readable for EXTISS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extiss1::W`](W) writer structure"]
impl crate::Writable for EXTISS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTISS1 to value 0"]
impl crate::Resettable for EXTISS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
