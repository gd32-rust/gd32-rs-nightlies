#[doc = "Register `RFIFOMI0` reader"]
pub type R = crate::R<Rfifomi0Spec>;
#[doc = "Frame type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft {
    #[doc = "0: Data frame"]
    Data = 0,
    #[doc = "1: Remote frame"]
    Remote = 1,
}
impl From<Ft> for bool {
    #[inline(always)]
    fn from(variant: Ft) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT` reader - Frame type"]
pub type FtR = crate::BitReader<Ft>;
impl FtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft {
        match self.bits {
            false => Ft::Data,
            true => Ft::Remote,
        }
    }
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Ft::Data
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == Ft::Remote
    }
}
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ff {
    #[doc = "0: Standard format frame"]
    Standard = 0,
    #[doc = "1: Extended format frame"]
    Extended = 1,
}
impl From<Ff> for bool {
    #[inline(always)]
    fn from(variant: Ff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FF` reader - Frame format"]
pub type FfR = crate::BitReader<Ff>;
impl FfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ff {
        match self.bits {
            false => Ff::Standard,
            true => Ff::Extended,
        }
    }
    #[doc = "Standard format frame"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Ff::Standard
    }
    #[doc = "Extended format frame"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == Ff::Extended
    }
}
#[doc = "Field `EFID` reader - The frame identifier"]
pub type EfidR = crate::FieldReader<u32>;
#[doc = "Field `SFID_EFID` reader - The frame identifier"]
pub type SfidEfidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FtR {
        FtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FfR {
        FfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EfidR {
        EfidR::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SfidEfidR {
        SfidEfidR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "Receive FIFO mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomi0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfifomi0Spec;
impl crate::RegisterSpec for Rfifomi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifomi0::R`](R) reader structure"]
impl crate::Readable for Rfifomi0Spec {}
#[doc = "`reset()` method sets RFIFOMI0 to value 0"]
impl crate::Resettable for Rfifomi0Spec {
    const RESET_VALUE: u32 = 0;
}
