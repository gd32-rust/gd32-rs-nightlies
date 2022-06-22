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
#[doc = "Field `RTC_HOLD` reader - RTC hold register"]
pub type RTC_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_HOLD` writer - RTC hold register"]
pub type RTC_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 10>;
#[doc = "Field `TIMER14_HOLD` reader - Timer 14 hold register"]
pub type TIMER14_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER14_HOLD` writer - Timer 14 hold register"]
pub type TIMER14_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 16>;
#[doc = "Field `TIMER15_HOLD` reader - Timer 15 hold register"]
pub type TIMER15_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER15_HOLD` writer - Timer 15 hold register"]
pub type TIMER15_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 17>;
#[doc = "Field `TIMER16_HOLD` reader - Timer 16 hold register"]
pub type TIMER16_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER16_HOLD` writer - Timer 16 hold register"]
pub type TIMER16_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 18>;
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
