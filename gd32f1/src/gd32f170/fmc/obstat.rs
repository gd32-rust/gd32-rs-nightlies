#[doc = "Register `OBSTAT` reader"]
pub type R = crate::R<OBSTAT_SPEC>;
#[doc = "Field `OBERR` reader - Option byte read error"]
pub type OBERR_R = crate::BitReader<OBERR_A>;
#[doc = "Option byte read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBERR_A {
    #[doc = "0: No error with option bytes"]
    NO_ERROR = 0,
    #[doc = "1: Option bytes and complement bytes do not match"]
    ERROR = 1,
}
impl From<OBERR_A> for bool {
    #[inline(always)]
    fn from(variant: OBERR_A) -> Self {
        variant as u8 != 0
    }
}
impl OBERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBERR_A {
        match self.bits {
            false => OBERR_A::NO_ERROR,
            true => OBERR_A::ERROR,
        }
    }
    #[doc = "No error with option bytes"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OBERR_A::NO_ERROR
    }
    #[doc = "Option bytes and complement bytes do not match"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OBERR_A::ERROR
    }
}
#[doc = "Field `PLEVEL` reader - Security Protection level"]
pub type PLEVEL_R = crate::FieldReader<PLEVEL_A>;
#[doc = "Security Protection level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for PLEVEL_A {
    type Ux = u8;
}
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
    #[doc = "No protection level"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLEVEL_A::NONE
    }
    #[doc = "Low protection level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PLEVEL_A::LOW
    }
    #[doc = "High protection level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PLEVEL_A::HIGH
    }
}
#[doc = "Field `OB_USER` reader - Store OB_USER byte of option byte block after system reset"]
pub type OB_USER_R = crate::FieldReader;
#[doc = "Field `OB_DATA` reader - Store OB_DATA\\[15:0\\]
of option byte block after system reset"]
pub type OB_DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Option byte read error"]
    #[inline(always)]
    pub fn oberr(&self) -> OBERR_R {
        OBERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Security Protection level"]
    #[inline(always)]
    pub fn plevel(&self) -> PLEVEL_R {
        PLEVEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Store OB_USER byte of option byte block after system reset"]
    #[inline(always)]
    pub fn ob_user(&self) -> OB_USER_R {
        OB_USER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Store OB_DATA\\[15:0\\]
of option byte block after system reset"]
    #[inline(always)]
    pub fn ob_data(&self) -> OB_DATA_R {
        OB_DATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Option byte status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OBSTAT_SPEC;
impl crate::RegisterSpec for OBSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obstat::R`](R) reader structure"]
impl crate::Readable for OBSTAT_SPEC {}
#[doc = "`reset()` method sets OBSTAT to value 0"]
impl crate::Resettable for OBSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
