#[doc = "Register `EXTISS1` reader"]
pub struct R(crate::R<EXTISS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTISS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTISS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTISS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTISS1` writer"]
pub struct W(crate::W<EXTISS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTISS1_SPEC>;
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
impl From<crate::W<EXTISS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTISS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EXTI 7 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EXTI7_SS` reader - EXTI 7 sources selection"]
pub type EXTI7_SS_R = crate::FieldReader<u8, EXTI7_SS_A>;
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
    #[doc = "Checks if the value of the field is `PA7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == EXTI7_SS_A::PA7
    }
    #[doc = "Checks if the value of the field is `PB7`"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == EXTI7_SS_A::PB7
    }
    #[doc = "Checks if the value of the field is `PC7`"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == EXTI7_SS_A::PC7
    }
    #[doc = "Checks if the value of the field is `PF7`"]
    #[inline(always)]
    pub fn is_pf7(&self) -> bool {
        *self == EXTI7_SS_A::PF7
    }
}
#[doc = "Field `EXTI7_SS` writer - EXTI 7 sources selection"]
pub type EXTI7_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS1_SPEC, u8, EXTI7_SS_A, 4, 12>;
impl<'a> EXTI7_SS_W<'a> {
    #[doc = "PA7 pin"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(EXTI7_SS_A::PA7)
    }
    #[doc = "PB7 pin"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut W {
        self.variant(EXTI7_SS_A::PB7)
    }
    #[doc = "PC7 pin"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut W {
        self.variant(EXTI7_SS_A::PC7)
    }
    #[doc = "PF7 pin"]
    #[inline(always)]
    pub fn pf7(self) -> &'a mut W {
        self.variant(EXTI7_SS_A::PF7)
    }
}
#[doc = "EXTI 6 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EXTI6_SS` reader - EXTI 6 sources selection"]
pub type EXTI6_SS_R = crate::FieldReader<u8, EXTI6_SS_A>;
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
    #[doc = "Checks if the value of the field is `PA6`"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == EXTI6_SS_A::PA6
    }
    #[doc = "Checks if the value of the field is `PB6`"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == EXTI6_SS_A::PB6
    }
    #[doc = "Checks if the value of the field is `PC6`"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == EXTI6_SS_A::PC6
    }
    #[doc = "Checks if the value of the field is `PF6`"]
    #[inline(always)]
    pub fn is_pf6(&self) -> bool {
        *self == EXTI6_SS_A::PF6
    }
}
#[doc = "Field `EXTI6_SS` writer - EXTI 6 sources selection"]
pub type EXTI6_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS1_SPEC, u8, EXTI6_SS_A, 4, 8>;
impl<'a> EXTI6_SS_W<'a> {
    #[doc = "PA6 pin"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut W {
        self.variant(EXTI6_SS_A::PA6)
    }
    #[doc = "PB6 pin"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(EXTI6_SS_A::PB6)
    }
    #[doc = "PC6 pin"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut W {
        self.variant(EXTI6_SS_A::PC6)
    }
    #[doc = "PF6 pin"]
    #[inline(always)]
    pub fn pf6(self) -> &'a mut W {
        self.variant(EXTI6_SS_A::PF6)
    }
}
#[doc = "EXTI 5 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EXTI5_SS` reader - EXTI 5 sources selection"]
pub type EXTI5_SS_R = crate::FieldReader<u8, EXTI5_SS_A>;
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
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == EXTI5_SS_A::PA5
    }
    #[doc = "Checks if the value of the field is `PB5`"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == EXTI5_SS_A::PB5
    }
    #[doc = "Checks if the value of the field is `PC5`"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == EXTI5_SS_A::PC5
    }
    #[doc = "Checks if the value of the field is `PF5`"]
    #[inline(always)]
    pub fn is_pf5(&self) -> bool {
        *self == EXTI5_SS_A::PF5
    }
}
#[doc = "Field `EXTI5_SS` writer - EXTI 5 sources selection"]
pub type EXTI5_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS1_SPEC, u8, EXTI5_SS_A, 4, 4>;
impl<'a> EXTI5_SS_W<'a> {
    #[doc = "PA5 pin"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(EXTI5_SS_A::PA5)
    }
    #[doc = "PB5 pin"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut W {
        self.variant(EXTI5_SS_A::PB5)
    }
    #[doc = "PC5 pin"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(EXTI5_SS_A::PC5)
    }
    #[doc = "PF5 pin"]
    #[inline(always)]
    pub fn pf5(self) -> &'a mut W {
        self.variant(EXTI5_SS_A::PF5)
    }
}
#[doc = "EXTI 4 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EXTI4_SS` reader - EXTI 4 sources selection"]
pub type EXTI4_SS_R = crate::FieldReader<u8, EXTI4_SS_A>;
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
    #[doc = "Checks if the value of the field is `PA4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == EXTI4_SS_A::PA4
    }
    #[doc = "Checks if the value of the field is `PB4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == EXTI4_SS_A::PB4
    }
    #[doc = "Checks if the value of the field is `PC4`"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == EXTI4_SS_A::PC4
    }
    #[doc = "Checks if the value of the field is `PF4`"]
    #[inline(always)]
    pub fn is_pf4(&self) -> bool {
        *self == EXTI4_SS_A::PF4
    }
}
#[doc = "Field `EXTI4_SS` writer - EXTI 4 sources selection"]
pub type EXTI4_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS1_SPEC, u8, EXTI4_SS_A, 4, 0>;
impl<'a> EXTI4_SS_W<'a> {
    #[doc = "PA4 pin"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(EXTI4_SS_A::PA4)
    }
    #[doc = "PB4 pin"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(EXTI4_SS_A::PB4)
    }
    #[doc = "PC4 pin"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut W {
        self.variant(EXTI4_SS_A::PC4)
    }
    #[doc = "PF4 pin"]
    #[inline(always)]
    pub fn pf4(self) -> &'a mut W {
        self.variant(EXTI4_SS_A::PF4)
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    pub fn exti7_ss(&self) -> EXTI7_SS_R {
        EXTI7_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    pub fn exti6_ss(&self) -> EXTI6_SS_R {
        EXTI6_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    pub fn exti5_ss(&self) -> EXTI5_SS_R {
        EXTI5_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    pub fn exti4_ss(&self) -> EXTI4_SS_R {
        EXTI4_SS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    pub fn exti7_ss(&mut self) -> EXTI7_SS_W {
        EXTI7_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    pub fn exti6_ss(&mut self) -> EXTI6_SS_W {
        EXTI6_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    pub fn exti5_ss(&mut self) -> EXTI5_SS_W {
        EXTI5_SS_W::new(self)
    }
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    pub fn exti4_ss(&mut self) -> EXTI4_SS_W {
        EXTI4_SS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI sources selection register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss1](index.html) module"]
pub struct EXTISS1_SPEC;
impl crate::RegisterSpec for EXTISS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extiss1::R](R) reader structure"]
impl crate::Readable for EXTISS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extiss1::W](W) writer structure"]
impl crate::Writable for EXTISS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTISS1 to value 0"]
impl crate::Resettable for EXTISS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
