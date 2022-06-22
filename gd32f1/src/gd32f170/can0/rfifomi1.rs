#[doc = "Register `RFIFOMI1` reader"]
pub struct R(crate::R<RFIFOMI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFOMI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFOMI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFOMI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SFID_EFID` reader - The frame identifier"]
pub type SFID_EFID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EFID` reader - The frame identifier"]
pub type EFID_R = crate::FieldReader<u32, u32>;
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FF` reader - Frame format"]
pub type FF_R = crate::BitReader<FF_A>;
impl FF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FF_A {
        match self.bits {
            false => FF_A::STANDARD,
            true => FF_A::EXTENDED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FF_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == FF_A::EXTENDED
    }
}
#[doc = "Frame type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FT` reader - Frame type"]
pub type FT_R = crate::BitReader<FT_A>;
impl FT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FT_A {
        match self.bits {
            false => FT_A::DATA,
            true => FT_A::REMOTE,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == FT_A::DATA
    }
    #[doc = "Checks if the value of the field is `REMOTE`"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == FT_A::REMOTE
    }
}
impl R {
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SFID_EFID_R {
        SFID_EFID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EFID_R {
        EFID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Receive FIFO mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomi1](index.html) module"]
pub struct RFIFOMI1_SPEC;
impl crate::RegisterSpec for RFIFOMI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifomi1::R](R) reader structure"]
impl crate::Readable for RFIFOMI1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFIFOMI1 to value 0"]
impl crate::Resettable for RFIFOMI1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
