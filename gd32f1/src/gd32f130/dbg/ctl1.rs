#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `RTC_HOLD` reader - RTC hold register"]
pub type RTC_HOLD_R = crate::BitReader<RTC_HOLD_A>;
#[doc = "RTC hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RTC_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_HOLD_A {
        match self.bits {
            false => RTC_HOLD_A::CONTINUE,
            true => RTC_HOLD_A::STOP,
        }
    }
    #[doc = "Continue running the RTC as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == RTC_HOLD_A::CONTINUE
    }
    #[doc = "Hold the RTC for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RTC_HOLD_A::STOP
    }
}
#[doc = "Field `RTC_HOLD` writer - RTC hold register"]
pub type RTC_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_HOLD_A>;
impl<'a, REG, const O: u8> RTC_HOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running the RTC as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the RTC for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_HOLD_A::STOP)
    }
}
#[doc = "Field `TIMER14_HOLD` reader - Timer 14 hold register"]
pub type TIMER14_HOLD_R = crate::BitReader<TIMER14_HOLD_A>;
#[doc = "Timer 14 hold register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TIMER14_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER14_HOLD_A {
        match self.bits {
            false => TIMER14_HOLD_A::CONTINUE,
            true => TIMER14_HOLD_A::STOP,
        }
    }
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == TIMER14_HOLD_A::CONTINUE
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == TIMER14_HOLD_A::STOP
    }
}
#[doc = "Field `TIMER14_HOLD` writer - Timer 14 hold register"]
pub type TIMER14_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER14_HOLD_A>;
impl<'a, REG, const O: u8> TIMER14_HOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running the timer as usual"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER14_HOLD_A::CONTINUE)
    }
    #[doc = "Hold the timer counter for debug when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER14_HOLD_A::STOP)
    }
}
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
    #[must_use]
    pub fn rtc_hold(&mut self) -> RTC_HOLD_W<CTL1_SPEC, 10> {
        RTC_HOLD_W::new(self)
    }
    #[doc = "Bit 16 - Timer 14 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer14_hold(&mut self) -> TIMER14_HOLD_W<CTL1_SPEC, 16> {
        TIMER14_HOLD_W::new(self)
    }
    #[doc = "Bit 17 - Timer 15 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer15_hold(&mut self) -> TIMER15_HOLD_W<CTL1_SPEC, 17> {
        TIMER15_HOLD_W::new(self)
    }
    #[doc = "Bit 18 - Timer 16 hold register"]
    #[inline(always)]
    #[must_use]
    pub fn timer16_hold(&mut self) -> TIMER16_HOLD_W<CTL1_SPEC, 18> {
        TIMER16_HOLD_W::new(self)
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
#[doc = "Debug Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
