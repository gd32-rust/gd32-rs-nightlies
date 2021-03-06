#[doc = "Register `EXTISS3` reader"]
pub struct R(crate::R<EXTISS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTISS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTISS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTISS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTISS3` writer"]
pub struct W(crate::W<EXTISS3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTISS3_SPEC>;
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
impl From<crate::W<EXTISS3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTISS3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EXTI 15 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI15_SS_A {
    #[doc = "0: PA15 pin"]
    PA15 = 0,
    #[doc = "1: PB15 pin"]
    PB15 = 1,
    #[doc = "2: PC15 pin"]
    PC15 = 2,
}
impl From<EXTI15_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI15_SS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI15_SS` reader - EXTI 15 sources selection"]
pub type EXTI15_SS_R = crate::FieldReader<u8, EXTI15_SS_A>;
impl EXTI15_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI15_SS_A> {
        match self.bits {
            0 => Some(EXTI15_SS_A::PA15),
            1 => Some(EXTI15_SS_A::PB15),
            2 => Some(EXTI15_SS_A::PC15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA15`"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == EXTI15_SS_A::PA15
    }
    #[doc = "Checks if the value of the field is `PB15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == EXTI15_SS_A::PB15
    }
    #[doc = "Checks if the value of the field is `PC15`"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == EXTI15_SS_A::PC15
    }
}
#[doc = "Field `EXTI15_SS` writer - EXTI 15 sources selection"]
pub type EXTI15_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS3_SPEC, u8, EXTI15_SS_A, 4, 12>;
impl<'a> EXTI15_SS_W<'a> {
    #[doc = "PA15 pin"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(EXTI15_SS_A::PA15)
    }
    #[doc = "PB15 pin"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(EXTI15_SS_A::PB15)
    }
    #[doc = "PC15 pin"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut W {
        self.variant(EXTI15_SS_A::PC15)
    }
}
#[doc = "EXTI 14 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI14_SS_A {
    #[doc = "0: PA14 pin"]
    PA14 = 0,
    #[doc = "1: PB14 pin"]
    PB14 = 1,
    #[doc = "2: PC14 pin"]
    PC14 = 2,
}
impl From<EXTI14_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI14_SS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI14_SS` reader - EXTI 14 sources selection"]
pub type EXTI14_SS_R = crate::FieldReader<u8, EXTI14_SS_A>;
impl EXTI14_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI14_SS_A> {
        match self.bits {
            0 => Some(EXTI14_SS_A::PA14),
            1 => Some(EXTI14_SS_A::PB14),
            2 => Some(EXTI14_SS_A::PC14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA14`"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == EXTI14_SS_A::PA14
    }
    #[doc = "Checks if the value of the field is `PB14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == EXTI14_SS_A::PB14
    }
    #[doc = "Checks if the value of the field is `PC14`"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == EXTI14_SS_A::PC14
    }
}
#[doc = "Field `EXTI14_SS` writer - EXTI 14 sources selection"]
pub type EXTI14_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS3_SPEC, u8, EXTI14_SS_A, 4, 8>;
impl<'a> EXTI14_SS_W<'a> {
    #[doc = "PA14 pin"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut W {
        self.variant(EXTI14_SS_A::PA14)
    }
    #[doc = "PB14 pin"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(EXTI14_SS_A::PB14)
    }
    #[doc = "PC14 pin"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut W {
        self.variant(EXTI14_SS_A::PC14)
    }
}
#[doc = "EXTI 13 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI13_SS_A {
    #[doc = "0: PA13 pin"]
    PA13 = 0,
    #[doc = "1: PB13 pin"]
    PB13 = 1,
    #[doc = "2: PC13 pin"]
    PC13 = 2,
}
impl From<EXTI13_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI13_SS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI13_SS` reader - EXTI 13 sources selection"]
pub type EXTI13_SS_R = crate::FieldReader<u8, EXTI13_SS_A>;
impl EXTI13_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI13_SS_A> {
        match self.bits {
            0 => Some(EXTI13_SS_A::PA13),
            1 => Some(EXTI13_SS_A::PB13),
            2 => Some(EXTI13_SS_A::PC13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA13`"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == EXTI13_SS_A::PA13
    }
    #[doc = "Checks if the value of the field is `PB13`"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == EXTI13_SS_A::PB13
    }
    #[doc = "Checks if the value of the field is `PC13`"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == EXTI13_SS_A::PC13
    }
}
#[doc = "Field `EXTI13_SS` writer - EXTI 13 sources selection"]
pub type EXTI13_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS3_SPEC, u8, EXTI13_SS_A, 4, 4>;
impl<'a> EXTI13_SS_W<'a> {
    #[doc = "PA13 pin"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut W {
        self.variant(EXTI13_SS_A::PA13)
    }
    #[doc = "PB13 pin"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut W {
        self.variant(EXTI13_SS_A::PB13)
    }
    #[doc = "PC13 pin"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut W {
        self.variant(EXTI13_SS_A::PC13)
    }
}
#[doc = "EXTI 12 sources selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI12_SS_A {
    #[doc = "0: PA12 pin"]
    PA12 = 0,
    #[doc = "1: PB12 pin"]
    PB12 = 1,
    #[doc = "2: PC12 pin"]
    PC12 = 2,
}
impl From<EXTI12_SS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI12_SS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI12_SS` reader - EXTI 12 sources selection"]
pub type EXTI12_SS_R = crate::FieldReader<u8, EXTI12_SS_A>;
impl EXTI12_SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI12_SS_A> {
        match self.bits {
            0 => Some(EXTI12_SS_A::PA12),
            1 => Some(EXTI12_SS_A::PB12),
            2 => Some(EXTI12_SS_A::PC12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA12`"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == EXTI12_SS_A::PA12
    }
    #[doc = "Checks if the value of the field is `PB12`"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == EXTI12_SS_A::PB12
    }
    #[doc = "Checks if the value of the field is `PC12`"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == EXTI12_SS_A::PC12
    }
}
#[doc = "Field `EXTI12_SS` writer - EXTI 12 sources selection"]
pub type EXTI12_SS_W<'a> = crate::FieldWriter<'a, u32, EXTISS3_SPEC, u8, EXTI12_SS_A, 4, 0>;
impl<'a> EXTI12_SS_W<'a> {
    #[doc = "PA12 pin"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut W {
        self.variant(EXTI12_SS_A::PA12)
    }
    #[doc = "PB12 pin"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut W {
        self.variant(EXTI12_SS_A::PB12)
    }
    #[doc = "PC12 pin"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut W {
        self.variant(EXTI12_SS_A::PC12)
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline(always)]
    pub fn exti15_ss(&self) -> EXTI15_SS_R {
        EXTI15_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline(always)]
    pub fn exti14_ss(&self) -> EXTI14_SS_R {
        EXTI14_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline(always)]
    pub fn exti13_ss(&self) -> EXTI13_SS_R {
        EXTI13_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline(always)]
    pub fn exti12_ss(&self) -> EXTI12_SS_R {
        EXTI12_SS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline(always)]
    pub fn exti15_ss(&mut self) -> EXTI15_SS_W {
        EXTI15_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline(always)]
    pub fn exti14_ss(&mut self) -> EXTI14_SS_W {
        EXTI14_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline(always)]
    pub fn exti13_ss(&mut self) -> EXTI13_SS_W {
        EXTI13_SS_W::new(self)
    }
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline(always)]
    pub fn exti12_ss(&mut self) -> EXTI12_SS_W {
        EXTI12_SS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI sources selection register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss3](index.html) module"]
pub struct EXTISS3_SPEC;
impl crate::RegisterSpec for EXTISS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extiss3::R](R) reader structure"]
impl crate::Readable for EXTISS3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extiss3::W](W) writer structure"]
impl crate::Writable for EXTISS3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTISS3 to value 0"]
impl crate::Resettable for EXTISS3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
