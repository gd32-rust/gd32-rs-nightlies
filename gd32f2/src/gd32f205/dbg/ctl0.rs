#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `SLP_HOLD` reader - Sleep mode hold register"]
pub type SLP_HOLD_R = crate::BitReader<SLP_HOLD_A>;
#[doc = "Sleep mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLP_HOLD_A {
    #[doc = "0: No effect"]
    DISABLED = 0,
    #[doc = "1: In sleep mode the AHB clock is on"]
    ENABLED = 1,
}
impl From<SLP_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: SLP_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl SLP_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLP_HOLD_A {
        match self.bits {
            false => SLP_HOLD_A::DISABLED,
            true => SLP_HOLD_A::ENABLED,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLP_HOLD_A::DISABLED
    }
    #[doc = "In sleep mode the AHB clock is on"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLP_HOLD_A::ENABLED
    }
}
#[doc = "Field `SLP_HOLD` writer - Sleep mode hold register"]
pub type SLP_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLP_HOLD_A>;
impl<'a, REG, const O: u8> SLP_HOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLP_HOLD_A::DISABLED)
    }
    #[doc = "In sleep mode the AHB clock is on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLP_HOLD_A::ENABLED)
    }
}
#[doc = "Field `DSLP_HOLD` reader - Deep-sleep mode hold register"]
pub type DSLP_HOLD_R = crate::BitReader<DSLP_HOLD_A>;
#[doc = "Deep-sleep mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSLP_HOLD_A {
    #[doc = "0: No effect"]
    DISABLED = 0,
    #[doc = "1: In deep-sleep mode the AHB clock and system clock are provided by IRC8M"]
    ENABLED = 1,
}
impl From<DSLP_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: DSLP_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl DSLP_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSLP_HOLD_A {
        match self.bits {
            false => DSLP_HOLD_A::DISABLED,
            true => DSLP_HOLD_A::ENABLED,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSLP_HOLD_A::DISABLED
    }
    #[doc = "In deep-sleep mode the AHB clock and system clock are provided by IRC8M"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSLP_HOLD_A::ENABLED
    }
}
#[doc = "Field `DSLP_HOLD` writer - Deep-sleep mode hold register"]
pub type DSLP_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DSLP_HOLD_A>;
impl<'a, REG, const O: u8> DSLP_HOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSLP_HOLD_A::DISABLED)
    }
    #[doc = "In deep-sleep mode the AHB clock and system clock are provided by IRC8M"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSLP_HOLD_A::ENABLED)
    }
}
#[doc = "Field `STB_HOLD` reader - Standby mode hold register"]
pub type STB_HOLD_R = crate::BitReader<STB_HOLD_A>;
#[doc = "Standby mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STB_HOLD_A {
    #[doc = "0: No effect"]
    DISABLED = 0,
    #[doc = "1: In standby mode the AHB clock and system clock are provided by IRC8M"]
    ENABLED = 1,
}
impl From<STB_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: STB_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl STB_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STB_HOLD_A {
        match self.bits {
            false => STB_HOLD_A::DISABLED,
            true => STB_HOLD_A::ENABLED,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STB_HOLD_A::DISABLED
    }
    #[doc = "In standby mode the AHB clock and system clock are provided by IRC8M"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STB_HOLD_A::ENABLED
    }
}
#[doc = "Field `STB_HOLD` writer - Standby mode hold register"]
pub type STB_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STB_HOLD_A>;
impl<'a, REG, const O: u8> STB_HOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STB_HOLD_A::DISABLED)
    }
    #[doc = "In standby mode the AHB clock and system clock are provided by IRC8M"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(STB_HOLD_A::ENABLED)
    }
}
#[doc = "Field `TRACE_IOEN` reader - Trace pin allocation enable"]
pub type TRACE_IOEN_R = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - Trace pin allocation enable"]
pub type TRACE_IOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRACE_MODE` reader - Trace pin allocation mode"]
pub type TRACE_MODE_R = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - Trace pin allocation mode"]
pub type TRACE_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FWDGT_HOLD` reader - FWDGT hold bit"]
pub type FWDGT_HOLD_R = crate::BitReader<FWDGT_HOLD_A>;
#[doc = "FWDGT hold bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWDGT_HOLD_A {
    #[doc = "0: Continue running the watchdog timer as usual"]
    CONTINUE = 0,
    #[doc = "1: Hold the watchdog timer for debug when the core is halted"]
    STOP = 1,
}
impl From<FWDGT_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: FWDGT_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl FWDGT_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWDGT_HOLD_A {
        match self.bits {
            false => FWDGT_HOLD_A::CONTINUE,
            true => FWDGT_HOLD_A::STOP,
        }
    }
    #[doc = "Continue running the watchdog timer as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == FWDGT_HOLD_A::CONTINUE
    }
    #[doc = "Hold the watchdog timer for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == FWDGT_HOLD_A::STOP
    }
}
#[doc = "Field `FWDGT_HOLD` writer - FWDGT hold bit"]
pub type FWDGT_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FWDGT_HOLD_A>;
impl<'a, REG, const O: u8> FWDGT_HOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running the watchdog timer as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(FWDGT_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the watchdog timer for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(FWDGT_HOLD_A::STOP)
    }
}
#[doc = "Field `WWDGT_HOLD` reader - WWDGT hold bit"]
pub use FWDGT_HOLD_R as WWDGT_HOLD_R;
#[doc = "Field `WWDGT_HOLD` writer - WWDGT hold bit"]
pub use FWDGT_HOLD_W as WWDGT_HOLD_W;
#[doc = "Field `TIMER0_HOLD` reader - TIMER 0 hold bit"]
pub type TIMER0_HOLD_R = crate::BitReader<TIMER0_HOLD_A>;
#[doc = "TIMER 0 hold bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER0_HOLD_A {
    #[doc = "0: Continue running the timer as usual"]
    CONTINUE = 0,
    #[doc = "1: Hold the timer counter for debug when the core is halted"]
    STOP = 1,
}
impl From<TIMER0_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER0_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_HOLD_A {
        match self.bits {
            false => TIMER0_HOLD_A::CONTINUE,
            true => TIMER0_HOLD_A::STOP,
        }
    }
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == TIMER0_HOLD_A::CONTINUE
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == TIMER0_HOLD_A::STOP
    }
}
#[doc = "Field `TIMER0_HOLD` writer - TIMER 0 hold bit"]
pub type TIMER0_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER0_HOLD_A>;
impl<'a, REG, const O: u8> TIMER0_HOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER0_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER0_HOLD_A::STOP)
    }
}
#[doc = "Field `TIMER1_HOLD` reader - TIMER 1 hold bit"]
pub use TIMER0_HOLD_R as TIMER1_HOLD_R;
#[doc = "Field `TIMER2_HOLD` reader - TIMER 2 hold bit"]
pub use TIMER0_HOLD_R as TIMER2_HOLD_R;
#[doc = "Field `TIMER3_HOLD` reader - TIMER 23 hold bit"]
pub use TIMER0_HOLD_R as TIMER3_HOLD_R;
#[doc = "Field `TIMER1_HOLD` writer - TIMER 1 hold bit"]
pub use TIMER0_HOLD_W as TIMER1_HOLD_W;
#[doc = "Field `TIMER2_HOLD` writer - TIMER 2 hold bit"]
pub use TIMER0_HOLD_W as TIMER2_HOLD_W;
#[doc = "Field `TIMER3_HOLD` writer - TIMER 23 hold bit"]
pub use TIMER0_HOLD_W as TIMER3_HOLD_W;
#[doc = "Field `CAN0_HOLD` reader - CAN0 hold bit"]
pub type CAN0_HOLD_R = crate::BitReader<CAN0_HOLD_A>;
#[doc = "CAN0 hold bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAN0_HOLD_A {
    #[doc = "0: Continue running CAN as usual"]
    CONTINUE = 0,
    #[doc = "1: Hold the CAN for debug when the core is halted"]
    STOP = 1,
}
impl From<CAN0_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: CAN0_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl CAN0_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN0_HOLD_A {
        match self.bits {
            false => CAN0_HOLD_A::CONTINUE,
            true => CAN0_HOLD_A::STOP,
        }
    }
    #[doc = "Continue running CAN as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == CAN0_HOLD_A::CONTINUE
    }
    #[doc = "Hold the CAN for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CAN0_HOLD_A::STOP
    }
}
#[doc = "Field `CAN0_HOLD` writer - CAN0 hold bit"]
pub type CAN0_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAN0_HOLD_A>;
impl<'a, REG, const O: u8> CAN0_HOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running CAN as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(CAN0_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the CAN for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(CAN0_HOLD_A::STOP)
    }
}
#[doc = "Field `I2C0_HOLD` reader - I2C0 hold bit"]
pub type I2C0_HOLD_R = crate::BitReader<I2C0_HOLD_A>;
#[doc = "I2C0 hold bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C0_HOLD_A {
    #[doc = "0: Continue running I2C as usual"]
    CONTINUE = 0,
    #[doc = "1: Hold the I2C timeout for debug when the core is halted"]
    STOP = 1,
}
impl From<I2C0_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_HOLD_A {
        match self.bits {
            false => I2C0_HOLD_A::CONTINUE,
            true => I2C0_HOLD_A::STOP,
        }
    }
    #[doc = "Continue running I2C as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == I2C0_HOLD_A::CONTINUE
    }
    #[doc = "Hold the I2C timeout for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == I2C0_HOLD_A::STOP
    }
}
#[doc = "Field `I2C0_HOLD` writer - I2C0 hold bit"]
pub type I2C0_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2C0_HOLD_A>;
impl<'a, REG, const O: u8> I2C0_HOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running I2C as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the I2C timeout for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0_HOLD_A::STOP)
    }
}
#[doc = "Field `CAN1_HOLD` reader - CAN1 hold bit"]
pub use CAN0_HOLD_R as CAN1_HOLD_R;
#[doc = "Field `CAN1_HOLD` writer - CAN1 hold bit"]
pub use CAN0_HOLD_W as CAN1_HOLD_W;
#[doc = "Field `I2C1_HOLD` reader - I2C1 hold bit"]
pub use I2C0_HOLD_R as I2C1_HOLD_R;
#[doc = "Field `I2C1_HOLD` writer - I2C1 hold bit"]
pub use I2C0_HOLD_W as I2C1_HOLD_W;
#[doc = "Field `TIMER4_HOLD` reader - TIMER 4 hold bit"]
pub use TIMER0_HOLD_R as TIMER4_HOLD_R;
#[doc = "Field `TIMER5_HOLD` reader - TIMER 5 hold bit"]
pub use TIMER0_HOLD_R as TIMER5_HOLD_R;
#[doc = "Field `TIMER6_HOLD` reader - TIMER 6 hold bit"]
pub use TIMER0_HOLD_R as TIMER6_HOLD_R;
#[doc = "Field `TIMER7_HOLD` reader - TIMER 7 hold bit"]
pub use TIMER0_HOLD_R as TIMER7_HOLD_R;
#[doc = "Field `TIMER11_HOLD` reader - TIMER 11 hold bit"]
pub use TIMER0_HOLD_R as TIMER11_HOLD_R;
#[doc = "Field `TIMER12_HOLD` reader - TIMER 12 hold bit"]
pub use TIMER0_HOLD_R as TIMER12_HOLD_R;
#[doc = "Field `TIMER13_HOLD` reader - TIMER 13 hold bit"]
pub use TIMER0_HOLD_R as TIMER13_HOLD_R;
#[doc = "Field `TIMER8_HOLD` reader - TIMER 8 hold bit"]
pub use TIMER0_HOLD_R as TIMER8_HOLD_R;
#[doc = "Field `TIMER9_HOLD` reader - TIMER 9 hold bit"]
pub use TIMER0_HOLD_R as TIMER9_HOLD_R;
#[doc = "Field `TIMER10_HOLD` reader - TIMER 10 hold bit"]
pub use TIMER0_HOLD_R as TIMER10_HOLD_R;
#[doc = "Field `TIMER4_HOLD` writer - TIMER 4 hold bit"]
pub use TIMER0_HOLD_W as TIMER4_HOLD_W;
#[doc = "Field `TIMER5_HOLD` writer - TIMER 5 hold bit"]
pub use TIMER0_HOLD_W as TIMER5_HOLD_W;
#[doc = "Field `TIMER6_HOLD` writer - TIMER 6 hold bit"]
pub use TIMER0_HOLD_W as TIMER6_HOLD_W;
#[doc = "Field `TIMER7_HOLD` writer - TIMER 7 hold bit"]
pub use TIMER0_HOLD_W as TIMER7_HOLD_W;
#[doc = "Field `TIMER11_HOLD` writer - TIMER 11 hold bit"]
pub use TIMER0_HOLD_W as TIMER11_HOLD_W;
#[doc = "Field `TIMER12_HOLD` writer - TIMER 12 hold bit"]
pub use TIMER0_HOLD_W as TIMER12_HOLD_W;
#[doc = "Field `TIMER13_HOLD` writer - TIMER 13 hold bit"]
pub use TIMER0_HOLD_W as TIMER13_HOLD_W;
#[doc = "Field `TIMER8_HOLD` writer - TIMER 8 hold bit"]
pub use TIMER0_HOLD_W as TIMER8_HOLD_W;
#[doc = "Field `TIMER9_HOLD` writer - TIMER 9 hold bit"]
pub use TIMER0_HOLD_W as TIMER9_HOLD_W;
#[doc = "Field `TIMER10_HOLD` writer - TIMER 10 hold bit"]
pub use TIMER0_HOLD_W as TIMER10_HOLD_W;
impl R {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&self) -> SLP_HOLD_R {
        SLP_HOLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    pub fn dslp_hold(&self) -> DSLP_HOLD_R {
        DSLP_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    pub fn stb_hold(&self) -> STB_HOLD_R {
        STB_HOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Trace pin allocation enable"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Trace pin allocation mode"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - FWDGT hold bit"]
    #[inline(always)]
    pub fn fwdgt_hold(&self) -> FWDGT_HOLD_R {
        FWDGT_HOLD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WWDGT hold bit"]
    #[inline(always)]
    pub fn wwdgt_hold(&self) -> WWDGT_HOLD_R {
        WWDGT_HOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TIMER 0 hold bit"]
    #[inline(always)]
    pub fn timer0_hold(&self) -> TIMER0_HOLD_R {
        TIMER0_HOLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER 1 hold bit"]
    #[inline(always)]
    pub fn timer1_hold(&self) -> TIMER1_HOLD_R {
        TIMER1_HOLD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIMER 2 hold bit"]
    #[inline(always)]
    pub fn timer2_hold(&self) -> TIMER2_HOLD_R {
        TIMER2_HOLD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIMER 23 hold bit"]
    #[inline(always)]
    pub fn timer3_hold(&self) -> TIMER3_HOLD_R {
        TIMER3_HOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CAN0 hold bit"]
    #[inline(always)]
    pub fn can0_hold(&self) -> CAN0_HOLD_R {
        CAN0_HOLD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C0 hold bit"]
    #[inline(always)]
    pub fn i2c0_hold(&self) -> I2C0_HOLD_R {
        I2C0_HOLD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C1 hold bit"]
    #[inline(always)]
    pub fn i2c1_hold(&self) -> I2C1_HOLD_R {
        I2C1_HOLD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER 4 hold bit"]
    #[inline(always)]
    pub fn timer4_hold(&self) -> TIMER4_HOLD_R {
        TIMER4_HOLD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER 5 hold bit"]
    #[inline(always)]
    pub fn timer5_hold(&self) -> TIMER5_HOLD_R {
        TIMER5_HOLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TIMER 6 hold bit"]
    #[inline(always)]
    pub fn timer6_hold(&self) -> TIMER6_HOLD_R {
        TIMER6_HOLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIMER 7 hold bit"]
    #[inline(always)]
    pub fn timer7_hold(&self) -> TIMER7_HOLD_R {
        TIMER7_HOLD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CAN1 hold bit"]
    #[inline(always)]
    pub fn can1_hold(&self) -> CAN1_HOLD_R {
        CAN1_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - TIMER 11 hold bit"]
    #[inline(always)]
    pub fn timer11_hold(&self) -> TIMER11_HOLD_R {
        TIMER11_HOLD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TIMER 12 hold bit"]
    #[inline(always)]
    pub fn timer12_hold(&self) -> TIMER12_HOLD_R {
        TIMER12_HOLD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TIMER 13 hold bit"]
    #[inline(always)]
    pub fn timer13_hold(&self) -> TIMER13_HOLD_R {
        TIMER13_HOLD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TIMER 8 hold bit"]
    #[inline(always)]
    pub fn timer8_hold(&self) -> TIMER8_HOLD_R {
        TIMER8_HOLD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TIMER 9 hold bit"]
    #[inline(always)]
    pub fn timer9_hold(&self) -> TIMER9_HOLD_R {
        TIMER9_HOLD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TIMER 10 hold bit"]
    #[inline(always)]
    pub fn timer10_hold(&self) -> TIMER10_HOLD_R {
        TIMER10_HOLD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn slp_hold(&mut self) -> SLP_HOLD_W<CTL0_SPEC, 0> {
        SLP_HOLD_W::new(self)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn dslp_hold(&mut self) -> DSLP_HOLD_W<CTL0_SPEC, 1> {
        DSLP_HOLD_W::new(self)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn stb_hold(&mut self) -> STB_HOLD_W<CTL0_SPEC, 2> {
        STB_HOLD_W::new(self)
    }
    #[doc = "Bit 5 - Trace pin allocation enable"]
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<CTL0_SPEC, 5> {
        TRACE_IOEN_W::new(self)
    }
    #[doc = "Bits 6:7 - Trace pin allocation mode"]
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<CTL0_SPEC, 6> {
        TRACE_MODE_W::new(self)
    }
    #[doc = "Bit 8 - FWDGT hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn fwdgt_hold(&mut self) -> FWDGT_HOLD_W<CTL0_SPEC, 8> {
        FWDGT_HOLD_W::new(self)
    }
    #[doc = "Bit 9 - WWDGT hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgt_hold(&mut self) -> WWDGT_HOLD_W<CTL0_SPEC, 9> {
        WWDGT_HOLD_W::new(self)
    }
    #[doc = "Bit 10 - TIMER 0 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_hold(&mut self) -> TIMER0_HOLD_W<CTL0_SPEC, 10> {
        TIMER0_HOLD_W::new(self)
    }
    #[doc = "Bit 11 - TIMER 1 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_hold(&mut self) -> TIMER1_HOLD_W<CTL0_SPEC, 11> {
        TIMER1_HOLD_W::new(self)
    }
    #[doc = "Bit 12 - TIMER 2 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_hold(&mut self) -> TIMER2_HOLD_W<CTL0_SPEC, 12> {
        TIMER2_HOLD_W::new(self)
    }
    #[doc = "Bit 13 - TIMER 23 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer3_hold(&mut self) -> TIMER3_HOLD_W<CTL0_SPEC, 13> {
        TIMER3_HOLD_W::new(self)
    }
    #[doc = "Bit 14 - CAN0 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn can0_hold(&mut self) -> CAN0_HOLD_W<CTL0_SPEC, 14> {
        CAN0_HOLD_W::new(self)
    }
    #[doc = "Bit 15 - I2C0 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_hold(&mut self) -> I2C0_HOLD_W<CTL0_SPEC, 15> {
        I2C0_HOLD_W::new(self)
    }
    #[doc = "Bit 16 - I2C1 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_hold(&mut self) -> I2C1_HOLD_W<CTL0_SPEC, 16> {
        I2C1_HOLD_W::new(self)
    }
    #[doc = "Bit 17 - TIMER 4 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer4_hold(&mut self) -> TIMER4_HOLD_W<CTL0_SPEC, 17> {
        TIMER4_HOLD_W::new(self)
    }
    #[doc = "Bit 18 - TIMER 5 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer5_hold(&mut self) -> TIMER5_HOLD_W<CTL0_SPEC, 18> {
        TIMER5_HOLD_W::new(self)
    }
    #[doc = "Bit 19 - TIMER 6 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer6_hold(&mut self) -> TIMER6_HOLD_W<CTL0_SPEC, 19> {
        TIMER6_HOLD_W::new(self)
    }
    #[doc = "Bit 20 - TIMER 7 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer7_hold(&mut self) -> TIMER7_HOLD_W<CTL0_SPEC, 20> {
        TIMER7_HOLD_W::new(self)
    }
    #[doc = "Bit 21 - CAN1 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn can1_hold(&mut self) -> CAN1_HOLD_W<CTL0_SPEC, 21> {
        CAN1_HOLD_W::new(self)
    }
    #[doc = "Bit 25 - TIMER 11 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer11_hold(&mut self) -> TIMER11_HOLD_W<CTL0_SPEC, 25> {
        TIMER11_HOLD_W::new(self)
    }
    #[doc = "Bit 26 - TIMER 12 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer12_hold(&mut self) -> TIMER12_HOLD_W<CTL0_SPEC, 26> {
        TIMER12_HOLD_W::new(self)
    }
    #[doc = "Bit 27 - TIMER 13 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer13_hold(&mut self) -> TIMER13_HOLD_W<CTL0_SPEC, 27> {
        TIMER13_HOLD_W::new(self)
    }
    #[doc = "Bit 28 - TIMER 8 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer8_hold(&mut self) -> TIMER8_HOLD_W<CTL0_SPEC, 28> {
        TIMER8_HOLD_W::new(self)
    }
    #[doc = "Bit 29 - TIMER 9 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer9_hold(&mut self) -> TIMER9_HOLD_W<CTL0_SPEC, 29> {
        TIMER9_HOLD_W::new(self)
    }
    #[doc = "Bit 30 - TIMER 10 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer10_hold(&mut self) -> TIMER10_HOLD_W<CTL0_SPEC, 30> {
        TIMER10_HOLD_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
