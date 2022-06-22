#[doc = "Register `ERR` reader"]
pub struct R(crate::R<ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERR` writer"]
pub struct W(crate::W<ERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERR_SPEC>;
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
impl From<crate::W<ERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECNT` reader - Receive Error Count defined by the CAN standard"]
pub type RECNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TECNT` reader - Transmit Error Count defined by the CAN standard"]
pub type TECNT_R = crate::FieldReader<u8, u8>;
#[doc = "Error number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERRN_A {
    #[doc = "0: No Error"]
    NOERROR = 0,
    #[doc = "1: Stuff Error"]
    STUFF = 1,
    #[doc = "2: Form Error"]
    FORM = 2,
    #[doc = "3: Acknowledgment Error"]
    ACK = 3,
    #[doc = "4: Bit recessive Error"]
    BITRECESSIVE = 4,
    #[doc = "5: Bit dominant Error"]
    BITDOMINANT = 5,
    #[doc = "6: CRC Error"]
    CRC = 6,
    #[doc = "7: Set by software"]
    CUSTOM = 7,
}
impl From<ERRN_A> for u8 {
    #[inline(always)]
    fn from(variant: ERRN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ERRN` reader - Error number"]
pub type ERRN_R = crate::FieldReader<u8, ERRN_A>;
impl ERRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRN_A {
        match self.bits {
            0 => ERRN_A::NOERROR,
            1 => ERRN_A::STUFF,
            2 => ERRN_A::FORM,
            3 => ERRN_A::ACK,
            4 => ERRN_A::BITRECESSIVE,
            5 => ERRN_A::BITDOMINANT,
            6 => ERRN_A::CRC,
            7 => ERRN_A::CUSTOM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERRN_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `STUFF`"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == ERRN_A::STUFF
    }
    #[doc = "Checks if the value of the field is `FORM`"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == ERRN_A::FORM
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ERRN_A::ACK
    }
    #[doc = "Checks if the value of the field is `BITRECESSIVE`"]
    #[inline(always)]
    pub fn is_bit_recessive(&self) -> bool {
        *self == ERRN_A::BITRECESSIVE
    }
    #[doc = "Checks if the value of the field is `BITDOMINANT`"]
    #[inline(always)]
    pub fn is_bit_dominant(&self) -> bool {
        *self == ERRN_A::BITDOMINANT
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == ERRN_A::CRC
    }
    #[doc = "Checks if the value of the field is `CUSTOM`"]
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == ERRN_A::CUSTOM
    }
}
#[doc = "Field `ERRN` writer - Error number"]
pub type ERRN_W<'a> = crate::FieldWriterSafe<'a, u32, ERR_SPEC, u8, ERRN_A, 3, 4>;
impl<'a> ERRN_W<'a> {
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(ERRN_A::NOERROR)
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn stuff(self) -> &'a mut W {
        self.variant(ERRN_A::STUFF)
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn form(self) -> &'a mut W {
        self.variant(ERRN_A::FORM)
    }
    #[doc = "Acknowledgment Error"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(ERRN_A::ACK)
    }
    #[doc = "Bit recessive Error"]
    #[inline(always)]
    pub fn bit_recessive(self) -> &'a mut W {
        self.variant(ERRN_A::BITRECESSIVE)
    }
    #[doc = "Bit dominant Error"]
    #[inline(always)]
    pub fn bit_dominant(self) -> &'a mut W {
        self.variant(ERRN_A::BITDOMINANT)
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(ERRN_A::CRC)
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn custom(self) -> &'a mut W {
        self.variant(ERRN_A::CUSTOM)
    }
}
#[doc = "Bus-off error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOERR_A {
    #[doc = "0: No bus-off error"]
    NOERROR = 0,
    #[doc = "1: Bus-off error"]
    ERROR = 1,
}
impl From<BOERR_A> for bool {
    #[inline(always)]
    fn from(variant: BOERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOERR` reader - Bus-off error"]
pub type BOERR_R = crate::BitReader<BOERR_A>;
impl BOERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOERR_A {
        match self.bits {
            false => BOERR_A::NOERROR,
            true => BOERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BOERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BOERR_A::ERROR
    }
}
#[doc = "Passive error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERR_A {
    #[doc = "0: No passive error"]
    NOERROR = 0,
    #[doc = "1: Passive error"]
    ERROR = 1,
}
impl From<PERR_A> for bool {
    #[inline(always)]
    fn from(variant: PERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERR` reader - Passive error"]
pub type PERR_R = crate::BitReader<PERR_A>;
impl PERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERR_A {
        match self.bits {
            false => PERR_A::NOERROR,
            true => PERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PERR_A::ERROR
    }
}
#[doc = "Warning error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WERR_A {
    #[doc = "0: No warning error"]
    NOERROR = 0,
    #[doc = "1: Warning error"]
    ERROR = 1,
}
impl From<WERR_A> for bool {
    #[inline(always)]
    fn from(variant: WERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WERR` reader - Warning error"]
pub type WERR_R = crate::BitReader<WERR_A>;
impl WERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WERR_A {
        match self.bits {
            false => WERR_A::NOERROR,
            true => WERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WERR_A::ERROR
    }
}
impl R {
    #[doc = "Bits 24:31 - Receive Error Count defined by the CAN standard"]
    #[inline(always)]
    pub fn recnt(&self) -> RECNT_R {
        RECNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Error Count defined by the CAN standard"]
    #[inline(always)]
    pub fn tecnt(&self) -> TECNT_R {
        TECNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 4:6 - Error number"]
    #[inline(always)]
    pub fn errn(&self) -> ERRN_R {
        ERRN_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 2 - Bus-off error"]
    #[inline(always)]
    pub fn boerr(&self) -> BOERR_R {
        BOERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Passive error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Warning error"]
    #[inline(always)]
    pub fn werr(&self) -> WERR_R {
        WERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Error number"]
    #[inline(always)]
    pub fn errn(&mut self) -> ERRN_W {
        ERRN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err](index.html) module"]
pub struct ERR_SPEC;
impl crate::RegisterSpec for ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err::R](R) reader structure"]
impl crate::Readable for ERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [err::W](W) writer structure"]
impl crate::Writable for ERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
