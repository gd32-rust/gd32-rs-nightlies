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
#[doc = "Port x configuration bit 15"]
pub use OM0_A as OM15_A;
#[doc = "Port x configuration bit 14"]
pub use OM0_A as OM14_A;
#[doc = "Port x configuration bit 13"]
pub use OM0_A as OM13_A;
#[doc = "Port x configuration bit 12"]
pub use OM0_A as OM12_A;
#[doc = "Port x configuration bit 11"]
pub use OM0_A as OM11_A;
#[doc = "Port x configuration bit 10"]
pub use OM0_A as OM10_A;
#[doc = "Port x configuration bit 9"]
pub use OM0_A as OM9_A;
#[doc = "Port x configuration bit 8"]
pub use OM0_A as OM8_A;
#[doc = "Port x configuration bit 7"]
pub use OM0_A as OM7_A;
#[doc = "Port x configuration bit 6"]
pub use OM0_A as OM6_A;
#[doc = "Port x configuration bit 5"]
pub use OM0_A as OM5_A;
#[doc = "Port x configuration bit 4"]
pub use OM0_A as OM4_A;
#[doc = "Port x configuration bit 3"]
pub use OM0_A as OM3_A;
#[doc = "Port x configuration bit 2"]
pub use OM0_A as OM2_A;
#[doc = "Port x configuration bit 1"]
pub use OM0_A as OM1_A;
#[doc = "Field `OM15` reader - Port x configuration bit 15"]
pub use OM0_R as OM15_R;
#[doc = "Field `OM14` reader - Port x configuration bit 14"]
pub use OM0_R as OM14_R;
#[doc = "Field `OM13` reader - Port x configuration bit 13"]
pub use OM0_R as OM13_R;
#[doc = "Field `OM12` reader - Port x configuration bit 12"]
pub use OM0_R as OM12_R;
#[doc = "Field `OM11` reader - Port x configuration bit 11"]
pub use OM0_R as OM11_R;
#[doc = "Field `OM10` reader - Port x configuration bit 10"]
pub use OM0_R as OM10_R;
#[doc = "Field `OM9` reader - Port x configuration bit 9"]
pub use OM0_R as OM9_R;
#[doc = "Field `OM8` reader - Port x configuration bit 8"]
pub use OM0_R as OM8_R;
#[doc = "Field `OM7` reader - Port x configuration bit 7"]
pub use OM0_R as OM7_R;
#[doc = "Field `OM6` reader - Port x configuration bit 6"]
pub use OM0_R as OM6_R;
#[doc = "Field `OM5` reader - Port x configuration bit 5"]
pub use OM0_R as OM5_R;
#[doc = "Field `OM4` reader - Port x configuration bit 4"]
pub use OM0_R as OM4_R;
#[doc = "Field `OM3` reader - Port x configuration bit 3"]
pub use OM0_R as OM3_R;
#[doc = "Field `OM2` reader - Port x configuration bit 2"]
pub use OM0_R as OM2_R;
#[doc = "Field `OM1` reader - Port x configuration bit 1"]
pub use OM0_R as OM1_R;
#[doc = "Field `OM15` writer - Port x configuration bit 15"]
pub use OM0_W as OM15_W;
#[doc = "Field `OM14` writer - Port x configuration bit 14"]
pub use OM0_W as OM14_W;
#[doc = "Field `OM13` writer - Port x configuration bit 13"]
pub use OM0_W as OM13_W;
#[doc = "Field `OM12` writer - Port x configuration bit 12"]
pub use OM0_W as OM12_W;
#[doc = "Field `OM11` writer - Port x configuration bit 11"]
pub use OM0_W as OM11_W;
#[doc = "Field `OM10` writer - Port x configuration bit 10"]
pub use OM0_W as OM10_W;
#[doc = "Field `OM9` writer - Port x configuration bit 9"]
pub use OM0_W as OM9_W;
#[doc = "Field `OM8` writer - Port x configuration bit 8"]
pub use OM0_W as OM8_W;
#[doc = "Field `OM7` writer - Port x configuration bit 7"]
pub use OM0_W as OM7_W;
#[doc = "Field `OM6` writer - Port x configuration bit 6"]
pub use OM0_W as OM6_W;
#[doc = "Field `OM5` writer - Port x configuration bit 5"]
pub use OM0_W as OM5_W;
#[doc = "Field `OM4` writer - Port x configuration bit 4"]
pub use OM0_W as OM4_W;
#[doc = "Field `OM3` writer - Port x configuration bit 3"]
pub use OM0_W as OM3_W;
#[doc = "Field `OM2` writer - Port x configuration bit 2"]
pub use OM0_W as OM2_W;
#[doc = "Field `OM1` writer - Port x configuration bit 1"]
pub use OM0_W as OM1_W;
#[doc = "Port x configuration bit 0\n\nValue on reset: 0"]
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
#[doc = "Field `OM0` reader - Port x configuration bit 0"]
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
#[doc = "Field `OM0` writer - Port x configuration bit 0"]
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
    #[doc = "Bit 15 - Port x configuration bit 15"]
    #[inline(always)]
    pub fn om15(&self) -> OM15_R {
        OM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x configuration bit 14"]
    #[inline(always)]
    pub fn om14(&self) -> OM14_R {
        OM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x configuration bit 13"]
    #[inline(always)]
    pub fn om13(&self) -> OM13_R {
        OM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x configuration bit 12"]
    #[inline(always)]
    pub fn om12(&self) -> OM12_R {
        OM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x configuration bit 11"]
    #[inline(always)]
    pub fn om11(&self) -> OM11_R {
        OM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x configuration bit 10"]
    #[inline(always)]
    pub fn om10(&self) -> OM10_R {
        OM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x configuration bit 9"]
    #[inline(always)]
    pub fn om9(&self) -> OM9_R {
        OM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x configuration bit 8"]
    #[inline(always)]
    pub fn om8(&self) -> OM8_R {
        OM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x configuration bit 7"]
    #[inline(always)]
    pub fn om7(&self) -> OM7_R {
        OM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x configuration bit 6"]
    #[inline(always)]
    pub fn om6(&self) -> OM6_R {
        OM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x configuration bit 5"]
    #[inline(always)]
    pub fn om5(&self) -> OM5_R {
        OM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x configuration bit 4"]
    #[inline(always)]
    pub fn om4(&self) -> OM4_R {
        OM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x configuration bit 3"]
    #[inline(always)]
    pub fn om3(&self) -> OM3_R {
        OM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x configuration bit 2"]
    #[inline(always)]
    pub fn om2(&self) -> OM2_R {
        OM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Port x configuration bit 1"]
    #[inline(always)]
    pub fn om1(&self) -> OM1_R {
        OM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port x configuration bit 0"]
    #[inline(always)]
    pub fn om0(&self) -> OM0_R {
        OM0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port x configuration bit 15"]
    #[inline(always)]
    pub fn om15(&mut self) -> OM15_W {
        OM15_W::new(self)
    }
    #[doc = "Bit 14 - Port x configuration bit 14"]
    #[inline(always)]
    pub fn om14(&mut self) -> OM14_W {
        OM14_W::new(self)
    }
    #[doc = "Bit 13 - Port x configuration bit 13"]
    #[inline(always)]
    pub fn om13(&mut self) -> OM13_W {
        OM13_W::new(self)
    }
    #[doc = "Bit 12 - Port x configuration bit 12"]
    #[inline(always)]
    pub fn om12(&mut self) -> OM12_W {
        OM12_W::new(self)
    }
    #[doc = "Bit 11 - Port x configuration bit 11"]
    #[inline(always)]
    pub fn om11(&mut self) -> OM11_W {
        OM11_W::new(self)
    }
    #[doc = "Bit 10 - Port x configuration bit 10"]
    #[inline(always)]
    pub fn om10(&mut self) -> OM10_W {
        OM10_W::new(self)
    }
    #[doc = "Bit 9 - Port x configuration bit 9"]
    #[inline(always)]
    pub fn om9(&mut self) -> OM9_W {
        OM9_W::new(self)
    }
    #[doc = "Bit 8 - Port x configuration bit 8"]
    #[inline(always)]
    pub fn om8(&mut self) -> OM8_W {
        OM8_W::new(self)
    }
    #[doc = "Bit 7 - Port x configuration bit 7"]
    #[inline(always)]
    pub fn om7(&mut self) -> OM7_W {
        OM7_W::new(self)
    }
    #[doc = "Bit 6 - Port x configuration bit 6"]
    #[inline(always)]
    pub fn om6(&mut self) -> OM6_W {
        OM6_W::new(self)
    }
    #[doc = "Bit 5 - Port x configuration bit 5"]
    #[inline(always)]
    pub fn om5(&mut self) -> OM5_W {
        OM5_W::new(self)
    }
    #[doc = "Bit 4 - Port x configuration bit 4"]
    #[inline(always)]
    pub fn om4(&mut self) -> OM4_W {
        OM4_W::new(self)
    }
    #[doc = "Bit 3 - Port x configuration bit 3"]
    #[inline(always)]
    pub fn om3(&mut self) -> OM3_W {
        OM3_W::new(self)
    }
    #[doc = "Bit 2 - Port x configuration bit 2"]
    #[inline(always)]
    pub fn om2(&mut self) -> OM2_W {
        OM2_W::new(self)
    }
    #[doc = "Bit 1 - Port x configuration bit 1"]
    #[inline(always)]
    pub fn om1(&mut self) -> OM1_W {
        OM1_W::new(self)
    }
    #[doc = "Bit 0 - Port x configuration bit 0"]
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
