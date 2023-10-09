#[doc = "Register `CS` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `CMPEN` reader - Comparator enable"]
pub type CMPEN_R = crate::BitReader<CMPEN_A>;
#[doc = "Comparator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN_A {
        match self.bits {
            false => CMPEN_A::DISABLED,
            true => CMPEN_A::ENABLED,
        }
    }
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPEN_A::DISABLED
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPEN_A::ENABLED
    }
}
#[doc = "Field `CMPEN` writer - Comparator enable"]
pub type CMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPEN_A>;
impl<'a, REG, const O: u8> CMPEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMPEN_A::DISABLED)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMPEN_A::ENABLED)
    }
}
#[doc = "Field `CMPSW` reader - Comparator switch"]
pub type CMPSW_R = crate::BitReader<CMPSW_A>;
#[doc = "Comparator switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CMPSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSW_A {
        match self.bits {
            false => CMPSW_A::OPEN,
            true => CMPSW_A::CLOSED,
        }
    }
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == CMPSW_A::OPEN
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == CMPSW_A::CLOSED
    }
}
#[doc = "Field `CMPSW` writer - Comparator switch"]
pub type CMPSW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPSW_A>;
impl<'a, REG, const O: u8> CMPSW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSW_A::OPEN)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSW_A::CLOSED)
    }
}
#[doc = "Field `CMPM` reader - Comparator mode"]
pub type CMPM_R = crate::FieldReader<CMPM_A>;
#[doc = "Comparator mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPM_A {
    #[doc = "0: High speed / full power"]
    HIGH_SPEED = 0,
    #[doc = "1: Medium speed / medium power"]
    MEDIUM_SPEED = 1,
    #[doc = "2: Low speed / low power"]
    LOW_SPEED = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VERY_LOW_SPEED = 3,
}
impl From<CMPM_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPM_A {
    type Ux = u8;
}
impl CMPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPM_A {
        match self.bits {
            0 => CMPM_A::HIGH_SPEED,
            1 => CMPM_A::MEDIUM_SPEED,
            2 => CMPM_A::LOW_SPEED,
            3 => CMPM_A::VERY_LOW_SPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == CMPM_A::HIGH_SPEED
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == CMPM_A::MEDIUM_SPEED
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == CMPM_A::LOW_SPEED
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == CMPM_A::VERY_LOW_SPEED
    }
}
#[doc = "Field `CMPM` writer - Comparator mode"]
pub type CMPM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CMPM_A>;
impl<'a, REG, const O: u8> CMPM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(CMPM_A::HIGH_SPEED)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(CMPM_A::MEDIUM_SPEED)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(CMPM_A::LOW_SPEED)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(CMPM_A::VERY_LOW_SPEED)
    }
}
#[doc = "Field `CMPMSEL` reader - Comparator input selection"]
pub type CMPMSEL_R = crate::FieldReader<CMPMSEL_A>;
#[doc = "Comparator input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    ONE_QUARTER_VREF = 0,
    #[doc = "1: 1/2 of VRefint"]
    ONE_HALF_VREF = 1,
    #[doc = "2: 3/4 of VRefint"]
    THREE_QUARTER_VREF = 2,
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
impl crate::FieldSpec for CMPMSEL_A {
    type Ux = u8;
}
impl CMPMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPMSEL_A> {
        match self.bits {
            0 => Some(CMPMSEL_A::ONE_QUARTER_VREF),
            1 => Some(CMPMSEL_A::ONE_HALF_VREF),
            2 => Some(CMPMSEL_A::THREE_QUARTER_VREF),
            3 => Some(CMPMSEL_A::VREF),
            4 => Some(CMPMSEL_A::PA4),
            5 => Some(CMPMSEL_A::PA5),
            6 => Some(CMPMSEL_A::PA0),
            _ => None,
        }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == CMPMSEL_A::ONE_QUARTER_VREF
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == CMPMSEL_A::ONE_HALF_VREF
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == CMPMSEL_A::THREE_QUARTER_VREF
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == CMPMSEL_A::VREF
    }
    #[doc = "PA4 (DAC0"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == CMPMSEL_A::PA4
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == CMPMSEL_A::PA5
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == CMPMSEL_A::PA0
    }
}
#[doc = "Field `CMPMSEL` writer - Comparator input selection"]
pub type CMPMSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CMPMSEL_A>;
impl<'a, REG, const O: u8> CMPMSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMSEL_A::ONE_QUARTER_VREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMSEL_A::ONE_HALF_VREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMSEL_A::THREE_QUARTER_VREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMSEL_A::VREF)
    }
    #[doc = "PA4 (DAC0"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMSEL_A::PA4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMSEL_A::PA5)
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMSEL_A::PA0)
    }
}
#[doc = "Field `CMPOSEL` reader - Comparator output selection"]
pub type CMPOSEL_R = crate::FieldReader<CMPOSEL_A>;
#[doc = "Comparator output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPOSEL_A {
    #[doc = "0: No selection"]
    NO_SELECTION = 0,
    #[doc = "1: Timer 0 break input"]
    TIMER0BREAK_INPUT = 1,
    #[doc = "2: Timer 0 Input capture 0"]
    TIMER0INPUT_CAPTURE0 = 2,
    #[doc = "3: Timer 0 OCPRE_CLR input"]
    TIMER0OCPRECLEAR_INPUT = 3,
    #[doc = "4: Timer 1 input capture 3"]
    TIMER1INPUT_CAPTURE3 = 4,
    #[doc = "5: Timer 1 OCPRE_CLR input"]
    TIMER1OCPRECLEAR_INPUT = 5,
    #[doc = "6: Timer 2 input capture 0"]
    TIMER2INPUT_CAPTURE0 = 6,
    #[doc = "7: Timer 2 OCPRE_CLR input"]
    TIMER2OCPRECLEAR_INPUT = 7,
}
impl From<CMPOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPOSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPOSEL_A {
    type Ux = u8;
}
impl CMPOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPOSEL_A {
        match self.bits {
            0 => CMPOSEL_A::NO_SELECTION,
            1 => CMPOSEL_A::TIMER0BREAK_INPUT,
            2 => CMPOSEL_A::TIMER0INPUT_CAPTURE0,
            3 => CMPOSEL_A::TIMER0OCPRECLEAR_INPUT,
            4 => CMPOSEL_A::TIMER1INPUT_CAPTURE3,
            5 => CMPOSEL_A::TIMER1OCPRECLEAR_INPUT,
            6 => CMPOSEL_A::TIMER2INPUT_CAPTURE0,
            7 => CMPOSEL_A::TIMER2OCPRECLEAR_INPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == CMPOSEL_A::NO_SELECTION
    }
    #[doc = "Timer 0 break input"]
    #[inline(always)]
    pub fn is_timer0break_input(&self) -> bool {
        *self == CMPOSEL_A::TIMER0BREAK_INPUT
    }
    #[doc = "Timer 0 Input capture 0"]
    #[inline(always)]
    pub fn is_timer0input_capture0(&self) -> bool {
        *self == CMPOSEL_A::TIMER0INPUT_CAPTURE0
    }
    #[doc = "Timer 0 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer0ocpreclear_input(&self) -> bool {
        *self == CMPOSEL_A::TIMER0OCPRECLEAR_INPUT
    }
    #[doc = "Timer 1 input capture 3"]
    #[inline(always)]
    pub fn is_timer1input_capture3(&self) -> bool {
        *self == CMPOSEL_A::TIMER1INPUT_CAPTURE3
    }
    #[doc = "Timer 1 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer1ocpreclear_input(&self) -> bool {
        *self == CMPOSEL_A::TIMER1OCPRECLEAR_INPUT
    }
    #[doc = "Timer 2 input capture 0"]
    #[inline(always)]
    pub fn is_timer2input_capture0(&self) -> bool {
        *self == CMPOSEL_A::TIMER2INPUT_CAPTURE0
    }
    #[doc = "Timer 2 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer2ocpreclear_input(&self) -> bool {
        *self == CMPOSEL_A::TIMER2OCPRECLEAR_INPUT
    }
}
#[doc = "Field `CMPOSEL` writer - Comparator output selection"]
pub type CMPOSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CMPOSEL_A>;
impl<'a, REG, const O: u8> CMPOSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOSEL_A::NO_SELECTION)
    }
    #[doc = "Timer 0 break input"]
    #[inline(always)]
    pub fn timer0break_input(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOSEL_A::TIMER0BREAK_INPUT)
    }
    #[doc = "Timer 0 Input capture 0"]
    #[inline(always)]
    pub fn timer0input_capture0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOSEL_A::TIMER0INPUT_CAPTURE0)
    }
    #[doc = "Timer 0 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer0ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOSEL_A::TIMER0OCPRECLEAR_INPUT)
    }
    #[doc = "Timer 1 input capture 3"]
    #[inline(always)]
    pub fn timer1input_capture3(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOSEL_A::TIMER1INPUT_CAPTURE3)
    }
    #[doc = "Timer 1 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer1ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOSEL_A::TIMER1OCPRECLEAR_INPUT)
    }
    #[doc = "Timer 2 input capture 0"]
    #[inline(always)]
    pub fn timer2input_capture0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOSEL_A::TIMER2INPUT_CAPTURE0)
    }
    #[doc = "Timer 2 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer2ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOSEL_A::TIMER2OCPRECLEAR_INPUT)
    }
}
#[doc = "Field `CMPPL` reader - Polarity of comparator output"]
pub type CMPPL_R = crate::BitReader<CMPPL_A>;
#[doc = "Polarity of comparator output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPPL_A {
    #[doc = "0: Output is not inverted"]
    NOT_INVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<CMPPL_A> for bool {
    #[inline(always)]
    fn from(variant: CMPPL_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPPL_A {
        match self.bits {
            false => CMPPL_A::NOT_INVERTED,
            true => CMPPL_A::INVERTED,
        }
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == CMPPL_A::NOT_INVERTED
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CMPPL_A::INVERTED
    }
}
#[doc = "Field `CMPPL` writer - Polarity of comparator output"]
pub type CMPPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPPL_A>;
impl<'a, REG, const O: u8> CMPPL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(CMPPL_A::NOT_INVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(CMPPL_A::INVERTED)
    }
}
#[doc = "Field `CMPHST` reader - Comparator hysteresis"]
pub type CMPHST_R = crate::FieldReader<CMPHST_A>;
#[doc = "Comparator hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPHST_A {
    #[doc = "0: No hysteresis"]
    NO_HYSTERESIS = 0,
    #[doc = "1: Low hysteresis"]
    LOW_HYSTERESIS = 1,
    #[doc = "2: Medium hysteresis"]
    MEDIUM_HYSTERESIS = 2,
    #[doc = "3: High hysteresis"]
    HIGH_HYSTERESIS = 3,
}
impl From<CMPHST_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPHST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPHST_A {
    type Ux = u8;
}
impl CMPHST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPHST_A {
        match self.bits {
            0 => CMPHST_A::NO_HYSTERESIS,
            1 => CMPHST_A::LOW_HYSTERESIS,
            2 => CMPHST_A::MEDIUM_HYSTERESIS,
            3 => CMPHST_A::HIGH_HYSTERESIS,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == CMPHST_A::NO_HYSTERESIS
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == CMPHST_A::LOW_HYSTERESIS
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == CMPHST_A::MEDIUM_HYSTERESIS
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == CMPHST_A::HIGH_HYSTERESIS
    }
}
#[doc = "Field `CMPHST` writer - Comparator hysteresis"]
pub type CMPHST_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CMPHST_A>;
impl<'a, REG, const O: u8> CMPHST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(CMPHST_A::NO_HYSTERESIS)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(CMPHST_A::LOW_HYSTERESIS)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(CMPHST_A::MEDIUM_HYSTERESIS)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(CMPHST_A::HIGH_HYSTERESIS)
    }
}
#[doc = "Field `CMPO` reader - Comparator 0 output"]
pub type CMPO_R = crate::BitReader<CMPO_A>;
#[doc = "Comparator 0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CMPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPO_A {
        match self.bits {
            false => CMPO_A::LOW,
            true => CMPO_A::HIGH,
        }
    }
    #[doc = "Non-inverting input below inverting input"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CMPO_A::LOW
    }
    #[doc = "Non-inverting input above inverting input"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CMPO_A::HIGH
    }
}
#[doc = "Field `CMPLK` reader - Comparator 0 lock"]
pub type CMPLK_R = crate::BitReader<CMPLK_A>;
#[doc = "Comparator 0 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLK_A {
    #[doc = "0: Control bits are read-write"]
    READ_WRITE = 0,
    #[doc = "1: Control bits are read-only"]
    READ_ONLY = 1,
}
impl From<CMPLK_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLK_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLK_A {
        match self.bits {
            false => CMPLK_A::READ_WRITE,
            true => CMPLK_A::READ_ONLY,
        }
    }
    #[doc = "Control bits are read-write"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == CMPLK_A::READ_WRITE
    }
    #[doc = "Control bits are read-only"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == CMPLK_A::READ_ONLY
    }
}
#[doc = "Field `CMPLK` writer - Comparator 0 lock"]
pub type CMPLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLK_A>;
impl<'a, REG, const O: u8> CMPLK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Control bits are read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLK_A::READ_WRITE)
    }
    #[doc = "Control bits are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLK_A::READ_ONLY)
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
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<CS_SPEC, 0> {
        CMPEN_W::new(self)
    }
    #[doc = "Bit 1 - Comparator switch"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsw(&mut self) -> CMPSW_W<CS_SPEC, 1> {
        CMPSW_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm(&mut self) -> CMPM_W<CS_SPEC, 2> {
        CMPM_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmsel(&mut self) -> CMPMSEL_W<CS_SPEC, 4> {
        CMPMSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Comparator output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmposel(&mut self) -> CMPOSEL_W<CS_SPEC, 8> {
        CMPOSEL_W::new(self)
    }
    #[doc = "Bit 11 - Polarity of comparator output"]
    #[inline(always)]
    #[must_use]
    pub fn cmppl(&mut self) -> CMPPL_W<CS_SPEC, 11> {
        CMPPL_W::new(self)
    }
    #[doc = "Bits 12:13 - Comparator hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmphst(&mut self) -> CMPHST_W<CS_SPEC, 12> {
        CMPHST_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 0 lock"]
    #[inline(always)]
    #[must_use]
    pub fn cmplk(&mut self) -> CMPLK_W<CS_SPEC, 15> {
        CMPLK_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
