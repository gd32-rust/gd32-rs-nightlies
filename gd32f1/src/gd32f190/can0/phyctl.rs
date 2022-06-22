#[doc = "Register `PHYCTL` reader"]
pub struct R(crate::R<PHYCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CAN PHY output driver control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POMOD_A {
    #[doc = "0: Low slope mode"]
    LOWSLOPE = 0,
    #[doc = "1: Middle slope mode"]
    MIDDLESLOPE = 1,
    #[doc = "2: High slope mode"]
    HIGHSLOPE = 2,
    #[doc = "3: High speed mode"]
    HIGHSPEED = 3,
}
impl From<POMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: POMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POMOD` reader - CAN PHY output driver control"]
pub type POMOD_R = crate::FieldReader<u8, POMOD_A>;
impl POMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POMOD_A {
        match self.bits {
            0 => POMOD_A::LOWSLOPE,
            1 => POMOD_A::MIDDLESLOPE,
            2 => POMOD_A::HIGHSLOPE,
            3 => POMOD_A::HIGHSPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOWSLOPE`"]
    #[inline(always)]
    pub fn is_low_slope(&self) -> bool {
        *self == POMOD_A::LOWSLOPE
    }
    #[doc = "Checks if the value of the field is `MIDDLESLOPE`"]
    #[inline(always)]
    pub fn is_middle_slope(&self) -> bool {
        *self == POMOD_A::MIDDLESLOPE
    }
    #[doc = "Checks if the value of the field is `HIGHSLOPE`"]
    #[inline(always)]
    pub fn is_high_slope(&self) -> bool {
        *self == POMOD_A::HIGHSLOPE
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == POMOD_A::HIGHSPEED
    }
}
#[doc = "PHY enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PHYEN` reader - PHY enable bit"]
pub type PHYEN_R = crate::BitReader<PHYEN_A>;
impl PHYEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYEN_A {
        match self.bits {
            false => PHYEN_A::DISABLED,
            true => PHYEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PHYEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PHYEN_A::ENABLED
    }
}
impl R {
    #[doc = "Bits 8:9 - CAN PHY output driver control"]
    #[inline(always)]
    pub fn pomod(&self) -> POMOD_R {
        POMOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 0 - PHY enable bit"]
    #[inline(always)]
    pub fn phyen(&self) -> PHYEN_R {
        PHYEN_R::new((self.bits & 1) != 0)
    }
}
#[doc = "PHY control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phyctl](index.html) module"]
pub struct PHYCTL_SPEC;
impl crate::RegisterSpec for PHYCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phyctl::R](R) reader structure"]
impl crate::Readable for PHYCTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PHYCTL to value 0x0300"]
impl crate::Resettable for PHYCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300
    }
}
