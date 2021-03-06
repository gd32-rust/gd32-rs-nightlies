#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "End of Block interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBIE_A {
    #[doc = "0: End of block interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: End of block interrupt is enabled"]
    ENABLED = 1,
}
impl From<EBIE_A> for bool {
    #[inline(always)]
    fn from(variant: EBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBIE` reader - End of Block interrupt enable"]
pub type EBIE_R = crate::BitReader<EBIE_A>;
impl EBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBIE_A {
        match self.bits {
            false => EBIE_A::DISABLED,
            true => EBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EBIE_A::ENABLED
    }
}
#[doc = "Field `EBIE` writer - End of Block interrupt enable"]
pub type EBIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, EBIE_A, 27>;
impl<'a> EBIE_W<'a> {
    #[doc = "End of block interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EBIE_A::DISABLED)
    }
    #[doc = "End of block interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EBIE_A::ENABLED)
    }
}
#[doc = "Receiver timeout interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTIE_A {
    #[doc = "0: Receiver timeout interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Receiver timeout interrupt is enabled"]
    ENABLED = 1,
}
impl From<RTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTIE` reader - Receiver timeout interrupt enable"]
pub type RTIE_R = crate::BitReader<RTIE_A>;
impl RTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTIE_A {
        match self.bits {
            false => RTIE_A::DISABLED,
            true => RTIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTIE_A::ENABLED
    }
}
#[doc = "Field `RTIE` writer - Receiver timeout interrupt enable"]
pub type RTIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, RTIE_A, 26>;
impl<'a> RTIE_W<'a> {
    #[doc = "Receiver timeout interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTIE_A::DISABLED)
    }
    #[doc = "Receiver timeout interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTIE_A::ENABLED)
    }
}
#[doc = "Field `DEA` reader - Driver Enable assertion time"]
pub type DEA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEA` writer - Driver Enable assertion time"]
pub type DEA_W<'a> = crate::FieldWriterSafe<'a, u32, CTL0_SPEC, u8, u8, 5, 21>;
#[doc = "Field `DED` reader - Driver Enable deassertion time"]
pub type DED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DED` writer - Driver Enable deassertion time"]
pub type DED_W<'a> = crate::FieldWriterSafe<'a, u32, CTL0_SPEC, u8, u8, 5, 16>;
#[doc = "Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSMOD_A {
    #[doc = "0: Oversampling by 16"]
    OVERSAMPLING16 = 0,
    #[doc = "1: Oversampling by 8"]
    OVERSAMPLING8 = 1,
}
impl From<OVSMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVSMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVSMOD` reader - Oversampling mode"]
pub type OVSMOD_R = crate::BitReader<OVSMOD_A>;
impl OVSMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSMOD_A {
        match self.bits {
            false => OVSMOD_A::OVERSAMPLING16,
            true => OVSMOD_A::OVERSAMPLING8,
        }
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLING16`"]
    #[inline(always)]
    pub fn is_oversampling16(&self) -> bool {
        *self == OVSMOD_A::OVERSAMPLING16
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLING8`"]
    #[inline(always)]
    pub fn is_oversampling8(&self) -> bool {
        *self == OVSMOD_A::OVERSAMPLING8
    }
}
#[doc = "Field `OVSMOD` writer - Oversampling mode"]
pub type OVSMOD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, OVSMOD_A, 15>;
impl<'a> OVSMOD_W<'a> {
    #[doc = "Oversampling by 16"]
    #[inline(always)]
    pub fn oversampling16(self) -> &'a mut W {
        self.variant(OVSMOD_A::OVERSAMPLING16)
    }
    #[doc = "Oversampling by 8"]
    #[inline(always)]
    pub fn oversampling8(self) -> &'a mut W {
        self.variant(OVSMOD_A::OVERSAMPLING8)
    }
}
#[doc = "Character match interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMIE_A {
    #[doc = "0: Address match interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Address match interrupt is enabled"]
    ENABLED = 1,
}
impl From<AMIE_A> for bool {
    #[inline(always)]
    fn from(variant: AMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMIE` reader - Character match interrupt enable"]
pub type AMIE_R = crate::BitReader<AMIE_A>;
impl AMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMIE_A {
        match self.bits {
            false => AMIE_A::DISABLED,
            true => AMIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AMIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AMIE_A::ENABLED
    }
}
#[doc = "Field `AMIE` writer - Character match interrupt enable"]
pub type AMIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, AMIE_A, 14>;
impl<'a> AMIE_W<'a> {
    #[doc = "Address match interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AMIE_A::DISABLED)
    }
    #[doc = "Address match interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AMIE_A::ENABLED)
    }
}
#[doc = "Mute mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEN_A {
    #[doc = "0: Receiver in active mode permanently"]
    DISABLED = 0,
    #[doc = "1: Receiver can switch between mute mode and active mode"]
    ENABLED = 1,
}
impl From<MEN_A> for bool {
    #[inline(always)]
    fn from(variant: MEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEN` reader - Mute mode enable"]
pub type MEN_R = crate::BitReader<MEN_A>;
impl MEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEN_A {
        match self.bits {
            false => MEN_A::DISABLED,
            true => MEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MEN_A::ENABLED
    }
}
#[doc = "Field `MEN` writer - Mute mode enable"]
pub type MEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, MEN_A, 13>;
impl<'a> MEN_W<'a> {
    #[doc = "Receiver in active mode permanently"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MEN_A::DISABLED)
    }
    #[doc = "Receiver can switch between mute mode and active mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MEN_A::ENABLED)
    }
}
#[doc = "Word length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WL_A {
    #[doc = "0: 8 data bits"]
    BIT8 = 0,
    #[doc = "1: 9 data bits"]
    BIT9 = 1,
}
impl From<WL_A> for bool {
    #[inline(always)]
    fn from(variant: WL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WL` reader - Word length"]
pub type WL_R = crate::BitReader<WL_A>;
impl WL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WL_A {
        match self.bits {
            false => WL_A::BIT8,
            true => WL_A::BIT9,
        }
    }
    #[doc = "Checks if the value of the field is `BIT8`"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == WL_A::BIT8
    }
    #[doc = "Checks if the value of the field is `BIT9`"]
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == WL_A::BIT9
    }
}
#[doc = "Field `WL` writer - Word length"]
pub type WL_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, WL_A, 12>;
impl<'a> WL_W<'a> {
    #[doc = "8 data bits"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(WL_A::BIT8)
    }
    #[doc = "9 data bits"]
    #[inline(always)]
    pub fn bit9(self) -> &'a mut W {
        self.variant(WL_A::BIT9)
    }
}
#[doc = "Receiver wakeup method\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WM_A {
    #[doc = "0: Idle line"]
    IDLE = 0,
    #[doc = "1: Address mask"]
    ADDRESS = 1,
}
impl From<WM_A> for bool {
    #[inline(always)]
    fn from(variant: WM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WM` reader - Receiver wakeup method"]
pub type WM_R = crate::BitReader<WM_A>;
impl WM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WM_A {
        match self.bits {
            false => WM_A::IDLE,
            true => WM_A::ADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WM_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ADDRESS`"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WM_A::ADDRESS
    }
}
#[doc = "Field `WM` writer - Receiver wakeup method"]
pub type WM_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, WM_A, 11>;
impl<'a> WM_W<'a> {
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(WM_A::IDLE)
    }
    #[doc = "Address mask"]
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WM_A::ADDRESS)
    }
}
#[doc = "Parity control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCEN_A {
    #[doc = "0: Parity control disabled"]
    DISABLED = 0,
    #[doc = "1: Parity control enabled"]
    ENABLED = 1,
}
impl From<PCEN_A> for bool {
    #[inline(always)]
    fn from(variant: PCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCEN` reader - Parity control enable"]
pub type PCEN_R = crate::BitReader<PCEN_A>;
impl PCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCEN_A {
        match self.bits {
            false => PCEN_A::DISABLED,
            true => PCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCEN_A::ENABLED
    }
}
#[doc = "Field `PCEN` writer - Parity control enable"]
pub type PCEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, PCEN_A, 10>;
impl<'a> PCEN_W<'a> {
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PCEN_A::DISABLED)
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PCEN_A::ENABLED)
    }
}
#[doc = "Parity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PM_A {
    #[doc = "0: Even parity"]
    EVEN = 0,
    #[doc = "1: Odd parity"]
    ODD = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - Parity selection"]
pub type PM_R = crate::BitReader<PM_A>;
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::EVEN,
            true => PM_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PM_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PM_A::ODD
    }
}
#[doc = "Field `PM` writer - Parity selection"]
pub type PM_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, PM_A, 9>;
impl<'a> PM_W<'a> {
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PM_A::EVEN)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PM_A::ODD)
    }
}
#[doc = "PE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERRIE_A {
    #[doc = "0: Parity error interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever PERR=1 in the STAT register"]
    ENABLED = 1,
}
impl From<PERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: PERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERRIE` reader - PE interrupt enable"]
pub type PERRIE_R = crate::BitReader<PERRIE_A>;
impl PERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERRIE_A {
        match self.bits {
            false => PERRIE_A::DISABLED,
            true => PERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PERRIE_A::ENABLED
    }
}
#[doc = "Field `PERRIE` writer - PE interrupt enable"]
pub type PERRIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, PERRIE_A, 8>;
impl<'a> PERRIE_W<'a> {
    #[doc = "Parity error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PERRIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever PERR=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PERRIE_A::ENABLED)
    }
}
#[doc = "interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBEIE_A {
    #[doc = "0: Transmission register empty interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever TBE=1 in the STAT register"]
    ENABLED = 1,
}
impl From<TBEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TBEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBEIE` reader - interrupt enable"]
pub type TBEIE_R = crate::BitReader<TBEIE_A>;
impl TBEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBEIE_A {
        match self.bits {
            false => TBEIE_A::DISABLED,
            true => TBEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TBEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TBEIE_A::ENABLED
    }
}
#[doc = "Field `TBEIE` writer - interrupt enable"]
pub type TBEIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, TBEIE_A, 7>;
impl<'a> TBEIE_W<'a> {
    #[doc = "Transmission register empty interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TBEIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever TBE=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TBEIE_A::ENABLED)
    }
}
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    #[doc = "0: Transmission complete interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever TC=1 in the STAT register"]
    ENABLED = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::DISABLED,
            true => TCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::ENABLED
    }
}
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TCIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, TCIE_A, 6>;
impl<'a> TCIE_W<'a> {
    #[doc = "Transmission complete interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever TC=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::ENABLED)
    }
}
#[doc = "RXNE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBNEIE_A {
    #[doc = "0: Read data buffer not empty and overrrun error interrupts are disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever ORERR=1 or RBNE=1 in the STAT register"]
    ENABLED = 1,
}
impl From<RBNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBNEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBNEIE` reader - RXNE interrupt enable"]
pub type RBNEIE_R = crate::BitReader<RBNEIE_A>;
impl RBNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBNEIE_A {
        match self.bits {
            false => RBNEIE_A::DISABLED,
            true => RBNEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RBNEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RBNEIE_A::ENABLED
    }
}
#[doc = "Field `RBNEIE` writer - RXNE interrupt enable"]
pub type RBNEIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, RBNEIE_A, 5>;
impl<'a> RBNEIE_W<'a> {
    #[doc = "Read data buffer not empty and overrrun error interrupts are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RBNEIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever ORERR=1 or RBNE=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RBNEIE_A::ENABLED)
    }
}
#[doc = "IDLE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEIE_A {
    #[doc = "0: Idle line detected interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever IDLEF=1 in the STAT register"]
    ENABLED = 1,
}
impl From<IDLEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable"]
pub type IDLEIE_R = crate::BitReader<IDLEIE_A>;
impl IDLEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLEIE_A {
        match self.bits {
            false => IDLEIE_A::DISABLED,
            true => IDLEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIE_A::ENABLED
    }
}
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable"]
pub type IDLEIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, IDLEIE_A, 4>;
impl<'a> IDLEIE_W<'a> {
    #[doc = "Idle line detected interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever IDLEF=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::ENABLED)
    }
}
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN_A {
    #[doc = "0: Transmitter is disabled"]
    DISABLED = 0,
    #[doc = "1: Transmitter is enabled"]
    ENABLED = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader<TEN_A>;
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::DISABLED,
            true => TEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN_A::ENABLED
    }
}
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, TEN_A, 3>;
impl<'a> TEN_W<'a> {
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN_A::DISABLED)
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN_A::ENABLED)
    }
}
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REN_A {
    #[doc = "0: Receiver is disabled"]
    DISABLED = 0,
    #[doc = "1: Receiver is enabled"]
    ENABLED = 1,
}
impl From<REN_A> for bool {
    #[inline(always)]
    fn from(variant: REN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader<REN_A>;
impl REN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REN_A {
        match self.bits {
            false => REN_A::DISABLED,
            true => REN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REN_A::ENABLED
    }
}
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, REN_A, 2>;
impl<'a> REN_W<'a> {
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REN_A::DISABLED)
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REN_A::ENABLED)
    }
}
#[doc = "USART enable in Stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UESM_A {
    #[doc = "0: USART not able to wake the MCU from deep-sleep mode"]
    DISABLED = 0,
    #[doc = "1: USART is able to wake the MCU from deep-sleep mode, as long as the clock source for the USART is IRC8M or LXTAL"]
    ENABLED = 1,
}
impl From<UESM_A> for bool {
    #[inline(always)]
    fn from(variant: UESM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UESM` reader - USART enable in Stop mode"]
pub type UESM_R = crate::BitReader<UESM_A>;
impl UESM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UESM_A {
        match self.bits {
            false => UESM_A::DISABLED,
            true => UESM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UESM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UESM_A::ENABLED
    }
}
#[doc = "Field `UESM` writer - USART enable in Stop mode"]
pub type UESM_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, UESM_A, 1>;
impl<'a> UESM_W<'a> {
    #[doc = "USART not able to wake the MCU from deep-sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UESM_A::DISABLED)
    }
    #[doc = "USART is able to wake the MCU from deep-sleep mode, as long as the clock source for the USART is IRC8M or LXTAL"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UESM_A::ENABLED)
    }
}
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UEN_A {
    #[doc = "0: USART prescaler and outputs disabled"]
    DISABLED = 0,
    #[doc = "1: USART prescaler and outputs is enabled"]
    ENABLED = 1,
}
impl From<UEN_A> for bool {
    #[inline(always)]
    fn from(variant: UEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UEN` reader - USART enable"]
pub type UEN_R = crate::BitReader<UEN_A>;
impl UEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UEN_A {
        match self.bits {
            false => UEN_A::DISABLED,
            true => UEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UEN_A::ENABLED
    }
}
#[doc = "Field `UEN` writer - USART enable"]
pub type UEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, UEN_A, 0>;
impl<'a> UEN_W<'a> {
    #[doc = "USART prescaler and outputs disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UEN_A::DISABLED)
    }
    #[doc = "USART prescaler and outputs is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    pub fn ebie(&self) -> EBIE_R {
        EBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn dea(&self) -> DEA_R {
        DEA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline(always)]
    pub fn ded(&self) -> DED_R {
        DED_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn ovsmod(&self) -> OVSMOD_R {
        OVSMOD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn amie(&self) -> AMIE_R {
        AMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn men(&self) -> MEN_R {
        MEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn wl(&self) -> WL_R {
        WL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UEN_R {
        UEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    pub fn ebie(&mut self) -> EBIE_W {
        EBIE_W::new(self)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    pub fn rtie(&mut self) -> RTIE_W {
        RTIE_W::new(self)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn dea(&mut self) -> DEA_W {
        DEA_W::new(self)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline(always)]
    pub fn ded(&mut self) -> DED_W {
        DED_W::new(self)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn ovsmod(&mut self) -> OVSMOD_W {
        OVSMOD_W::new(self)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn amie(&mut self) -> AMIE_W {
        AMIE_W::new(self)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn men(&mut self) -> MEN_W {
        MEN_W::new(self)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn wl(&mut self) -> WL_W {
        WL_W::new(self)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wm(&mut self) -> WM_W {
        WM_W::new(self)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W::new(self)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W::new(self)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn perrie(&mut self) -> PERRIE_W {
        PERRIE_W::new(self)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&mut self) -> TBEIE_W {
        TBEIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W::new(self)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&mut self) -> RBNEIE_W {
        RBNEIE_W::new(self)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W {
        IDLEIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W {
        REN_W::new(self)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W {
        UESM_W::new(self)
    }
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn uen(&mut self) -> UEN_W {
        UEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
