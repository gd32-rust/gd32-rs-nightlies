#[doc = "Register `EXTISS0` reader"]
pub struct R(crate::R<EXTISS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTISS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTISS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTISS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTISS0` writer"]
pub struct W(crate::W<EXTISS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTISS0_SPEC>;
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
impl From<crate::W<EXTISS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTISS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EXTI 3 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EXTI3_SS` reader - EXTI 3 sources selection"]
pub type EXTI3_SS_R = crate::FieldReader<u8, EXTI3_SS_A>;
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
    #[doc = "Checks if the value of the field is `PA3`"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == EXTI3_SS_A::PA3
    }
    #[doc = "Checks if the value of the field is `PB3`"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == EXTI3_SS_A::PB3
    }
    #[doc = "Checks if the value of the field is `PC3`"]
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == EXTI3_SS_A::PC3
    }
}
#[doc = "Field `EXTI3_SS` writer - EXTI 3 sources selection"]
pub type EXTI3_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS0_SPEC, u8, EXTI3_SS_A, 4, 12>;
impl<'a> EXTI3_SS_W<'a> {
    #[doc = "PA3 pin"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut W {
        self.variant(EXTI3_SS_A::PA3)
    }
    #[doc = "PB3 pin"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(EXTI3_SS_A::PB3)
    }
    #[doc = "PC3 pin"]
    #[inline(always)]
    pub fn pc3(self) -> &'a mut W {
        self.variant(EXTI3_SS_A::PC3)
    }
}
#[doc = "EXTI 2 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EXTI2_SS` reader - EXTI 2 sources selection"]
pub type EXTI2_SS_R = crate::FieldReader<u8, EXTI2_SS_A>;
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
    #[doc = "Checks if the value of the field is `PA2`"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == EXTI2_SS_A::PA2
    }
    #[doc = "Checks if the value of the field is `PB2`"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == EXTI2_SS_A::PB2
    }
    #[doc = "Checks if the value of the field is `PC2`"]
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == EXTI2_SS_A::PC2
    }
    #[doc = "Checks if the value of the field is `PD2`"]
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == EXTI2_SS_A::PD2
    }
}
#[doc = "Field `EXTI2_SS` writer - EXTI 2 sources selection"]
pub type EXTI2_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS0_SPEC, u8, EXTI2_SS_A, 4, 8>;
impl<'a> EXTI2_SS_W<'a> {
    #[doc = "PA2 pin"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(EXTI2_SS_A::PA2)
    }
    #[doc = "PB2 pin"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(EXTI2_SS_A::PB2)
    }
    #[doc = "PC2 pin"]
    #[inline(always)]
    pub fn pc2(self) -> &'a mut W {
        self.variant(EXTI2_SS_A::PC2)
    }
    #[doc = "PD2 pin"]
    #[inline(always)]
    pub fn pd2(self) -> &'a mut W {
        self.variant(EXTI2_SS_A::PD2)
    }
}
#[doc = "EXTI 1 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EXTI1_SS` reader - EXTI 1 sources selection"]
pub type EXTI1_SS_R = crate::FieldReader<u8, EXTI1_SS_A>;
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
    #[doc = "Checks if the value of the field is `PA1`"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == EXTI1_SS_A::PA1
    }
    #[doc = "Checks if the value of the field is `PB1`"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == EXTI1_SS_A::PB1
    }
    #[doc = "Checks if the value of the field is `PC1`"]
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == EXTI1_SS_A::PC1
    }
    #[doc = "Checks if the value of the field is `PF1`"]
    #[inline(always)]
    pub fn is_pf1(&self) -> bool {
        *self == EXTI1_SS_A::PF1
    }
}
#[doc = "Field `EXTI1_SS` writer - EXTI 1 sources selection"]
pub type EXTI1_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS0_SPEC, u8, EXTI1_SS_A, 4, 4>;
impl<'a> EXTI1_SS_W<'a> {
    #[doc = "PA1 pin"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut W {
        self.variant(EXTI1_SS_A::PA1)
    }
    #[doc = "PB1 pin"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut W {
        self.variant(EXTI1_SS_A::PB1)
    }
    #[doc = "PC1 pin"]
    #[inline(always)]
    pub fn pc1(self) -> &'a mut W {
        self.variant(EXTI1_SS_A::PC1)
    }
    #[doc = "PF1 pin"]
    #[inline(always)]
    pub fn pf1(self) -> &'a mut W {
        self.variant(EXTI1_SS_A::PF1)
    }
}
#[doc = "EXTI 0 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EXTI0_SS` reader - EXTI 0 sources selection"]
pub type EXTI0_SS_R = crate::FieldReader<u8, EXTI0_SS_A>;
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
    #[doc = "Checks if the value of the field is `PA0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == EXTI0_SS_A::PA0
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == EXTI0_SS_A::PB0
    }
    #[doc = "Checks if the value of the field is `PC0`"]
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == EXTI0_SS_A::PC0
    }
    #[doc = "Checks if the value of the field is `PF0`"]
    #[inline(always)]
    pub fn is_pf0(&self) -> bool {
        *self == EXTI0_SS_A::PF0
    }
}
#[doc = "Field `EXTI0_SS` writer - EXTI 0 sources selection"]
pub type EXTI0_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS0_SPEC, u8, EXTI0_SS_A, 4, 0>;
impl<'a> EXTI0_SS_W<'a> {
    #[doc = "PA0 pin"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(EXTI0_SS_A::PA0)
    }
    #[doc = "PB0 pin"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(EXTI0_SS_A::PB0)
    }
    #[doc = "PC0 pin"]
    #[inline(always)]
    pub fn pc0(self) -> &'a mut W {
        self.variant(EXTI0_SS_A::PC0)
    }
    #[doc = "PF0 pin"]
    #[inline(always)]
    pub fn pf0(self) -> &'a mut W {
        self.variant(EXTI0_SS_A::PF0)
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline(always)]
    pub fn exti3_ss(&self) -> EXTI3_SS_R {
        EXTI3_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline(always)]
    pub fn exti2_ss(&self) -> EXTI2_SS_R {
        EXTI2_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline(always)]
    pub fn exti1_ss(&self) -> EXTI1_SS_R {
        EXTI1_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline(always)]
    pub fn exti0_ss(&self) -> EXTI0_SS_R {
        EXTI0_SS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline(always)]
    pub fn exti3_ss(&mut self) -> EXTI3_SS_W {
        EXTI3_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline(always)]
    pub fn exti2_ss(&mut self) -> EXTI2_SS_W {
        EXTI2_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline(always)]
    pub fn exti1_ss(&mut self) -> EXTI1_SS_W {
        EXTI1_SS_W::new(self)
    }
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline(always)]
    pub fn exti0_ss(&mut self) -> EXTI0_SS_W {
        EXTI0_SS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI sources selection register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss0](index.html) module"]
pub struct EXTISS0_SPEC;
impl crate::RegisterSpec for EXTISS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extiss0::R](R) reader structure"]
impl crate::Readable for EXTISS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extiss0::W](W) writer structure"]
impl crate::Writable for EXTISS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTISS0 to value 0"]
impl crate::Resettable for EXTISS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
