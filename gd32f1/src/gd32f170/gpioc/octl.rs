#[doc = "Register `OCTL` reader"]
pub struct R(crate::R<OCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTL` writer"]
pub struct W(crate::W<OCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTL_SPEC>;
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
impl From<crate::W<OCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port output data 15"]
pub use OCTL0_A as OCTL15_A;
#[doc = "Port output data 14"]
pub use OCTL0_A as OCTL14_A;
#[doc = "Port output data 13"]
pub use OCTL0_A as OCTL13_A;
#[doc = "Port output data 12"]
pub use OCTL0_A as OCTL12_A;
#[doc = "Port output data 11"]
pub use OCTL0_A as OCTL11_A;
#[doc = "Port output data 10"]
pub use OCTL0_A as OCTL10_A;
#[doc = "Port output data 9"]
pub use OCTL0_A as OCTL9_A;
#[doc = "Port output data 8"]
pub use OCTL0_A as OCTL8_A;
#[doc = "Port output data 7"]
pub use OCTL0_A as OCTL7_A;
#[doc = "Port output data 6"]
pub use OCTL0_A as OCTL6_A;
#[doc = "Port output data 5"]
pub use OCTL0_A as OCTL5_A;
#[doc = "Port output data 4"]
pub use OCTL0_A as OCTL4_A;
#[doc = "Port output data 3"]
pub use OCTL0_A as OCTL3_A;
#[doc = "Port output data 2"]
pub use OCTL0_A as OCTL2_A;
#[doc = "Port output data 1"]
pub use OCTL0_A as OCTL1_A;
#[doc = "Field `OCTL15` reader - Port output data 15"]
pub use OCTL0_R as OCTL15_R;
#[doc = "Field `OCTL14` reader - Port output data 14"]
pub use OCTL0_R as OCTL14_R;
#[doc = "Field `OCTL13` reader - Port output data 13"]
pub use OCTL0_R as OCTL13_R;
#[doc = "Field `OCTL12` reader - Port output data 12"]
pub use OCTL0_R as OCTL12_R;
#[doc = "Field `OCTL11` reader - Port output data 11"]
pub use OCTL0_R as OCTL11_R;
#[doc = "Field `OCTL10` reader - Port output data 10"]
pub use OCTL0_R as OCTL10_R;
#[doc = "Field `OCTL9` reader - Port output data 9"]
pub use OCTL0_R as OCTL9_R;
#[doc = "Field `OCTL8` reader - Port output data 8"]
pub use OCTL0_R as OCTL8_R;
#[doc = "Field `OCTL7` reader - Port output data 7"]
pub use OCTL0_R as OCTL7_R;
#[doc = "Field `OCTL6` reader - Port output data 6"]
pub use OCTL0_R as OCTL6_R;
#[doc = "Field `OCTL5` reader - Port output data 5"]
pub use OCTL0_R as OCTL5_R;
#[doc = "Field `OCTL4` reader - Port output data 4"]
pub use OCTL0_R as OCTL4_R;
#[doc = "Field `OCTL3` reader - Port output data 3"]
pub use OCTL0_R as OCTL3_R;
#[doc = "Field `OCTL2` reader - Port output data 2"]
pub use OCTL0_R as OCTL2_R;
#[doc = "Field `OCTL1` reader - Port output data 1"]
pub use OCTL0_R as OCTL1_R;
#[doc = "Field `OCTL15` writer - Port output data 15"]
pub use OCTL0_W as OCTL15_W;
#[doc = "Field `OCTL14` writer - Port output data 14"]
pub use OCTL0_W as OCTL14_W;
#[doc = "Field `OCTL13` writer - Port output data 13"]
pub use OCTL0_W as OCTL13_W;
#[doc = "Field `OCTL12` writer - Port output data 12"]
pub use OCTL0_W as OCTL12_W;
#[doc = "Field `OCTL11` writer - Port output data 11"]
pub use OCTL0_W as OCTL11_W;
#[doc = "Field `OCTL10` writer - Port output data 10"]
pub use OCTL0_W as OCTL10_W;
#[doc = "Field `OCTL9` writer - Port output data 9"]
pub use OCTL0_W as OCTL9_W;
#[doc = "Field `OCTL8` writer - Port output data 8"]
pub use OCTL0_W as OCTL8_W;
#[doc = "Field `OCTL7` writer - Port output data 7"]
pub use OCTL0_W as OCTL7_W;
#[doc = "Field `OCTL6` writer - Port output data 6"]
pub use OCTL0_W as OCTL6_W;
#[doc = "Field `OCTL5` writer - Port output data 5"]
pub use OCTL0_W as OCTL5_W;
#[doc = "Field `OCTL4` writer - Port output data 4"]
pub use OCTL0_W as OCTL4_W;
#[doc = "Field `OCTL3` writer - Port output data 3"]
pub use OCTL0_W as OCTL3_W;
#[doc = "Field `OCTL2` writer - Port output data 2"]
pub use OCTL0_W as OCTL2_W;
#[doc = "Field `OCTL1` writer - Port output data 1"]
pub use OCTL0_W as OCTL1_W;
#[doc = "Port output data 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCTL0_A {
    #[doc = "0: Set output to logic low"]
    LOW = 0,
    #[doc = "1: Set output to logic high"]
    HIGH = 1,
}
impl From<OCTL0_A> for bool {
    #[inline(always)]
    fn from(variant: OCTL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCTL0` reader - Port output data 0"]
pub type OCTL0_R = crate::BitReader<OCTL0_A>;
impl OCTL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCTL0_A {
        match self.bits {
            false => OCTL0_A::LOW,
            true => OCTL0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OCTL0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OCTL0_A::HIGH
    }
}
#[doc = "Field `OCTL0` writer - Port output data 0"]
pub type OCTL0_W<'a> = crate::BitWriter<'a, u32, OCTL_SPEC, OCTL0_A, 0>;
impl<'a> OCTL0_W<'a> {
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OCTL0_A::LOW)
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OCTL0_A::HIGH)
    }
}
impl R {
    #[doc = "Bit 15 - Port output data 15"]
    #[inline(always)]
    pub fn octl15(&self) -> OCTL15_R {
        OCTL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data 14"]
    #[inline(always)]
    pub fn octl14(&self) -> OCTL14_R {
        OCTL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data 13"]
    #[inline(always)]
    pub fn octl13(&self) -> OCTL13_R {
        OCTL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data 12"]
    #[inline(always)]
    pub fn octl12(&self) -> OCTL12_R {
        OCTL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data 11"]
    #[inline(always)]
    pub fn octl11(&self) -> OCTL11_R {
        OCTL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data 10"]
    #[inline(always)]
    pub fn octl10(&self) -> OCTL10_R {
        OCTL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data 9"]
    #[inline(always)]
    pub fn octl9(&self) -> OCTL9_R {
        OCTL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data 8"]
    #[inline(always)]
    pub fn octl8(&self) -> OCTL8_R {
        OCTL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data 7"]
    #[inline(always)]
    pub fn octl7(&self) -> OCTL7_R {
        OCTL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data 6"]
    #[inline(always)]
    pub fn octl6(&self) -> OCTL6_R {
        OCTL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data 5"]
    #[inline(always)]
    pub fn octl5(&self) -> OCTL5_R {
        OCTL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data 4"]
    #[inline(always)]
    pub fn octl4(&self) -> OCTL4_R {
        OCTL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data 3"]
    #[inline(always)]
    pub fn octl3(&self) -> OCTL3_R {
        OCTL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data 2"]
    #[inline(always)]
    pub fn octl2(&self) -> OCTL2_R {
        OCTL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data 1"]
    #[inline(always)]
    pub fn octl1(&self) -> OCTL1_R {
        OCTL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port output data 0"]
    #[inline(always)]
    pub fn octl0(&self) -> OCTL0_R {
        OCTL0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port output data 15"]
    #[inline(always)]
    pub fn octl15(&mut self) -> OCTL15_W {
        OCTL15_W::new(self)
    }
    #[doc = "Bit 14 - Port output data 14"]
    #[inline(always)]
    pub fn octl14(&mut self) -> OCTL14_W {
        OCTL14_W::new(self)
    }
    #[doc = "Bit 13 - Port output data 13"]
    #[inline(always)]
    pub fn octl13(&mut self) -> OCTL13_W {
        OCTL13_W::new(self)
    }
    #[doc = "Bit 12 - Port output data 12"]
    #[inline(always)]
    pub fn octl12(&mut self) -> OCTL12_W {
        OCTL12_W::new(self)
    }
    #[doc = "Bit 11 - Port output data 11"]
    #[inline(always)]
    pub fn octl11(&mut self) -> OCTL11_W {
        OCTL11_W::new(self)
    }
    #[doc = "Bit 10 - Port output data 10"]
    #[inline(always)]
    pub fn octl10(&mut self) -> OCTL10_W {
        OCTL10_W::new(self)
    }
    #[doc = "Bit 9 - Port output data 9"]
    #[inline(always)]
    pub fn octl9(&mut self) -> OCTL9_W {
        OCTL9_W::new(self)
    }
    #[doc = "Bit 8 - Port output data 8"]
    #[inline(always)]
    pub fn octl8(&mut self) -> OCTL8_W {
        OCTL8_W::new(self)
    }
    #[doc = "Bit 7 - Port output data 7"]
    #[inline(always)]
    pub fn octl7(&mut self) -> OCTL7_W {
        OCTL7_W::new(self)
    }
    #[doc = "Bit 6 - Port output data 6"]
    #[inline(always)]
    pub fn octl6(&mut self) -> OCTL6_W {
        OCTL6_W::new(self)
    }
    #[doc = "Bit 5 - Port output data 5"]
    #[inline(always)]
    pub fn octl5(&mut self) -> OCTL5_W {
        OCTL5_W::new(self)
    }
    #[doc = "Bit 4 - Port output data 4"]
    #[inline(always)]
    pub fn octl4(&mut self) -> OCTL4_W {
        OCTL4_W::new(self)
    }
    #[doc = "Bit 3 - Port output data 3"]
    #[inline(always)]
    pub fn octl3(&mut self) -> OCTL3_W {
        OCTL3_W::new(self)
    }
    #[doc = "Bit 2 - Port output data 2"]
    #[inline(always)]
    pub fn octl2(&mut self) -> OCTL2_W {
        OCTL2_W::new(self)
    }
    #[doc = "Bit 1 - Port output data 1"]
    #[inline(always)]
    pub fn octl1(&mut self) -> OCTL1_W {
        OCTL1_W::new(self)
    }
    #[doc = "Bit 0 - Port output data 0"]
    #[inline(always)]
    pub fn octl0(&mut self) -> OCTL0_W {
        OCTL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octl](index.html) module"]
pub struct OCTL_SPEC;
impl crate::RegisterSpec for OCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octl::R](R) reader structure"]
impl crate::Readable for OCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octl::W](W) writer structure"]
impl crate::Writable for OCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCTL to value 0"]
impl crate::Resettable for OCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
