#[doc = "Register `RFIFOMI0` reader"]
pub type R = crate::R<RFIFOMI0_SPEC>;
#[doc = "Field `FT` reader - Frame type"]
pub type FT_R = crate::BitReader<FT_A>;
#[doc = "Frame type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT_A {
    #[doc = "0: Data frame"]
    DATA = 0,
    #[doc = "1: Remote frame"]
    REMOTE = 1,
}
impl From<FT_A> for bool {
    #[inline(always)]
    fn from(variant: FT_A) -> Self {
        variant as u8 != 0
    }
}
impl FT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FT_A {
        match self.bits {
            false => FT_A::DATA,
            true => FT_A::REMOTE,
        }
    }
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == FT_A::DATA
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == FT_A::REMOTE
    }
}
#[doc = "Field `FF` reader - Frame format"]
pub type FF_R = crate::BitReader<FF_A>;
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FF_A {
    #[doc = "0: Standard format frame"]
    STANDARD = 0,
    #[doc = "1: Extended format frame"]
    EXTENDED = 1,
}
impl From<FF_A> for bool {
    #[inline(always)]
    fn from(variant: FF_A) -> Self {
        variant as u8 != 0
    }
}
impl FF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FF_A {
        match self.bits {
            false => FF_A::STANDARD,
            true => FF_A::EXTENDED,
        }
    }
    #[doc = "Standard format frame"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FF_A::STANDARD
    }
    #[doc = "Extended format frame"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == FF_A::EXTENDED
    }
}
#[doc = "Field `EFID` reader - The frame identifier"]
pub type EFID_R = crate::FieldReader<u32>;
#[doc = "Field `SFID_EFID` reader - The frame identifier"]
pub type SFID_EFID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EFID_R {
        EFID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SFID_EFID_R {
        SFID_EFID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "Receive FIFO mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomi0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIFOMI0_SPEC;
impl crate::RegisterSpec for RFIFOMI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifomi0::R`](R) reader structure"]
impl crate::Readable for RFIFOMI0_SPEC {}
#[doc = "`reset()` method sets RFIFOMI0 to value 0"]
impl crate::Resettable for RFIFOMI0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
