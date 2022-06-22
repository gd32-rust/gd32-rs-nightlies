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
#[doc = "Sleep mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SLP_HOLD` reader - Sleep mode hold register"]
pub type SLP_HOLD_R = crate::BitReader<SLP_HOLD_A>;
impl SLP_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLP_HOLD_A {
        match self.bits {
            false => SLP_HOLD_A::DISABLED,
            true => SLP_HOLD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLP_HOLD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLP_HOLD_A::ENABLED
    }
}
#[doc = "Field `SLP_HOLD` writer - Sleep mode hold register"]
pub type SLP_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, SLP_HOLD_A, 0>;
impl<'a> SLP_HOLD_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLP_HOLD_A::DISABLED)
    }
    #[doc = "In sleep mode the AHB clock is on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLP_HOLD_A::ENABLED)
    }
}
#[doc = "Deep-sleep mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DSLP_HOLD` reader - Deep-sleep mode hold register"]
pub type DSLP_HOLD_R = crate::BitReader<DSLP_HOLD_A>;
impl DSLP_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSLP_HOLD_A {
        match self.bits {
            false => DSLP_HOLD_A::DISABLED,
            true => DSLP_HOLD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSLP_HOLD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSLP_HOLD_A::ENABLED
    }
}
#[doc = "Field `DSLP_HOLD` writer - Deep-sleep mode hold register"]
pub type DSLP_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, DSLP_HOLD_A, 1>;
impl<'a> DSLP_HOLD_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DSLP_HOLD_A::DISABLED)
    }
    #[doc = "In deep-sleep mode the AHB clock and system clock are provided by IRC8M"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DSLP_HOLD_A::ENABLED)
    }
}
#[doc = "Standby mode hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `STB_HOLD` reader - Standby mode hold register"]
pub type STB_HOLD_R = crate::BitReader<STB_HOLD_A>;
impl STB_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STB_HOLD_A {
        match self.bits {
            false => STB_HOLD_A::DISABLED,
            true => STB_HOLD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STB_HOLD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STB_HOLD_A::ENABLED
    }
}
#[doc = "Field `STB_HOLD` writer - Standby mode hold register"]
pub type STB_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, STB_HOLD_A, 2>;
impl<'a> STB_HOLD_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STB_HOLD_A::DISABLED)
    }
    #[doc = "In standby mode the AHB clock and system clock are provided by IRC8M"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STB_HOLD_A::ENABLED)
    }
}
#[doc = "FWDGT hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FWDGT_HOLD` reader - FWDGT hold register"]
pub type FWDGT_HOLD_R = crate::BitReader<FWDGT_HOLD_A>;
impl FWDGT_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWDGT_HOLD_A {
        match self.bits {
            false => FWDGT_HOLD_A::CONTINUE,
            true => FWDGT_HOLD_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == FWDGT_HOLD_A::CONTINUE
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == FWDGT_HOLD_A::STOP
    }
}
#[doc = "Field `FWDGT_HOLD` writer - FWDGT hold register"]
pub type FWDGT_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, FWDGT_HOLD_A, 8>;
impl<'a> FWDGT_HOLD_W<'a> {
    #[doc = "Continue running the watchdog timer as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(FWDGT_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the watchdog timer for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(FWDGT_HOLD_A::STOP)
    }
}
#[doc = "WWDGT hold register"]
pub use FWDGT_HOLD_A as WWDGT_HOLD_A;
#[doc = "Field `WWDGT_HOLD` reader - WWDGT hold register"]
pub use FWDGT_HOLD_R as WWDGT_HOLD_R;
#[doc = "Field `WWDGT_HOLD` writer - WWDGT hold register"]
pub use FWDGT_HOLD_W as WWDGT_HOLD_W;
#[doc = "Timer 0 hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TIMER0_HOLD` reader - Timer 0 hold register"]
pub type TIMER0_HOLD_R = crate::BitReader<TIMER0_HOLD_A>;
impl TIMER0_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_HOLD_A {
        match self.bits {
            false => TIMER0_HOLD_A::CONTINUE,
            true => TIMER0_HOLD_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == TIMER0_HOLD_A::CONTINUE
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == TIMER0_HOLD_A::STOP
    }
}
#[doc = "Field `TIMER0_HOLD` writer - Timer 0 hold register"]
pub type TIMER0_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, TIMER0_HOLD_A, 10>;
impl<'a> TIMER0_HOLD_W<'a> {
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(TIMER0_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(TIMER0_HOLD_A::STOP)
    }
}
#[doc = "Timer 1 hold register"]
pub use TIMER0_HOLD_A as TIMER1_HOLD_A;
#[doc = "Timer 2 hold register"]
pub use TIMER0_HOLD_A as TIMER2_HOLD_A;
#[doc = "Field `TIMER1_HOLD` reader - Timer 1 hold register"]
pub use TIMER0_HOLD_R as TIMER1_HOLD_R;
#[doc = "Field `TIMER2_HOLD` reader - Timer 2 hold register"]
pub use TIMER0_HOLD_R as TIMER2_HOLD_R;
#[doc = "Field `TIMER1_HOLD` writer - Timer 1 hold register"]
pub use TIMER0_HOLD_W as TIMER1_HOLD_W;
#[doc = "Field `TIMER2_HOLD` writer - Timer 2 hold register"]
pub use TIMER0_HOLD_W as TIMER2_HOLD_W;
#[doc = "I2C0 hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `I2C0_HOLD` reader - I2C0 hold register"]
pub type I2C0_HOLD_R = crate::BitReader<I2C0_HOLD_A>;
impl I2C0_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_HOLD_A {
        match self.bits {
            false => I2C0_HOLD_A::CONTINUE,
            true => I2C0_HOLD_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == I2C0_HOLD_A::CONTINUE
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == I2C0_HOLD_A::STOP
    }
}
#[doc = "Field `I2C0_HOLD` writer - I2C0 hold register"]
pub type I2C0_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, I2C0_HOLD_A, 15>;
impl<'a> I2C0_HOLD_W<'a> {
    #[doc = "Continue running I2C as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(I2C0_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the I2C timeout for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(I2C0_HOLD_A::STOP)
    }
}
#[doc = "I2C1 hold register"]
pub use I2C0_HOLD_A as I2C1_HOLD_A;
#[doc = "I2C2 hold register"]
pub use I2C0_HOLD_A as I2C2_HOLD_A;
#[doc = "Field `I2C1_HOLD` reader - I2C1 hold register"]
pub use I2C0_HOLD_R as I2C1_HOLD_R;
#[doc = "Field `I2C2_HOLD` reader - I2C2 hold register"]
pub use I2C0_HOLD_R as I2C2_HOLD_R;
#[doc = "Field `I2C1_HOLD` writer - I2C1 hold register"]
pub use I2C0_HOLD_W as I2C1_HOLD_W;
#[doc = "Field `I2C2_HOLD` writer - I2C2 hold register"]
pub use I2C0_HOLD_W as I2C2_HOLD_W;
#[doc = "Timer 5 hold register"]
pub use TIMER0_HOLD_A as TIMER5_HOLD_A;
#[doc = "Timer 13 hold register"]
pub use TIMER0_HOLD_A as TIMER13_HOLD_A;
#[doc = "Field `TIMER5_HOLD` reader - Timer 5 hold register"]
pub use TIMER0_HOLD_R as TIMER5_HOLD_R;
#[doc = "Field `TIMER13_HOLD` reader - Timer 13 hold register"]
pub use TIMER0_HOLD_R as TIMER13_HOLD_R;
#[doc = "Field `TIMER5_HOLD` writer - Timer 5 hold register"]
pub use TIMER0_HOLD_W as TIMER5_HOLD_W;
#[doc = "Field `TIMER13_HOLD` writer - Timer 13 hold register"]
pub use TIMER0_HOLD_W as TIMER13_HOLD_W;
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
    #[doc = "Bit 8 - FWDGT hold register"]
    #[inline(always)]
    pub fn fwdgt_hold(&self) -> FWDGT_HOLD_R {
        FWDGT_HOLD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WWDGT hold register"]
    #[inline(always)]
    pub fn wwdgt_hold(&self) -> WWDGT_HOLD_R {
        WWDGT_HOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer 0 hold register"]
    #[inline(always)]
    pub fn timer0_hold(&self) -> TIMER0_HOLD_R {
        TIMER0_HOLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 1 hold register"]
    #[inline(always)]
    pub fn timer1_hold(&self) -> TIMER1_HOLD_R {
        TIMER1_HOLD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer 2 hold register"]
    #[inline(always)]
    pub fn timer2_hold(&self) -> TIMER2_HOLD_R {
        TIMER2_HOLD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C0 hold register"]
    #[inline(always)]
    pub fn i2c0_hold(&self) -> I2C0_HOLD_R {
        I2C0_HOLD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C1 hold register"]
    #[inline(always)]
    pub fn i2c1_hold(&self) -> I2C1_HOLD_R {
        I2C1_HOLD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C2 hold register"]
    #[inline(always)]
    pub fn i2c2_hold(&self) -> I2C2_HOLD_R {
        I2C2_HOLD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer 5 hold register"]
    #[inline(always)]
    pub fn timer5_hold(&self) -> TIMER5_HOLD_R {
        TIMER5_HOLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer 13 hold register"]
    #[inline(always)]
    pub fn timer13_hold(&self) -> TIMER13_HOLD_R {
        TIMER13_HOLD_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&mut self) -> SLP_HOLD_W {
        SLP_HOLD_W::new(self)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    pub fn dslp_hold(&mut self) -> DSLP_HOLD_W {
        DSLP_HOLD_W::new(self)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    pub fn stb_hold(&mut self) -> STB_HOLD_W {
        STB_HOLD_W::new(self)
    }
    #[doc = "Bit 8 - FWDGT hold register"]
    #[inline(always)]
    pub fn fwdgt_hold(&mut self) -> FWDGT_HOLD_W {
        FWDGT_HOLD_W::new(self)
    }
    #[doc = "Bit 9 - WWDGT hold register"]
    #[inline(always)]
    pub fn wwdgt_hold(&mut self) -> WWDGT_HOLD_W {
        WWDGT_HOLD_W::new(self)
    }
    #[doc = "Bit 10 - Timer 0 hold register"]
    #[inline(always)]
    pub fn timer0_hold(&mut self) -> TIMER0_HOLD_W {
        TIMER0_HOLD_W::new(self)
    }
    #[doc = "Bit 11 - Timer 1 hold register"]
    #[inline(always)]
    pub fn timer1_hold(&mut self) -> TIMER1_HOLD_W {
        TIMER1_HOLD_W::new(self)
    }
    #[doc = "Bit 12 - Timer 2 hold register"]
    #[inline(always)]
    pub fn timer2_hold(&mut self) -> TIMER2_HOLD_W {
        TIMER2_HOLD_W::new(self)
    }
    #[doc = "Bit 15 - I2C0 hold register"]
    #[inline(always)]
    pub fn i2c0_hold(&mut self) -> I2C0_HOLD_W {
        I2C0_HOLD_W::new(self)
    }
    #[doc = "Bit 16 - I2C1 hold register"]
    #[inline(always)]
    pub fn i2c1_hold(&mut self) -> I2C1_HOLD_W {
        I2C1_HOLD_W::new(self)
    }
    #[doc = "Bit 17 - I2C2 hold register"]
    #[inline(always)]
    pub fn i2c2_hold(&mut self) -> I2C2_HOLD_W {
        I2C2_HOLD_W::new(self)
    }
    #[doc = "Bit 19 - Timer 5 hold register"]
    #[inline(always)]
    pub fn timer5_hold(&mut self) -> TIMER5_HOLD_W {
        TIMER5_HOLD_W::new(self)
    }
    #[doc = "Bit 27 - Timer 13 hold register"]
    #[inline(always)]
    pub fn timer13_hold(&mut self) -> TIMER13_HOLD_W {
        TIMER13_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
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
