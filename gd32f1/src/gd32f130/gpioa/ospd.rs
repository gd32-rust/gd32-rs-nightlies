#[doc = "Register `OSPD` reader"]
pub struct R(crate::R<OSPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSPD` writer"]
pub struct W(crate::W<OSPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPD_SPEC>;
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
impl From<crate::W<OSPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pin 15 output max speed bits"]
pub use OSPD0_A as OSPD15_A;
#[doc = "Pin 14 output max speed bits"]
pub use OSPD0_A as OSPD14_A;
#[doc = "Pin 13 output max speed bits"]
pub use OSPD0_A as OSPD13_A;
#[doc = "Pin 12 output max speed bits"]
pub use OSPD0_A as OSPD12_A;
#[doc = "Pin 11 output max speed bits"]
pub use OSPD0_A as OSPD11_A;
#[doc = "Pin 10 output max speed bits"]
pub use OSPD0_A as OSPD10_A;
#[doc = "Pin 9 output max speed bits"]
pub use OSPD0_A as OSPD9_A;
#[doc = "Pin 8 output max speed bits"]
pub use OSPD0_A as OSPD8_A;
#[doc = "Pin 7 output max speed bits"]
pub use OSPD0_A as OSPD7_A;
#[doc = "Pin 6 output max speed bits"]
pub use OSPD0_A as OSPD6_A;
#[doc = "Pin 5 output max speed bits"]
pub use OSPD0_A as OSPD5_A;
#[doc = "Pin 4 output max speed bits"]
pub use OSPD0_A as OSPD4_A;
#[doc = "Pin 3 output max speed bits"]
pub use OSPD0_A as OSPD3_A;
#[doc = "Pin 2 output max speed bits"]
pub use OSPD0_A as OSPD2_A;
#[doc = "Pin 1 output max speed bits"]
pub use OSPD0_A as OSPD1_A;
#[doc = "Field `OSPD15` reader - Pin 15 output max speed bits"]
pub use OSPD0_R as OSPD15_R;
#[doc = "Field `OSPD14` reader - Pin 14 output max speed bits"]
pub use OSPD0_R as OSPD14_R;
#[doc = "Field `OSPD13` reader - Pin 13 output max speed bits"]
pub use OSPD0_R as OSPD13_R;
#[doc = "Field `OSPD12` reader - Pin 12 output max speed bits"]
pub use OSPD0_R as OSPD12_R;
#[doc = "Field `OSPD11` reader - Pin 11 output max speed bits"]
pub use OSPD0_R as OSPD11_R;
#[doc = "Field `OSPD10` reader - Pin 10 output max speed bits"]
pub use OSPD0_R as OSPD10_R;
#[doc = "Field `OSPD9` reader - Pin 9 output max speed bits"]
pub use OSPD0_R as OSPD9_R;
#[doc = "Field `OSPD8` reader - Pin 8 output max speed bits"]
pub use OSPD0_R as OSPD8_R;
#[doc = "Field `OSPD7` reader - Pin 7 output max speed bits"]
pub use OSPD0_R as OSPD7_R;
#[doc = "Field `OSPD6` reader - Pin 6 output max speed bits"]
pub use OSPD0_R as OSPD6_R;
#[doc = "Field `OSPD5` reader - Pin 5 output max speed bits"]
pub use OSPD0_R as OSPD5_R;
#[doc = "Field `OSPD4` reader - Pin 4 output max speed bits"]
pub use OSPD0_R as OSPD4_R;
#[doc = "Field `OSPD3` reader - Pin 3 output max speed bits"]
pub use OSPD0_R as OSPD3_R;
#[doc = "Field `OSPD2` reader - Pin 2 output max speed bits"]
pub use OSPD0_R as OSPD2_R;
#[doc = "Field `OSPD1` reader - Pin 1 output max speed bits"]
pub use OSPD0_R as OSPD1_R;
#[doc = "Field `OSPD15` writer - Pin 15 output max speed bits"]
pub use OSPD0_W as OSPD15_W;
#[doc = "Field `OSPD14` writer - Pin 14 output max speed bits"]
pub use OSPD0_W as OSPD14_W;
#[doc = "Field `OSPD13` writer - Pin 13 output max speed bits"]
pub use OSPD0_W as OSPD13_W;
#[doc = "Field `OSPD12` writer - Pin 12 output max speed bits"]
pub use OSPD0_W as OSPD12_W;
#[doc = "Field `OSPD11` writer - Pin 11 output max speed bits"]
pub use OSPD0_W as OSPD11_W;
#[doc = "Field `OSPD10` writer - Pin 10 output max speed bits"]
pub use OSPD0_W as OSPD10_W;
#[doc = "Field `OSPD9` writer - Pin 9 output max speed bits"]
pub use OSPD0_W as OSPD9_W;
#[doc = "Field `OSPD8` writer - Pin 8 output max speed bits"]
pub use OSPD0_W as OSPD8_W;
#[doc = "Field `OSPD7` writer - Pin 7 output max speed bits"]
pub use OSPD0_W as OSPD7_W;
#[doc = "Field `OSPD6` writer - Pin 6 output max speed bits"]
pub use OSPD0_W as OSPD6_W;
#[doc = "Field `OSPD5` writer - Pin 5 output max speed bits"]
pub use OSPD0_W as OSPD5_W;
#[doc = "Field `OSPD4` writer - Pin 4 output max speed bits"]
pub use OSPD0_W as OSPD4_W;
#[doc = "Field `OSPD3` writer - Pin 3 output max speed bits"]
pub use OSPD0_W as OSPD3_W;
#[doc = "Field `OSPD2` writer - Pin 2 output max speed bits"]
pub use OSPD0_W as OSPD2_W;
#[doc = "Field `OSPD1` writer - Pin 1 output max speed bits"]
pub use OSPD0_W as OSPD1_W;
#[doc = "Pin 0 output max speed bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSPD0_A {
    #[doc = "0: Max output speed 2 MHz"]
    SPEED2M = 0,
    #[doc = "1: Max output speed 10 MHz"]
    SPEED10M = 1,
    #[doc = "3: Max output speed 50 MHz"]
    SPEED50M = 3,
}
impl From<OSPD0_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPD0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OSPD0` reader - Pin 0 output max speed bits"]
pub type OSPD0_R = crate::FieldReader<u8, OSPD0_A>;
impl OSPD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSPD0_A> {
        match self.bits {
            0 => Some(OSPD0_A::SPEED2M),
            1 => Some(OSPD0_A::SPEED10M),
            3 => Some(OSPD0_A::SPEED50M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPEED2M`"]
    #[inline(always)]
    pub fn is_speed2m(&self) -> bool {
        *self == OSPD0_A::SPEED2M
    }
    #[doc = "Checks if the value of the field is `SPEED10M`"]
    #[inline(always)]
    pub fn is_speed10m(&self) -> bool {
        *self == OSPD0_A::SPEED10M
    }
    #[doc = "Checks if the value of the field is `SPEED50M`"]
    #[inline(always)]
    pub fn is_speed50m(&self) -> bool {
        *self == OSPD0_A::SPEED50M
    }
}
#[doc = "Field `OSPD0` writer - Pin 0 output max speed bits"]
pub type OSPD0_W<'a> = crate::FieldWriter<'a, u32, OSPD_SPEC, u8, OSPD0_A, 2, 0>;
impl<'a> OSPD0_W<'a> {
    #[doc = "Max output speed 2 MHz"]
    #[inline(always)]
    pub fn speed2m(self) -> &'a mut W {
        self.variant(OSPD0_A::SPEED2M)
    }
    #[doc = "Max output speed 10 MHz"]
    #[inline(always)]
    pub fn speed10m(self) -> &'a mut W {
        self.variant(OSPD0_A::SPEED10M)
    }
    #[doc = "Max output speed 50 MHz"]
    #[inline(always)]
    pub fn speed50m(self) -> &'a mut W {
        self.variant(OSPD0_A::SPEED50M)
    }
}
impl R {
    #[doc = "Bits 30:31 - Pin 15 output max speed bits"]
    #[inline(always)]
    pub fn ospd15(&self) -> OSPD15_R {
        OSPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Pin 14 output max speed bits"]
    #[inline(always)]
    pub fn ospd14(&self) -> OSPD14_R {
        OSPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin 13 output max speed bits"]
    #[inline(always)]
    pub fn ospd13(&self) -> OSPD13_R {
        OSPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pin 12 output max speed bits"]
    #[inline(always)]
    pub fn ospd12(&self) -> OSPD12_R {
        OSPD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin 11 output max speed bits"]
    #[inline(always)]
    pub fn ospd11(&self) -> OSPD11_R {
        OSPD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin 10 output max speed bits"]
    #[inline(always)]
    pub fn ospd10(&self) -> OSPD10_R {
        OSPD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin 9 output max speed bits"]
    #[inline(always)]
    pub fn ospd9(&self) -> OSPD9_R {
        OSPD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin 8 output max speed bits"]
    #[inline(always)]
    pub fn ospd8(&self) -> OSPD8_R {
        OSPD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin 7 output max speed bits"]
    #[inline(always)]
    pub fn ospd7(&self) -> OSPD7_R {
        OSPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin 6 output max speed bits"]
    #[inline(always)]
    pub fn ospd6(&self) -> OSPD6_R {
        OSPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin 5 output max speed bits"]
    #[inline(always)]
    pub fn ospd5(&self) -> OSPD5_R {
        OSPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin 4 output max speed bits"]
    #[inline(always)]
    pub fn ospd4(&self) -> OSPD4_R {
        OSPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin 3 output max speed bits"]
    #[inline(always)]
    pub fn ospd3(&self) -> OSPD3_R {
        OSPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin 2 output max speed bits"]
    #[inline(always)]
    pub fn ospd2(&self) -> OSPD2_R {
        OSPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin 1 output max speed bits"]
    #[inline(always)]
    pub fn ospd1(&self) -> OSPD1_R {
        OSPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Pin 0 output max speed bits"]
    #[inline(always)]
    pub fn ospd0(&self) -> OSPD0_R {
        OSPD0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Pin 15 output max speed bits"]
    #[inline(always)]
    pub fn ospd15(&mut self) -> OSPD15_W {
        OSPD15_W::new(self)
    }
    #[doc = "Bits 28:29 - Pin 14 output max speed bits"]
    #[inline(always)]
    pub fn ospd14(&mut self) -> OSPD14_W {
        OSPD14_W::new(self)
    }
    #[doc = "Bits 26:27 - Pin 13 output max speed bits"]
    #[inline(always)]
    pub fn ospd13(&mut self) -> OSPD13_W {
        OSPD13_W::new(self)
    }
    #[doc = "Bits 24:25 - Pin 12 output max speed bits"]
    #[inline(always)]
    pub fn ospd12(&mut self) -> OSPD12_W {
        OSPD12_W::new(self)
    }
    #[doc = "Bits 22:23 - Pin 11 output max speed bits"]
    #[inline(always)]
    pub fn ospd11(&mut self) -> OSPD11_W {
        OSPD11_W::new(self)
    }
    #[doc = "Bits 20:21 - Pin 10 output max speed bits"]
    #[inline(always)]
    pub fn ospd10(&mut self) -> OSPD10_W {
        OSPD10_W::new(self)
    }
    #[doc = "Bits 18:19 - Pin 9 output max speed bits"]
    #[inline(always)]
    pub fn ospd9(&mut self) -> OSPD9_W {
        OSPD9_W::new(self)
    }
    #[doc = "Bits 16:17 - Pin 8 output max speed bits"]
    #[inline(always)]
    pub fn ospd8(&mut self) -> OSPD8_W {
        OSPD8_W::new(self)
    }
    #[doc = "Bits 14:15 - Pin 7 output max speed bits"]
    #[inline(always)]
    pub fn ospd7(&mut self) -> OSPD7_W {
        OSPD7_W::new(self)
    }
    #[doc = "Bits 12:13 - Pin 6 output max speed bits"]
    #[inline(always)]
    pub fn ospd6(&mut self) -> OSPD6_W {
        OSPD6_W::new(self)
    }
    #[doc = "Bits 10:11 - Pin 5 output max speed bits"]
    #[inline(always)]
    pub fn ospd5(&mut self) -> OSPD5_W {
        OSPD5_W::new(self)
    }
    #[doc = "Bits 8:9 - Pin 4 output max speed bits"]
    #[inline(always)]
    pub fn ospd4(&mut self) -> OSPD4_W {
        OSPD4_W::new(self)
    }
    #[doc = "Bits 6:7 - Pin 3 output max speed bits"]
    #[inline(always)]
    pub fn ospd3(&mut self) -> OSPD3_W {
        OSPD3_W::new(self)
    }
    #[doc = "Bits 4:5 - Pin 2 output max speed bits"]
    #[inline(always)]
    pub fn ospd2(&mut self) -> OSPD2_W {
        OSPD2_W::new(self)
    }
    #[doc = "Bits 2:3 - Pin 1 output max speed bits"]
    #[inline(always)]
    pub fn ospd1(&mut self) -> OSPD1_W {
        OSPD1_W::new(self)
    }
    #[doc = "Bits 0:1 - Pin 0 output max speed bits"]
    #[inline(always)]
    pub fn ospd0(&mut self) -> OSPD0_W {
        OSPD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospd](index.html) module"]
pub struct OSPD_SPEC;
impl crate::RegisterSpec for OSPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ospd::R](R) reader structure"]
impl crate::Readable for OSPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ospd::W](W) writer structure"]
impl crate::Writable for OSPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSPD to value 0x0c00_0000"]
impl crate::Resettable for OSPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00_0000
    }
}
