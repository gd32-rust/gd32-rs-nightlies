#[doc = "Register `CS` reader"]
pub struct R(crate::R<CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS` writer"]
pub struct W(crate::W<CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS_SPEC>;
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
impl From<crate::W<CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CMP0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0EN_A {
    #[doc = "0: Comparator disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator enabled"]
    ENABLED = 1,
}
impl From<CMP0EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0EN` reader - CMP0 enable"]
pub type CMP0EN_R = crate::BitReader<CMP0EN_A>;
impl CMP0EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0EN_A {
        match self.bits {
            false => CMP0EN_A::DISABLED,
            true => CMP0EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP0EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP0EN_A::ENABLED
    }
}
#[doc = "Field `CMP0EN` writer - CMP0 enable"]
pub type CMP0EN_W<'a> = crate::BitWriter<'a, u32, CS_SPEC, CMP0EN_A, 0>;
impl<'a> CMP0EN_W<'a> {
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP0EN_A::DISABLED)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP0EN_A::ENABLED)
    }
}
#[doc = "CMP0 switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0SW_A {
    #[doc = "0: Switch open"]
    OPEN = 0,
    #[doc = "1: Switch closed"]
    CLOSED = 1,
}
impl From<CMP0SW_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0SW` reader - CMP0 switch"]
pub type CMP0SW_R = crate::BitReader<CMP0SW_A>;
impl CMP0SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0SW_A {
        match self.bits {
            false => CMP0SW_A::OPEN,
            true => CMP0SW_A::CLOSED,
        }
    }
    #[doc = "Checks if the value of the field is `OPEN`"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == CMP0SW_A::OPEN
    }
    #[doc = "Checks if the value of the field is `CLOSED`"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == CMP0SW_A::CLOSED
    }
}
#[doc = "Field `CMP0SW` writer - CMP0 switch"]
pub type CMP0SW_W<'a> = crate::BitWriter<'a, u32, CS_SPEC, CMP0SW_A, 1>;
impl<'a> CMP0SW_W<'a> {
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut W {
        self.variant(CMP0SW_A::OPEN)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut W {
        self.variant(CMP0SW_A::CLOSED)
    }
}
#[doc = "CMP0 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP0M_A {
    #[doc = "0: High speed / full power"]
    HIGHSPEED = 0,
    #[doc = "1: Medium speed / medium power"]
    MEDIUMSPEED = 1,
    #[doc = "2: Low speed / low power"]
    LOWSPEED = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VERYLOWSPEED = 3,
}
impl From<CMP0M_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0M_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP0M` reader - CMP0 mode"]
pub type CMP0M_R = crate::FieldReader<u8, CMP0M_A>;
impl CMP0M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0M_A {
        match self.bits {
            0 => CMP0M_A::HIGHSPEED,
            1 => CMP0M_A::MEDIUMSPEED,
            2 => CMP0M_A::LOWSPEED,
            3 => CMP0M_A::VERYLOWSPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == CMP0M_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `MEDIUMSPEED`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == CMP0M_A::MEDIUMSPEED
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == CMP0M_A::LOWSPEED
    }
    #[doc = "Checks if the value of the field is `VERYLOWSPEED`"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == CMP0M_A::VERYLOWSPEED
    }
}
#[doc = "Field `CMP0M` writer - CMP0 mode"]
pub type CMP0M_W<'a> = crate::FieldWriterSafe<'a, u32, CS_SPEC, u8, CMP0M_A, 2, 2>;
impl<'a> CMP0M_W<'a> {
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(CMP0M_A::HIGHSPEED)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(CMP0M_A::MEDIUMSPEED)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(CMP0M_A::LOWSPEED)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut W {
        self.variant(CMP0M_A::VERYLOWSPEED)
    }
}
#[doc = "CMP0_M input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP0MSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    ONEQUARTERVREF = 0,
    #[doc = "1: 1/2 of VRefint"]
    ONEHALFVREF = 1,
    #[doc = "2: 3/4 of VRefint"]
    THREEQUARTERVREF = 2,
    #[doc = "3: VRefint"]
    VREF = 3,
    #[doc = "4: PA4 (DAC0"]
    PA4 = 4,
    #[doc = "5: PA5"]
    PA5 = 5,
    #[doc = "6: PA0"]
    PA0 = 6,
}
impl From<CMP0MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0MSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP0MSEL` reader - CMP0_M input selection"]
pub type CMP0MSEL_R = crate::FieldReader<u8, CMP0MSEL_A>;
impl CMP0MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMP0MSEL_A> {
        match self.bits {
            0 => Some(CMP0MSEL_A::ONEQUARTERVREF),
            1 => Some(CMP0MSEL_A::ONEHALFVREF),
            2 => Some(CMP0MSEL_A::THREEQUARTERVREF),
            3 => Some(CMP0MSEL_A::VREF),
            4 => Some(CMP0MSEL_A::PA4),
            5 => Some(CMP0MSEL_A::PA5),
            6 => Some(CMP0MSEL_A::PA0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == CMP0MSEL_A::ONEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `ONEHALFVREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == CMP0MSEL_A::ONEHALFVREF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == CMP0MSEL_A::THREEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == CMP0MSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `PA4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == CMP0MSEL_A::PA4
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == CMP0MSEL_A::PA5
    }
    #[doc = "Checks if the value of the field is `PA0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == CMP0MSEL_A::PA0
    }
}
#[doc = "Field `CMP0MSEL` writer - CMP0_M input selection"]
pub type CMP0MSEL_W<'a> = crate::FieldWriter<'a, u32, CS_SPEC, u8, CMP0MSEL_A, 3, 4>;
impl<'a> CMP0MSEL_W<'a> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(CMP0MSEL_A::ONEQUARTERVREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(CMP0MSEL_A::ONEHALFVREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(CMP0MSEL_A::THREEQUARTERVREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(CMP0MSEL_A::VREF)
    }
    #[doc = "PA4 (DAC0"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(CMP0MSEL_A::PA4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(CMP0MSEL_A::PA5)
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(CMP0MSEL_A::PA0)
    }
}
#[doc = "Comparator 0 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP0OSEL_A {
    #[doc = "0: No selection"]
    NOSELECTION = 0,
    #[doc = "1: Timer 0 break input"]
    TIMER0BREAKINPUT = 1,
    #[doc = "2: Timer 0 Input capture 0"]
    TIMER0INPUTCAPTURE0 = 2,
    #[doc = "3: Timer 0 OCPRE_CLR input"]
    TIMER0OCPRECLEARINPUT = 3,
    #[doc = "4: Timer 1 input capture 3"]
    TIMER1INPUTCAPTURE3 = 4,
    #[doc = "5: Timer 1 OCPRE_CLR input"]
    TIMER1OCPRECLEARINPUT = 5,
    #[doc = "6: Timer 2 input capture 0"]
    TIMER2INPUTCAPTURE0 = 6,
    #[doc = "7: Timer 2 OCPRE_CLR input"]
    TIMER2OCPRECLEARINPUT = 7,
}
impl From<CMP0OSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0OSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP0OSEL` reader - Comparator 0 output selection"]
pub type CMP0OSEL_R = crate::FieldReader<u8, CMP0OSEL_A>;
impl CMP0OSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0OSEL_A {
        match self.bits {
            0 => CMP0OSEL_A::NOSELECTION,
            1 => CMP0OSEL_A::TIMER0BREAKINPUT,
            2 => CMP0OSEL_A::TIMER0INPUTCAPTURE0,
            3 => CMP0OSEL_A::TIMER0OCPRECLEARINPUT,
            4 => CMP0OSEL_A::TIMER1INPUTCAPTURE3,
            5 => CMP0OSEL_A::TIMER1OCPRECLEARINPUT,
            6 => CMP0OSEL_A::TIMER2INPUTCAPTURE0,
            7 => CMP0OSEL_A::TIMER2OCPRECLEARINPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOSELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == CMP0OSEL_A::NOSELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER0BREAKINPUT`"]
    #[inline(always)]
    pub fn is_timer0break_input(&self) -> bool {
        *self == CMP0OSEL_A::TIMER0BREAKINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER0INPUTCAPTURE0`"]
    #[inline(always)]
    pub fn is_timer0input_capture0(&self) -> bool {
        *self == CMP0OSEL_A::TIMER0INPUTCAPTURE0
    }
    #[doc = "Checks if the value of the field is `TIMER0OCPRECLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer0ocpreclear_input(&self) -> bool {
        *self == CMP0OSEL_A::TIMER0OCPRECLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1INPUTCAPTURE3`"]
    #[inline(always)]
    pub fn is_timer1input_capture3(&self) -> bool {
        *self == CMP0OSEL_A::TIMER1INPUTCAPTURE3
    }
    #[doc = "Checks if the value of the field is `TIMER1OCPRECLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer1ocpreclear_input(&self) -> bool {
        *self == CMP0OSEL_A::TIMER1OCPRECLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER2INPUTCAPTURE0`"]
    #[inline(always)]
    pub fn is_timer2input_capture0(&self) -> bool {
        *self == CMP0OSEL_A::TIMER2INPUTCAPTURE0
    }
    #[doc = "Checks if the value of the field is `TIMER2OCPRECLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer2ocpreclear_input(&self) -> bool {
        *self == CMP0OSEL_A::TIMER2OCPRECLEARINPUT
    }
}
#[doc = "Field `CMP0OSEL` writer - Comparator 0 output selection"]
pub type CMP0OSEL_W<'a> = crate::FieldWriterSafe<'a, u32, CS_SPEC, u8, CMP0OSEL_A, 3, 8>;
impl<'a> CMP0OSEL_W<'a> {
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(CMP0OSEL_A::NOSELECTION)
    }
    #[doc = "Timer 0 break input"]
    #[inline(always)]
    pub fn timer0break_input(self) -> &'a mut W {
        self.variant(CMP0OSEL_A::TIMER0BREAKINPUT)
    }
    #[doc = "Timer 0 Input capture 0"]
    #[inline(always)]
    pub fn timer0input_capture0(self) -> &'a mut W {
        self.variant(CMP0OSEL_A::TIMER0INPUTCAPTURE0)
    }
    #[doc = "Timer 0 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer0ocpreclear_input(self) -> &'a mut W {
        self.variant(CMP0OSEL_A::TIMER0OCPRECLEARINPUT)
    }
    #[doc = "Timer 1 input capture 3"]
    #[inline(always)]
    pub fn timer1input_capture3(self) -> &'a mut W {
        self.variant(CMP0OSEL_A::TIMER1INPUTCAPTURE3)
    }
    #[doc = "Timer 1 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer1ocpreclear_input(self) -> &'a mut W {
        self.variant(CMP0OSEL_A::TIMER1OCPRECLEARINPUT)
    }
    #[doc = "Timer 2 input capture 0"]
    #[inline(always)]
    pub fn timer2input_capture0(self) -> &'a mut W {
        self.variant(CMP0OSEL_A::TIMER2INPUTCAPTURE0)
    }
    #[doc = "Timer 2 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer2ocpreclear_input(self) -> &'a mut W {
        self.variant(CMP0OSEL_A::TIMER2OCPRECLEARINPUT)
    }
}
#[doc = "Polarity of CMP0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0PL_A {
    #[doc = "0: Output is not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<CMP0PL_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0PL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0PL` reader - Polarity of CMP0 output"]
pub type CMP0PL_R = crate::BitReader<CMP0PL_A>;
impl CMP0PL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0PL_A {
        match self.bits {
            false => CMP0PL_A::NOTINVERTED,
            true => CMP0PL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == CMP0PL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CMP0PL_A::INVERTED
    }
}
#[doc = "Field `CMP0PL` writer - Polarity of CMP0 output"]
pub type CMP0PL_W<'a> = crate::BitWriter<'a, u32, CS_SPEC, CMP0PL_A, 11>;
impl<'a> CMP0PL_W<'a> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(CMP0PL_A::NOTINVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CMP0PL_A::INVERTED)
    }
}
#[doc = "CMP0 hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP0HST_A {
    #[doc = "0: No hysteresis"]
    NOHYSTERESIS = 0,
    #[doc = "1: Low hysteresis"]
    LOWHYSTERESIS = 1,
    #[doc = "2: Medium hysteresis"]
    MEDIUMHYSTERESIS = 2,
    #[doc = "3: High hysteresis"]
    HIGHHYSTERESIS = 3,
}
impl From<CMP0HST_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0HST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP0HST` reader - CMP0 hysteresis"]
pub type CMP0HST_R = crate::FieldReader<u8, CMP0HST_A>;
impl CMP0HST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0HST_A {
        match self.bits {
            0 => CMP0HST_A::NOHYSTERESIS,
            1 => CMP0HST_A::LOWHYSTERESIS,
            2 => CMP0HST_A::MEDIUMHYSTERESIS,
            3 => CMP0HST_A::HIGHHYSTERESIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOHYSTERESIS`"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == CMP0HST_A::NOHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `LOWHYSTERESIS`"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == CMP0HST_A::LOWHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `MEDIUMHYSTERESIS`"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == CMP0HST_A::MEDIUMHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `HIGHHYSTERESIS`"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == CMP0HST_A::HIGHHYSTERESIS
    }
}
#[doc = "Field `CMP0HST` writer - CMP0 hysteresis"]
pub type CMP0HST_W<'a> = crate::FieldWriterSafe<'a, u32, CS_SPEC, u8, CMP0HST_A, 2, 12>;
impl<'a> CMP0HST_W<'a> {
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut W {
        self.variant(CMP0HST_A::NOHYSTERESIS)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut W {
        self.variant(CMP0HST_A::LOWHYSTERESIS)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut W {
        self.variant(CMP0HST_A::MEDIUMHYSTERESIS)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut W {
        self.variant(CMP0HST_A::HIGHHYSTERESIS)
    }
}
#[doc = "CMP0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0O_A {
    #[doc = "0: Non-inverting input below inverting input"]
    LOW = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    HIGH = 1,
}
impl From<CMP0O_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0O` reader - CMP0 output"]
pub type CMP0O_R = crate::BitReader<CMP0O_A>;
impl CMP0O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0O_A {
        match self.bits {
            false => CMP0O_A::LOW,
            true => CMP0O_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CMP0O_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CMP0O_A::HIGH
    }
}
#[doc = "CMP0 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0LK_A {
    #[doc = "0: Control bits are read-write"]
    READWRITE = 0,
    #[doc = "1: Control bits are read-only"]
    READONLY = 1,
}
impl From<CMP0LK_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0LK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0LK` reader - CMP0 lock"]
pub type CMP0LK_R = crate::BitReader<CMP0LK_A>;
impl CMP0LK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0LK_A {
        match self.bits {
            false => CMP0LK_A::READWRITE,
            true => CMP0LK_A::READONLY,
        }
    }
    #[doc = "Checks if the value of the field is `READWRITE`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == CMP0LK_A::READWRITE
    }
    #[doc = "Checks if the value of the field is `READONLY`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == CMP0LK_A::READONLY
    }
}
#[doc = "Field `CMP0LK` writer - CMP0 lock"]
pub type CMP0LK_W<'a> = crate::BitWriter<'a, u32, CS_SPEC, CMP0LK_A, 15>;
impl<'a> CMP0LK_W<'a> {
    #[doc = "Control bits are read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut W {
        self.variant(CMP0LK_A::READWRITE)
    }
    #[doc = "Control bits are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut W {
        self.variant(CMP0LK_A::READONLY)
    }
}
#[doc = "CMP1 enable"]
pub use CMP0EN_A as CMP1EN_A;
#[doc = "Field `CMP1EN` reader - CMP1 enable"]
pub use CMP0EN_R as CMP1EN_R;
#[doc = "Field `CMP1EN` writer - CMP1 enable"]
pub use CMP0EN_W as CMP1EN_W;
#[doc = "CMP1_M input selection"]
pub use CMP0MSEL_A as CMP1MSEL_A;
#[doc = "Field `CMP1MSEL` reader - CMP1_M input selection"]
pub use CMP0MSEL_R as CMP1MSEL_R;
#[doc = "Field `CMP1MSEL` writer - CMP1_M input selection"]
pub use CMP0MSEL_W as CMP1MSEL_W;
#[doc = "CMP1 mode"]
pub use CMP0M_A as CMP1M_A;
#[doc = "Field `CMP1M` reader - CMP1 mode"]
pub use CMP0M_R as CMP1M_R;
#[doc = "Field `CMP1M` writer - CMP1 mode"]
pub use CMP0M_W as CMP1M_W;
#[doc = "Window mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WNDEN_A {
    #[doc = "0: Window mode disabled"]
    DISABLED = 0,
    #[doc = "1: Window mode enabled"]
    ENABLED = 1,
}
impl From<WNDEN_A> for bool {
    #[inline(always)]
    fn from(variant: WNDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WNDEN` reader - Window mode enable"]
pub type WNDEN_R = crate::BitReader<WNDEN_A>;
impl WNDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WNDEN_A {
        match self.bits {
            false => WNDEN_A::DISABLED,
            true => WNDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WNDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WNDEN_A::ENABLED
    }
}
#[doc = "Field `WNDEN` writer - Window mode enable"]
pub type WNDEN_W<'a> = crate::BitWriter<'a, u32, CS_SPEC, WNDEN_A, 23>;
impl<'a> WNDEN_W<'a> {
    #[doc = "Window mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WNDEN_A::DISABLED)
    }
    #[doc = "Window mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WNDEN_A::ENABLED)
    }
}
#[doc = "CMP1 hysteresis"]
pub use CMP0HST_A as CMP1HST_A;
#[doc = "Field `CMP1HST` reader - CMP1 hysteresis"]
pub use CMP0HST_R as CMP1HST_R;
#[doc = "Field `CMP1HST` writer - CMP1 hysteresis"]
pub use CMP0HST_W as CMP1HST_W;
#[doc = "CMP1 lock"]
pub use CMP0LK_A as CMP1LK_A;
#[doc = "Field `CMP1LK` reader - CMP1 lock"]
pub use CMP0LK_R as CMP1LK_R;
#[doc = "Field `CMP1LK` writer - CMP1 lock"]
pub use CMP0LK_W as CMP1LK_W;
#[doc = "CMP1 output selection"]
pub use CMP0OSEL_A as CMP1OSEL_A;
#[doc = "Field `CMP1OSEL` reader - CMP1 output selection"]
pub use CMP0OSEL_R as CMP1OSEL_R;
#[doc = "Field `CMP1OSEL` writer - CMP1 output selection"]
pub use CMP0OSEL_W as CMP1OSEL_W;
#[doc = "CMP1 output"]
pub use CMP0O_A as CMP1O_A;
#[doc = "Field `CMP1O` reader - CMP1 output"]
pub use CMP0O_R as CMP1O_R;
#[doc = "Polarity of CMP1 output"]
pub use CMP0PL_A as CMP1PL_A;
#[doc = "Field `CMP1PL` reader - Polarity of CMP1 output"]
pub use CMP0PL_R as CMP1PL_R;
#[doc = "Field `CMP1PL` writer - Polarity of CMP1 output"]
pub use CMP0PL_W as CMP1PL_W;
impl R {
    #[doc = "Bit 0 - CMP0 enable"]
    #[inline(always)]
    pub fn cmp0en(&self) -> CMP0EN_R {
        CMP0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP0 switch"]
    #[inline(always)]
    pub fn cmp0sw(&self) -> CMP0SW_R {
        CMP0SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - CMP0 mode"]
    #[inline(always)]
    pub fn cmp0m(&self) -> CMP0M_R {
        CMP0M_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - CMP0_M input selection"]
    #[inline(always)]
    pub fn cmp0msel(&self) -> CMP0MSEL_R {
        CMP0MSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 0 output selection"]
    #[inline(always)]
    pub fn cmp0osel(&self) -> CMP0OSEL_R {
        CMP0OSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Polarity of CMP0 output"]
    #[inline(always)]
    pub fn cmp0pl(&self) -> CMP0PL_R {
        CMP0PL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - CMP0 hysteresis"]
    #[inline(always)]
    pub fn cmp0hst(&self) -> CMP0HST_R {
        CMP0HST_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - CMP0 output"]
    #[inline(always)]
    pub fn cmp0o(&self) -> CMP0O_R {
        CMP0O_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CMP0 lock"]
    #[inline(always)]
    pub fn cmp0lk(&self) -> CMP0LK_R {
        CMP0LK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CMP1 enable"]
    #[inline(always)]
    pub fn cmp1en(&self) -> CMP1EN_R {
        CMP1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - CMP1 mode"]
    #[inline(always)]
    pub fn cmp1m(&self) -> CMP1M_R {
        CMP1M_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - CMP1_M input selection"]
    #[inline(always)]
    pub fn cmp1msel(&self) -> CMP1MSEL_R {
        CMP1MSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wnden(&self) -> WNDEN_R {
        WNDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - CMP1 output selection"]
    #[inline(always)]
    pub fn cmp1osel(&self) -> CMP1OSEL_R {
        CMP1OSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Polarity of CMP1 output"]
    #[inline(always)]
    pub fn cmp1pl(&self) -> CMP1PL_R {
        CMP1PL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - CMP1 hysteresis"]
    #[inline(always)]
    pub fn cmp1hst(&self) -> CMP1HST_R {
        CMP1HST_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - CMP1 output"]
    #[inline(always)]
    pub fn cmp1o(&self) -> CMP1O_R {
        CMP1O_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CMP1 lock"]
    #[inline(always)]
    pub fn cmp1lk(&self) -> CMP1LK_R {
        CMP1LK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP0 enable"]
    #[inline(always)]
    pub fn cmp0en(&mut self) -> CMP0EN_W {
        CMP0EN_W::new(self)
    }
    #[doc = "Bit 1 - CMP0 switch"]
    #[inline(always)]
    pub fn cmp0sw(&mut self) -> CMP0SW_W {
        CMP0SW_W::new(self)
    }
    #[doc = "Bits 2:3 - CMP0 mode"]
    #[inline(always)]
    pub fn cmp0m(&mut self) -> CMP0M_W {
        CMP0M_W::new(self)
    }
    #[doc = "Bits 4:6 - CMP0_M input selection"]
    #[inline(always)]
    pub fn cmp0msel(&mut self) -> CMP0MSEL_W {
        CMP0MSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Comparator 0 output selection"]
    #[inline(always)]
    pub fn cmp0osel(&mut self) -> CMP0OSEL_W {
        CMP0OSEL_W::new(self)
    }
    #[doc = "Bit 11 - Polarity of CMP0 output"]
    #[inline(always)]
    pub fn cmp0pl(&mut self) -> CMP0PL_W {
        CMP0PL_W::new(self)
    }
    #[doc = "Bits 12:13 - CMP0 hysteresis"]
    #[inline(always)]
    pub fn cmp0hst(&mut self) -> CMP0HST_W {
        CMP0HST_W::new(self)
    }
    #[doc = "Bit 15 - CMP0 lock"]
    #[inline(always)]
    pub fn cmp0lk(&mut self) -> CMP0LK_W {
        CMP0LK_W::new(self)
    }
    #[doc = "Bit 16 - CMP1 enable"]
    #[inline(always)]
    pub fn cmp1en(&mut self) -> CMP1EN_W {
        CMP1EN_W::new(self)
    }
    #[doc = "Bits 18:19 - CMP1 mode"]
    #[inline(always)]
    pub fn cmp1m(&mut self) -> CMP1M_W {
        CMP1M_W::new(self)
    }
    #[doc = "Bits 20:22 - CMP1_M input selection"]
    #[inline(always)]
    pub fn cmp1msel(&mut self) -> CMP1MSEL_W {
        CMP1MSEL_W::new(self)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wnden(&mut self) -> WNDEN_W {
        WNDEN_W::new(self)
    }
    #[doc = "Bits 24:26 - CMP1 output selection"]
    #[inline(always)]
    pub fn cmp1osel(&mut self) -> CMP1OSEL_W {
        CMP1OSEL_W::new(self)
    }
    #[doc = "Bit 27 - Polarity of CMP1 output"]
    #[inline(always)]
    pub fn cmp1pl(&mut self) -> CMP1PL_W {
        CMP1PL_W::new(self)
    }
    #[doc = "Bits 28:29 - CMP1 hysteresis"]
    #[inline(always)]
    pub fn cmp1hst(&mut self) -> CMP1HST_W {
        CMP1HST_W::new(self)
    }
    #[doc = "Bit 31 - CMP1 lock"]
    #[inline(always)]
    pub fn cmp1lk(&mut self) -> CMP1LK_W {
        CMP1LK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](index.html) module"]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs::R](R) reader structure"]
impl crate::Readable for CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs::W](W) writer structure"]
impl crate::Writable for CS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
