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
#[doc = "Wakeup from Deep-sleep mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUIE_A {
    #[doc = "0: Wake-up from deep-sleep mode interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Wake-up from deep-sleep mode interrupt is generated whenever WUF=1 in the STAT register"]
    ENABLED = 1,
}
impl From<WUIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUIE` reader - Wakeup from Deep-sleep mode interrupt enable"]
pub type WUIE_R = crate::BitReader<WUIE_A>;
impl WUIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUIE_A {
        match self.bits {
            false => WUIE_A::DISABLED,
            true => WUIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUIE_A::ENABLED
    }
}
#[doc = "Field `WUIE` writer - Wakeup from Deep-sleep mode interrupt enable"]
pub type WUIE_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, WUIE_A, 22>;
impl<'a> WUIE_W<'a> {
    #[doc = "Wake-up from deep-sleep mode interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUIE_A::DISABLED)
    }
    #[doc = "Wake-up from deep-sleep mode interrupt is generated whenever WUF=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUIE_A::ENABLED)
    }
}
#[doc = "Wakeup mode from Deep-sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUM_A {
    #[doc = "0: WUF active on address match"]
    ADDRESS = 0,
    #[doc = "2: WUF active on start bit detection"]
    START = 2,
    #[doc = "3: WUF active on RBNE"]
    RXNE = 3,
}
impl From<WUM_A> for u8 {
    #[inline(always)]
    fn from(variant: WUM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUM` reader - Wakeup mode from Deep-sleep mode"]
pub type WUM_R = crate::FieldReader<u8, WUM_A>;
impl WUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUM_A> {
        match self.bits {
            0 => Some(WUM_A::ADDRESS),
            2 => Some(WUM_A::START),
            3 => Some(WUM_A::RXNE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS`"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WUM_A::ADDRESS
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WUM_A::START
    }
    #[doc = "Checks if the value of the field is `RXNE`"]
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        *self == WUM_A::RXNE
    }
}
#[doc = "Field `WUM` writer - Wakeup mode from Deep-sleep mode"]
pub type WUM_W<'a> = crate::FieldWriter<'a, u32, CTL2_SPEC, u8, WUM_A, 2, 20>;
impl<'a> WUM_W<'a> {
    #[doc = "WUF active on address match"]
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WUM_A::ADDRESS)
    }
    #[doc = "WUF active on start bit detection"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(WUM_A::START)
    }
    #[doc = "WUF active on RBNE"]
    #[inline(always)]
    pub fn rxne(self) -> &'a mut W {
        self.variant(WUM_A::RXNE)
    }
}
#[doc = "Field `SCRTNUM` reader - Smartcard auto-retry number"]
pub type SCRTNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCRTNUM` writer - Smartcard auto-retry number"]
pub type SCRTNUM_W<'a> = crate::FieldWriterSafe<'a, u32, CTL2_SPEC, u8, u8, 3, 17>;
#[doc = "Driver enable polarity mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEP_A {
    #[doc = "0: DE signal is active high"]
    HIGH = 0,
    #[doc = "1: DE signal is active low"]
    LOW = 1,
}
impl From<DEP_A> for bool {
    #[inline(always)]
    fn from(variant: DEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEP` reader - Driver enable polarity mode"]
pub type DEP_R = crate::BitReader<DEP_A>;
impl DEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEP_A {
        match self.bits {
            false => DEP_A::HIGH,
            true => DEP_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEP_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEP_A::LOW
    }
}
#[doc = "Field `DEP` writer - Driver enable polarity mode"]
pub type DEP_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, DEP_A, 15>;
impl<'a> DEP_W<'a> {
    #[doc = "DE signal is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DEP_A::HIGH)
    }
    #[doc = "DE signal is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DEP_A::LOW)
    }
}
#[doc = "Driver enable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEM_A {
    #[doc = "0: DE function is disabled"]
    DISABLED = 0,
    #[doc = "1: The DE signal is output on the RTS pin"]
    ENABLED = 1,
}
impl From<DEM_A> for bool {
    #[inline(always)]
    fn from(variant: DEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEM` reader - Driver enable mode"]
pub type DEM_R = crate::BitReader<DEM_A>;
impl DEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEM_A {
        match self.bits {
            false => DEM_A::DISABLED,
            true => DEM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEM_A::ENABLED
    }
}
#[doc = "Field `DEM` writer - Driver enable mode"]
pub type DEM_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, DEM_A, 14>;
impl<'a> DEM_W<'a> {
    #[doc = "DE function is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEM_A::DISABLED)
    }
    #[doc = "The DE signal is output on the RTS pin"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEM_A::ENABLED)
    }
}
#[doc = "Disable DMA on reception error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRE_A {
    #[doc = "0: DMA is not disabled in case of reception error"]
    NOTDISABLED = 0,
    #[doc = "1: DMA is disabled following a reception error"]
    DISABLED = 1,
}
impl From<DDRE_A> for bool {
    #[inline(always)]
    fn from(variant: DDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRE` reader - Disable DMA on reception error"]
pub type DDRE_R = crate::BitReader<DDRE_A>;
impl DDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRE_A {
        match self.bits {
            false => DDRE_A::NOTDISABLED,
            true => DDRE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDISABLED`"]
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        *self == DDRE_A::NOTDISABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDRE_A::DISABLED
    }
}
#[doc = "Field `DDRE` writer - Disable DMA on reception error"]
pub type DDRE_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, DDRE_A, 13>;
impl<'a> DDRE_W<'a> {
    #[doc = "DMA is not disabled in case of reception error"]
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut W {
        self.variant(DDRE_A::NOTDISABLED)
    }
    #[doc = "DMA is disabled following a reception error"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDRE_A::DISABLED)
    }
}
#[doc = "Overrun Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRD_A {
    #[doc = "0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    ENABLED = 0,
    #[doc = "1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDATA register"]
    DISABLED = 1,
}
impl From<OVRD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRD` reader - Overrun Disable"]
pub type OVRD_R = crate::BitReader<OVRD_A>;
impl OVRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRD_A {
        match self.bits {
            false => OVRD_A::ENABLED,
            true => OVRD_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRD_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRD_A::DISABLED
    }
}
#[doc = "Field `OVRD` writer - Overrun Disable"]
pub type OVRD_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, OVRD_A, 12>;
impl<'a> OVRD_W<'a> {
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRD_A::ENABLED)
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDATA register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRD_A::DISABLED)
    }
}
#[doc = "One sample bit method\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSB_A {
    #[doc = "0: Three sample bit method"]
    SAMPLE3 = 0,
    #[doc = "1: One sample bit method"]
    SAMPLE1 = 1,
}
impl From<OSB_A> for bool {
    #[inline(always)]
    fn from(variant: OSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSB` reader - One sample bit method"]
pub type OSB_R = crate::BitReader<OSB_A>;
impl OSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSB_A {
        match self.bits {
            false => OSB_A::SAMPLE3,
            true => OSB_A::SAMPLE1,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE3`"]
    #[inline(always)]
    pub fn is_sample3(&self) -> bool {
        *self == OSB_A::SAMPLE3
    }
    #[doc = "Checks if the value of the field is `SAMPLE1`"]
    #[inline(always)]
    pub fn is_sample1(&self) -> bool {
        *self == OSB_A::SAMPLE1
    }
}
#[doc = "Field `OSB` writer - One sample bit method"]
pub type OSB_W<'a> = crate::BitWriter<'a, u32, CTL2_SPEC, OSB_A, 11>;
impl<'a> OSB_W<'a> {
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn sample3(self) -> &'a mut W {
        self.variant(OSB_A::SAMPLE3)
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn sample1(self) -> &'a mut W {
        self.variant(OSB_A::SAMPLE1)
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
#[doc = "DMA enable transmitter\n\nValue on reset: 0"]
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
#[doc = "Field `DENT` reader - DMA enable transmitter"]
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
#[doc = "Field `DENT` writer - DMA enable transmitter"]
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
#[doc = "DMA enable for reception\n\nValue on reset: 0"]
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
#[doc = "Field `DENR` reader - DMA enable for reception"]
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
#[doc = "Field `DENR` writer - DMA enable for reception"]
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
#[doc = "NKEN enable in Smartcard mode\n\nValue on reset: 0"]
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
#[doc = "Field `NKEN` reader - NKEN enable in Smartcard mode"]
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
#[doc = "Field `NKEN` writer - NKEN enable in Smartcard mode"]
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
    #[doc = "Bit 22 - Wakeup from Deep-sleep mode interrupt enable"]
    #[inline(always)]
    pub fn wuie(&self) -> WUIE_R {
        WUIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Wakeup mode from Deep-sleep mode"]
    #[inline(always)]
    pub fn wum(&self) -> WUM_R {
        WUM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry number"]
    #[inline(always)]
    pub fn scrtnum(&self) -> SCRTNUM_R {
        SCRTNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 15 - Driver enable polarity mode"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable DMA on reception error"]
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrd(&self) -> OVRD_R {
        OVRD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method"]
    #[inline(always)]
    pub fn osb(&self) -> OSB_R {
        OSB_R::new(((self.bits >> 11) & 1) != 0)
    }
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
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dent(&self) -> DENT_R {
        DENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable for reception"]
    #[inline(always)]
    pub fn denr(&self) -> DENR_R {
        DENR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - NKEN enable in Smartcard mode"]
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
    #[doc = "Bit 22 - Wakeup from Deep-sleep mode interrupt enable"]
    #[inline(always)]
    pub fn wuie(&mut self) -> WUIE_W {
        WUIE_W::new(self)
    }
    #[doc = "Bits 20:21 - Wakeup mode from Deep-sleep mode"]
    #[inline(always)]
    pub fn wum(&mut self) -> WUM_W {
        WUM_W::new(self)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry number"]
    #[inline(always)]
    pub fn scrtnum(&mut self) -> SCRTNUM_W {
        SCRTNUM_W::new(self)
    }
    #[doc = "Bit 15 - Driver enable polarity mode"]
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W {
        DEP_W::new(self)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W {
        DEM_W::new(self)
    }
    #[doc = "Bit 13 - Disable DMA on reception error"]
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W {
        DDRE_W::new(self)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrd(&mut self) -> OVRD_W {
        OVRD_W::new(self)
    }
    #[doc = "Bit 11 - One sample bit method"]
    #[inline(always)]
    pub fn osb(&mut self) -> OSB_W {
        OSB_W::new(self)
    }
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
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dent(&mut self) -> DENT_W {
        DENT_W::new(self)
    }
    #[doc = "Bit 6 - DMA enable for reception"]
    #[inline(always)]
    pub fn denr(&mut self) -> DENR_W {
        DENR_W::new(self)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W {
        SCEN_W::new(self)
    }
    #[doc = "Bit 4 - NKEN enable in Smartcard mode"]
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
