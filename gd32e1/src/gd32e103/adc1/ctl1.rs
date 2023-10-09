#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `ADCON` reader - ADC on"]
pub type ADCON_R = crate::BitReader<ADCON_A>;
#[doc = "ADC on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCON_A {
    #[doc = "0: Disable ADC conversion/calibration and go to power down mode"]
    DISABLED = 0,
    #[doc = "1: Enable ADC and to start conversion"]
    ENABLED = 1,
}
impl From<ADCON_A> for bool {
    #[inline(always)]
    fn from(variant: ADCON_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCON_A {
        match self.bits {
            false => ADCON_A::DISABLED,
            true => ADCON_A::ENABLED,
        }
    }
    #[doc = "Disable ADC conversion/calibration and go to power down mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCON_A::DISABLED
    }
    #[doc = "Enable ADC and to start conversion"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCON_A::ENABLED
    }
}
#[doc = "Field `ADCON` writer - ADC on"]
pub type ADCON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADCON_A>;
impl<'a, REG, const O: u8> ADCON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ADC conversion/calibration and go to power down mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCON_A::DISABLED)
    }
    #[doc = "Enable ADC and to start conversion"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCON_A::ENABLED)
    }
}
#[doc = "Field `CTN` reader - Continuous mode"]
pub type CTN_R = crate::BitReader<CTN_A>;
#[doc = "Continuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTN_A {
    #[doc = "0: Single conversion mode"]
    SINGLE = 0,
    #[doc = "1: Continuous conversion mode"]
    CONTINUOUS = 1,
}
impl From<CTN_A> for bool {
    #[inline(always)]
    fn from(variant: CTN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTN_A {
        match self.bits {
            false => CTN_A::SINGLE,
            true => CTN_A::CONTINUOUS,
        }
    }
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CTN_A::SINGLE
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CTN_A::CONTINUOUS
    }
}
#[doc = "Field `CTN` writer - Continuous mode"]
pub type CTN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTN_A>;
impl<'a, REG, const O: u8> CTN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(CTN_A::SINGLE)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CTN_A::CONTINUOUS)
    }
}
#[doc = "Field `CLB` reader - ADC calibration"]
pub type CLB_R = crate::BitReader<CLBR_A>;
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLBR_A {
    #[doc = "0: Calibration completed"]
    COMPLETE = 0,
    #[doc = "1: Calibrating"]
    NOT_COMPLETE = 1,
}
impl From<CLBR_A> for bool {
    #[inline(always)]
    fn from(variant: CLBR_A) -> Self {
        variant as u8 != 0
    }
}
impl CLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLBR_A {
        match self.bits {
            false => CLBR_A::COMPLETE,
            true => CLBR_A::NOT_COMPLETE,
        }
    }
    #[doc = "Calibration completed"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CLBR_A::COMPLETE
    }
    #[doc = "Calibrating"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CLBR_A::NOT_COMPLETE
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLBW_AW {
    #[doc = "1: Enable calibration"]
    START = 1,
}
impl From<CLBW_AW> for bool {
    #[inline(always)]
    fn from(variant: CLBW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLB` writer - ADC calibration"]
pub type CLB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLBW_AW>;
impl<'a, REG, const O: u8> CLB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CLBW_AW::START)
    }
}
#[doc = "Field `RSTCLB` reader - Reset calibration"]
pub type RSTCLB_R = crate::BitReader<RSTCLBR_A>;
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTCLBR_A {
    #[doc = "0: Calibration completed"]
    COMPLETE = 0,
    #[doc = "1: Calibrating"]
    NOT_COMPLETE = 1,
}
impl From<RSTCLBR_A> for bool {
    #[inline(always)]
    fn from(variant: RSTCLBR_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTCLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTCLBR_A {
        match self.bits {
            false => RSTCLBR_A::COMPLETE,
            true => RSTCLBR_A::NOT_COMPLETE,
        }
    }
    #[doc = "Calibration completed"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == RSTCLBR_A::COMPLETE
    }
    #[doc = "Calibrating"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == RSTCLBR_A::NOT_COMPLETE
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTCLBW_AW {
    #[doc = "1: Enable calibration"]
    START = 1,
}
impl From<RSTCLBW_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTCLBW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCLB` writer - Reset calibration"]
pub type RSTCLB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RSTCLBW_AW>;
impl<'a, REG, const O: u8> RSTCLB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RSTCLBW_AW::START)
    }
}
#[doc = "Field `DMA` reader - DMA request enable"]
pub type DMA_R = crate::BitReader<DMA_A>;
#[doc = "DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_A {
    #[doc = "0: DMA mode disabled"]
    DISABLED = 0,
    #[doc = "1: DMA mode enabled"]
    ENABLED = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::DISABLED,
            true => DMA_A::ENABLED,
        }
    }
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_A::DISABLED
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_A::ENABLED
    }
}
#[doc = "Field `DMA` writer - DMA request enable"]
pub type DMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_A>;
impl<'a, REG, const O: u8> DMA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_A::DISABLED)
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_A::ENABLED)
    }
}
#[doc = "Field `DAL` reader - Data alignment"]
pub type DAL_R = crate::BitReader<DAL_A>;
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAL_A {
    #[doc = "0: Right alignment"]
    RIGHT = 0,
    #[doc = "1: Left alignment"]
    LEFT = 1,
}
impl From<DAL_A> for bool {
    #[inline(always)]
    fn from(variant: DAL_A) -> Self {
        variant as u8 != 0
    }
}
impl DAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAL_A {
        match self.bits {
            false => DAL_A::RIGHT,
            true => DAL_A::LEFT,
        }
    }
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == DAL_A::RIGHT
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == DAL_A::LEFT
    }
}
#[doc = "Field `DAL` writer - Data alignment"]
pub type DAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DAL_A>;
impl<'a, REG, const O: u8> DAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(DAL_A::RIGHT)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(DAL_A::LEFT)
    }
}
#[doc = "Field `ETSIC` reader - External trigger select for inserted channel"]
pub type ETSIC_R = crate::FieldReader<ETSIC_A>;
#[doc = "External trigger select for inserted channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETSIC_A {
    #[doc = "0: Timer 0 TRGO event"]
    TIMER0TRGO = 0,
    #[doc = "1: Timer 0 channel 3 event"]
    TIMER0CH3 = 1,
    #[doc = "2: Timer 1 TRGO event"]
    TIMER1TRGO = 2,
    #[doc = "3: Timer 1 channel 0 event"]
    TIMER1CH0 = 3,
    #[doc = "4: Timer 2 channel 3 event"]
    TIMER2CH2 = 4,
    #[doc = "5: Timer 14 TRGO event"]
    TIMER14TRGO = 5,
    #[doc = "6: EXTI line 15"]
    EXTI15 = 6,
    #[doc = "7: SWICST"]
    SWICST = 7,
}
impl From<ETSIC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETSIC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETSIC_A {
    type Ux = u8;
}
impl ETSIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETSIC_A {
        match self.bits {
            0 => ETSIC_A::TIMER0TRGO,
            1 => ETSIC_A::TIMER0CH3,
            2 => ETSIC_A::TIMER1TRGO,
            3 => ETSIC_A::TIMER1CH0,
            4 => ETSIC_A::TIMER2CH2,
            5 => ETSIC_A::TIMER14TRGO,
            6 => ETSIC_A::EXTI15,
            7 => ETSIC_A::SWICST,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer 0 TRGO event"]
    #[inline(always)]
    pub fn is_timer0trgo(&self) -> bool {
        *self == ETSIC_A::TIMER0TRGO
    }
    #[doc = "Timer 0 channel 3 event"]
    #[inline(always)]
    pub fn is_timer0ch3(&self) -> bool {
        *self == ETSIC_A::TIMER0CH3
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn is_timer1trgo(&self) -> bool {
        *self == ETSIC_A::TIMER1TRGO
    }
    #[doc = "Timer 1 channel 0 event"]
    #[inline(always)]
    pub fn is_timer1ch0(&self) -> bool {
        *self == ETSIC_A::TIMER1CH0
    }
    #[doc = "Timer 2 channel 3 event"]
    #[inline(always)]
    pub fn is_timer2ch2(&self) -> bool {
        *self == ETSIC_A::TIMER2CH2
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn is_timer14trgo(&self) -> bool {
        *self == ETSIC_A::TIMER14TRGO
    }
    #[doc = "EXTI line 15"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == ETSIC_A::EXTI15
    }
    #[doc = "SWICST"]
    #[inline(always)]
    pub fn is_swicst(&self) -> bool {
        *self == ETSIC_A::SWICST
    }
}
#[doc = "Field `ETSIC` writer - External trigger select for inserted channel"]
pub type ETSIC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, ETSIC_A>;
impl<'a, REG, const O: u8> ETSIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 0 TRGO event"]
    #[inline(always)]
    pub fn timer0trgo(self) -> &'a mut crate::W<REG> {
        self.variant(ETSIC_A::TIMER0TRGO)
    }
    #[doc = "Timer 0 channel 3 event"]
    #[inline(always)]
    pub fn timer0ch3(self) -> &'a mut crate::W<REG> {
        self.variant(ETSIC_A::TIMER0CH3)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn timer1trgo(self) -> &'a mut crate::W<REG> {
        self.variant(ETSIC_A::TIMER1TRGO)
    }
    #[doc = "Timer 1 channel 0 event"]
    #[inline(always)]
    pub fn timer1ch0(self) -> &'a mut crate::W<REG> {
        self.variant(ETSIC_A::TIMER1CH0)
    }
    #[doc = "Timer 2 channel 3 event"]
    #[inline(always)]
    pub fn timer2ch2(self) -> &'a mut crate::W<REG> {
        self.variant(ETSIC_A::TIMER2CH2)
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn timer14trgo(self) -> &'a mut crate::W<REG> {
        self.variant(ETSIC_A::TIMER14TRGO)
    }
    #[doc = "EXTI line 15"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(ETSIC_A::EXTI15)
    }
    #[doc = "SWICST"]
    #[inline(always)]
    pub fn swicst(self) -> &'a mut crate::W<REG> {
        self.variant(ETSIC_A::SWICST)
    }
}
#[doc = "Field `ETEIC` reader - External trigger enable for insert channel"]
pub type ETEIC_R = crate::BitReader<ETEIC_A>;
#[doc = "External trigger enable for insert channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETEIC_A {
    #[doc = "0: Conversion on external event disabled"]
    DISABLED = 0,
    #[doc = "1: Conversion on external event enabled"]
    ENABLED = 1,
}
impl From<ETEIC_A> for bool {
    #[inline(always)]
    fn from(variant: ETEIC_A) -> Self {
        variant as u8 != 0
    }
}
impl ETEIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETEIC_A {
        match self.bits {
            false => ETEIC_A::DISABLED,
            true => ETEIC_A::ENABLED,
        }
    }
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ETEIC_A::DISABLED
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ETEIC_A::ENABLED
    }
}
#[doc = "Field `ETEIC` writer - External trigger enable for insert channel"]
pub type ETEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ETEIC_A>;
impl<'a, REG, const O: u8> ETEIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ETEIC_A::DISABLED)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ETEIC_A::ENABLED)
    }
}
#[doc = "Field `ETSRC` reader - External trigger select for regular channel"]
pub type ETSRC_R = crate::FieldReader<ETSRC_A>;
#[doc = "External trigger select for regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETSRC_A {
    #[doc = "0: Timer 0 channel 0 event"]
    TIMER0CH0 = 0,
    #[doc = "1: Timer 0 channel 1 event"]
    TIMER0CH1 = 1,
    #[doc = "2: Timer 0 channel 2 event"]
    TIMER0CH2 = 2,
    #[doc = "3: Timer 1 channel 1 event"]
    TIMER1CH1 = 3,
    #[doc = "4: Timer 2 TRGO event"]
    TIMER2TRGO = 4,
    #[doc = "5: Timer 14 channel 0 event"]
    TIMER14CH0 = 5,
    #[doc = "6: EXTI line 11"]
    EXTI11 = 6,
    #[doc = "7: SWRCST"]
    SWRCST = 7,
}
impl From<ETSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETSRC_A {
    type Ux = u8;
}
impl ETSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETSRC_A {
        match self.bits {
            0 => ETSRC_A::TIMER0CH0,
            1 => ETSRC_A::TIMER0CH1,
            2 => ETSRC_A::TIMER0CH2,
            3 => ETSRC_A::TIMER1CH1,
            4 => ETSRC_A::TIMER2TRGO,
            5 => ETSRC_A::TIMER14CH0,
            6 => ETSRC_A::EXTI11,
            7 => ETSRC_A::SWRCST,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer 0 channel 0 event"]
    #[inline(always)]
    pub fn is_timer0ch0(&self) -> bool {
        *self == ETSRC_A::TIMER0CH0
    }
    #[doc = "Timer 0 channel 1 event"]
    #[inline(always)]
    pub fn is_timer0ch1(&self) -> bool {
        *self == ETSRC_A::TIMER0CH1
    }
    #[doc = "Timer 0 channel 2 event"]
    #[inline(always)]
    pub fn is_timer0ch2(&self) -> bool {
        *self == ETSRC_A::TIMER0CH2
    }
    #[doc = "Timer 1 channel 1 event"]
    #[inline(always)]
    pub fn is_timer1ch1(&self) -> bool {
        *self == ETSRC_A::TIMER1CH1
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_timer2trgo(&self) -> bool {
        *self == ETSRC_A::TIMER2TRGO
    }
    #[doc = "Timer 14 channel 0 event"]
    #[inline(always)]
    pub fn is_timer14ch0(&self) -> bool {
        *self == ETSRC_A::TIMER14CH0
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == ETSRC_A::EXTI11
    }
    #[doc = "SWRCST"]
    #[inline(always)]
    pub fn is_swrcst(&self) -> bool {
        *self == ETSRC_A::SWRCST
    }
}
#[doc = "Field `ETSRC` writer - External trigger select for regular channel"]
pub type ETSRC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, ETSRC_A>;
impl<'a, REG, const O: u8> ETSRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 0 channel 0 event"]
    #[inline(always)]
    pub fn timer0ch0(self) -> &'a mut crate::W<REG> {
        self.variant(ETSRC_A::TIMER0CH0)
    }
    #[doc = "Timer 0 channel 1 event"]
    #[inline(always)]
    pub fn timer0ch1(self) -> &'a mut crate::W<REG> {
        self.variant(ETSRC_A::TIMER0CH1)
    }
    #[doc = "Timer 0 channel 2 event"]
    #[inline(always)]
    pub fn timer0ch2(self) -> &'a mut crate::W<REG> {
        self.variant(ETSRC_A::TIMER0CH2)
    }
    #[doc = "Timer 1 channel 1 event"]
    #[inline(always)]
    pub fn timer1ch1(self) -> &'a mut crate::W<REG> {
        self.variant(ETSRC_A::TIMER1CH1)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn timer2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(ETSRC_A::TIMER2TRGO)
    }
    #[doc = "Timer 14 channel 0 event"]
    #[inline(always)]
    pub fn timer14ch0(self) -> &'a mut crate::W<REG> {
        self.variant(ETSRC_A::TIMER14CH0)
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(ETSRC_A::EXTI11)
    }
    #[doc = "SWRCST"]
    #[inline(always)]
    pub fn swrcst(self) -> &'a mut crate::W<REG> {
        self.variant(ETSRC_A::SWRCST)
    }
}
#[doc = "Field `ETERC` reader - External trigger enable for regular channel"]
pub type ETERC_R = crate::BitReader<ETERC_A>;
#[doc = "External trigger enable for regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETERC_A {
    #[doc = "0: Conversion on external event disabled"]
    DISABLED = 0,
    #[doc = "1: Conversion on external event enabled"]
    ENABLED = 1,
}
impl From<ETERC_A> for bool {
    #[inline(always)]
    fn from(variant: ETERC_A) -> Self {
        variant as u8 != 0
    }
}
impl ETERC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETERC_A {
        match self.bits {
            false => ETERC_A::DISABLED,
            true => ETERC_A::ENABLED,
        }
    }
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ETERC_A::DISABLED
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ETERC_A::ENABLED
    }
}
#[doc = "Field `ETERC` writer - External trigger enable for regular channel"]
pub type ETERC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ETERC_A>;
impl<'a, REG, const O: u8> ETERC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ETERC_A::DISABLED)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ETERC_A::ENABLED)
    }
}
#[doc = "Field `SWICST` reader - Start on inserted channel"]
pub type SWICST_R = crate::BitReader<SWICSTR_A>;
#[doc = "Start on inserted channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWICSTR_A {
    #[doc = "0: Reset state"]
    STARTED = 0,
    #[doc = "1: Starting conversion of inserted channels"]
    NOT_STARTED = 1,
}
impl From<SWICSTR_A> for bool {
    #[inline(always)]
    fn from(variant: SWICSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl SWICST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWICSTR_A {
        match self.bits {
            false => SWICSTR_A::STARTED,
            true => SWICSTR_A::NOT_STARTED,
        }
    }
    #[doc = "Reset state"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SWICSTR_A::STARTED
    }
    #[doc = "Starting conversion of inserted channels"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == SWICSTR_A::NOT_STARTED
    }
}
#[doc = "Start on inserted channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWICSTW_AW {
    #[doc = "1: Start conversion of inserted channels"]
    START = 1,
}
impl From<SWICSTW_AW> for bool {
    #[inline(always)]
    fn from(variant: SWICSTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWICST` writer - Start on inserted channel"]
pub type SWICST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWICSTW_AW>;
impl<'a, REG, const O: u8> SWICST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start conversion of inserted channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SWICSTW_AW::START)
    }
}
#[doc = "Field `TSVREN` reader - Channel 16 and 17 enable of ADC0"]
pub type TSVREN_R = crate::BitReader<TSVREN_A>;
#[doc = "Channel 16 and 17 enable of ADC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSVREN_A {
    #[doc = "0: Channel 16 and 17 disabled"]
    DISABLED = 0,
    #[doc = "1: Channel 16 and 17 enabled"]
    ENABLED = 1,
}
impl From<TSVREN_A> for bool {
    #[inline(always)]
    fn from(variant: TSVREN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSVREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSVREN_A {
        match self.bits {
            false => TSVREN_A::DISABLED,
            true => TSVREN_A::ENABLED,
        }
    }
    #[doc = "Channel 16 and 17 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREN_A::DISABLED
    }
    #[doc = "Channel 16 and 17 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREN_A::ENABLED
    }
}
#[doc = "Field `TSVREN` writer - Channel 16 and 17 enable of ADC0"]
pub type TSVREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSVREN_A>;
impl<'a, REG, const O: u8> TSVREN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 16 and 17 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSVREN_A::DISABLED)
    }
    #[doc = "Channel 16 and 17 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSVREN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    pub fn adcon(&self) -> ADCON_R {
        ADCON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&self) -> CTN_R {
        CTN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    pub fn clb(&self) -> CLB_R {
        CLB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstclb(&self) -> RSTCLB_R {
        RSTCLB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&self) -> DAL_R {
        DAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn etsic(&self) -> ETSIC_R {
        ETSIC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - External trigger enable for insert channel"]
    #[inline(always)]
    pub fn eteic(&self) -> ETEIC_R {
        ETEIC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - External trigger select for regular channel"]
    #[inline(always)]
    pub fn etsrc(&self) -> ETSRC_R {
        ETSRC_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - External trigger enable for regular channel"]
    #[inline(always)]
    pub fn eterc(&self) -> ETERC_R {
        ETERC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Start on inserted channel"]
    #[inline(always)]
    pub fn swicst(&self) -> SWICST_R {
        SWICST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 16 and 17 enable of ADC0"]
    #[inline(always)]
    pub fn tsvren(&self) -> TSVREN_R {
        TSVREN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    #[must_use]
    pub fn adcon(&mut self) -> ADCON_W<CTL1_SPEC, 0> {
        ADCON_W::new(self)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctn(&mut self) -> CTN_W<CTL1_SPEC, 1> {
        CTN_W::new(self)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    #[must_use]
    pub fn clb(&mut self) -> CLB_W<CTL1_SPEC, 2> {
        CLB_W::new(self)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    #[must_use]
    pub fn rstclb(&mut self) -> RSTCLB_W<CTL1_SPEC, 3> {
        RSTCLB_W::new(self)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<CTL1_SPEC, 8> {
        DMA_W::new(self)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn dal(&mut self) -> DAL_W<CTL1_SPEC, 11> {
        DAL_W::new(self)
    }
    #[doc = "Bits 12:14 - External trigger select for inserted channel"]
    #[inline(always)]
    #[must_use]
    pub fn etsic(&mut self) -> ETSIC_W<CTL1_SPEC, 12> {
        ETSIC_W::new(self)
    }
    #[doc = "Bit 15 - External trigger enable for insert channel"]
    #[inline(always)]
    #[must_use]
    pub fn eteic(&mut self) -> ETEIC_W<CTL1_SPEC, 15> {
        ETEIC_W::new(self)
    }
    #[doc = "Bits 17:19 - External trigger select for regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn etsrc(&mut self) -> ETSRC_W<CTL1_SPEC, 17> {
        ETSRC_W::new(self)
    }
    #[doc = "Bit 20 - External trigger enable for regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn eterc(&mut self) -> ETERC_W<CTL1_SPEC, 20> {
        ETERC_W::new(self)
    }
    #[doc = "Bit 21 - Start on inserted channel"]
    #[inline(always)]
    #[must_use]
    pub fn swicst(&mut self) -> SWICST_W<CTL1_SPEC, 21> {
        SWICST_W::new(self)
    }
    #[doc = "Bit 23 - Channel 16 and 17 enable of ADC0"]
    #[inline(always)]
    #[must_use]
    pub fn tsvren(&mut self) -> TSVREN_W<CTL1_SPEC, 23> {
        TSVREN_W::new(self)
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
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
