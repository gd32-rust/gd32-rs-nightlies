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
#[doc = "RTC hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_HOLD_A {
    #[doc = "0: Continue running the RTC as usual"]
    CONTINUE = 0,
    #[doc = "1: Hold the RTC for debug when the core is halted"]
    STOP = 1,
}
impl From<RTC_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_HOLD` reader - RTC hold register"]
pub type RTC_HOLD_R = crate::BitReader<RTC_HOLD_A>;
impl RTC_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_HOLD_A {
        match self.bits {
            false => RTC_HOLD_A::CONTINUE,
            true => RTC_HOLD_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == RTC_HOLD_A::CONTINUE
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RTC_HOLD_A::STOP
    }
}
#[doc = "Field `RTC_HOLD` writer - RTC hold register"]
pub type RTC_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, RTC_HOLD_A, 10>;
impl<'a> RTC_HOLD_W<'a> {
    #[doc = "Continue running the RTC as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(RTC_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the RTC for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(RTC_HOLD_A::STOP)
    }
}
#[doc = "Timer 14 hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER14_HOLD_A {
    #[doc = "0: Continue running the timer as usual"]
    CONTINUE = 0,
    #[doc = "1: Hold the timer counter for debug when the core is halted"]
    STOP = 1,
}
impl From<TIMER14_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER14_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER14_HOLD` reader - Timer 14 hold register"]
pub type TIMER14_HOLD_R = crate::BitReader<TIMER14_HOLD_A>;
impl TIMER14_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER14_HOLD_A {
        match self.bits {
            false => TIMER14_HOLD_A::CONTINUE,
            true => TIMER14_HOLD_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == TIMER14_HOLD_A::CONTINUE
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == TIMER14_HOLD_A::STOP
    }
}
#[doc = "Field `TIMER14_HOLD` writer - Timer 14 hold register"]
pub type TIMER14_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, TIMER14_HOLD_A, 16>;
impl<'a> TIMER14_HOLD_W<'a> {
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(TIMER14_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(TIMER14_HOLD_A::STOP)
    }
}
#[doc = "Timer 15 hold register"]
pub use TIMER14_HOLD_A as TIMER15_HOLD_A;
#[doc = "Timer 16 hold register"]
pub use TIMER14_HOLD_A as TIMER16_HOLD_A;
#[doc = "Field `TIMER15_HOLD` reader - Timer 15 hold register"]
pub use TIMER14_HOLD_R as TIMER15_HOLD_R;
#[doc = "Field `TIMER16_HOLD` reader - Timer 16 hold register"]
pub use TIMER14_HOLD_R as TIMER16_HOLD_R;
#[doc = "Field `TIMER15_HOLD` writer - Timer 15 hold register"]
pub use TIMER14_HOLD_W as TIMER15_HOLD_W;
#[doc = "Field `TIMER16_HOLD` writer - Timer 16 hold register"]
pub use TIMER14_HOLD_W as TIMER16_HOLD_W;
impl R {
    #[doc = "Bit 10 - RTC hold register"]
    #[inline(always)]
    pub fn rtc_hold(&self) -> RTC_HOLD_R {
        RTC_HOLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer 14 hold register"]
    #[inline(always)]
    pub fn timer14_hold(&self) -> TIMER14_HOLD_R {
        TIMER14_HOLD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer 15 hold register"]
    #[inline(always)]
    pub fn timer15_hold(&self) -> TIMER15_HOLD_R {
        TIMER15_HOLD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer 16 hold register"]
    #[inline(always)]
    pub fn timer16_hold(&self) -> TIMER16_HOLD_R {
        TIMER16_HOLD_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - RTC hold register"]
    #[inline(always)]
    pub fn rtc_hold(&mut self) -> RTC_HOLD_W {
        RTC_HOLD_W::new(self)
    }
    #[doc = "Bit 16 - Timer 14 hold register"]
    #[inline(always)]
    pub fn timer14_hold(&mut self) -> TIMER14_HOLD_W {
        TIMER14_HOLD_W::new(self)
    }
    #[doc = "Bit 17 - Timer 15 hold register"]
    #[inline(always)]
    pub fn timer15_hold(&mut self) -> TIMER15_HOLD_W {
        TIMER15_HOLD_W::new(self)
    }
    #[doc = "Bit 18 - Timer 16 hold register"]
    #[inline(always)]
    pub fn timer16_hold(&mut self) -> TIMER16_HOLD_W {
        TIMER16_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
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
