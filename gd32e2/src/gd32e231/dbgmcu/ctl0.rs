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
#[doc = "Field `SLP_HOLD` reader - Sleep mode hold register"]
pub type SLP_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `SLP_HOLD` writer - Sleep mode hold register"]
pub type SLP_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 0>;
#[doc = "Field `DSLP_HOLD` reader - DEEPSLEEP mode hold Mode"]
pub type DSLP_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `DSLP_HOLD` writer - DEEPSLEEP mode hold Mode"]
pub type DSLP_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 1>;
#[doc = "Field `STB_HOLD` reader - Standby mode hold Mode"]
pub type STB_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `STB_HOLD` writer - Standby mode hold Mode"]
pub type STB_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 2>;
#[doc = "Field `FWDGT_HOLD` reader - FWDGT hold register"]
pub type FWDGT_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `FWDGT_HOLD` writer - FWDGT hold register"]
pub type FWDGT_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 8>;
#[doc = "Field `WWDGT_HOLD` reader - WWDGT hold register"]
pub type WWDGT_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `WWDGT_HOLD` writer - WWDGT hold register"]
pub type WWDGT_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 9>;
#[doc = "Field `TIMER0_HOLD` reader - Timer 0 hold register"]
pub type TIMER0_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0_HOLD` writer - Timer 0 hold register"]
pub type TIMER0_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 10>;
#[doc = "Field `TIMER2_HOLD` reader - Timer 2 hold register"]
pub type TIMER2_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2_HOLD` writer - Timer 2 hold register"]
pub type TIMER2_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 12>;
#[doc = "Field `I2C0_HOLD` reader - I2C0 hold register"]
pub type I2C0_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `I2C0_HOLD` writer - I2C0 hold register"]
pub type I2C0_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 15>;
#[doc = "Field `I2C1_HOLD` reader - I2C1 hold register"]
pub type I2C1_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `I2C1_HOLD` writer - I2C1 hold register"]
pub type I2C1_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 16>;
#[doc = "Field `TIMER5_HOLD` reader - Timer 5 hold register"]
pub type TIMER5_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER5_HOLD` writer - Timer 5 hold register"]
pub type TIMER5_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 19>;
#[doc = "Field `TIMER13_HOLD` reader - Timer 13 hold register"]
pub type TIMER13_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER13_HOLD` writer - Timer 13 hold register"]
pub type TIMER13_HOLD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 27>;
impl R {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&self) -> SLP_HOLD_R {
        SLP_HOLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DEEPSLEEP mode hold Mode"]
    #[inline(always)]
    pub fn dslp_hold(&self) -> DSLP_HOLD_R {
        DSLP_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Standby mode hold Mode"]
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
    #[doc = "Bit 1 - DEEPSLEEP mode hold Mode"]
    #[inline(always)]
    pub fn dslp_hold(&mut self) -> DSLP_HOLD_W {
        DSLP_HOLD_W::new(self)
    }
    #[doc = "Bit 2 - Standby mode hold Mode"]
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
