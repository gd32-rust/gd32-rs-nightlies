#[doc = "Register `OPA_CTL` reader"]
pub struct R(crate::R<OPA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPA_CTL` writer"]
pub struct W(crate::W<OPA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPA_CTL_SPEC>;
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
impl From<crate::W<OPA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "OPA0 power down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPA0PD_A {
    #[doc = "0: OPA enabled"]
    ENABLED = 0,
    #[doc = "1: OPA disabled"]
    DISABLED = 1,
}
impl From<OPA0PD_A> for bool {
    #[inline(always)]
    fn from(variant: OPA0PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA0PD` reader - OPA0 power down"]
pub type OPA0PD_R = crate::BitReader<OPA0PD_A>;
impl OPA0PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPA0PD_A {
        match self.bits {
            false => OPA0PD_A::ENABLED,
            true => OPA0PD_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPA0PD_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPA0PD_A::DISABLED
    }
}
#[doc = "Field `OPA0PD` writer - OPA0 power down"]
pub type OPA0PD_W<'a> = crate::BitWriter<'a, u32, OPA_CTL_SPEC, OPA0PD_A, 0>;
impl<'a> OPA0PD_W<'a> {
    #[doc = "OPA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPA0PD_A::ENABLED)
    }
    #[doc = "OPA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPA0PD_A::DISABLED)
    }
}
#[doc = "T3 switch enable for OPA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T3OPA0_A {
    #[doc = "0: Switch opened"]
    OPENED = 0,
    #[doc = "1: Switch closed"]
    CLOSED = 1,
}
impl From<T3OPA0_A> for bool {
    #[inline(always)]
    fn from(variant: T3OPA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T3OPA0` reader - T3 switch enable for OPA0"]
pub type T3OPA0_R = crate::BitReader<T3OPA0_A>;
impl T3OPA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T3OPA0_A {
        match self.bits {
            false => T3OPA0_A::OPENED,
            true => T3OPA0_A::CLOSED,
        }
    }
    #[doc = "Checks if the value of the field is `OPENED`"]
    #[inline(always)]
    pub fn is_opened(&self) -> bool {
        *self == T3OPA0_A::OPENED
    }
    #[doc = "Checks if the value of the field is `CLOSED`"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == T3OPA0_A::CLOSED
    }
}
#[doc = "Field `T3OPA0` writer - T3 switch enable for OPA0"]
pub type T3OPA0_W<'a> = crate::BitWriter<'a, u32, OPA_CTL_SPEC, T3OPA0_A, 1>;
impl<'a> T3OPA0_W<'a> {
    #[doc = "Switch opened"]
    #[inline(always)]
    pub fn opened(self) -> &'a mut W {
        self.variant(T3OPA0_A::OPENED)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut W {
        self.variant(T3OPA0_A::CLOSED)
    }
}
#[doc = "S1 switch enable for OPA0"]
pub use T3OPA0_A as S1OPA0_A;
#[doc = "S2 switch enable for OPA0"]
pub use T3OPA0_A as S2OPA0_A;
#[doc = "S3 switch enable for OPA0"]
pub use T3OPA0_A as S3OPA0_A;
#[doc = "Field `S1OPA0` reader - S1 switch enable for OPA0"]
pub use T3OPA0_R as S1OPA0_R;
#[doc = "Field `S2OPA0` reader - S2 switch enable for OPA0"]
pub use T3OPA0_R as S2OPA0_R;
#[doc = "Field `S3OPA0` reader - S3 switch enable for OPA0"]
pub use T3OPA0_R as S3OPA0_R;
#[doc = "Field `S1OPA0` writer - S1 switch enable for OPA0"]
pub use T3OPA0_W as S1OPA0_W;
#[doc = "Field `S2OPA0` writer - S2 switch enable for OPA0"]
pub use T3OPA0_W as S2OPA0_W;
#[doc = "Field `S3OPA0` writer - S3 switch enable for OPA0"]
pub use T3OPA0_W as S3OPA0_W;
#[doc = "OPA0 offset calibration for P diff\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPA0CAL_L_A {
    #[doc = "0: Offset calibration for P diff disabled"]
    DISABLED = 0,
    #[doc = "1: Offset calibration for P diff enabled"]
    ENABLED = 1,
}
impl From<OPA0CAL_L_A> for bool {
    #[inline(always)]
    fn from(variant: OPA0CAL_L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA0CAL_L` reader - OPA0 offset calibration for P diff"]
pub type OPA0CAL_L_R = crate::BitReader<OPA0CAL_L_A>;
impl OPA0CAL_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPA0CAL_L_A {
        match self.bits {
            false => OPA0CAL_L_A::DISABLED,
            true => OPA0CAL_L_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPA0CAL_L_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPA0CAL_L_A::ENABLED
    }
}
#[doc = "Field `OPA0CAL_L` writer - OPA0 offset calibration for P diff"]
pub type OPA0CAL_L_W<'a> = crate::BitWriter<'a, u32, OPA_CTL_SPEC, OPA0CAL_L_A, 5>;
impl<'a> OPA0CAL_L_W<'a> {
    #[doc = "Offset calibration for P diff disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPA0CAL_L_A::DISABLED)
    }
    #[doc = "Offset calibration for P diff enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPA0CAL_L_A::ENABLED)
    }
}
#[doc = "OPA0 offset calibration for N diff\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPA0CAL_H_A {
    #[doc = "0: Offset calibration for N diff disabled"]
    DISABLED = 0,
    #[doc = "1: Offset calibration for N diff enabled"]
    ENABLED = 1,
}
impl From<OPA0CAL_H_A> for bool {
    #[inline(always)]
    fn from(variant: OPA0CAL_H_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA0CAL_H` reader - OPA0 offset calibration for N diff"]
pub type OPA0CAL_H_R = crate::BitReader<OPA0CAL_H_A>;
impl OPA0CAL_H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPA0CAL_H_A {
        match self.bits {
            false => OPA0CAL_H_A::DISABLED,
            true => OPA0CAL_H_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPA0CAL_H_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPA0CAL_H_A::ENABLED
    }
}
#[doc = "Field `OPA0CAL_H` writer - OPA0 offset calibration for N diff"]
pub type OPA0CAL_H_W<'a> = crate::BitWriter<'a, u32, OPA_CTL_SPEC, OPA0CAL_H_A, 6>;
impl<'a> OPA0CAL_H_W<'a> {
    #[doc = "Offset calibration for N diff disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPA0CAL_H_A::DISABLED)
    }
    #[doc = "Offset calibration for N diff enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPA0CAL_H_A::ENABLED)
    }
}
#[doc = "OPA0 low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPA0LPM_A {
    #[doc = "0: Low power mode disabled"]
    DISABLED = 0,
    #[doc = "1: Low power mode enabled"]
    ENABLED = 1,
}
impl From<OPA0LPM_A> for bool {
    #[inline(always)]
    fn from(variant: OPA0LPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA0LPM` reader - OPA0 low power mode"]
pub type OPA0LPM_R = crate::BitReader<OPA0LPM_A>;
impl OPA0LPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPA0LPM_A {
        match self.bits {
            false => OPA0LPM_A::DISABLED,
            true => OPA0LPM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPA0LPM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPA0LPM_A::ENABLED
    }
}
#[doc = "Field `OPA0LPM` writer - OPA0 low power mode"]
pub type OPA0LPM_W<'a> = crate::BitWriter<'a, u32, OPA_CTL_SPEC, OPA0LPM_A, 7>;
impl<'a> OPA0LPM_W<'a> {
    #[doc = "Low power mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPA0LPM_A::DISABLED)
    }
    #[doc = "Low power mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPA0LPM_A::ENABLED)
    }
}
#[doc = "OPA1 offset calibration for N diff"]
pub use OPA0CAL_H_A as OPA1CAL_H_A;
#[doc = "OPA2 offset calibration for N diff"]
pub use OPA0CAL_H_A as OPA2CAL_H_A;
#[doc = "Field `OPA1CAL_H` reader - OPA1 offset calibration for N diff"]
pub use OPA0CAL_H_R as OPA1CAL_H_R;
#[doc = "Field `OPA2CAL_H` reader - OPA2 offset calibration for N diff"]
pub use OPA0CAL_H_R as OPA2CAL_H_R;
#[doc = "Field `OPA1CAL_H` writer - OPA1 offset calibration for N diff"]
pub use OPA0CAL_H_W as OPA1CAL_H_W;
#[doc = "Field `OPA2CAL_H` writer - OPA2 offset calibration for N diff"]
pub use OPA0CAL_H_W as OPA2CAL_H_W;
#[doc = "OPA1 offset calibration for P diff"]
pub use OPA0CAL_L_A as OPA1CAL_L_A;
#[doc = "OPA2 offset calibration for P diff"]
pub use OPA0CAL_L_A as OPA2CAL_L_A;
#[doc = "Field `OPA1CAL_L` reader - OPA1 offset calibration for P diff"]
pub use OPA0CAL_L_R as OPA1CAL_L_R;
#[doc = "Field `OPA2CAL_L` reader - OPA2 offset calibration for P diff"]
pub use OPA0CAL_L_R as OPA2CAL_L_R;
#[doc = "Field `OPA1CAL_L` writer - OPA1 offset calibration for P diff"]
pub use OPA0CAL_L_W as OPA1CAL_L_W;
#[doc = "Field `OPA2CAL_L` writer - OPA2 offset calibration for P diff"]
pub use OPA0CAL_L_W as OPA2CAL_L_W;
#[doc = "OPA1 low power mode"]
pub use OPA0LPM_A as OPA1LPM_A;
#[doc = "OPA2 low power mode"]
pub use OPA0LPM_A as OPA2LPM_A;
#[doc = "Field `OPA1LPM` reader - OPA1 low power mode"]
pub use OPA0LPM_R as OPA1LPM_R;
#[doc = "Field `OPA2LPM` reader - OPA2 low power mode"]
pub use OPA0LPM_R as OPA2LPM_R;
#[doc = "Field `OPA1LPM` writer - OPA1 low power mode"]
pub use OPA0LPM_W as OPA1LPM_W;
#[doc = "Field `OPA2LPM` writer - OPA2 low power mode"]
pub use OPA0LPM_W as OPA2LPM_W;
#[doc = "OPA1 power down"]
pub use OPA0PD_A as OPA1PD_A;
#[doc = "OPA2 power down"]
pub use OPA0PD_A as OPA2PD_A;
#[doc = "Field `OPA1PD` reader - OPA1 power down"]
pub use OPA0PD_R as OPA1PD_R;
#[doc = "Field `OPA2PD` reader - OPA2 power down"]
pub use OPA0PD_R as OPA2PD_R;
#[doc = "Field `OPA1PD` writer - OPA1 power down"]
pub use OPA0PD_W as OPA1PD_W;
#[doc = "Field `OPA2PD` writer - OPA2 power down"]
pub use OPA0PD_W as OPA2PD_W;
#[doc = "T3 switch enable for OPA1"]
pub use T3OPA0_A as T3OPA1_A;
#[doc = "S1 switch enable for OPA1"]
pub use T3OPA0_A as S1OPA1_A;
#[doc = "S2 switch enable for OPA1"]
pub use T3OPA0_A as S2OPA1_A;
#[doc = "S3 switch enable for OPA1"]
pub use T3OPA0_A as S3OPA1_A;
#[doc = "T3 switch enable for OPA2"]
pub use T3OPA0_A as T3OPA2_A;
#[doc = "S1 switch enable for OPA2"]
pub use T3OPA0_A as S1OPA2_A;
#[doc = "S2 switch enable for OPA2"]
pub use T3OPA0_A as S2OPA2_A;
#[doc = "S3 switch enable for OPA2"]
pub use T3OPA0_A as S3OPA2_A;
#[doc = "S4 switch enable for OPA1"]
pub use T3OPA0_A as S4OPA1_A;
#[doc = "Field `T3OPA1` reader - T3 switch enable for OPA1"]
pub use T3OPA0_R as T3OPA1_R;
#[doc = "Field `S1OPA1` reader - S1 switch enable for OPA1"]
pub use T3OPA0_R as S1OPA1_R;
#[doc = "Field `S2OPA1` reader - S2 switch enable for OPA1"]
pub use T3OPA0_R as S2OPA1_R;
#[doc = "Field `S3OPA1` reader - S3 switch enable for OPA1"]
pub use T3OPA0_R as S3OPA1_R;
#[doc = "Field `T3OPA2` reader - T3 switch enable for OPA2"]
pub use T3OPA0_R as T3OPA2_R;
#[doc = "Field `S1OPA2` reader - S1 switch enable for OPA2"]
pub use T3OPA0_R as S1OPA2_R;
#[doc = "Field `S2OPA2` reader - S2 switch enable for OPA2"]
pub use T3OPA0_R as S2OPA2_R;
#[doc = "Field `S3OPA2` reader - S3 switch enable for OPA2"]
pub use T3OPA0_R as S3OPA2_R;
#[doc = "Field `S4OPA1` reader - S4 switch enable for OPA1"]
pub use T3OPA0_R as S4OPA1_R;
#[doc = "Field `T3OPA1` writer - T3 switch enable for OPA1"]
pub use T3OPA0_W as T3OPA1_W;
#[doc = "Field `S1OPA1` writer - S1 switch enable for OPA1"]
pub use T3OPA0_W as S1OPA1_W;
#[doc = "Field `S2OPA1` writer - S2 switch enable for OPA1"]
pub use T3OPA0_W as S2OPA1_W;
#[doc = "Field `S3OPA1` writer - S3 switch enable for OPA1"]
pub use T3OPA0_W as S3OPA1_W;
#[doc = "Field `T3OPA2` writer - T3 switch enable for OPA2"]
pub use T3OPA0_W as T3OPA2_W;
#[doc = "Field `S1OPA2` writer - S1 switch enable for OPA2"]
pub use T3OPA0_W as S1OPA2_W;
#[doc = "Field `S2OPA2` writer - S2 switch enable for OPA2"]
pub use T3OPA0_W as S2OPA2_W;
#[doc = "Field `S3OPA2` writer - S3 switch enable for OPA2"]
pub use T3OPA0_W as S3OPA2_W;
#[doc = "Field `S4OPA1` writer - S4 switch enable for OPA1"]
pub use T3OPA0_W as S4OPA1_W;
#[doc = "Power supply range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPA_RANGE_A {
    #[doc = "0: Low range, < 3.3 V"]
    LOW = 0,
    #[doc = "1: High range, > 3.3 V"]
    HIGH = 1,
}
impl From<OPA_RANGE_A> for bool {
    #[inline(always)]
    fn from(variant: OPA_RANGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA_RANGE` reader - Power supply range"]
pub type OPA_RANGE_R = crate::BitReader<OPA_RANGE_A>;
impl OPA_RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPA_RANGE_A {
        match self.bits {
            false => OPA_RANGE_A::LOW,
            true => OPA_RANGE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OPA_RANGE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OPA_RANGE_A::HIGH
    }
}
#[doc = "Field `OPA_RANGE` writer - Power supply range"]
pub type OPA_RANGE_W<'a> = crate::BitWriter<'a, u32, OPA_CTL_SPEC, OPA_RANGE_A, 28>;
impl<'a> OPA_RANGE_W<'a> {
    #[doc = "Low range, < 3.3 V"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OPA_RANGE_A::LOW)
    }
    #[doc = "High range, > 3.3 V"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OPA_RANGE_A::HIGH)
    }
}
#[doc = "OPA0 calibration output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPA0CALOUT_A {
    #[doc = "0: The offset is not trimmed"]
    NOTTRIMMED = 0,
    #[doc = "1: The offset is trimmed, in calibration mode"]
    TRIMMED = 1,
}
impl From<OPA0CALOUT_A> for bool {
    #[inline(always)]
    fn from(variant: OPA0CALOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA0CALOUT` reader - OPA0 calibration output"]
pub type OPA0CALOUT_R = crate::BitReader<OPA0CALOUT_A>;
impl OPA0CALOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPA0CALOUT_A {
        match self.bits {
            false => OPA0CALOUT_A::NOTTRIMMED,
            true => OPA0CALOUT_A::TRIMMED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTTRIMMED`"]
    #[inline(always)]
    pub fn is_not_trimmed(&self) -> bool {
        *self == OPA0CALOUT_A::NOTTRIMMED
    }
    #[doc = "Checks if the value of the field is `TRIMMED`"]
    #[inline(always)]
    pub fn is_trimmed(&self) -> bool {
        *self == OPA0CALOUT_A::TRIMMED
    }
}
#[doc = "Field `OPA0CALOUT` writer - OPA0 calibration output"]
pub type OPA0CALOUT_W<'a> = crate::BitWriter<'a, u32, OPA_CTL_SPEC, OPA0CALOUT_A, 29>;
impl<'a> OPA0CALOUT_W<'a> {
    #[doc = "The offset is not trimmed"]
    #[inline(always)]
    pub fn not_trimmed(self) -> &'a mut W {
        self.variant(OPA0CALOUT_A::NOTTRIMMED)
    }
    #[doc = "The offset is trimmed, in calibration mode"]
    #[inline(always)]
    pub fn trimmed(self) -> &'a mut W {
        self.variant(OPA0CALOUT_A::TRIMMED)
    }
}
#[doc = "OPA1 calibration output"]
pub use OPA0CALOUT_A as OPA1CALOUT_A;
#[doc = "OPA2 calibration output"]
pub use OPA0CALOUT_A as OPA2CALOUT_A;
#[doc = "Field `OPA1CALOUT` reader - OPA1 calibration output"]
pub use OPA0CALOUT_R as OPA1CALOUT_R;
#[doc = "Field `OPA2CALOUT` reader - OPA2 calibration output"]
pub use OPA0CALOUT_R as OPA2CALOUT_R;
#[doc = "Field `OPA1CALOUT` writer - OPA1 calibration output"]
pub use OPA0CALOUT_W as OPA1CALOUT_W;
#[doc = "Field `OPA2CALOUT` writer - OPA2 calibration output"]
pub use OPA0CALOUT_W as OPA2CALOUT_W;
impl R {
    #[doc = "Bit 0 - OPA0 power down"]
    #[inline(always)]
    pub fn opa0pd(&self) -> OPA0PD_R {
        OPA0PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - T3 switch enable for OPA0"]
    #[inline(always)]
    pub fn t3opa0(&self) -> T3OPA0_R {
        T3OPA0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - S1 switch enable for OPA0"]
    #[inline(always)]
    pub fn s1opa0(&self) -> S1OPA0_R {
        S1OPA0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - S2 switch enable for OPA0"]
    #[inline(always)]
    pub fn s2opa0(&self) -> S2OPA0_R {
        S2OPA0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - S3 switch enable for OPA0"]
    #[inline(always)]
    pub fn s3opa0(&self) -> S3OPA0_R {
        S3OPA0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OPA0 offset calibration for P diff"]
    #[inline(always)]
    pub fn opa0cal_l(&self) -> OPA0CAL_L_R {
        OPA0CAL_L_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OPA0 offset calibration for N diff"]
    #[inline(always)]
    pub fn opa0cal_h(&self) -> OPA0CAL_H_R {
        OPA0CAL_H_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OPA0 low power mode"]
    #[inline(always)]
    pub fn opa0lpm(&self) -> OPA0LPM_R {
        OPA0LPM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPA1 power down"]
    #[inline(always)]
    pub fn opa1pd(&self) -> OPA1PD_R {
        OPA1PD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - T3 switch enable for OPA1"]
    #[inline(always)]
    pub fn t3opa1(&self) -> T3OPA1_R {
        T3OPA1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - S1 switch enable for OPA1"]
    #[inline(always)]
    pub fn s1opa1(&self) -> S1OPA1_R {
        S1OPA1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - S2 switch enable for OPA1"]
    #[inline(always)]
    pub fn s2opa1(&self) -> S2OPA1_R {
        S2OPA1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - S3 switch enable for OPA1"]
    #[inline(always)]
    pub fn s3opa1(&self) -> S3OPA1_R {
        S3OPA1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OPA1 offset calibration for P diff"]
    #[inline(always)]
    pub fn opa1cal_l(&self) -> OPA1CAL_L_R {
        OPA1CAL_L_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OPA1 offset calibration for N diff"]
    #[inline(always)]
    pub fn opa1cal_h(&self) -> OPA1CAL_H_R {
        OPA1CAL_H_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OPA1 low power mode"]
    #[inline(always)]
    pub fn opa1lpm(&self) -> OPA1LPM_R {
        OPA1LPM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OPA2 power down"]
    #[inline(always)]
    pub fn opa2pd(&self) -> OPA2PD_R {
        OPA2PD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - T3 switch enable for OPA2"]
    #[inline(always)]
    pub fn t3opa2(&self) -> T3OPA2_R {
        T3OPA2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - S1 switch enable for OPA2"]
    #[inline(always)]
    pub fn s1opa2(&self) -> S1OPA2_R {
        S1OPA2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - S2 switch enable for OPA2"]
    #[inline(always)]
    pub fn s2opa2(&self) -> S2OPA2_R {
        S2OPA2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - S3 switch enable for OPA2"]
    #[inline(always)]
    pub fn s3opa2(&self) -> S3OPA2_R {
        S3OPA2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPA2 offset calibration for P diff"]
    #[inline(always)]
    pub fn opa2cal_l(&self) -> OPA2CAL_L_R {
        OPA2CAL_L_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA2 offset calibration for N diff"]
    #[inline(always)]
    pub fn opa2cal_h(&self) -> OPA2CAL_H_R {
        OPA2CAL_H_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPA2 low power mode"]
    #[inline(always)]
    pub fn opa2lpm(&self) -> OPA2LPM_R {
        OPA2LPM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - S4 switch enable for OPA1"]
    #[inline(always)]
    pub fn s4opa1(&self) -> S4OPA1_R {
        S4OPA1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power supply range"]
    #[inline(always)]
    pub fn opa_range(&self) -> OPA_RANGE_R {
        OPA_RANGE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OPA0 calibration output"]
    #[inline(always)]
    pub fn opa0calout(&self) -> OPA0CALOUT_R {
        OPA0CALOUT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPA1 calibration output"]
    #[inline(always)]
    pub fn opa1calout(&self) -> OPA1CALOUT_R {
        OPA1CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA2 calibration output"]
    #[inline(always)]
    pub fn opa2calout(&self) -> OPA2CALOUT_R {
        OPA2CALOUT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA0 power down"]
    #[inline(always)]
    pub fn opa0pd(&mut self) -> OPA0PD_W {
        OPA0PD_W::new(self)
    }
    #[doc = "Bit 1 - T3 switch enable for OPA0"]
    #[inline(always)]
    pub fn t3opa0(&mut self) -> T3OPA0_W {
        T3OPA0_W::new(self)
    }
    #[doc = "Bit 2 - S1 switch enable for OPA0"]
    #[inline(always)]
    pub fn s1opa0(&mut self) -> S1OPA0_W {
        S1OPA0_W::new(self)
    }
    #[doc = "Bit 3 - S2 switch enable for OPA0"]
    #[inline(always)]
    pub fn s2opa0(&mut self) -> S2OPA0_W {
        S2OPA0_W::new(self)
    }
    #[doc = "Bit 4 - S3 switch enable for OPA0"]
    #[inline(always)]
    pub fn s3opa0(&mut self) -> S3OPA0_W {
        S3OPA0_W::new(self)
    }
    #[doc = "Bit 5 - OPA0 offset calibration for P diff"]
    #[inline(always)]
    pub fn opa0cal_l(&mut self) -> OPA0CAL_L_W {
        OPA0CAL_L_W::new(self)
    }
    #[doc = "Bit 6 - OPA0 offset calibration for N diff"]
    #[inline(always)]
    pub fn opa0cal_h(&mut self) -> OPA0CAL_H_W {
        OPA0CAL_H_W::new(self)
    }
    #[doc = "Bit 7 - OPA0 low power mode"]
    #[inline(always)]
    pub fn opa0lpm(&mut self) -> OPA0LPM_W {
        OPA0LPM_W::new(self)
    }
    #[doc = "Bit 8 - OPA1 power down"]
    #[inline(always)]
    pub fn opa1pd(&mut self) -> OPA1PD_W {
        OPA1PD_W::new(self)
    }
    #[doc = "Bit 9 - T3 switch enable for OPA1"]
    #[inline(always)]
    pub fn t3opa1(&mut self) -> T3OPA1_W {
        T3OPA1_W::new(self)
    }
    #[doc = "Bit 10 - S1 switch enable for OPA1"]
    #[inline(always)]
    pub fn s1opa1(&mut self) -> S1OPA1_W {
        S1OPA1_W::new(self)
    }
    #[doc = "Bit 11 - S2 switch enable for OPA1"]
    #[inline(always)]
    pub fn s2opa1(&mut self) -> S2OPA1_W {
        S2OPA1_W::new(self)
    }
    #[doc = "Bit 12 - S3 switch enable for OPA1"]
    #[inline(always)]
    pub fn s3opa1(&mut self) -> S3OPA1_W {
        S3OPA1_W::new(self)
    }
    #[doc = "Bit 13 - OPA1 offset calibration for P diff"]
    #[inline(always)]
    pub fn opa1cal_l(&mut self) -> OPA1CAL_L_W {
        OPA1CAL_L_W::new(self)
    }
    #[doc = "Bit 14 - OPA1 offset calibration for N diff"]
    #[inline(always)]
    pub fn opa1cal_h(&mut self) -> OPA1CAL_H_W {
        OPA1CAL_H_W::new(self)
    }
    #[doc = "Bit 15 - OPA1 low power mode"]
    #[inline(always)]
    pub fn opa1lpm(&mut self) -> OPA1LPM_W {
        OPA1LPM_W::new(self)
    }
    #[doc = "Bit 16 - OPA2 power down"]
    #[inline(always)]
    pub fn opa2pd(&mut self) -> OPA2PD_W {
        OPA2PD_W::new(self)
    }
    #[doc = "Bit 17 - T3 switch enable for OPA2"]
    #[inline(always)]
    pub fn t3opa2(&mut self) -> T3OPA2_W {
        T3OPA2_W::new(self)
    }
    #[doc = "Bit 18 - S1 switch enable for OPA2"]
    #[inline(always)]
    pub fn s1opa2(&mut self) -> S1OPA2_W {
        S1OPA2_W::new(self)
    }
    #[doc = "Bit 19 - S2 switch enable for OPA2"]
    #[inline(always)]
    pub fn s2opa2(&mut self) -> S2OPA2_W {
        S2OPA2_W::new(self)
    }
    #[doc = "Bit 20 - S3 switch enable for OPA2"]
    #[inline(always)]
    pub fn s3opa2(&mut self) -> S3OPA2_W {
        S3OPA2_W::new(self)
    }
    #[doc = "Bit 21 - OPA2 offset calibration for P diff"]
    #[inline(always)]
    pub fn opa2cal_l(&mut self) -> OPA2CAL_L_W {
        OPA2CAL_L_W::new(self)
    }
    #[doc = "Bit 22 - OPA2 offset calibration for N diff"]
    #[inline(always)]
    pub fn opa2cal_h(&mut self) -> OPA2CAL_H_W {
        OPA2CAL_H_W::new(self)
    }
    #[doc = "Bit 23 - OPA2 low power mode"]
    #[inline(always)]
    pub fn opa2lpm(&mut self) -> OPA2LPM_W {
        OPA2LPM_W::new(self)
    }
    #[doc = "Bit 27 - S4 switch enable for OPA1"]
    #[inline(always)]
    pub fn s4opa1(&mut self) -> S4OPA1_W {
        S4OPA1_W::new(self)
    }
    #[doc = "Bit 28 - Power supply range"]
    #[inline(always)]
    pub fn opa_range(&mut self) -> OPA_RANGE_W {
        OPA_RANGE_W::new(self)
    }
    #[doc = "Bit 29 - OPA0 calibration output"]
    #[inline(always)]
    pub fn opa0calout(&mut self) -> OPA0CALOUT_W {
        OPA0CALOUT_W::new(self)
    }
    #[doc = "Bit 30 - OPA1 calibration output"]
    #[inline(always)]
    pub fn opa1calout(&mut self) -> OPA1CALOUT_W {
        OPA1CALOUT_W::new(self)
    }
    #[doc = "Bit 31 - OPA2 calibration output"]
    #[inline(always)]
    pub fn opa2calout(&mut self) -> OPA2CALOUT_W {
        OPA2CALOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa_ctl](index.html) module"]
pub struct OPA_CTL_SPEC;
impl crate::RegisterSpec for OPA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opa_ctl::R](R) reader structure"]
impl crate::Readable for OPA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opa_ctl::W](W) writer structure"]
impl crate::Writable for OPA_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPA_CTL to value 0x0001_0101"]
impl crate::Resettable for OPA_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0101
    }
}
