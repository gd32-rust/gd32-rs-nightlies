#[doc = "Register `CS` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `CMP0EN` reader - CMP0 enable"]
pub type CMP0EN_R = crate::BitReader<CMP0EN_A>;
#[doc = "CMP0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CMP0EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0EN_A {
        match self.bits {
            false => CMP0EN_A::DISABLED,
            true => CMP0EN_A::ENABLED,
        }
    }
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP0EN_A::DISABLED
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP0EN_A::ENABLED
    }
}
#[doc = "Field `CMP0EN` writer - CMP0 enable"]
pub type CMP0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMP0EN_A>;
impl<'a, REG, const O: u8> CMP0EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0EN_A::DISABLED)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0EN_A::ENABLED)
    }
}
#[doc = "Field `CMP0SW` reader - CMP0 switch"]
pub type CMP0SW_R = crate::BitReader<CMP0SW_A>;
#[doc = "CMP0 switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CMP0SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0SW_A {
        match self.bits {
            false => CMP0SW_A::OPEN,
            true => CMP0SW_A::CLOSED,
        }
    }
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == CMP0SW_A::OPEN
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == CMP0SW_A::CLOSED
    }
}
#[doc = "Field `CMP0SW` writer - CMP0 switch"]
pub type CMP0SW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMP0SW_A>;
impl<'a, REG, const O: u8> CMP0SW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0SW_A::OPEN)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0SW_A::CLOSED)
    }
}
#[doc = "Field `CMP0M` reader - CMP0 mode"]
pub type CMP0M_R = crate::FieldReader<CMP0M_A>;
#[doc = "CMP0 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP0M_A {
    #[doc = "0: High speed / full power"]
    HIGH_SPEED = 0,
    #[doc = "1: Medium speed / medium power"]
    MEDIUM_SPEED = 1,
    #[doc = "2: Low speed / low power"]
    LOW_SPEED = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VERY_LOW_SPEED = 3,
}
impl From<CMP0M_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMP0M_A {
    type Ux = u8;
}
impl CMP0M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0M_A {
        match self.bits {
            0 => CMP0M_A::HIGH_SPEED,
            1 => CMP0M_A::MEDIUM_SPEED,
            2 => CMP0M_A::LOW_SPEED,
            3 => CMP0M_A::VERY_LOW_SPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == CMP0M_A::HIGH_SPEED
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == CMP0M_A::MEDIUM_SPEED
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == CMP0M_A::LOW_SPEED
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == CMP0M_A::VERY_LOW_SPEED
    }
}
#[doc = "Field `CMP0M` writer - CMP0 mode"]
pub type CMP0M_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CMP0M_A>;
impl<'a, REG, const O: u8> CMP0M_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0M_A::HIGH_SPEED)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0M_A::MEDIUM_SPEED)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0M_A::LOW_SPEED)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0M_A::VERY_LOW_SPEED)
    }
}
#[doc = "Field `CMP0MSEL` reader - CMP0_M input selection"]
pub type CMP0MSEL_R = crate::FieldReader<CMP0MSEL_A>;
#[doc = "CMP0_M input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP0MSEL_A {
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
impl From<CMP0MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0MSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMP0MSEL_A {
    type Ux = u8;
}
impl CMP0MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMP0MSEL_A> {
        match self.bits {
            0 => Some(CMP0MSEL_A::ONE_QUARTER_VREF),
            1 => Some(CMP0MSEL_A::ONE_HALF_VREF),
            2 => Some(CMP0MSEL_A::THREE_QUARTER_VREF),
            3 => Some(CMP0MSEL_A::VREF),
            4 => Some(CMP0MSEL_A::PA4),
            5 => Some(CMP0MSEL_A::PA5),
            6 => Some(CMP0MSEL_A::PA0),
            _ => None,
        }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == CMP0MSEL_A::ONE_QUARTER_VREF
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == CMP0MSEL_A::ONE_HALF_VREF
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == CMP0MSEL_A::THREE_QUARTER_VREF
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == CMP0MSEL_A::VREF
    }
    #[doc = "PA4 (DAC0"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == CMP0MSEL_A::PA4
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == CMP0MSEL_A::PA5
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == CMP0MSEL_A::PA0
    }
}
#[doc = "Field `CMP0MSEL` writer - CMP0_M input selection"]
pub type CMP0MSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CMP0MSEL_A>;
impl<'a, REG, const O: u8> CMP0MSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0MSEL_A::ONE_QUARTER_VREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0MSEL_A::ONE_HALF_VREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0MSEL_A::THREE_QUARTER_VREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0MSEL_A::VREF)
    }
    #[doc = "PA4 (DAC0"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0MSEL_A::PA4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0MSEL_A::PA5)
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0MSEL_A::PA0)
    }
}
#[doc = "Field `CMP0OSEL` reader - Comparator 0 output selection"]
pub type CMP0OSEL_R = crate::FieldReader<CMP0OSEL_A>;
#[doc = "Comparator 0 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP0OSEL_A {
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
impl From<CMP0OSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0OSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMP0OSEL_A {
    type Ux = u8;
}
impl CMP0OSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0OSEL_A {
        match self.bits {
            0 => CMP0OSEL_A::NO_SELECTION,
            1 => CMP0OSEL_A::TIMER0BREAK_INPUT,
            2 => CMP0OSEL_A::TIMER0INPUT_CAPTURE0,
            3 => CMP0OSEL_A::TIMER0OCPRECLEAR_INPUT,
            4 => CMP0OSEL_A::TIMER1INPUT_CAPTURE3,
            5 => CMP0OSEL_A::TIMER1OCPRECLEAR_INPUT,
            6 => CMP0OSEL_A::TIMER2INPUT_CAPTURE0,
            7 => CMP0OSEL_A::TIMER2OCPRECLEAR_INPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == CMP0OSEL_A::NO_SELECTION
    }
    #[doc = "Timer 0 break input"]
    #[inline(always)]
    pub fn is_timer0break_input(&self) -> bool {
        *self == CMP0OSEL_A::TIMER0BREAK_INPUT
    }
    #[doc = "Timer 0 Input capture 0"]
    #[inline(always)]
    pub fn is_timer0input_capture0(&self) -> bool {
        *self == CMP0OSEL_A::TIMER0INPUT_CAPTURE0
    }
    #[doc = "Timer 0 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer0ocpreclear_input(&self) -> bool {
        *self == CMP0OSEL_A::TIMER0OCPRECLEAR_INPUT
    }
    #[doc = "Timer 1 input capture 3"]
    #[inline(always)]
    pub fn is_timer1input_capture3(&self) -> bool {
        *self == CMP0OSEL_A::TIMER1INPUT_CAPTURE3
    }
    #[doc = "Timer 1 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer1ocpreclear_input(&self) -> bool {
        *self == CMP0OSEL_A::TIMER1OCPRECLEAR_INPUT
    }
    #[doc = "Timer 2 input capture 0"]
    #[inline(always)]
    pub fn is_timer2input_capture0(&self) -> bool {
        *self == CMP0OSEL_A::TIMER2INPUT_CAPTURE0
    }
    #[doc = "Timer 2 OCPRE_CLR input"]
    #[inline(always)]
    pub fn is_timer2ocpreclear_input(&self) -> bool {
        *self == CMP0OSEL_A::TIMER2OCPRECLEAR_INPUT
    }
}
#[doc = "Field `CMP0OSEL` writer - Comparator 0 output selection"]
pub type CMP0OSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CMP0OSEL_A>;
impl<'a, REG, const O: u8> CMP0OSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0OSEL_A::NO_SELECTION)
    }
    #[doc = "Timer 0 break input"]
    #[inline(always)]
    pub fn timer0break_input(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0OSEL_A::TIMER0BREAK_INPUT)
    }
    #[doc = "Timer 0 Input capture 0"]
    #[inline(always)]
    pub fn timer0input_capture0(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0OSEL_A::TIMER0INPUT_CAPTURE0)
    }
    #[doc = "Timer 0 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer0ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0OSEL_A::TIMER0OCPRECLEAR_INPUT)
    }
    #[doc = "Timer 1 input capture 3"]
    #[inline(always)]
    pub fn timer1input_capture3(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0OSEL_A::TIMER1INPUT_CAPTURE3)
    }
    #[doc = "Timer 1 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer1ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0OSEL_A::TIMER1OCPRECLEAR_INPUT)
    }
    #[doc = "Timer 2 input capture 0"]
    #[inline(always)]
    pub fn timer2input_capture0(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0OSEL_A::TIMER2INPUT_CAPTURE0)
    }
    #[doc = "Timer 2 OCPRE_CLR input"]
    #[inline(always)]
    pub fn timer2ocpreclear_input(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0OSEL_A::TIMER2OCPRECLEAR_INPUT)
    }
}
#[doc = "Field `CMP0PL` reader - Polarity of CMP0 output"]
pub type CMP0PL_R = crate::BitReader<CMP0PL_A>;
#[doc = "Polarity of CMP0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP0PL_A {
    #[doc = "0: Output is not inverted"]
    NOT_INVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<CMP0PL_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0PL_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP0PL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0PL_A {
        match self.bits {
            false => CMP0PL_A::NOT_INVERTED,
            true => CMP0PL_A::INVERTED,
        }
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == CMP0PL_A::NOT_INVERTED
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CMP0PL_A::INVERTED
    }
}
#[doc = "Field `CMP0PL` writer - Polarity of CMP0 output"]
pub type CMP0PL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMP0PL_A>;
impl<'a, REG, const O: u8> CMP0PL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0PL_A::NOT_INVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0PL_A::INVERTED)
    }
}
#[doc = "Field `CMP0HST` reader - CMP0 hysteresis"]
pub type CMP0HST_R = crate::FieldReader<CMP0HST_A>;
#[doc = "CMP0 hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP0HST_A {
    #[doc = "0: No hysteresis"]
    NO_HYSTERESIS = 0,
    #[doc = "1: Low hysteresis"]
    LOW_HYSTERESIS = 1,
    #[doc = "2: Medium hysteresis"]
    MEDIUM_HYSTERESIS = 2,
    #[doc = "3: High hysteresis"]
    HIGH_HYSTERESIS = 3,
}
impl From<CMP0HST_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0HST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMP0HST_A {
    type Ux = u8;
}
impl CMP0HST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0HST_A {
        match self.bits {
            0 => CMP0HST_A::NO_HYSTERESIS,
            1 => CMP0HST_A::LOW_HYSTERESIS,
            2 => CMP0HST_A::MEDIUM_HYSTERESIS,
            3 => CMP0HST_A::HIGH_HYSTERESIS,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == CMP0HST_A::NO_HYSTERESIS
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == CMP0HST_A::LOW_HYSTERESIS
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == CMP0HST_A::MEDIUM_HYSTERESIS
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == CMP0HST_A::HIGH_HYSTERESIS
    }
}
#[doc = "Field `CMP0HST` writer - CMP0 hysteresis"]
pub type CMP0HST_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CMP0HST_A>;
impl<'a, REG, const O: u8> CMP0HST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0HST_A::NO_HYSTERESIS)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0HST_A::LOW_HYSTERESIS)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0HST_A::MEDIUM_HYSTERESIS)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0HST_A::HIGH_HYSTERESIS)
    }
}
#[doc = "Field `CMP0O` reader - CMP0 output"]
pub type CMP0O_R = crate::BitReader<CMP0O_A>;
#[doc = "CMP0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CMP0O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0O_A {
        match self.bits {
            false => CMP0O_A::LOW,
            true => CMP0O_A::HIGH,
        }
    }
    #[doc = "Non-inverting input below inverting input"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CMP0O_A::LOW
    }
    #[doc = "Non-inverting input above inverting input"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CMP0O_A::HIGH
    }
}
#[doc = "Field `CMP0LK` reader - CMP0 lock"]
pub type CMP0LK_R = crate::BitReader<CMP0LK_A>;
#[doc = "CMP0 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP0LK_A {
    #[doc = "0: Control bits are read-write"]
    READ_WRITE = 0,
    #[doc = "1: Control bits are read-only"]
    READ_ONLY = 1,
}
impl From<CMP0LK_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0LK_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP0LK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0LK_A {
        match self.bits {
            false => CMP0LK_A::READ_WRITE,
            true => CMP0LK_A::READ_ONLY,
        }
    }
    #[doc = "Control bits are read-write"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == CMP0LK_A::READ_WRITE
    }
    #[doc = "Control bits are read-only"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == CMP0LK_A::READ_ONLY
    }
}
#[doc = "Field `CMP0LK` writer - CMP0 lock"]
pub type CMP0LK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMP0LK_A>;
impl<'a, REG, const O: u8> CMP0LK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Control bits are read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0LK_A::READ_WRITE)
    }
    #[doc = "Control bits are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(CMP0LK_A::READ_ONLY)
    }
}
#[doc = "Field `CMP1EN` reader - CMP1 enable"]
pub use CMP0EN_R as CMP1EN_R;
#[doc = "Field `CMP1EN` writer - CMP1 enable"]
pub use CMP0EN_W as CMP1EN_W;
#[doc = "Field `CMP1MSEL` reader - CMP1_M input selection"]
pub use CMP0MSEL_R as CMP1MSEL_R;
#[doc = "Field `CMP1MSEL` writer - CMP1_M input selection"]
pub use CMP0MSEL_W as CMP1MSEL_W;
#[doc = "Field `CMP1M` reader - CMP1 mode"]
pub use CMP0M_R as CMP1M_R;
#[doc = "Field `CMP1M` writer - CMP1 mode"]
pub use CMP0M_W as CMP1M_W;
#[doc = "Field `WNDEN` reader - Window mode enable"]
pub type WNDEN_R = crate::BitReader<WNDEN_A>;
#[doc = "Window mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WNDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WNDEN_A {
        match self.bits {
            false => WNDEN_A::DISABLED,
            true => WNDEN_A::ENABLED,
        }
    }
    #[doc = "Window mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WNDEN_A::DISABLED
    }
    #[doc = "Window mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WNDEN_A::ENABLED
    }
}
#[doc = "Field `WNDEN` writer - Window mode enable"]
pub type WNDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WNDEN_A>;
impl<'a, REG, const O: u8> WNDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WNDEN_A::DISABLED)
    }
    #[doc = "Window mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WNDEN_A::ENABLED)
    }
}
#[doc = "Field `CMP1HST` reader - CMP1 hysteresis"]
pub use CMP0HST_R as CMP1HST_R;
#[doc = "Field `CMP1HST` writer - CMP1 hysteresis"]
pub use CMP0HST_W as CMP1HST_W;
#[doc = "Field `CMP1LK` reader - CMP1 lock"]
pub use CMP0LK_R as CMP1LK_R;
#[doc = "Field `CMP1LK` writer - CMP1 lock"]
pub use CMP0LK_W as CMP1LK_W;
#[doc = "Field `CMP1OSEL` reader - CMP1 output selection"]
pub use CMP0OSEL_R as CMP1OSEL_R;
#[doc = "Field `CMP1OSEL` writer - CMP1 output selection"]
pub use CMP0OSEL_W as CMP1OSEL_W;
#[doc = "Field `CMP1O` reader - CMP1 output"]
pub use CMP0O_R as CMP1O_R;
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
    #[must_use]
    pub fn cmp0en(&mut self) -> CMP0EN_W<CS_SPEC, 0> {
        CMP0EN_W::new(self)
    }
    #[doc = "Bit 1 - CMP0 switch"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0sw(&mut self) -> CMP0SW_W<CS_SPEC, 1> {
        CMP0SW_W::new(self)
    }
    #[doc = "Bits 2:3 - CMP0 mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0m(&mut self) -> CMP0M_W<CS_SPEC, 2> {
        CMP0M_W::new(self)
    }
    #[doc = "Bits 4:6 - CMP0_M input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0msel(&mut self) -> CMP0MSEL_W<CS_SPEC, 4> {
        CMP0MSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Comparator 0 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0osel(&mut self) -> CMP0OSEL_W<CS_SPEC, 8> {
        CMP0OSEL_W::new(self)
    }
    #[doc = "Bit 11 - Polarity of CMP0 output"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0pl(&mut self) -> CMP0PL_W<CS_SPEC, 11> {
        CMP0PL_W::new(self)
    }
    #[doc = "Bits 12:13 - CMP0 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0hst(&mut self) -> CMP0HST_W<CS_SPEC, 12> {
        CMP0HST_W::new(self)
    }
    #[doc = "Bit 15 - CMP0 lock"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0lk(&mut self) -> CMP0LK_W<CS_SPEC, 15> {
        CMP0LK_W::new(self)
    }
    #[doc = "Bit 16 - CMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1en(&mut self) -> CMP1EN_W<CS_SPEC, 16> {
        CMP1EN_W::new(self)
    }
    #[doc = "Bits 18:19 - CMP1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1m(&mut self) -> CMP1M_W<CS_SPEC, 18> {
        CMP1M_W::new(self)
    }
    #[doc = "Bits 20:22 - CMP1_M input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1msel(&mut self) -> CMP1MSEL_W<CS_SPEC, 20> {
        CMP1MSEL_W::new(self)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn wnden(&mut self) -> WNDEN_W<CS_SPEC, 23> {
        WNDEN_W::new(self)
    }
    #[doc = "Bits 24:26 - CMP1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1osel(&mut self) -> CMP1OSEL_W<CS_SPEC, 24> {
        CMP1OSEL_W::new(self)
    }
    #[doc = "Bit 27 - Polarity of CMP1 output"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1pl(&mut self) -> CMP1PL_W<CS_SPEC, 27> {
        CMP1PL_W::new(self)
    }
    #[doc = "Bits 28:29 - CMP1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1hst(&mut self) -> CMP1HST_W<CS_SPEC, 28> {
        CMP1HST_W::new(self)
    }
    #[doc = "Bit 31 - CMP1 lock"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1lk(&mut self) -> CMP1LK_W<CS_SPEC, 31> {
        CMP1LK_W::new(self)
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
