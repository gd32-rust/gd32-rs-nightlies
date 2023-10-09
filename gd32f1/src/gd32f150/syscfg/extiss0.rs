#[doc = "Register `EXTISS0` reader"]
pub type R = crate::R<EXTISS0_SPEC>;
#[doc = "Register `EXTISS0` writer"]
pub type W = crate::W<EXTISS0_SPEC>;
#[doc = "Field `EXTI0_SS` reader - EXTI 0 sources selection"]
pub type EXTI0_SS_R = crate::FieldReader<EXTI0_SS_A>;
#[doc = "EXTI 0 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0_SS_A {
    #[doc = "0: PA0 pin"]
    PA0 = 0,
    #[doc = "1: PB0 pin"]
    PB0 = 1,
    #[doc = "2: PC0 pin"]
    PC0 = 2,
    #[doc = "5: PF0 pin"]
    PF0 = 5,
}
impl From<EXTI0_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI0_SS_A {
    type Ux = u8;
}
impl EXTI0_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI0_SS_A> {
        match self.bits {
            0 => Some(EXTI0_SS_A::PA0),
            1 => Some(EXTI0_SS_A::PB0),
            2 => Some(EXTI0_SS_A::PC0),
            5 => Some(EXTI0_SS_A::PF0),
            _ => None,
        }
    }
    #[doc = "PA0 pin"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == EXTI0_SS_A::PA0
    }
    #[doc = "PB0 pin"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == EXTI0_SS_A::PB0
    }
    #[doc = "PC0 pin"]
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == EXTI0_SS_A::PC0
    }
    #[doc = "PF0 pin"]
    #[inline(always)]
    pub fn is_pf0(&self) -> bool {
        *self == EXTI0_SS_A::PF0
    }
}
#[doc = "Field `EXTI0_SS` writer - EXTI 0 sources selection"]
pub type EXTI0_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI0_SS_A>;
impl<'a, REG, const O: u8> EXTI0_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA0 pin"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_SS_A::PA0)
    }
    #[doc = "PB0 pin"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_SS_A::PB0)
    }
    #[doc = "PC0 pin"]
    #[inline(always)]
    pub fn pc0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_SS_A::PC0)
    }
    #[doc = "PF0 pin"]
    #[inline(always)]
    pub fn pf0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_SS_A::PF0)
    }
}
#[doc = "Field `EXTI1_SS` reader - EXTI 1 sources selection"]
pub type EXTI1_SS_R = crate::FieldReader<EXTI1_SS_A>;
#[doc = "EXTI 1 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI1_SS_A {
    #[doc = "0: PA1 pin"]
    PA1 = 0,
    #[doc = "1: PB1 pin"]
    PB1 = 1,
    #[doc = "2: PC1 pin"]
    PC1 = 2,
    #[doc = "5: PF1 pin"]
    PF1 = 5,
}
impl From<EXTI1_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI1_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI1_SS_A {
    type Ux = u8;
}
impl EXTI1_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI1_SS_A> {
        match self.bits {
            0 => Some(EXTI1_SS_A::PA1),
            1 => Some(EXTI1_SS_A::PB1),
            2 => Some(EXTI1_SS_A::PC1),
            5 => Some(EXTI1_SS_A::PF1),
            _ => None,
        }
    }
    #[doc = "PA1 pin"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == EXTI1_SS_A::PA1
    }
    #[doc = "PB1 pin"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == EXTI1_SS_A::PB1
    }
    #[doc = "PC1 pin"]
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == EXTI1_SS_A::PC1
    }
    #[doc = "PF1 pin"]
    #[inline(always)]
    pub fn is_pf1(&self) -> bool {
        *self == EXTI1_SS_A::PF1
    }
}
#[doc = "Field `EXTI1_SS` writer - EXTI 1 sources selection"]
pub type EXTI1_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI1_SS_A>;
impl<'a, REG, const O: u8> EXTI1_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA1 pin"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1_SS_A::PA1)
    }
    #[doc = "PB1 pin"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1_SS_A::PB1)
    }
    #[doc = "PC1 pin"]
    #[inline(always)]
    pub fn pc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1_SS_A::PC1)
    }
    #[doc = "PF1 pin"]
    #[inline(always)]
    pub fn pf1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1_SS_A::PF1)
    }
}
#[doc = "Field `EXTI2_SS` reader - EXTI 2 sources selection"]
pub type EXTI2_SS_R = crate::FieldReader<EXTI2_SS_A>;
#[doc = "EXTI 2 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI2_SS_A {
    #[doc = "0: PA2 pin"]
    PA2 = 0,
    #[doc = "1: PB2 pin"]
    PB2 = 1,
    #[doc = "2: PC2 pin"]
    PC2 = 2,
    #[doc = "3: PD2 pin"]
    PD2 = 3,
}
impl From<EXTI2_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI2_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI2_SS_A {
    type Ux = u8;
}
impl EXTI2_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI2_SS_A> {
        match self.bits {
            0 => Some(EXTI2_SS_A::PA2),
            1 => Some(EXTI2_SS_A::PB2),
            2 => Some(EXTI2_SS_A::PC2),
            3 => Some(EXTI2_SS_A::PD2),
            _ => None,
        }
    }
    #[doc = "PA2 pin"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == EXTI2_SS_A::PA2
    }
    #[doc = "PB2 pin"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == EXTI2_SS_A::PB2
    }
    #[doc = "PC2 pin"]
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == EXTI2_SS_A::PC2
    }
    #[doc = "PD2 pin"]
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == EXTI2_SS_A::PD2
    }
}
#[doc = "Field `EXTI2_SS` writer - EXTI 2 sources selection"]
pub type EXTI2_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI2_SS_A>;
impl<'a, REG, const O: u8> EXTI2_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA2 pin"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2_SS_A::PA2)
    }
    #[doc = "PB2 pin"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2_SS_A::PB2)
    }
    #[doc = "PC2 pin"]
    #[inline(always)]
    pub fn pc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2_SS_A::PC2)
    }
    #[doc = "PD2 pin"]
    #[inline(always)]
    pub fn pd2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2_SS_A::PD2)
    }
}
#[doc = "Field `EXTI3_SS` reader - EXTI 3 sources selection"]
pub type EXTI3_SS_R = crate::FieldReader<EXTI3_SS_A>;
#[doc = "EXTI 3 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI3_SS_A {
    #[doc = "0: PA3 pin"]
    PA3 = 0,
    #[doc = "1: PB3 pin"]
    PB3 = 1,
    #[doc = "2: PC3 pin"]
    PC3 = 2,
}
impl From<EXTI3_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI3_SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI3_SS_A {
    type Ux = u8;
}
impl EXTI3_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI3_SS_A> {
        match self.bits {
            0 => Some(EXTI3_SS_A::PA3),
            1 => Some(EXTI3_SS_A::PB3),
            2 => Some(EXTI3_SS_A::PC3),
            _ => None,
        }
    }
    #[doc = "PA3 pin"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == EXTI3_SS_A::PA3
    }
    #[doc = "PB3 pin"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == EXTI3_SS_A::PB3
    }
    #[doc = "PC3 pin"]
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == EXTI3_SS_A::PC3
    }
}
#[doc = "Field `EXTI3_SS` writer - EXTI 3 sources selection"]
pub type EXTI3_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXTI3_SS_A>;
impl<'a, REG, const O: u8> EXTI3_SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA3 pin"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3_SS_A::PA3)
    }
    #[doc = "PB3 pin"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3_SS_A::PB3)
    }
    #[doc = "PC3 pin"]
    #[inline(always)]
    pub fn pc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3_SS_A::PC3)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline(always)]
    pub fn exti0_ss(&self) -> EXTI0_SS_R {
        EXTI0_SS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline(always)]
    pub fn exti1_ss(&self) -> EXTI1_SS_R {
        EXTI1_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline(always)]
    pub fn exti2_ss(&self) -> EXTI2_SS_R {
        EXTI2_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline(always)]
    pub fn exti3_ss(&self) -> EXTI3_SS_R {
        EXTI3_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti0_ss(&mut self) -> EXTI0_SS_W<EXTISS0_SPEC, 0> {
        EXTI0_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti1_ss(&mut self) -> EXTI1_SS_W<EXTISS0_SPEC, 4> {
        EXTI1_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti2_ss(&mut self) -> EXTI2_SS_W<EXTISS0_SPEC, 8> {
        EXTI2_SS_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti3_ss(&mut self) -> EXTI3_SS_W<EXTISS0_SPEC, 12> {
        EXTI3_SS_W::new(self)
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
#[doc = "EXTI sources selection register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTISS0_SPEC;
impl crate::RegisterSpec for EXTISS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss0::R`](R) reader structure"]
impl crate::Readable for EXTISS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extiss0::W`](W) writer structure"]
impl crate::Writable for EXTISS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTISS0 to value 0"]
impl crate::Resettable for EXTISS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
