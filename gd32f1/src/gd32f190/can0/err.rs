#[doc = "Register `ERR` reader"]
pub type R = crate::R<ERR_SPEC>;
#[doc = "Register `ERR` writer"]
pub type W = crate::W<ERR_SPEC>;
#[doc = "Field `WERR` reader - Warning error"]
pub type WERR_R = crate::BitReader<WERR_A>;
#[doc = "Warning error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WERR_A {
    #[doc = "0: No warning error"]
    NO_ERROR = 0,
    #[doc = "1: Warning error"]
    ERROR = 1,
}
impl From<WERR_A> for bool {
    #[inline(always)]
    fn from(variant: WERR_A) -> Self {
        variant as u8 != 0
    }
}
impl WERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WERR_A {
        match self.bits {
            false => WERR_A::NO_ERROR,
            true => WERR_A::ERROR,
        }
    }
    #[doc = "No warning error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WERR_A::NO_ERROR
    }
    #[doc = "Warning error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WERR_A::ERROR
    }
}
#[doc = "Field `PERR` reader - Passive error"]
pub type PERR_R = crate::BitReader<PERR_A>;
#[doc = "Passive error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERR_A {
    #[doc = "0: No passive error"]
    NO_ERROR = 0,
    #[doc = "1: Passive error"]
    ERROR = 1,
}
impl From<PERR_A> for bool {
    #[inline(always)]
    fn from(variant: PERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERR_A {
        match self.bits {
            false => PERR_A::NO_ERROR,
            true => PERR_A::ERROR,
        }
    }
    #[doc = "No passive error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PERR_A::NO_ERROR
    }
    #[doc = "Passive error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PERR_A::ERROR
    }
}
#[doc = "Field `BOERR` reader - Bus-off error"]
pub type BOERR_R = crate::BitReader<BOERR_A>;
#[doc = "Bus-off error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOERR_A {
    #[doc = "0: No bus-off error"]
    NO_ERROR = 0,
    #[doc = "1: Bus-off error"]
    ERROR = 1,
}
impl From<BOERR_A> for bool {
    #[inline(always)]
    fn from(variant: BOERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BOERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOERR_A {
        match self.bits {
            false => BOERR_A::NO_ERROR,
            true => BOERR_A::ERROR,
        }
    }
    #[doc = "No bus-off error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BOERR_A::NO_ERROR
    }
    #[doc = "Bus-off error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BOERR_A::ERROR
    }
}
#[doc = "Field `ERRN` reader - Error number"]
pub type ERRN_R = crate::FieldReader<ERRN_A>;
#[doc = "Error number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERRN_A {
    #[doc = "0: No Error"]
    NO_ERROR = 0,
    #[doc = "1: Stuff Error"]
    STUFF = 1,
    #[doc = "2: Form Error"]
    FORM = 2,
    #[doc = "3: Acknowledgment Error"]
    ACK = 3,
    #[doc = "4: Bit recessive Error"]
    BIT_RECESSIVE = 4,
    #[doc = "5: Bit dominant Error"]
    BIT_DOMINANT = 5,
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
impl crate::FieldSpec for ERRN_A {
    type Ux = u8;
}
impl ERRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRN_A {
        match self.bits {
            0 => ERRN_A::NO_ERROR,
            1 => ERRN_A::STUFF,
            2 => ERRN_A::FORM,
            3 => ERRN_A::ACK,
            4 => ERRN_A::BIT_RECESSIVE,
            5 => ERRN_A::BIT_DOMINANT,
            6 => ERRN_A::CRC,
            7 => ERRN_A::CUSTOM,
            _ => unreachable!(),
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERRN_A::NO_ERROR
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == ERRN_A::STUFF
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == ERRN_A::FORM
    }
    #[doc = "Acknowledgment Error"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ERRN_A::ACK
    }
    #[doc = "Bit recessive Error"]
    #[inline(always)]
    pub fn is_bit_recessive(&self) -> bool {
        *self == ERRN_A::BIT_RECESSIVE
    }
    #[doc = "Bit dominant Error"]
    #[inline(always)]
    pub fn is_bit_dominant(&self) -> bool {
        *self == ERRN_A::BIT_DOMINANT
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == ERRN_A::CRC
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == ERRN_A::CUSTOM
    }
}
#[doc = "Field `ERRN` writer - Error number"]
pub type ERRN_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, ERRN_A>;
impl<'a, REG, const O: u8> ERRN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(ERRN_A::NO_ERROR)
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn stuff(self) -> &'a mut crate::W<REG> {
        self.variant(ERRN_A::STUFF)
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn form(self) -> &'a mut crate::W<REG> {
        self.variant(ERRN_A::FORM)
    }
    #[doc = "Acknowledgment Error"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(ERRN_A::ACK)
    }
    #[doc = "Bit recessive Error"]
    #[inline(always)]
    pub fn bit_recessive(self) -> &'a mut crate::W<REG> {
        self.variant(ERRN_A::BIT_RECESSIVE)
    }
    #[doc = "Bit dominant Error"]
    #[inline(always)]
    pub fn bit_dominant(self) -> &'a mut crate::W<REG> {
        self.variant(ERRN_A::BIT_DOMINANT)
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(ERRN_A::CRC)
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn custom(self) -> &'a mut crate::W<REG> {
        self.variant(ERRN_A::CUSTOM)
    }
}
#[doc = "Field `TECNT` reader - Transmit Error Count"]
pub type TECNT_R = crate::FieldReader;
#[doc = "Field `RECNT` reader - Receive Error Count"]
pub type RECNT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Warning error"]
    #[inline(always)]
    pub fn werr(&self) -> WERR_R {
        WERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Passive error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus-off error"]
    #[inline(always)]
    pub fn boerr(&self) -> BOERR_R {
        BOERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Error number"]
    #[inline(always)]
    pub fn errn(&self) -> ERRN_R {
        ERRN_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Error Count"]
    #[inline(always)]
    pub fn tecnt(&self) -> TECNT_R {
        TECNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Error Count"]
    #[inline(always)]
    pub fn recnt(&self) -> RECNT_R {
        RECNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Error number"]
    #[inline(always)]
    #[must_use]
    pub fn errn(&mut self) -> ERRN_W<ERR_SPEC, 4> {
        ERRN_W::new(self)
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
#[doc = "Error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR_SPEC;
impl crate::RegisterSpec for ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err::R`](R) reader structure"]
impl crate::Readable for ERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`err::W`](W) writer structure"]
impl crate::Writable for ERR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
