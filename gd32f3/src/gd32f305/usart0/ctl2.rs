#[doc = "Register `CTL2` reader"]
pub struct R(crate::R<CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL2` writer"]
pub struct W(crate::W<CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL2_SPEC>;
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
impl From<crate::W<CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CTS interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: An interrupt is generated whenever CTS=1 in the STAT register"]
    ENABLED = 1,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader<CTSIE_A>;
impl CTSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::DISABLED,
            true => CTSIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIE_A::ENABLED
    }
}
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, CTSIE_A, 10>;
impl<'a> CTSIE_W<'a> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSIE_A::DISABLED)
    }
    #[doc = "An interrupt is generated whenever CTS=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSIE_A::ENABLED)
    }
}
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEN_A {
    #[doc = "0: CTS hardware flow control disabled"]
    DISABLED = 0,
    #[doc = "1: CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    ENABLED = 1,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - CTS enable"]
pub type CTSEN_R = crate::BitReader<CTSEN_A>;
impl CTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLED,
            true => CTSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSEN_A::ENABLED
    }
}
#[doc = "Field `CTSEN` writer - CTS enable"]
pub type CTSEN_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, CTSEN_A, 9>;
impl<'a> CTSEN_W<'a> {
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLED)
    }
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLED)
    }
}
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSEN_A {
    #[doc = "0: RTS hardware flow control disabled"]
    DISABLED = 0,
    #[doc = "1: RTS output enabled, data is only requested when there is space in the receive buffer"]
    ENABLED = 1,
}
impl From<RTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` reader - RTS enable"]
pub type RTSEN_R = crate::BitReader<RTSEN_A>;
impl RTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSEN_A {
        match self.bits {
            false => RTSEN_A::DISABLED,
            true => RTSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSEN_A::ENABLED
    }
}
#[doc = "Field `RTSEN` writer - RTS enable"]
pub type RTSEN_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, RTSEN_A, 8>;
impl<'a> RTSEN_W<'a> {
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTSEN_A::DISABLED)
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTSEN_A::ENABLED)
    }
}
#[doc = "DMA request enable for transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DENT_A {
    #[doc = "0: DMA mode is disabled for transmission"]
    DISABLED = 0,
    #[doc = "1: DMA mode is enabled for transmission"]
    ENABLED = 1,
}
impl From<DENT_A> for bool {
    #[inline(always)]
    fn from(variant: DENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DENT` reader - DMA request enable for transmission"]
pub type DENT_R = crate::BitReader<DENT_A>;
impl DENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DENT_A {
        match self.bits {
            false => DENT_A::DISABLED,
            true => DENT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DENT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DENT_A::ENABLED
    }
}
#[doc = "Field `DENT` writer - DMA request enable for transmission"]
pub type DENT_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, DENT_A, 7>;
impl<'a> DENT_W<'a> {
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DENT_A::DISABLED)
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DENT_A::ENABLED)
    }
}
#[doc = "DMA request enable for reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DENR_A {
    #[doc = "0: DMA mode is disabled for reception"]
    DISABLED = 0,
    #[doc = "1: DMA mode is enabled for reception"]
    ENABLED = 1,
}
impl From<DENR_A> for bool {
    #[inline(always)]
    fn from(variant: DENR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DENR` reader - DMA request enable for reception"]
pub type DENR_R = crate::BitReader<DENR_A>;
impl DENR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DENR_A {
        match self.bits {
            false => DENR_A::DISABLED,
            true => DENR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DENR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DENR_A::ENABLED
    }
}
#[doc = "Field `DENR` writer - DMA request enable for reception"]
pub type DENR_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, DENR_A, 6>;
impl<'a> DENR_W<'a> {
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DENR_A::DISABLED)
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DENR_A::ENABLED)
    }
}
#[doc = "Smartcard mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCEN_A {
    #[doc = "0: Smartcard Mode disabled"]
    DISABLED = 0,
    #[doc = "1: Smartcard Mode enabled"]
    ENABLED = 1,
}
impl From<SCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type SCEN_R = crate::BitReader<SCEN_A>;
impl SCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCEN_A {
        match self.bits {
            false => SCEN_A::DISABLED,
            true => SCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCEN_A::ENABLED
    }
}
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type SCEN_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, SCEN_A, 5>;
impl<'a> SCEN_W<'a> {
    #[doc = "Smartcard Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCEN_A::DISABLED)
    }
    #[doc = "Smartcard Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCEN_A::ENABLED)
    }
}
#[doc = "Smartcard NACK enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NKEN_A {
    #[doc = "0: NACK transmission in case of parity error is disabled"]
    DISABLED = 0,
    #[doc = "1: NACK transmission during parity error is enabled"]
    ENABLED = 1,
}
impl From<NKEN_A> for bool {
    #[inline(always)]
    fn from(variant: NKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NKEN` reader - Smartcard NACK enable"]
pub type NKEN_R = crate::BitReader<NKEN_A>;
impl NKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NKEN_A {
        match self.bits {
            false => NKEN_A::DISABLED,
            true => NKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NKEN_A::ENABLED
    }
}
#[doc = "Field `NKEN` writer - Smartcard NACK enable"]
pub type NKEN_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, NKEN_A, 4>;
impl<'a> NKEN_W<'a> {
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NKEN_A::DISABLED)
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NKEN_A::ENABLED)
    }
}
#[doc = "Half-duplex selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDEN_A {
    #[doc = "0: Half duplex mode is not selected"]
    NOTSELECTED = 0,
    #[doc = "1: Half duplex mode is selected"]
    SELECTED = 1,
}
impl From<HDEN_A> for bool {
    #[inline(always)]
    fn from(variant: HDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDEN` reader - Half-duplex selection"]
pub type HDEN_R = crate::BitReader<HDEN_A>;
impl HDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDEN_A {
        match self.bits {
            false => HDEN_A::NOTSELECTED,
            true => HDEN_A::SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == HDEN_A::NOTSELECTED
    }
    #[doc = "Checks if the value of the field is `SELECTED`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == HDEN_A::SELECTED
    }
}
#[doc = "Field `HDEN` writer - Half-duplex selection"]
pub type HDEN_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, HDEN_A, 3>;
impl<'a> HDEN_W<'a> {
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(HDEN_A::NOTSELECTED)
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(HDEN_A::SELECTED)
    }
}
#[doc = "IrDA low-power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRLP_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Low-power mode"]
    LOWPOWER = 1,
}
impl From<IRLP_A> for bool {
    #[inline(always)]
    fn from(variant: IRLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IRLP_R = crate::BitReader<IRLP_A>;
impl IRLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRLP_A {
        match self.bits {
            false => IRLP_A::NORMAL,
            true => IRLP_A::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRLP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == IRLP_A::LOWPOWER
    }
}
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IRLP_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, IRLP_A, 2>;
impl<'a> IRLP_W<'a> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IRLP_A::NORMAL)
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(IRLP_A::LOWPOWER)
    }
}
#[doc = "IrDA mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREN_A {
    #[doc = "0: IrDA disabled"]
    DISABLED = 0,
    #[doc = "1: IrDA enabled"]
    ENABLED = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IREN_R = crate::BitReader<IREN_A>;
impl IREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::DISABLED,
            true => IREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN_A::ENABLED
    }
}
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IREN_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, IREN_A, 1>;
impl<'a> IREN_W<'a> {
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IREN_A::DISABLED)
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IREN_A::ENABLED)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: An interrupt is generated when FERR=1 or ORERR=1 or NERR=1 in the STAT register"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, ERRIE_A, 0>;
impl<'a> ERRIE_W<'a> {
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "An interrupt is generated when FERR=1 or ORERR=1 or NERR=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA request enable for transmission"]
    #[inline(always)]
    pub fn dent(&self) -> DENT_R {
        DENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA request enable for reception"]
    #[inline(always)]
    pub fn denr(&self) -> DENR_R {
        DENR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nken(&self) -> NKEN_R {
        NKEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W {
        CTSIE_W::new(self)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W::new(self)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 7 - DMA request enable for transmission"]
    #[inline(always)]
    pub fn dent(&mut self) -> DENT_W {
        DENT_W::new(self)
    }
    #[doc = "Bit 6 - DMA request enable for reception"]
    #[inline(always)]
    pub fn denr(&mut self) -> DENR_W {
        DENR_W::new(self)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W {
        SCEN_W::new(self)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nken(&mut self) -> NKEN_W {
        NKEN_W::new(self)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hden(&mut self) -> HDEN_W {
        HDEN_W::new(self)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W {
        IRLP_W::new(self)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W::new(self)
    }
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl2](index.html) module"]
pub struct CTL2_SPEC;
impl crate::RegisterSpec for CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl2::R](R) reader structure"]
impl crate::Readable for CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl2::W](W) writer structure"]
impl crate::Writable for CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
