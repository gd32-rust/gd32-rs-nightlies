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
#[doc = "Comparator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPEN_A {
    #[doc = "0: Comparator disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator enabled"]
    ENABLED = 1,
}
impl From<CMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEN` reader - Comparator enable"]
pub type CMPEN_R = crate::BitReader<CMPEN_A>;
impl CMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN_A {
        match self.bits {
            false => CMPEN_A::DISABLED,
            true => CMPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPEN_A::ENABLED
    }
}
#[doc = "Field `CMPEN` writer - Comparator enable"]
pub type CMPEN_W<'a> = crate::BitWriter<'a, u32, CS_SPEC, CMPEN_A, 0>;
impl<'a> CMPEN_W<'a> {
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMPEN_A::DISABLED)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMPEN_A::ENABLED)
    }
}
#[doc = "Comparator switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPSW_A {
    #[doc = "0: Switch open"]
    OPEN = 0,
    #[doc = "1: Switch closed"]
    CLOSED = 1,
}
impl From<CMPSW_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSW` reader - Comparator switch"]
pub type CMPSW_R = crate::BitReader<CMPSW_A>;
impl CMPSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSW_A {
        match self.bits {
            false => CMPSW_A::OPEN,
            true => CMPSW_A::CLOSED,
        }
    }
    #[doc = "Checks if the value of the field is `OPEN`"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == CMPSW_A::OPEN
    }
    #[doc = "Checks if the value of the field is `CLOSED`"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == CMPSW_A::CLOSED
    }
}
#[doc = "Field `CMPSW` writer - Comparator switch"]
pub type CMPSW_W<'a> = crate::BitWriter<'a, u32, CS_SPEC, CMPSW_A, 1>;
impl<'a> CMPSW_W<'a> {
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut W {
        self.variant(CMPSW_A::OPEN)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut W {
        self.variant(CMPSW_A::CLOSED)
    }
}
#[doc = "Comparator mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPM_A {
    #[doc = "0: High speed / full power"]
    HIGHSPEED = 0,
    #[doc = "1: Medium speed / medium power"]
    MEDIUMSPEED = 1,
    #[doc = "2: Low speed / low power"]
    LOWSPEED = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VERYLOWSPEED = 3,
}
impl From<CMPM_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPM` reader - Comparator mode"]
pub type CMPM_R = crate::FieldReader<u8, CMPM_A>;
impl CMPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPM_A {
        match self.bits {
            0 => CMPM_A::HIGHSPEED,
            1 => CMPM_A::MEDIUMSPEED,
            2 => CMPM_A::LOWSPEED,
            3 => CMPM_A::VERYLOWSPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == CMPM_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `MEDIUMSPEED`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == CMPM_A::MEDIUMSPEED
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == CMPM_A::LOWSPEED
    }
    #[doc = "Checks if the value of the field is `VERYLOWSPEED`"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == CMPM_A::VERYLOWSPEED
    }
}
#[doc = "Field `CMPM` writer - Comparator mode"]
pub type CMPM_W<'a> = crate::FieldWriterSafe<'a, u32, CS_SPEC, u8, CMPM_A, 2, 2>;
impl<'a> CMPM_W<'a> {
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(CMPM_A::HIGHSPEED)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(CMPM_A::MEDIUMSPEED)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(CMPM_A::LOWSPEED)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut W {
        self.variant(CMPM_A::VERYLOWSPEED)
    }
}
#[doc = "Comparator input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPMSEL_A {
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
impl From<CMPMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPMSEL` reader - Comparator input selection"]
pub type CMPMSEL_R = crate::FieldReader<u8, CMPMSEL_A>;
impl CMPMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPMSEL_A> {
        match self.bits {
            0 => Some(CMPMSEL_A::ONEQUARTERVREF),
            1 => Some(CMPMSEL_A::ONEHALFVREF),
            2 => Some(CMPMSEL_A::THREEQUARTERVREF),
            3 => Some(CMPMSEL_A::VREF),
            4 => Some(CMPMSEL_A::PA4),
            5 => Some(CMPMSEL_A::PA5),
            6 => Some(CMPMSEL_A::PA0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == CMPMSEL_A::ONEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `ONEHALFVREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == CMPMSEL_A::ONEHALFVREF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == CMPMSEL_A::THREEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == CMPMSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `PA4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == CMPMSEL_A::PA4
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == CMPMSEL_A::PA5
    }
    #[doc = "Checks if the value of the field is `PA0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == CMPMSEL_A::PA0
    }
}
#[doc = "Field `CMPMSEL` writer - Comparator input selection"]
pub type CMPMSEL_W<'a> = crate::FieldWriter<'a, u32, CS_SPEC, u8, CMPMSEL_A, 3, 4>;
impl<'a> CMPMSEL_W<'a> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(CMPMSEL_A::ONEQUARTERVREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(CMPMSEL_A::ONEHALFVREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(CMPMSEL_A::THREEQUARTERVREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(CMPMSEL_A::VREF)
    }
    #[doc = "PA4 (DAC0"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(CMPMSEL_A::PA4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(CMPMSEL_A::PA5)
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(CMPMSEL_A::PA0)
    }
}
#[doc = "Comparator output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPOSEL_A {
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
impl From<CMPOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPOSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPOSEL` reader - Comparator output selection"]
pub type CMPOSEL_R = crate::FieldReader<u8, CMPOSEL_A>;
impl CMPOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPOSEL_A {
        match self.bits {
            0 => CMPOSEL_A::NOSELECTION,
            1 => CMPOSEL_A::TIMER0BREAKINPUT,
            2 => CMPOSEL_A::TIMER0INPUTCAPTURE0,
            3 => CMPOSEL_A::TIMER0OCPRECLEARINPUT,
            4 => CMPOSEL_A::TIMER1INPUTCAPTURE3,
            5 => CMPOSEL_A::TIMER1OCPRECLEARINPUT,
            6 => CMPOSEL_A::TIMER2INPUTCAPTURE0,
            7 => CMPOSEL_A::TIMER2OCPRECLEARINPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOSELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == CMPOSEL_A::NOSELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER0BREAKINPUT`"]
    #[inline(always)]
    pub fn is_timer0break_input(&self) -> bool {
        *self == CMPOSEL_A::TIMER0BREAKINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER0INPUTCAPTURE0`"]
    #[inline(always)]
    pub fn is_timer0input_capture0(&self) -> bool {
        *self == CMPOSEL_A::TIMER0INPUTCAPTURE0
    }
    #[doc = "Checks if the value of the field is `TIMER0OCPRECLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer0ocpreclear_input(&self) -> bool {
        *self == CMPOSEL_A::TIMER0OCPRECLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1INPUTCAPTURE3`"]
    #[inline(always)]
    pub fn is_timer1input_capture3(&self) -> bool {
        *self == CMPOSEL_A::TIMER1INPUTCAPTURE3
    }
    #[doc = "Checks if the value of the field is `TIMER1OCPRECLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer1ocpreclear_input(&self) -> bool {
        *self == CMPOSEL_A::TIMER1OCPRECLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER2INPUTCAPTURE0`"]
    #[inline(always)]
    pub fn is_timer2input_capture0(&self) -> bool {
        *self == CMPOSEL_A::TIMER2INPUTCAPTURE0
    }
    #[doc = "Checks if the value of the field is `TIMER2OCPRECLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer2ocpreclear_input(&self) -> bool {
        *self == CMPOSEL_A::TIMER2OCPRECLEARINPUT
    }
}
#[doc = "Field `CMPOSEL` writer - Comparator output selection"]
pub type CMPOSEL_W<'a> = crate::FieldWriterSafe<'a, u32, CS_SPEC, u8, CMPOSEL_A, 3, 8>;
impl<'a> CMPOSEL_W<'a> {
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(CMPOSEL_A::NOSELECTION)
    }
    #[doc = "Timer 0 break input"]
    #[inline(always)]
    pub fn timer0break_input(self) -> &'a mut W {
        self.variant(CMPOSEL_A::TIMER0BREAKINPUT)
    }
    #[doc = "Timer 0 Input capture 0"]
    #[inline(always)]
    pub fn timer0input_capture0(self) -> &'a mut W {
        self.variant(CMPOSEL_A::TIMER0INPUTCAPTURE0)
    }
    #[doc = "Timer 0 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer0ocpreclear_input(self) -> &'a mut W {
        self.variant(CMPOSEL_A::TIMER0OCPRECLEARINPUT)
    }
    #[doc = "Timer 1 input capture 3"]
    #[inline(always)]
    pub fn timer1input_capture3(self) -> &'a mut W {
        self.variant(CMPOSEL_A::TIMER1INPUTCAPTURE3)
    }
    #[doc = "Timer 1 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer1ocpreclear_input(self) -> &'a mut W {
        self.variant(CMPOSEL_A::TIMER1OCPRECLEARINPUT)
    }
    #[doc = "Timer 2 input capture 0"]
    #[inline(always)]
    pub fn timer2input_capture0(self) -> &'a mut W {
        self.variant(CMPOSEL_A::TIMER2INPUTCAPTURE0)
    }
    #[doc = "Timer 2 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer2ocpreclear_input(self) -> &'a mut W {
        self.variant(CMPOSEL_A::TIMER2OCPRECLEARINPUT)
    }
}
#[doc = "Polarity of comparator output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPPL_A {
    #[doc = "0: Output is not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<CMPPL_A> for bool {
    #[inline(always)]
    fn from(variant: CMPPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPPL` reader - Polarity of comparator output"]
pub type CMPPL_R = crate::BitReader<CMPPL_A>;
impl CMPPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPPL_A {
        match self.bits {
            false => CMPPL_A::NOTINVERTED,
            true => CMPPL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == CMPPL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CMPPL_A::INVERTED
    }
}
#[doc = "Field `CMPPL` writer - Polarity of comparator output"]
pub type CMPPL_W<'a> = crate::BitWriter<'a, u32, CS_SPEC, CMPPL_A, 11>;
impl<'a> CMPPL_W<'a> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(CMPPL_A::NOTINVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CMPPL_A::INVERTED)
    }
}
#[doc = "Comparator hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPHST_A {
    #[doc = "0: No hysteresis"]
    NOHYSTERESIS = 0,
    #[doc = "1: Low hysteresis"]
    LOWHYSTERESIS = 1,
    #[doc = "2: Medium hysteresis"]
    MEDIUMHYSTERESIS = 2,
    #[doc = "3: High hysteresis"]
    HIGHHYSTERESIS = 3,
}
impl From<CMPHST_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPHST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPHST` reader - Comparator hysteresis"]
pub type CMPHST_R = crate::FieldReader<u8, CMPHST_A>;
impl CMPHST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPHST_A {
        match self.bits {
            0 => CMPHST_A::NOHYSTERESIS,
            1 => CMPHST_A::LOWHYSTERESIS,
            2 => CMPHST_A::MEDIUMHYSTERESIS,
            3 => CMPHST_A::HIGHHYSTERESIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOHYSTERESIS`"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == CMPHST_A::NOHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `LOWHYSTERESIS`"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == CMPHST_A::LOWHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `MEDIUMHYSTERESIS`"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == CMPHST_A::MEDIUMHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `HIGHHYSTERESIS`"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == CMPHST_A::HIGHHYSTERESIS
    }
}
#[doc = "Field `CMPHST` writer - Comparator hysteresis"]
pub type CMPHST_W<'a> = crate::FieldWriterSafe<'a, u32, CS_SPEC, u8, CMPHST_A, 2, 12>;
impl<'a> CMPHST_W<'a> {
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut W {
        self.variant(CMPHST_A::NOHYSTERESIS)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut W {
        self.variant(CMPHST_A::LOWHYSTERESIS)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut W {
        self.variant(CMPHST_A::MEDIUMHYSTERESIS)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut W {
        self.variant(CMPHST_A::HIGHHYSTERESIS)
    }
}
#[doc = "Comparator 0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPO_A {
    #[doc = "0: Non-inverting input below inverting input"]
    LOW = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    HIGH = 1,
}
impl From<CMPO_A> for bool {
    #[inline(always)]
    fn from(variant: CMPO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPO` reader - Comparator 0 output"]
pub type CMPO_R = crate::BitReader<CMPO_A>;
impl CMPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPO_A {
        match self.bits {
            false => CMPO_A::LOW,
            true => CMPO_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CMPO_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CMPO_A::HIGH
    }
}
#[doc = "Comparator 0 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPLK_A {
    #[doc = "0: Control bits are read-write"]
    READWRITE = 0,
    #[doc = "1: Control bits are read-only"]
    READONLY = 1,
}
impl From<CMPLK_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLK` reader - Comparator 0 lock"]
pub type CMPLK_R = crate::BitReader<CMPLK_A>;
impl CMPLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLK_A {
        match self.bits {
            false => CMPLK_A::READWRITE,
            true => CMPLK_A::READONLY,
        }
    }
    #[doc = "Checks if the value of the field is `READWRITE`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == CMPLK_A::READWRITE
    }
    #[doc = "Checks if the value of the field is `READONLY`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == CMPLK_A::READONLY
    }
}
#[doc = "Field `CMPLK` writer - Comparator 0 lock"]
pub type CMPLK_W<'a> = crate::BitWriter<'a, u32, CS_SPEC, CMPLK_A, 15>;
impl<'a> CMPLK_W<'a> {
    #[doc = "Control bits are read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut W {
        self.variant(CMPLK_A::READWRITE)
    }
    #[doc = "Control bits are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut W {
        self.variant(CMPLK_A::READONLY)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator switch"]
    #[inline(always)]
    pub fn cmpsw(&self) -> CMPSW_R {
        CMPSW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator mode"]
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator input selection"]
    #[inline(always)]
    pub fn cmpmsel(&self) -> CMPMSEL_R {
        CMPMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator output selection"]
    #[inline(always)]
    pub fn cmposel(&self) -> CMPOSEL_R {
        CMPOSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Polarity of comparator output"]
    #[inline(always)]
    pub fn cmppl(&self) -> CMPPL_R {
        CMPPL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Comparator hysteresis"]
    #[inline(always)]
    pub fn cmphst(&self) -> CMPHST_R {
        CMPHST_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Comparator 0 output"]
    #[inline(always)]
    pub fn cmpo(&self) -> CMPO_R {
        CMPO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 0 lock"]
    #[inline(always)]
    pub fn cmplk(&self) -> CMPLK_R {
        CMPLK_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator enable"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W::new(self)
    }
    #[doc = "Bit 1 - Comparator switch"]
    #[inline(always)]
    pub fn cmpsw(&mut self) -> CMPSW_W {
        CMPSW_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator mode"]
    #[inline(always)]
    pub fn cmpm(&mut self) -> CMPM_W {
        CMPM_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator input selection"]
    #[inline(always)]
    pub fn cmpmsel(&mut self) -> CMPMSEL_W {
        CMPMSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Comparator output selection"]
    #[inline(always)]
    pub fn cmposel(&mut self) -> CMPOSEL_W {
        CMPOSEL_W::new(self)
    }
    #[doc = "Bit 11 - Polarity of comparator output"]
    #[inline(always)]
    pub fn cmppl(&mut self) -> CMPPL_W {
        CMPPL_W::new(self)
    }
    #[doc = "Bits 12:13 - Comparator hysteresis"]
    #[inline(always)]
    pub fn cmphst(&mut self) -> CMPHST_W {
        CMPHST_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 0 lock"]
    #[inline(always)]
    pub fn cmplk(&mut self) -> CMPLK_W {
        CMPLK_W::new(self)
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
