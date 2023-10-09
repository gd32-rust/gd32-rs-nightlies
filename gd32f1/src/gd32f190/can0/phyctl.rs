#[doc = "Register `PHYCTL` reader"]
pub type R = crate::R<PHYCTL_SPEC>;
#[doc = "Field `PHYEN` reader - PHY enable bit"]
pub type PHYEN_R = crate::BitReader<PHYEN_A>;
#[doc = "PHY enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYEN_A {
    #[doc = "0: CAN PHY disabled"]
    DISABLED = 0,
    #[doc = "1: CAN PHY enabled"]
    ENABLED = 1,
}
impl From<PHYEN_A> for bool {
    #[inline(always)]
    fn from(variant: PHYEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PHYEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYEN_A {
        match self.bits {
            false => PHYEN_A::DISABLED,
            true => PHYEN_A::ENABLED,
        }
    }
    #[doc = "CAN PHY disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PHYEN_A::DISABLED
    }
    #[doc = "CAN PHY enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PHYEN_A::ENABLED
    }
}
#[doc = "Field `POMOD` reader - CAN PHY output driver control"]
pub type POMOD_R = crate::FieldReader<POMOD_A>;
#[doc = "CAN PHY output driver control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POMOD_A {
    #[doc = "0: Low slope mode"]
    LOW_SLOPE = 0,
    #[doc = "1: Middle slope mode"]
    MIDDLE_SLOPE = 1,
    #[doc = "2: High slope mode"]
    HIGH_SLOPE = 2,
    #[doc = "3: High speed mode"]
    HIGH_SPEED = 3,
}
impl From<POMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: POMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POMOD_A {
    type Ux = u8;
}
impl POMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POMOD_A {
        match self.bits {
            0 => POMOD_A::LOW_SLOPE,
            1 => POMOD_A::MIDDLE_SLOPE,
            2 => POMOD_A::HIGH_SLOPE,
            3 => POMOD_A::HIGH_SPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "Low slope mode"]
    #[inline(always)]
    pub fn is_low_slope(&self) -> bool {
        *self == POMOD_A::LOW_SLOPE
    }
    #[doc = "Middle slope mode"]
    #[inline(always)]
    pub fn is_middle_slope(&self) -> bool {
        *self == POMOD_A::MIDDLE_SLOPE
    }
    #[doc = "High slope mode"]
    #[inline(always)]
    pub fn is_high_slope(&self) -> bool {
        *self == POMOD_A::HIGH_SLOPE
    }
    #[doc = "High speed mode"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == POMOD_A::HIGH_SPEED
    }
}
impl R {
    #[doc = "Bit 0 - PHY enable bit"]
    #[inline(always)]
    pub fn phyen(&self) -> PHYEN_R {
        PHYEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - CAN PHY output driver control"]
    #[inline(always)]
    pub fn pomod(&self) -> POMOD_R {
        POMOD_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "PHY control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phyctl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHYCTL_SPEC;
impl crate::RegisterSpec for PHYCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phyctl::R`](R) reader structure"]
impl crate::Readable for PHYCTL_SPEC {}
#[doc = "`reset()` method sets PHYCTL to value 0x0300"]
impl crate::Resettable for PHYCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300;
}
