#[doc = "Register `INT` reader"]
pub struct R(crate::R<INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT` writer"]
pub struct W(crate::W<INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SPEC>;
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
impl From<crate::W<INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IRC40K stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC40KSTBIF_A {
    #[doc = "0: No interrupt generated"]
    NOTINTERRUPTED = 0,
    #[doc = "1: IRC40K stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<IRC40KSTBIF_A> for bool {
    #[inline(always)]
    fn from(variant: IRC40KSTBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KSTBIF` reader - IRC40K stabilization interrupt flag"]
pub type IRC40KSTBIF_R = crate::BitReader<IRC40KSTBIF_A>;
impl IRC40KSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC40KSTBIF_A {
        match self.bits {
            false => IRC40KSTBIF_A::NOTINTERRUPTED,
            true => IRC40KSTBIF_A::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == IRC40KSTBIF_A::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == IRC40KSTBIF_A::INTERRUPTED
    }
}
#[doc = "LXTAL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTALSTBIF_A {
    #[doc = "0: No interrupt generated"]
    NOTINTERRUPTED = 0,
    #[doc = "1: LXTAL stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<LXTALSTBIF_A> for bool {
    #[inline(always)]
    fn from(variant: LXTALSTBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTBIF` reader - LXTAL stabilization interrupt flag"]
pub type LXTALSTBIF_R = crate::BitReader<LXTALSTBIF_A>;
impl LXTALSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALSTBIF_A {
        match self.bits {
            false => LXTALSTBIF_A::NOTINTERRUPTED,
            true => LXTALSTBIF_A::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LXTALSTBIF_A::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LXTALSTBIF_A::INTERRUPTED
    }
}
#[doc = "IRC8M stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC8MSTBIF_A {
    #[doc = "0: No interrupt generated"]
    NOTINTERRUPTED = 0,
    #[doc = "1: IRC8M stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<IRC8MSTBIF_A> for bool {
    #[inline(always)]
    fn from(variant: IRC8MSTBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MSTBIF` reader - IRC8M stabilization interrupt flag"]
pub type IRC8MSTBIF_R = crate::BitReader<IRC8MSTBIF_A>;
impl IRC8MSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC8MSTBIF_A {
        match self.bits {
            false => IRC8MSTBIF_A::NOTINTERRUPTED,
            true => IRC8MSTBIF_A::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == IRC8MSTBIF_A::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == IRC8MSTBIF_A::INTERRUPTED
    }
}
#[doc = "HXTAL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTALSTBIF_A {
    #[doc = "0: No interrupt generated"]
    NOTINTERRUPTED = 0,
    #[doc = "1: HXTAL stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<HXTALSTBIF_A> for bool {
    #[inline(always)]
    fn from(variant: HXTALSTBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTBIF` reader - HXTAL stabilization interrupt flag"]
pub type HXTALSTBIF_R = crate::BitReader<HXTALSTBIF_A>;
impl HXTALSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTALSTBIF_A {
        match self.bits {
            false => HXTALSTBIF_A::NOTINTERRUPTED,
            true => HXTALSTBIF_A::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == HXTALSTBIF_A::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == HXTALSTBIF_A::INTERRUPTED
    }
}
#[doc = "PLL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTBIF_A {
    #[doc = "0: No interrupt generated"]
    NOTINTERRUPTED = 0,
    #[doc = "1: PLL stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<PLLSTBIF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBIF` reader - PLL stabilization interrupt flag"]
pub type PLLSTBIF_R = crate::BitReader<PLLSTBIF_A>;
impl PLLSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTBIF_A {
        match self.bits {
            false => PLLSTBIF_A::NOTINTERRUPTED,
            true => PLLSTBIF_A::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == PLLSTBIF_A::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == PLLSTBIF_A::INTERRUPTED
    }
}
#[doc = "IRC14M stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC14MSTBIF_A {
    #[doc = "0: No interrupt generated"]
    NOTINTERRUPTED = 0,
    #[doc = "1: IRC14M stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<IRC14MSTBIF_A> for bool {
    #[inline(always)]
    fn from(variant: IRC14MSTBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC14MSTBIF` reader - IRC14M stabilization interrupt flag"]
pub type IRC14MSTBIF_R = crate::BitReader<IRC14MSTBIF_A>;
impl IRC14MSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC14MSTBIF_A {
        match self.bits {
            false => IRC14MSTBIF_A::NOTINTERRUPTED,
            true => IRC14MSTBIF_A::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == IRC14MSTBIF_A::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == IRC14MSTBIF_A::INTERRUPTED
    }
}
#[doc = "HXTAL Clock Stuck Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKMIF_A {
    #[doc = "0: Clock operating normally"]
    NOTINTERRUPTED = 0,
    #[doc = "1: HXTAL clock stuck"]
    INTERRUPTED = 1,
}
impl From<CKMIF_A> for bool {
    #[inline(always)]
    fn from(variant: CKMIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMIF` reader - HXTAL Clock Stuck Interrupt Flag"]
pub type CKMIF_R = crate::BitReader<CKMIF_A>;
impl CKMIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMIF_A {
        match self.bits {
            false => CKMIF_A::NOTINTERRUPTED,
            true => CKMIF_A::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CKMIF_A::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CKMIF_A::INTERRUPTED
    }
}
#[doc = "IRC40K Stabilization interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC40KSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<IRC40KSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: IRC40KSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KSTBIE` reader - IRC40K Stabilization interrupt enable"]
pub type IRC40KSTBIE_R = crate::BitReader<IRC40KSTBIE_A>;
impl IRC40KSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC40KSTBIE_A {
        match self.bits {
            false => IRC40KSTBIE_A::DISABLED,
            true => IRC40KSTBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRC40KSTBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRC40KSTBIE_A::ENABLED
    }
}
#[doc = "Field `IRC40KSTBIE` writer - IRC40K Stabilization interrupt enable"]
pub type IRC40KSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, IRC40KSTBIE_A, 8>;
impl<'a> IRC40KSTBIE_W<'a> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IRC40KSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IRC40KSTBIE_A::ENABLED)
    }
}
#[doc = "LXTAL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTALSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<LXTALSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: LXTALSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTBIE` reader - LXTAL Stabilization Interrupt Enable"]
pub type LXTALSTBIE_R = crate::BitReader<LXTALSTBIE_A>;
impl LXTALSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALSTBIE_A {
        match self.bits {
            false => LXTALSTBIE_A::DISABLED,
            true => LXTALSTBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LXTALSTBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LXTALSTBIE_A::ENABLED
    }
}
#[doc = "Field `LXTALSTBIE` writer - LXTAL Stabilization Interrupt Enable"]
pub type LXTALSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, LXTALSTBIE_A, 9>;
impl<'a> LXTALSTBIE_W<'a> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LXTALSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LXTALSTBIE_A::ENABLED)
    }
}
#[doc = "IRC8M Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC8MSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<IRC8MSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: IRC8MSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MSTBIE` reader - IRC8M Stabilization Interrupt Enable"]
pub type IRC8MSTBIE_R = crate::BitReader<IRC8MSTBIE_A>;
impl IRC8MSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC8MSTBIE_A {
        match self.bits {
            false => IRC8MSTBIE_A::DISABLED,
            true => IRC8MSTBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRC8MSTBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRC8MSTBIE_A::ENABLED
    }
}
#[doc = "Field `IRC8MSTBIE` writer - IRC8M Stabilization Interrupt Enable"]
pub type IRC8MSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, IRC8MSTBIE_A, 10>;
impl<'a> IRC8MSTBIE_W<'a> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IRC8MSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IRC8MSTBIE_A::ENABLED)
    }
}
#[doc = "HXTAL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTALSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<HXTALSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: HXTALSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTBIE` reader - HXTAL Stabilization Interrupt Enable"]
pub type HXTALSTBIE_R = crate::BitReader<HXTALSTBIE_A>;
impl HXTALSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTALSTBIE_A {
        match self.bits {
            false => HXTALSTBIE_A::DISABLED,
            true => HXTALSTBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HXTALSTBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HXTALSTBIE_A::ENABLED
    }
}
#[doc = "Field `HXTALSTBIE` writer - HXTAL Stabilization Interrupt Enable"]
pub type HXTALSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, HXTALSTBIE_A, 11>;
impl<'a> HXTALSTBIE_W<'a> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HXTALSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HXTALSTBIE_A::ENABLED)
    }
}
#[doc = "PLL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<PLLSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBIE` reader - PLL Stabilization Interrupt Enable"]
pub type PLLSTBIE_R = crate::BitReader<PLLSTBIE_A>;
impl PLLSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTBIE_A {
        match self.bits {
            false => PLLSTBIE_A::DISABLED,
            true => PLLSTBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSTBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSTBIE_A::ENABLED
    }
}
#[doc = "Field `PLLSTBIE` writer - PLL Stabilization Interrupt Enable"]
pub type PLLSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, PLLSTBIE_A, 12>;
impl<'a> PLLSTBIE_W<'a> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSTBIE_A::ENABLED)
    }
}
#[doc = "IRC14M Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC14MSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<IRC14MSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: IRC14MSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC14MSTBIE` reader - IRC14M Stabilization Interrupt Enable"]
pub type IRC14MSTBIE_R = crate::BitReader<IRC14MSTBIE_A>;
impl IRC14MSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC14MSTBIE_A {
        match self.bits {
            false => IRC14MSTBIE_A::DISABLED,
            true => IRC14MSTBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRC14MSTBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRC14MSTBIE_A::ENABLED
    }
}
#[doc = "Field `IRC14MSTBIE` writer - IRC14M Stabilization Interrupt Enable"]
pub type IRC14MSTBIE_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, IRC14MSTBIE_A, 13>;
impl<'a> IRC14MSTBIE_W<'a> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IRC14MSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IRC14MSTBIE_A::ENABLED)
    }
}
#[doc = "IRC40K Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC40KSTBIC_AW {
    #[doc = "1: Clear IRC40KSTBIF flag"]
    CLEAR = 1,
}
impl From<IRC40KSTBIC_AW> for bool {
    #[inline(always)]
    fn from(variant: IRC40KSTBIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KSTBIC` writer - IRC40K Stabilization Interrupt Clear"]
pub type IRC40KSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, IRC40KSTBIC_AW, 16>;
impl<'a> IRC40KSTBIC_W<'a> {
    #[doc = "Clear IRC40KSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IRC40KSTBIC_AW::CLEAR)
    }
}
#[doc = "LXTAL Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTALSTBIC_AW {
    #[doc = "1: Clear LXTALSTBIF flag"]
    CLEAR = 1,
}
impl From<LXTALSTBIC_AW> for bool {
    #[inline(always)]
    fn from(variant: LXTALSTBIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTBIC` writer - LXTAL Stabilization Interrupt Clear"]
pub type LXTALSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, LXTALSTBIC_AW, 17>;
impl<'a> LXTALSTBIC_W<'a> {
    #[doc = "Clear LXTALSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LXTALSTBIC_AW::CLEAR)
    }
}
#[doc = "IRC8M Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC8MSTBIC_AW {
    #[doc = "1: Clear IRC8MSTBIF flag"]
    CLEAR = 1,
}
impl From<IRC8MSTBIC_AW> for bool {
    #[inline(always)]
    fn from(variant: IRC8MSTBIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MSTBIC` writer - IRC8M Stabilization Interrupt Clear"]
pub type IRC8MSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, IRC8MSTBIC_AW, 18>;
impl<'a> IRC8MSTBIC_W<'a> {
    #[doc = "Clear IRC8MSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IRC8MSTBIC_AW::CLEAR)
    }
}
#[doc = "HXTAL Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTALSTBIC_AW {
    #[doc = "1: Clear HXTALSTBIF flag"]
    CLEAR = 1,
}
impl From<HXTALSTBIC_AW> for bool {
    #[inline(always)]
    fn from(variant: HXTALSTBIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTBIC` writer - HXTAL Stabilization Interrupt Clear"]
pub type HXTALSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, HXTALSTBIC_AW, 19>;
impl<'a> HXTALSTBIC_W<'a> {
    #[doc = "Clear HXTALSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HXTALSTBIC_AW::CLEAR)
    }
}
#[doc = "PLL stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTBIC_AW {
    #[doc = "1: Clear PLLSTBIF flag"]
    CLEAR = 1,
}
impl From<PLLSTBIC_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLSTBIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBIC` writer - PLL stabilization Interrupt Clear"]
pub type PLLSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, PLLSTBIC_AW, 20>;
impl<'a> PLLSTBIC_W<'a> {
    #[doc = "Clear PLLSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLSTBIC_AW::CLEAR)
    }
}
#[doc = "IRC14M stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC14MSTBIC_AW {
    #[doc = "1: Clear IRC14MSTBIF flag"]
    CLEAR = 1,
}
impl From<IRC14MSTBIC_AW> for bool {
    #[inline(always)]
    fn from(variant: IRC14MSTBIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC14MSTBIC` writer - IRC14M stabilization Interrupt Clear"]
pub type IRC14MSTBIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, IRC14MSTBIC_AW, 21>;
impl<'a> IRC14MSTBIC_W<'a> {
    #[doc = "Clear IRC14MSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IRC14MSTBIC_AW::CLEAR)
    }
}
#[doc = "HXTAL Clock Stuck Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKMIC_AW {
    #[doc = "1: Clear CKMIF flag"]
    CLEAR = 1,
}
impl From<CKMIC_AW> for bool {
    #[inline(always)]
    fn from(variant: CKMIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMIC` writer - HXTAL Clock Stuck Interrupt Clear"]
pub type CKMIC_W<'a> = crate::BitWriter<'a, u32, INT_SPEC, CKMIC_AW, 23>;
impl<'a> CKMIC_W<'a> {
    #[doc = "Clear CKMIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CKMIC_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - IRC40K stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc40kstbif(&self) -> IRC40KSTBIF_R {
        IRC40KSTBIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn lxtalstbif(&self) -> LXTALSTBIF_R {
        LXTALSTBIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRC8M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc8mstbif(&self) -> IRC8MSTBIF_R {
        IRC8MSTBIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn hxtalstbif(&self) -> HXTALSTBIF_R {
        HXTALSTBIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllstbif(&self) -> PLLSTBIF_R {
        PLLSTBIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRC14M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc14mstbif(&self) -> IRC14MSTBIF_R {
        IRC14MSTBIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - HXTAL Clock Stuck Interrupt Flag"]
    #[inline(always)]
    pub fn ckmif(&self) -> CKMIF_R {
        CKMIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc40kstbie(&self) -> IRC40KSTBIE_R {
        IRC40KSTBIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&self) -> LXTALSTBIE_R {
        LXTALSTBIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc8mstbie(&self) -> IRC8MSTBIE_R {
        IRC8MSTBIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&self) -> HXTALSTBIE_R {
        HXTALSTBIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&self) -> PLLSTBIE_R {
        PLLSTBIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IRC14M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc14mstbie(&self) -> IRC14MSTBIE_R {
        IRC14MSTBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc40kstbie(&mut self) -> IRC40KSTBIE_W {
        IRC40KSTBIE_W::new(self)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&mut self) -> LXTALSTBIE_W {
        LXTALSTBIE_W::new(self)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc8mstbie(&mut self) -> IRC8MSTBIE_W {
        IRC8MSTBIE_W::new(self)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&mut self) -> HXTALSTBIE_W {
        HXTALSTBIE_W::new(self)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&mut self) -> PLLSTBIE_W {
        PLLSTBIE_W::new(self)
    }
    #[doc = "Bit 13 - IRC14M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc14mstbie(&mut self) -> IRC14MSTBIE_W {
        IRC14MSTBIE_W::new(self)
    }
    #[doc = "Bit 16 - IRC40K Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc40kstbic(&mut self) -> IRC40KSTBIC_W {
        IRC40KSTBIC_W::new(self)
    }
    #[doc = "Bit 17 - LXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn lxtalstbic(&mut self) -> LXTALSTBIC_W {
        LXTALSTBIC_W::new(self)
    }
    #[doc = "Bit 18 - IRC8M Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc8mstbic(&mut self) -> IRC8MSTBIC_W {
        IRC8MSTBIC_W::new(self)
    }
    #[doc = "Bit 19 - HXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn hxtalstbic(&mut self) -> HXTALSTBIC_W {
        HXTALSTBIC_W::new(self)
    }
    #[doc = "Bit 20 - PLL stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn pllstbic(&mut self) -> PLLSTBIC_W {
        PLLSTBIC_W::new(self)
    }
    #[doc = "Bit 21 - IRC14M stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc14mstbic(&mut self) -> IRC14MSTBIC_W {
        IRC14MSTBIC_W::new(self)
    }
    #[doc = "Bit 23 - HXTAL Clock Stuck Interrupt Clear"]
    #[inline(always)]
    pub fn ckmic(&mut self) -> CKMIC_W {
        CKMIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt register (RCU_INT)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](index.html) module"]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int::R](R) reader structure"]
impl crate::Readable for INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int::W](W) writer structure"]
impl crate::Writable for INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
