#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 16 and 17 enable of ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TSVREN` reader - Channel 16 and 17 enable of ADC"]
pub type TSVREN_R = crate::BitReader<TSVREN_A>;
impl TSVREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSVREN_A {
        match self.bits {
            false => TSVREN_A::DISABLED,
            true => TSVREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREN_A::ENABLED
    }
}
#[doc = "Field `TSVREN` writer - Channel 16 and 17 enable of ADC"]
pub type TSVREN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, TSVREN_A, 23>;
impl<'a> TSVREN_W<'a> {
    #[doc = "Channel 16 and 17 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSVREN_A::DISABLED)
    }
    #[doc = "Channel 16 and 17 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSVREN_A::ENABLED)
    }
}
#[doc = "Start on regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRCST_A {
    #[doc = "0: Reset state"]
    STARTED = 0,
    #[doc = "1: Starting conversion of regular channels"]
    NOTSTARTED = 1,
}
impl From<SWRCST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRCST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRCST` reader - Start on regular channel"]
pub type SWRCST_R = crate::BitReader<SWRCST_A>;
impl SWRCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRCST_A {
        match self.bits {
            false => SWRCST_A::STARTED,
            true => SWRCST_A::NOTSTARTED,
        }
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SWRCST_A::STARTED
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == SWRCST_A::NOTSTARTED
    }
}
#[doc = "Start on regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRCST_AW {
    #[doc = "1: Start conversion of regular channels"]
    START = 1,
}
impl From<SWRCST_AW> for bool {
    #[inline(always)]
    fn from(variant: SWRCST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRCST` writer - Start on regular channel"]
pub type SWRCST_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, SWRCST_AW, 22>;
impl<'a> SWRCST_W<'a> {
    #[doc = "Start conversion of regular channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SWRCST_AW::START)
    }
}
#[doc = "Start on inserted channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWICST_A {
    #[doc = "0: Reset state"]
    STARTED = 0,
    #[doc = "1: Starting conversion of inserted channels"]
    NOTSTARTED = 1,
}
impl From<SWICST_A> for bool {
    #[inline(always)]
    fn from(variant: SWICST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWICST` reader - Start on inserted channel"]
pub type SWICST_R = crate::BitReader<SWICST_A>;
impl SWICST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWICST_A {
        match self.bits {
            false => SWICST_A::STARTED,
            true => SWICST_A::NOTSTARTED,
        }
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SWICST_A::STARTED
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == SWICST_A::NOTSTARTED
    }
}
#[doc = "Start on inserted channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWICST_AW {
    #[doc = "1: Start conversion of inserted channels"]
    START = 1,
}
impl From<SWICST_AW> for bool {
    #[inline(always)]
    fn from(variant: SWICST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWICST` writer - Start on inserted channel"]
pub type SWICST_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, SWICST_AW, 21>;
impl<'a> SWICST_W<'a> {
    #[doc = "Start conversion of inserted channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SWICST_AW::START)
    }
}
#[doc = "External trigger enable for regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ETERC` reader - External trigger enable for regular channel"]
pub type ETERC_R = crate::BitReader<ETERC_A>;
impl ETERC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETERC_A {
        match self.bits {
            false => ETERC_A::DISABLED,
            true => ETERC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ETERC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ETERC_A::ENABLED
    }
}
#[doc = "Field `ETERC` writer - External trigger enable for regular channel"]
pub type ETERC_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, ETERC_A, 20>;
impl<'a> ETERC_W<'a> {
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ETERC_A::DISABLED)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ETERC_A::ENABLED)
    }
}
#[doc = "External trigger select for regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ETSRC` reader - External trigger select for regular channel"]
pub type ETSRC_R = crate::FieldReader<u8, ETSRC_A>;
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
    #[doc = "Checks if the value of the field is `TIMER0CH0`"]
    #[inline(always)]
    pub fn is_timer0ch0(&self) -> bool {
        *self == ETSRC_A::TIMER0CH0
    }
    #[doc = "Checks if the value of the field is `TIMER0CH1`"]
    #[inline(always)]
    pub fn is_timer0ch1(&self) -> bool {
        *self == ETSRC_A::TIMER0CH1
    }
    #[doc = "Checks if the value of the field is `TIMER0CH2`"]
    #[inline(always)]
    pub fn is_timer0ch2(&self) -> bool {
        *self == ETSRC_A::TIMER0CH2
    }
    #[doc = "Checks if the value of the field is `TIMER1CH1`"]
    #[inline(always)]
    pub fn is_timer1ch1(&self) -> bool {
        *self == ETSRC_A::TIMER1CH1
    }
    #[doc = "Checks if the value of the field is `TIMER2TRGO`"]
    #[inline(always)]
    pub fn is_timer2trgo(&self) -> bool {
        *self == ETSRC_A::TIMER2TRGO
    }
    #[doc = "Checks if the value of the field is `TIMER14CH0`"]
    #[inline(always)]
    pub fn is_timer14ch0(&self) -> bool {
        *self == ETSRC_A::TIMER14CH0
    }
    #[doc = "Checks if the value of the field is `EXTI11`"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == ETSRC_A::EXTI11
    }
    #[doc = "Checks if the value of the field is `SWRCST`"]
    #[inline(always)]
    pub fn is_swrcst(&self) -> bool {
        *self == ETSRC_A::SWRCST
    }
}
#[doc = "Field `ETSRC` writer - External trigger select for regular channel"]
pub type ETSRC_W<'a> = crate::FieldWriterSafe<'a, u32, CTL1_SPEC, u8, ETSRC_A, 3, 17>;
impl<'a> ETSRC_W<'a> {
    #[doc = "Timer 0 channel 0 event"]
    #[inline(always)]
    pub fn timer0ch0(self) -> &'a mut W {
        self.variant(ETSRC_A::TIMER0CH0)
    }
    #[doc = "Timer 0 channel 1 event"]
    #[inline(always)]
    pub fn timer0ch1(self) -> &'a mut W {
        self.variant(ETSRC_A::TIMER0CH1)
    }
    #[doc = "Timer 0 channel 2 event"]
    #[inline(always)]
    pub fn timer0ch2(self) -> &'a mut W {
        self.variant(ETSRC_A::TIMER0CH2)
    }
    #[doc = "Timer 1 channel 1 event"]
    #[inline(always)]
    pub fn timer1ch1(self) -> &'a mut W {
        self.variant(ETSRC_A::TIMER1CH1)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn timer2trgo(self) -> &'a mut W {
        self.variant(ETSRC_A::TIMER2TRGO)
    }
    #[doc = "Timer 14 channel 0 event"]
    #[inline(always)]
    pub fn timer14ch0(self) -> &'a mut W {
        self.variant(ETSRC_A::TIMER14CH0)
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(ETSRC_A::EXTI11)
    }
    #[doc = "SWRCST"]
    #[inline(always)]
    pub fn swrcst(self) -> &'a mut W {
        self.variant(ETSRC_A::SWRCST)
    }
}
#[doc = "External trigger enable for inserted channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ETEIC` reader - External trigger enable for inserted channels"]
pub type ETEIC_R = crate::BitReader<ETEIC_A>;
impl ETEIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETEIC_A {
        match self.bits {
            false => ETEIC_A::DISABLED,
            true => ETEIC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ETEIC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ETEIC_A::ENABLED
    }
}
#[doc = "Field `ETEIC` writer - External trigger enable for inserted channels"]
pub type ETEIC_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, ETEIC_A, 15>;
impl<'a> ETEIC_W<'a> {
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ETEIC_A::DISABLED)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ETEIC_A::ENABLED)
    }
}
#[doc = "External trigger select for inserted channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ETSIC` reader - External trigger select for inserted channel"]
pub type ETSIC_R = crate::FieldReader<u8, ETSIC_A>;
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
    #[doc = "Checks if the value of the field is `TIMER0TRGO`"]
    #[inline(always)]
    pub fn is_timer0trgo(&self) -> bool {
        *self == ETSIC_A::TIMER0TRGO
    }
    #[doc = "Checks if the value of the field is `TIMER0CH3`"]
    #[inline(always)]
    pub fn is_timer0ch3(&self) -> bool {
        *self == ETSIC_A::TIMER0CH3
    }
    #[doc = "Checks if the value of the field is `TIMER1TRGO`"]
    #[inline(always)]
    pub fn is_timer1trgo(&self) -> bool {
        *self == ETSIC_A::TIMER1TRGO
    }
    #[doc = "Checks if the value of the field is `TIMER1CH0`"]
    #[inline(always)]
    pub fn is_timer1ch0(&self) -> bool {
        *self == ETSIC_A::TIMER1CH0
    }
    #[doc = "Checks if the value of the field is `TIMER2CH2`"]
    #[inline(always)]
    pub fn is_timer2ch2(&self) -> bool {
        *self == ETSIC_A::TIMER2CH2
    }
    #[doc = "Checks if the value of the field is `TIMER14TRGO`"]
    #[inline(always)]
    pub fn is_timer14trgo(&self) -> bool {
        *self == ETSIC_A::TIMER14TRGO
    }
    #[doc = "Checks if the value of the field is `EXTI15`"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == ETSIC_A::EXTI15
    }
    #[doc = "Checks if the value of the field is `SWICST`"]
    #[inline(always)]
    pub fn is_swicst(&self) -> bool {
        *self == ETSIC_A::SWICST
    }
}
#[doc = "Field `ETSIC` writer - External trigger select for inserted channel"]
pub type ETSIC_W<'a> = crate::FieldWriterSafe<'a, u32, CTL1_SPEC, u8, ETSIC_A, 3, 12>;
impl<'a> ETSIC_W<'a> {
    #[doc = "Timer 0 TRGO event"]
    #[inline(always)]
    pub fn timer0trgo(self) -> &'a mut W {
        self.variant(ETSIC_A::TIMER0TRGO)
    }
    #[doc = "Timer 0 channel 3 event"]
    #[inline(always)]
    pub fn timer0ch3(self) -> &'a mut W {
        self.variant(ETSIC_A::TIMER0CH3)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn timer1trgo(self) -> &'a mut W {
        self.variant(ETSIC_A::TIMER1TRGO)
    }
    #[doc = "Timer 1 channel 0 event"]
    #[inline(always)]
    pub fn timer1ch0(self) -> &'a mut W {
        self.variant(ETSIC_A::TIMER1CH0)
    }
    #[doc = "Timer 2 channel 3 event"]
    #[inline(always)]
    pub fn timer2ch2(self) -> &'a mut W {
        self.variant(ETSIC_A::TIMER2CH2)
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn timer14trgo(self) -> &'a mut W {
        self.variant(ETSIC_A::TIMER14TRGO)
    }
    #[doc = "EXTI line 15"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(ETSIC_A::EXTI15)
    }
    #[doc = "SWICST"]
    #[inline(always)]
    pub fn swicst(self) -> &'a mut W {
        self.variant(ETSIC_A::SWICST)
    }
}
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DAL` reader - Data alignment"]
pub type DAL_R = crate::BitReader<DAL_A>;
impl DAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAL_A {
        match self.bits {
            false => DAL_A::RIGHT,
            true => DAL_A::LEFT,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == DAL_A::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == DAL_A::LEFT
    }
}
#[doc = "Field `DAL` writer - Data alignment"]
pub type DAL_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, DAL_A, 11>;
impl<'a> DAL_W<'a> {
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(DAL_A::RIGHT)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(DAL_A::LEFT)
    }
}
#[doc = "DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DMA` reader - DMA request enable"]
pub type DMA_R = crate::BitReader<DMA_A>;
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::DISABLED,
            true => DMA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_A::ENABLED
    }
}
#[doc = "Field `DMA` writer - DMA request enable"]
pub type DMA_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, DMA_A, 8>;
impl<'a> DMA_W<'a> {
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_A::DISABLED)
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_A::ENABLED)
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTCLB_A {
    #[doc = "0: Calibration completed"]
    COMPLETE = 0,
    #[doc = "1: Calibrating"]
    NOTCOMPLETE = 1,
}
impl From<RSTCLB_A> for bool {
    #[inline(always)]
    fn from(variant: RSTCLB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCLB` reader - Reset calibration"]
pub type RSTCLB_R = crate::BitReader<RSTCLB_A>;
impl RSTCLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTCLB_A {
        match self.bits {
            false => RSTCLB_A::COMPLETE,
            true => RSTCLB_A::NOTCOMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == RSTCLB_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == RSTCLB_A::NOTCOMPLETE
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTCLB_AW {
    #[doc = "1: Enable calibration"]
    START = 1,
}
impl From<RSTCLB_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTCLB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCLB` writer - Reset calibration"]
pub type RSTCLB_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, RSTCLB_AW, 3>;
impl<'a> RSTCLB_W<'a> {
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(RSTCLB_AW::START)
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLB_A {
    #[doc = "0: Calibration completed"]
    COMPLETE = 0,
    #[doc = "1: Calibrating"]
    NOTCOMPLETE = 1,
}
impl From<CLB_A> for bool {
    #[inline(always)]
    fn from(variant: CLB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLB` reader - ADC calibration"]
pub type CLB_R = crate::BitReader<CLB_A>;
impl CLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLB_A {
        match self.bits {
            false => CLB_A::COMPLETE,
            true => CLB_A::NOTCOMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CLB_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CLB_A::NOTCOMPLETE
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLB_AW {
    #[doc = "1: Enable calibration"]
    START = 1,
}
impl From<CLB_AW> for bool {
    #[inline(always)]
    fn from(variant: CLB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLB` writer - ADC calibration"]
pub type CLB_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, CLB_AW, 2>;
impl<'a> CLB_W<'a> {
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CLB_AW::START)
    }
}
#[doc = "Continuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CTN` reader - Continuous mode"]
pub type CTN_R = crate::BitReader<CTN_A>;
impl CTN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTN_A {
        match self.bits {
            false => CTN_A::SINGLE,
            true => CTN_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CTN_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CTN_A::CONTINUOUS
    }
}
#[doc = "Field `CTN` writer - Continuous mode"]
pub type CTN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, CTN_A, 1>;
impl<'a> CTN_W<'a> {
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CTN_A::SINGLE)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CTN_A::CONTINUOUS)
    }
}
#[doc = "ADC ON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ADCON` reader - ADC ON"]
pub type ADCON_R = crate::BitReader<ADCON_A>;
impl ADCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCON_A {
        match self.bits {
            false => ADCON_A::DISABLED,
            true => ADCON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCON_A::ENABLED
    }
}
#[doc = "Field `ADCON` writer - ADC ON"]
pub type ADCON_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, ADCON_A, 0>;
impl<'a> ADCON_W<'a> {
    #[doc = "Disable ADC conversion/calibration and go to power down mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCON_A::DISABLED)
    }
    #[doc = "Enable ADC and to start conversion"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCON_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 23 - Channel 16 and 17 enable of ADC"]
    #[inline(always)]
    pub fn tsvren(&self) -> TSVREN_R {
        TSVREN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - Start on regular channel"]
    #[inline(always)]
    pub fn swrcst(&self) -> SWRCST_R {
        SWRCST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Start on inserted channel"]
    #[inline(always)]
    pub fn swicst(&self) -> SWICST_R {
        SWICST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - External trigger enable for regular channel"]
    #[inline(always)]
    pub fn eterc(&self) -> ETERC_R {
        ETERC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 17:19 - External trigger select for regular channel"]
    #[inline(always)]
    pub fn etsrc(&self) -> ETSRC_R {
        ETSRC_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 15 - External trigger enable for inserted channels"]
    #[inline(always)]
    pub fn eteic(&self) -> ETEIC_R {
        ETEIC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn etsic(&self) -> ETSIC_R {
        ETSIC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&self) -> DAL_R {
        DAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstclb(&self) -> RSTCLB_R {
        RSTCLB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    pub fn clb(&self) -> CLB_R {
        CLB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&self) -> CTN_R {
        CTN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - ADC ON"]
    #[inline(always)]
    pub fn adcon(&self) -> ADCON_R {
        ADCON_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Channel 16 and 17 enable of ADC"]
    #[inline(always)]
    pub fn tsvren(&mut self) -> TSVREN_W {
        TSVREN_W::new(self)
    }
    #[doc = "Bit 22 - Start on regular channel"]
    #[inline(always)]
    pub fn swrcst(&mut self) -> SWRCST_W {
        SWRCST_W::new(self)
    }
    #[doc = "Bit 21 - Start on inserted channel"]
    #[inline(always)]
    pub fn swicst(&mut self) -> SWICST_W {
        SWICST_W::new(self)
    }
    #[doc = "Bit 20 - External trigger enable for regular channel"]
    #[inline(always)]
    pub fn eterc(&mut self) -> ETERC_W {
        ETERC_W::new(self)
    }
    #[doc = "Bits 17:19 - External trigger select for regular channel"]
    #[inline(always)]
    pub fn etsrc(&mut self) -> ETSRC_W {
        ETSRC_W::new(self)
    }
    #[doc = "Bit 15 - External trigger enable for inserted channels"]
    #[inline(always)]
    pub fn eteic(&mut self) -> ETEIC_W {
        ETEIC_W::new(self)
    }
    #[doc = "Bits 12:14 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn etsic(&mut self) -> ETSIC_W {
        ETSIC_W::new(self)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&mut self) -> DAL_W {
        DAL_W::new(self)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W::new(self)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstclb(&mut self) -> RSTCLB_W {
        RSTCLB_W::new(self)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    pub fn clb(&mut self) -> CLB_W {
        CLB_W::new(self)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&mut self) -> CTN_W {
        CTN_W::new(self)
    }
    #[doc = "Bit 0 - ADC ON"]
    #[inline(always)]
    pub fn adcon(&mut self) -> ADCON_W {
        ADCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
