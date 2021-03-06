#[doc = "Register `FSCFG` reader"]
pub struct R(crate::R<FSCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSCFG` writer"]
pub struct W(crate::W<FSCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSCFG_SPEC>;
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
impl From<crate::W<FSCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Filter scale\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS0_A {
    #[doc = "0: Filter with 16-bit scale"]
    SCALE16 = 0,
    #[doc = "1: Filter with 32-bit scale"]
    SCALE32 = 1,
}
impl From<FS0_A> for bool {
    #[inline(always)]
    fn from(variant: FS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FS0` reader - Filter scale"]
pub type FS0_R = crate::BitReader<FS0_A>;
impl FS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS0_A {
        match self.bits {
            false => FS0_A::SCALE16,
            true => FS0_A::SCALE32,
        }
    }
    #[doc = "Checks if the value of the field is `SCALE16`"]
    #[inline(always)]
    pub fn is_scale16(&self) -> bool {
        *self == FS0_A::SCALE16
    }
    #[doc = "Checks if the value of the field is `SCALE32`"]
    #[inline(always)]
    pub fn is_scale32(&self) -> bool {
        *self == FS0_A::SCALE32
    }
}
#[doc = "Field `FS0` writer - Filter scale"]
pub type FS0_W<'a> = crate::BitWriter<'a, u32, FSCFG_SPEC, FS0_A, 0>;
impl<'a> FS0_W<'a> {
    #[doc = "Filter with 16-bit scale"]
    #[inline(always)]
    pub fn scale16(self) -> &'a mut W {
        self.variant(FS0_A::SCALE16)
    }
    #[doc = "Filter with 32-bit scale"]
    #[inline(always)]
    pub fn scale32(self) -> &'a mut W {
        self.variant(FS0_A::SCALE32)
    }
}
#[doc = "Filter scale"]
pub use FS0_A as FS1_A;
#[doc = "Filter scale"]
pub use FS0_A as FS2_A;
#[doc = "Filter scale"]
pub use FS0_A as FS3_A;
#[doc = "Filter scale"]
pub use FS0_A as FS4_A;
#[doc = "Filter scale"]
pub use FS0_A as FS5_A;
#[doc = "Filter scale"]
pub use FS0_A as FS6_A;
#[doc = "Filter scale"]
pub use FS0_A as FS7_A;
#[doc = "Filter scale"]
pub use FS0_A as FS8_A;
#[doc = "Filter scale"]
pub use FS0_A as FS9_A;
#[doc = "Filter scale"]
pub use FS0_A as FS10_A;
#[doc = "Filter scale"]
pub use FS0_A as FS11_A;
#[doc = "Filter scale"]
pub use FS0_A as FS12_A;
#[doc = "Filter scale"]
pub use FS0_A as FS13_A;
#[doc = "Filter scale"]
pub use FS0_A as FS14_A;
#[doc = "Filter scale"]
pub use FS0_A as FS15_A;
#[doc = "Filter scale"]
pub use FS0_A as FS16_A;
#[doc = "Filter scale"]
pub use FS0_A as FS17_A;
#[doc = "Filter scale"]
pub use FS0_A as FS18_A;
#[doc = "Filter scale"]
pub use FS0_A as FS19_A;
#[doc = "Filter scale"]
pub use FS0_A as FS20_A;
#[doc = "Filter scale"]
pub use FS0_A as FS21_A;
#[doc = "Filter scale"]
pub use FS0_A as FS22_A;
#[doc = "Filter scale"]
pub use FS0_A as FS23_A;
#[doc = "Filter scale"]
pub use FS0_A as FS24_A;
#[doc = "Filter scale"]
pub use FS0_A as FS25_A;
#[doc = "Filter scale"]
pub use FS0_A as FS26_A;
#[doc = "Filter scale"]
pub use FS0_A as FS27_A;
#[doc = "Field `FS1` reader - Filter scale"]
pub use FS0_R as FS1_R;
#[doc = "Field `FS2` reader - Filter scale"]
pub use FS0_R as FS2_R;
#[doc = "Field `FS3` reader - Filter scale"]
pub use FS0_R as FS3_R;
#[doc = "Field `FS4` reader - Filter scale"]
pub use FS0_R as FS4_R;
#[doc = "Field `FS5` reader - Filter scale"]
pub use FS0_R as FS5_R;
#[doc = "Field `FS6` reader - Filter scale"]
pub use FS0_R as FS6_R;
#[doc = "Field `FS7` reader - Filter scale"]
pub use FS0_R as FS7_R;
#[doc = "Field `FS8` reader - Filter scale"]
pub use FS0_R as FS8_R;
#[doc = "Field `FS9` reader - Filter scale"]
pub use FS0_R as FS9_R;
#[doc = "Field `FS10` reader - Filter scale"]
pub use FS0_R as FS10_R;
#[doc = "Field `FS11` reader - Filter scale"]
pub use FS0_R as FS11_R;
#[doc = "Field `FS12` reader - Filter scale"]
pub use FS0_R as FS12_R;
#[doc = "Field `FS13` reader - Filter scale"]
pub use FS0_R as FS13_R;
#[doc = "Field `FS14` reader - Filter scale"]
pub use FS0_R as FS14_R;
#[doc = "Field `FS15` reader - Filter scale"]
pub use FS0_R as FS15_R;
#[doc = "Field `FS16` reader - Filter scale"]
pub use FS0_R as FS16_R;
#[doc = "Field `FS17` reader - Filter scale"]
pub use FS0_R as FS17_R;
#[doc = "Field `FS18` reader - Filter scale"]
pub use FS0_R as FS18_R;
#[doc = "Field `FS19` reader - Filter scale"]
pub use FS0_R as FS19_R;
#[doc = "Field `FS20` reader - Filter scale"]
pub use FS0_R as FS20_R;
#[doc = "Field `FS21` reader - Filter scale"]
pub use FS0_R as FS21_R;
#[doc = "Field `FS22` reader - Filter scale"]
pub use FS0_R as FS22_R;
#[doc = "Field `FS23` reader - Filter scale"]
pub use FS0_R as FS23_R;
#[doc = "Field `FS24` reader - Filter scale"]
pub use FS0_R as FS24_R;
#[doc = "Field `FS25` reader - Filter scale"]
pub use FS0_R as FS25_R;
#[doc = "Field `FS26` reader - Filter scale"]
pub use FS0_R as FS26_R;
#[doc = "Field `FS27` reader - Filter scale"]
pub use FS0_R as FS27_R;
#[doc = "Field `FS1` writer - Filter scale"]
pub use FS0_W as FS1_W;
#[doc = "Field `FS2` writer - Filter scale"]
pub use FS0_W as FS2_W;
#[doc = "Field `FS3` writer - Filter scale"]
pub use FS0_W as FS3_W;
#[doc = "Field `FS4` writer - Filter scale"]
pub use FS0_W as FS4_W;
#[doc = "Field `FS5` writer - Filter scale"]
pub use FS0_W as FS5_W;
#[doc = "Field `FS6` writer - Filter scale"]
pub use FS0_W as FS6_W;
#[doc = "Field `FS7` writer - Filter scale"]
pub use FS0_W as FS7_W;
#[doc = "Field `FS8` writer - Filter scale"]
pub use FS0_W as FS8_W;
#[doc = "Field `FS9` writer - Filter scale"]
pub use FS0_W as FS9_W;
#[doc = "Field `FS10` writer - Filter scale"]
pub use FS0_W as FS10_W;
#[doc = "Field `FS11` writer - Filter scale"]
pub use FS0_W as FS11_W;
#[doc = "Field `FS12` writer - Filter scale"]
pub use FS0_W as FS12_W;
#[doc = "Field `FS13` writer - Filter scale"]
pub use FS0_W as FS13_W;
#[doc = "Field `FS14` writer - Filter scale"]
pub use FS0_W as FS14_W;
#[doc = "Field `FS15` writer - Filter scale"]
pub use FS0_W as FS15_W;
#[doc = "Field `FS16` writer - Filter scale"]
pub use FS0_W as FS16_W;
#[doc = "Field `FS17` writer - Filter scale"]
pub use FS0_W as FS17_W;
#[doc = "Field `FS18` writer - Filter scale"]
pub use FS0_W as FS18_W;
#[doc = "Field `FS19` writer - Filter scale"]
pub use FS0_W as FS19_W;
#[doc = "Field `FS20` writer - Filter scale"]
pub use FS0_W as FS20_W;
#[doc = "Field `FS21` writer - Filter scale"]
pub use FS0_W as FS21_W;
#[doc = "Field `FS22` writer - Filter scale"]
pub use FS0_W as FS22_W;
#[doc = "Field `FS23` writer - Filter scale"]
pub use FS0_W as FS23_W;
#[doc = "Field `FS24` writer - Filter scale"]
pub use FS0_W as FS24_W;
#[doc = "Field `FS25` writer - Filter scale"]
pub use FS0_W as FS25_W;
#[doc = "Field `FS26` writer - Filter scale"]
pub use FS0_W as FS26_W;
#[doc = "Field `FS27` writer - Filter scale"]
pub use FS0_W as FS27_W;
impl R {
    #[doc = "Bit 0 - Filter scale"]
    #[inline(always)]
    pub fn fs0(&self) -> FS0_R {
        FS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter scale"]
    #[inline(always)]
    pub fn fs1(&self) -> FS1_R {
        FS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter scale"]
    #[inline(always)]
    pub fn fs2(&self) -> FS2_R {
        FS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter scale"]
    #[inline(always)]
    pub fn fs3(&self) -> FS3_R {
        FS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter scale"]
    #[inline(always)]
    pub fn fs4(&self) -> FS4_R {
        FS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter scale"]
    #[inline(always)]
    pub fn fs5(&self) -> FS5_R {
        FS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter scale"]
    #[inline(always)]
    pub fn fs6(&self) -> FS6_R {
        FS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter scale"]
    #[inline(always)]
    pub fn fs7(&self) -> FS7_R {
        FS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter scale"]
    #[inline(always)]
    pub fn fs8(&self) -> FS8_R {
        FS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter scale"]
    #[inline(always)]
    pub fn fs9(&self) -> FS9_R {
        FS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter scale"]
    #[inline(always)]
    pub fn fs10(&self) -> FS10_R {
        FS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter scale"]
    #[inline(always)]
    pub fn fs11(&self) -> FS11_R {
        FS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter scale"]
    #[inline(always)]
    pub fn fs12(&self) -> FS12_R {
        FS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter scale"]
    #[inline(always)]
    pub fn fs13(&self) -> FS13_R {
        FS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter scale"]
    #[inline(always)]
    pub fn fs14(&self) -> FS14_R {
        FS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter scale"]
    #[inline(always)]
    pub fn fs15(&self) -> FS15_R {
        FS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter scale"]
    #[inline(always)]
    pub fn fs16(&self) -> FS16_R {
        FS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter scale"]
    #[inline(always)]
    pub fn fs17(&self) -> FS17_R {
        FS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter scale"]
    #[inline(always)]
    pub fn fs18(&self) -> FS18_R {
        FS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter scale"]
    #[inline(always)]
    pub fn fs19(&self) -> FS19_R {
        FS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter scale"]
    #[inline(always)]
    pub fn fs20(&self) -> FS20_R {
        FS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter scale"]
    #[inline(always)]
    pub fn fs21(&self) -> FS21_R {
        FS21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter scale"]
    #[inline(always)]
    pub fn fs22(&self) -> FS22_R {
        FS22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter scale"]
    #[inline(always)]
    pub fn fs23(&self) -> FS23_R {
        FS23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter scale"]
    #[inline(always)]
    pub fn fs24(&self) -> FS24_R {
        FS24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter scale"]
    #[inline(always)]
    pub fn fs25(&self) -> FS25_R {
        FS25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter scale"]
    #[inline(always)]
    pub fn fs26(&self) -> FS26_R {
        FS26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter scale"]
    #[inline(always)]
    pub fn fs27(&self) -> FS27_R {
        FS27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter scale"]
    #[inline(always)]
    pub fn fs0(&mut self) -> FS0_W {
        FS0_W::new(self)
    }
    #[doc = "Bit 1 - Filter scale"]
    #[inline(always)]
    pub fn fs1(&mut self) -> FS1_W {
        FS1_W::new(self)
    }
    #[doc = "Bit 2 - Filter scale"]
    #[inline(always)]
    pub fn fs2(&mut self) -> FS2_W {
        FS2_W::new(self)
    }
    #[doc = "Bit 3 - Filter scale"]
    #[inline(always)]
    pub fn fs3(&mut self) -> FS3_W {
        FS3_W::new(self)
    }
    #[doc = "Bit 4 - Filter scale"]
    #[inline(always)]
    pub fn fs4(&mut self) -> FS4_W {
        FS4_W::new(self)
    }
    #[doc = "Bit 5 - Filter scale"]
    #[inline(always)]
    pub fn fs5(&mut self) -> FS5_W {
        FS5_W::new(self)
    }
    #[doc = "Bit 6 - Filter scale"]
    #[inline(always)]
    pub fn fs6(&mut self) -> FS6_W {
        FS6_W::new(self)
    }
    #[doc = "Bit 7 - Filter scale"]
    #[inline(always)]
    pub fn fs7(&mut self) -> FS7_W {
        FS7_W::new(self)
    }
    #[doc = "Bit 8 - Filter scale"]
    #[inline(always)]
    pub fn fs8(&mut self) -> FS8_W {
        FS8_W::new(self)
    }
    #[doc = "Bit 9 - Filter scale"]
    #[inline(always)]
    pub fn fs9(&mut self) -> FS9_W {
        FS9_W::new(self)
    }
    #[doc = "Bit 10 - Filter scale"]
    #[inline(always)]
    pub fn fs10(&mut self) -> FS10_W {
        FS10_W::new(self)
    }
    #[doc = "Bit 11 - Filter scale"]
    #[inline(always)]
    pub fn fs11(&mut self) -> FS11_W {
        FS11_W::new(self)
    }
    #[doc = "Bit 12 - Filter scale"]
    #[inline(always)]
    pub fn fs12(&mut self) -> FS12_W {
        FS12_W::new(self)
    }
    #[doc = "Bit 13 - Filter scale"]
    #[inline(always)]
    pub fn fs13(&mut self) -> FS13_W {
        FS13_W::new(self)
    }
    #[doc = "Bit 14 - Filter scale"]
    #[inline(always)]
    pub fn fs14(&mut self) -> FS14_W {
        FS14_W::new(self)
    }
    #[doc = "Bit 15 - Filter scale"]
    #[inline(always)]
    pub fn fs15(&mut self) -> FS15_W {
        FS15_W::new(self)
    }
    #[doc = "Bit 16 - Filter scale"]
    #[inline(always)]
    pub fn fs16(&mut self) -> FS16_W {
        FS16_W::new(self)
    }
    #[doc = "Bit 17 - Filter scale"]
    #[inline(always)]
    pub fn fs17(&mut self) -> FS17_W {
        FS17_W::new(self)
    }
    #[doc = "Bit 18 - Filter scale"]
    #[inline(always)]
    pub fn fs18(&mut self) -> FS18_W {
        FS18_W::new(self)
    }
    #[doc = "Bit 19 - Filter scale"]
    #[inline(always)]
    pub fn fs19(&mut self) -> FS19_W {
        FS19_W::new(self)
    }
    #[doc = "Bit 20 - Filter scale"]
    #[inline(always)]
    pub fn fs20(&mut self) -> FS20_W {
        FS20_W::new(self)
    }
    #[doc = "Bit 21 - Filter scale"]
    #[inline(always)]
    pub fn fs21(&mut self) -> FS21_W {
        FS21_W::new(self)
    }
    #[doc = "Bit 22 - Filter scale"]
    #[inline(always)]
    pub fn fs22(&mut self) -> FS22_W {
        FS22_W::new(self)
    }
    #[doc = "Bit 23 - Filter scale"]
    #[inline(always)]
    pub fn fs23(&mut self) -> FS23_W {
        FS23_W::new(self)
    }
    #[doc = "Bit 24 - Filter scale"]
    #[inline(always)]
    pub fn fs24(&mut self) -> FS24_W {
        FS24_W::new(self)
    }
    #[doc = "Bit 25 - Filter scale"]
    #[inline(always)]
    pub fn fs25(&mut self) -> FS25_W {
        FS25_W::new(self)
    }
    #[doc = "Bit 26 - Filter scale"]
    #[inline(always)]
    pub fn fs26(&mut self) -> FS26_W {
        FS26_W::new(self)
    }
    #[doc = "Bit 27 - Filter scale"]
    #[inline(always)]
    pub fn fs27(&mut self) -> FS27_W {
        FS27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter scale configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscfg](index.html) module"]
pub struct FSCFG_SPEC;
impl crate::RegisterSpec for FSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fscfg::R](R) reader structure"]
impl crate::Readable for FSCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fscfg::W](W) writer structure"]
impl crate::Writable for FSCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSCFG to value 0"]
impl crate::Resettable for FSCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
