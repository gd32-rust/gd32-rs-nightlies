#[doc = "Register `OBSTAT` reader"]
pub struct R(crate::R<OBSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OB_DATA` reader - Store OB_DATA\\[15:0\\]
of option byte block after system reset"]
pub type OB_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OB_USER` reader - Store OB_USER byte of option byte block after system reset"]
pub type OB_USER_R = crate::FieldReader<u8, u8>;
#[doc = "Security Protection level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLEVEL_A {
    #[doc = "0: No protection level"]
    NONE = 0,
    #[doc = "1: Low protection level"]
    LOW = 1,
    #[doc = "3: High protection level"]
    HIGH = 3,
}
impl From<PLEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLEVEL` reader - Security Protection level"]
pub type PLEVEL_R = crate::FieldReader<u8, PLEVEL_A>;
impl PLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLEVEL_A> {
        match self.bits {
            0 => Some(PLEVEL_A::NONE),
            1 => Some(PLEVEL_A::LOW),
            3 => Some(PLEVEL_A::HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLEVEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PLEVEL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PLEVEL_A::HIGH
    }
}
#[doc = "Option byte read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OBERR_A {
    #[doc = "0: No error with option bytes"]
    NOERROR = 0,
    #[doc = "1: Option bytes and complement bytes do not match"]
    ERROR = 1,
}
impl From<OBERR_A> for bool {
    #[inline(always)]
    fn from(variant: OBERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBERR` reader - Option byte read error"]
pub type OBERR_R = crate::BitReader<OBERR_A>;
impl OBERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBERR_A {
        match self.bits {
            false => OBERR_A::NOERROR,
            true => OBERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OBERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OBERR_A::ERROR
    }
}
impl R {
    #[doc = "Bits 16:31 - Store OB_DATA\\[15:0\\]
of option byte block after system reset"]
    #[inline(always)]
    pub fn ob_data(&self) -> OB_DATA_R {
        OB_DATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - Store OB_USER byte of option byte block after system reset"]
    #[inline(always)]
    pub fn ob_user(&self) -> OB_USER_R {
        OB_USER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 1:2 - Security Protection level"]
    #[inline(always)]
    pub fn plevel(&self) -> PLEVEL_R {
        PLEVEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 0 - Option byte read error"]
    #[inline(always)]
    pub fn oberr(&self) -> OBERR_R {
        OBERR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Option byte status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obstat](index.html) module"]
pub struct OBSTAT_SPEC;
impl crate::RegisterSpec for OBSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obstat::R](R) reader structure"]
impl crate::Readable for OBSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OBSTAT to value 0"]
impl crate::Resettable for OBSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
