#[doc = "Register `STAT1` reader"]
pub struct R(crate::R<STAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT1` writer"]
pub struct W(crate::W<STAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT1_SPEC>;
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
impl From<crate::W<STAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "End of operation flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDF_A {
    #[doc = "0: No end of operation occurred"]
    NOEVENT = 0,
    #[doc = "1: An end of operation event occurred"]
    EVENT = 1,
}
impl From<ENDF_A> for bool {
    #[inline(always)]
    fn from(variant: ENDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDF` reader - End of operation flag bit"]
pub type ENDF_R = crate::BitReader<ENDF_A>;
impl ENDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDF_A {
        match self.bits {
            false => ENDF_A::NOEVENT,
            true => ENDF_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == ENDF_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == ENDF_A::EVENT
    }
}
#[doc = "End of operation flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDF_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<ENDF_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDF` writer - End of operation flag bit"]
pub type ENDF_W<'a> = crate::BitWriter<'a, u32, STAT1_SPEC, ENDF_AW, 5>;
impl<'a> ENDF_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDF_AW::CLEAR)
    }
}
#[doc = "Erase/Program protection error flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPERR_A {
    #[doc = "0: There was no error"]
    NOERROR = 0,
    #[doc = "1: There was an error erasing/programming protected pages"]
    ERROR = 1,
}
impl From<WPERR_A> for bool {
    #[inline(always)]
    fn from(variant: WPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPERR` reader - Erase/Program protection error flag bit"]
pub type WPERR_R = crate::BitReader<WPERR_A>;
impl WPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPERR_A {
        match self.bits {
            false => WPERR_A::NOERROR,
            true => WPERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WPERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WPERR_A::ERROR
    }
}
#[doc = "Erase/Program protection error flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPERR_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<WPERR_AW> for bool {
    #[inline(always)]
    fn from(variant: WPERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPERR` writer - Erase/Program protection error flag bit"]
pub type WPERR_W<'a> = crate::BitWriter<'a, u32, STAT1_SPEC, WPERR_AW, 4>;
impl<'a> WPERR_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WPERR_AW::CLEAR)
    }
}
#[doc = "Program error flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGERR_A {
    #[doc = "0: There was no error"]
    NOERROR = 0,
    #[doc = "1: There was an error programming flash"]
    ERROR = 1,
}
impl From<PGERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGERR` reader - Program error flag bit"]
pub type PGERR_R = crate::BitReader<PGERR_A>;
impl PGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGERR_A {
        match self.bits {
            false => PGERR_A::NOERROR,
            true => PGERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGERR_A::ERROR
    }
}
#[doc = "Program error flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGERR_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<PGERR_AW> for bool {
    #[inline(always)]
    fn from(variant: PGERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGERR` writer - Program error flag bit"]
pub type PGERR_W<'a> = crate::BitWriter<'a, u32, STAT1_SPEC, PGERR_AW, 2>;
impl<'a> PGERR_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGERR_AW::CLEAR)
    }
}
#[doc = "The flash is busy bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: No operation is in progress"]
    INACTIVE = 0,
    #[doc = "1: An operation is in progress"]
    ACTIVE = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - The flash is busy bit"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::INACTIVE,
            true => BUSY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BUSY_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BUSY_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 5 - End of operation flag bit"]
    #[inline(always)]
    pub fn endf(&self) -> ENDF_R {
        ENDF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    pub fn wperr(&self) -> WPERR_R {
        WPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - Program error flag bit"]
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - The flash is busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - End of operation flag bit"]
    #[inline(always)]
    pub fn endf(&mut self) -> ENDF_W {
        ENDF_W::new(self)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    pub fn wperr(&mut self) -> WPERR_W {
        WPERR_W::new(self)
    }
    #[doc = "Bit 2 - Program error flag bit"]
    #[inline(always)]
    pub fn pgerr(&mut self) -> PGERR_W {
        PGERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat1](index.html) module"]
pub struct STAT1_SPEC;
impl crate::RegisterSpec for STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat1::R](R) reader structure"]
impl crate::Readable for STAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat1::W](W) writer structure"]
impl crate::Writable for STAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT1 to value 0"]
impl crate::Resettable for STAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
