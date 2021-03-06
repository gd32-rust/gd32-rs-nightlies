#[doc = "Register `OMODE` reader"]
pub struct R(crate::R<OMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OMODE` writer"]
pub struct W(crate::W<OMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OMODE_SPEC>;
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
impl From<crate::W<OMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pin 15 output mode"]
pub use OM0_A as OM15_A;
#[doc = "Pin 14 output mode"]
pub use OM0_A as OM14_A;
#[doc = "Pin 13 output mode"]
pub use OM0_A as OM13_A;
#[doc = "Pin 12 output mode"]
pub use OM0_A as OM12_A;
#[doc = "Pin 11 output mode"]
pub use OM0_A as OM11_A;
#[doc = "Pin 10 output mode"]
pub use OM0_A as OM10_A;
#[doc = "Pin 9 output mode"]
pub use OM0_A as OM9_A;
#[doc = "Pin 8 output mode"]
pub use OM0_A as OM8_A;
#[doc = "Pin 7 output mode"]
pub use OM0_A as OM7_A;
#[doc = "Pin 6 output mode"]
pub use OM0_A as OM6_A;
#[doc = "Pin 5 output mode"]
pub use OM0_A as OM5_A;
#[doc = "Pin 4 output mode"]
pub use OM0_A as OM4_A;
#[doc = "Pin 3 output mode"]
pub use OM0_A as OM3_A;
#[doc = "Pin 2 output mode"]
pub use OM0_A as OM2_A;
#[doc = "Pin 1 output mode"]
pub use OM0_A as OM1_A;
#[doc = "Field `OM15` reader - Pin 15 output mode"]
pub use OM0_R as OM15_R;
#[doc = "Field `OM14` reader - Pin 14 output mode"]
pub use OM0_R as OM14_R;
#[doc = "Field `OM13` reader - Pin 13 output mode"]
pub use OM0_R as OM13_R;
#[doc = "Field `OM12` reader - Pin 12 output mode"]
pub use OM0_R as OM12_R;
#[doc = "Field `OM11` reader - Pin 11 output mode"]
pub use OM0_R as OM11_R;
#[doc = "Field `OM10` reader - Pin 10 output mode"]
pub use OM0_R as OM10_R;
#[doc = "Field `OM9` reader - Pin 9 output mode"]
pub use OM0_R as OM9_R;
#[doc = "Field `OM8` reader - Pin 8 output mode"]
pub use OM0_R as OM8_R;
#[doc = "Field `OM7` reader - Pin 7 output mode"]
pub use OM0_R as OM7_R;
#[doc = "Field `OM6` reader - Pin 6 output mode"]
pub use OM0_R as OM6_R;
#[doc = "Field `OM5` reader - Pin 5 output mode"]
pub use OM0_R as OM5_R;
#[doc = "Field `OM4` reader - Pin 4 output mode"]
pub use OM0_R as OM4_R;
#[doc = "Field `OM3` reader - Pin 3 output mode"]
pub use OM0_R as OM3_R;
#[doc = "Field `OM2` reader - Pin 2 output mode"]
pub use OM0_R as OM2_R;
#[doc = "Field `OM1` reader - Pin 1 output mode"]
pub use OM0_R as OM1_R;
#[doc = "Field `OM15` writer - Pin 15 output mode"]
pub use OM0_W as OM15_W;
#[doc = "Field `OM14` writer - Pin 14 output mode"]
pub use OM0_W as OM14_W;
#[doc = "Field `OM13` writer - Pin 13 output mode"]
pub use OM0_W as OM13_W;
#[doc = "Field `OM12` writer - Pin 12 output mode"]
pub use OM0_W as OM12_W;
#[doc = "Field `OM11` writer - Pin 11 output mode"]
pub use OM0_W as OM11_W;
#[doc = "Field `OM10` writer - Pin 10 output mode"]
pub use OM0_W as OM10_W;
#[doc = "Field `OM9` writer - Pin 9 output mode"]
pub use OM0_W as OM9_W;
#[doc = "Field `OM8` writer - Pin 8 output mode"]
pub use OM0_W as OM8_W;
#[doc = "Field `OM7` writer - Pin 7 output mode"]
pub use OM0_W as OM7_W;
#[doc = "Field `OM6` writer - Pin 6 output mode"]
pub use OM0_W as OM6_W;
#[doc = "Field `OM5` writer - Pin 5 output mode"]
pub use OM0_W as OM5_W;
#[doc = "Field `OM4` writer - Pin 4 output mode"]
pub use OM0_W as OM4_W;
#[doc = "Field `OM3` writer - Pin 3 output mode"]
pub use OM0_W as OM3_W;
#[doc = "Field `OM2` writer - Pin 2 output mode"]
pub use OM0_W as OM2_W;
#[doc = "Field `OM1` writer - Pin 1 output mode"]
pub use OM0_W as OM1_W;
#[doc = "Pin 0 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OM0_A {
    #[doc = "0: Output push-pull (reset state)"]
    PUSHPULL = 0,
    #[doc = "1: Output open-drain"]
    OPENDRAIN = 1,
}
impl From<OM0_A> for bool {
    #[inline(always)]
    fn from(variant: OM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OM0` reader - Pin 0 output mode"]
pub type OM0_R = crate::BitReader<OM0_A>;
impl OM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OM0_A {
        match self.bits {
            false => OM0_A::PUSHPULL,
            true => OM0_A::OPENDRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OM0_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OPENDRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OM0_A::OPENDRAIN
    }
}
#[doc = "Field `OM0` writer - Pin 0 output mode"]
pub type OM0_W<'a> = crate::BitWriter<'a, u32, OMODE_SPEC, OM0_A, 0>;
impl<'a> OM0_W<'a> {
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OM0_A::PUSHPULL)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OM0_A::OPENDRAIN)
    }
}
impl R {
    #[doc = "Bit 15 - Pin 15 output mode"]
    #[inline(always)]
    pub fn om15(&self) -> OM15_R {
        OM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14 output mode"]
    #[inline(always)]
    pub fn om14(&self) -> OM14_R {
        OM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13 output mode"]
    #[inline(always)]
    pub fn om13(&self) -> OM13_R {
        OM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12 output mode"]
    #[inline(always)]
    pub fn om12(&self) -> OM12_R {
        OM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11 output mode"]
    #[inline(always)]
    pub fn om11(&self) -> OM11_R {
        OM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10 output mode"]
    #[inline(always)]
    pub fn om10(&self) -> OM10_R {
        OM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9 output mode"]
    #[inline(always)]
    pub fn om9(&self) -> OM9_R {
        OM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8 output mode"]
    #[inline(always)]
    pub fn om8(&self) -> OM8_R {
        OM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7 output mode"]
    #[inline(always)]
    pub fn om7(&self) -> OM7_R {
        OM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6 output mode"]
    #[inline(always)]
    pub fn om6(&self) -> OM6_R {
        OM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5 output mode"]
    #[inline(always)]
    pub fn om5(&self) -> OM5_R {
        OM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4 output mode"]
    #[inline(always)]
    pub fn om4(&self) -> OM4_R {
        OM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3 output mode"]
    #[inline(always)]
    pub fn om3(&self) -> OM3_R {
        OM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2 output mode"]
    #[inline(always)]
    pub fn om2(&self) -> OM2_R {
        OM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1 output mode"]
    #[inline(always)]
    pub fn om1(&self) -> OM1_R {
        OM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Pin 0 output mode"]
    #[inline(always)]
    pub fn om0(&self) -> OM0_R {
        OM0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Pin 15 output mode"]
    #[inline(always)]
    pub fn om15(&mut self) -> OM15_W {
        OM15_W::new(self)
    }
    #[doc = "Bit 14 - Pin 14 output mode"]
    #[inline(always)]
    pub fn om14(&mut self) -> OM14_W {
        OM14_W::new(self)
    }
    #[doc = "Bit 13 - Pin 13 output mode"]
    #[inline(always)]
    pub fn om13(&mut self) -> OM13_W {
        OM13_W::new(self)
    }
    #[doc = "Bit 12 - Pin 12 output mode"]
    #[inline(always)]
    pub fn om12(&mut self) -> OM12_W {
        OM12_W::new(self)
    }
    #[doc = "Bit 11 - Pin 11 output mode"]
    #[inline(always)]
    pub fn om11(&mut self) -> OM11_W {
        OM11_W::new(self)
    }
    #[doc = "Bit 10 - Pin 10 output mode"]
    #[inline(always)]
    pub fn om10(&mut self) -> OM10_W {
        OM10_W::new(self)
    }
    #[doc = "Bit 9 - Pin 9 output mode"]
    #[inline(always)]
    pub fn om9(&mut self) -> OM9_W {
        OM9_W::new(self)
    }
    #[doc = "Bit 8 - Pin 8 output mode"]
    #[inline(always)]
    pub fn om8(&mut self) -> OM8_W {
        OM8_W::new(self)
    }
    #[doc = "Bit 7 - Pin 7 output mode"]
    #[inline(always)]
    pub fn om7(&mut self) -> OM7_W {
        OM7_W::new(self)
    }
    #[doc = "Bit 6 - Pin 6 output mode"]
    #[inline(always)]
    pub fn om6(&mut self) -> OM6_W {
        OM6_W::new(self)
    }
    #[doc = "Bit 5 - Pin 5 output mode"]
    #[inline(always)]
    pub fn om5(&mut self) -> OM5_W {
        OM5_W::new(self)
    }
    #[doc = "Bit 4 - Pin 4 output mode"]
    #[inline(always)]
    pub fn om4(&mut self) -> OM4_W {
        OM4_W::new(self)
    }
    #[doc = "Bit 3 - Pin 3 output mode"]
    #[inline(always)]
    pub fn om3(&mut self) -> OM3_W {
        OM3_W::new(self)
    }
    #[doc = "Bit 2 - Pin 2 output mode"]
    #[inline(always)]
    pub fn om2(&mut self) -> OM2_W {
        OM2_W::new(self)
    }
    #[doc = "Bit 1 - Pin 1 output mode"]
    #[inline(always)]
    pub fn om1(&mut self) -> OM1_W {
        OM1_W::new(self)
    }
    #[doc = "Bit 0 - Pin 0 output mode"]
    #[inline(always)]
    pub fn om0(&mut self) -> OM0_W {
        OM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [omode](index.html) module"]
pub struct OMODE_SPEC;
impl crate::RegisterSpec for OMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [omode::R](R) reader structure"]
impl crate::Readable for OMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [omode::W](W) writer structure"]
impl crate::Writable for OMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OMODE to value 0"]
impl crate::Resettable for OMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
