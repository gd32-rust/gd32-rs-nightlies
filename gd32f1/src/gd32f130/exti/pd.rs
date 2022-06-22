#[doc = "Register `PD` reader"]
pub struct R(crate::R<PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD` writer"]
pub struct W(crate::W<PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_SPEC>;
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
impl From<crate::W<PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt pending status of line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD0_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PD0_A> for bool {
    #[inline(always)]
    fn from(variant: PD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD0` reader - Interrupt pending status of line 0"]
pub type PD0_R = crate::BitReader<PD0_A>;
impl PD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD0_A {
        match self.bits {
            false => PD0_A::NOTPENDING,
            true => PD0_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PD0_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD0_A::PENDING
    }
}
#[doc = "Interrupt pending status of line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD0_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PD0_AW> for bool {
    #[inline(always)]
    fn from(variant: PD0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD0` writer - Interrupt pending status of line 0"]
pub type PD0_W<'a> = crate::BitWriter<'a, u32, PD_SPEC, PD0_AW, 0>;
impl<'a> PD0_W<'a> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PD0_AW::CLEAR)
    }
}
#[doc = "Interrupt pending status of line 1"]
pub use PD0_A as PD1_A;
#[doc = "Interrupt pending status of line 2"]
pub use PD0_A as PD2_A;
#[doc = "Interrupt pending status of line 3"]
pub use PD0_A as PD3_A;
#[doc = "Interrupt pending status of line 4"]
pub use PD0_A as PD4_A;
#[doc = "Interrupt pending status of line 5"]
pub use PD0_A as PD5_A;
#[doc = "Interrupt pending status of line 6"]
pub use PD0_A as PD6_A;
#[doc = "Interrupt pending status of line 7"]
pub use PD0_A as PD7_A;
#[doc = "Interrupt pending status of line 8"]
pub use PD0_A as PD8_A;
#[doc = "Interrupt pending status of line 9"]
pub use PD0_A as PD9_A;
#[doc = "Interrupt pending status of line 10"]
pub use PD0_A as PD10_A;
#[doc = "Interrupt pending status of line 11"]
pub use PD0_A as PD11_A;
#[doc = "Interrupt pending status of line 12"]
pub use PD0_A as PD12_A;
#[doc = "Interrupt pending status of line 13"]
pub use PD0_A as PD13_A;
#[doc = "Interrupt pending status of line 14"]
pub use PD0_A as PD14_A;
#[doc = "Interrupt pending status of line 15"]
pub use PD0_A as PD15_A;
#[doc = "Interrupt pending status of line 16"]
pub use PD0_A as PD16_A;
#[doc = "Interrupt pending status of line 17"]
pub use PD0_A as PD17_A;
#[doc = "Interrupt pending status of line 18"]
pub use PD0_A as PD18_A;
#[doc = "Interrupt pending status of line 19"]
pub use PD0_A as PD19_A;
#[doc = "Interrupt pending status of line 21"]
pub use PD0_A as PD21_A;
#[doc = "Interrupt pending status of line 22"]
pub use PD0_A as PD22_A;
#[doc = "Interrupt pending status of line 1"]
pub use PD0_AW as PD1_AW;
#[doc = "Interrupt pending status of line 2"]
pub use PD0_AW as PD2_AW;
#[doc = "Interrupt pending status of line 3"]
pub use PD0_AW as PD3_AW;
#[doc = "Interrupt pending status of line 4"]
pub use PD0_AW as PD4_AW;
#[doc = "Interrupt pending status of line 5"]
pub use PD0_AW as PD5_AW;
#[doc = "Interrupt pending status of line 6"]
pub use PD0_AW as PD6_AW;
#[doc = "Interrupt pending status of line 7"]
pub use PD0_AW as PD7_AW;
#[doc = "Interrupt pending status of line 8"]
pub use PD0_AW as PD8_AW;
#[doc = "Interrupt pending status of line 9"]
pub use PD0_AW as PD9_AW;
#[doc = "Interrupt pending status of line 10"]
pub use PD0_AW as PD10_AW;
#[doc = "Interrupt pending status of line 11"]
pub use PD0_AW as PD11_AW;
#[doc = "Interrupt pending status of line 12"]
pub use PD0_AW as PD12_AW;
#[doc = "Interrupt pending status of line 13"]
pub use PD0_AW as PD13_AW;
#[doc = "Interrupt pending status of line 14"]
pub use PD0_AW as PD14_AW;
#[doc = "Interrupt pending status of line 15"]
pub use PD0_AW as PD15_AW;
#[doc = "Interrupt pending status of line 16"]
pub use PD0_AW as PD16_AW;
#[doc = "Interrupt pending status of line 17"]
pub use PD0_AW as PD17_AW;
#[doc = "Interrupt pending status of line 18"]
pub use PD0_AW as PD18_AW;
#[doc = "Interrupt pending status of line 19"]
pub use PD0_AW as PD19_AW;
#[doc = "Interrupt pending status of line 21"]
pub use PD0_AW as PD21_AW;
#[doc = "Interrupt pending status of line 22"]
pub use PD0_AW as PD22_AW;
#[doc = "Field `PD1` reader - Interrupt pending status of line 1"]
pub use PD0_R as PD1_R;
#[doc = "Field `PD2` reader - Interrupt pending status of line 2"]
pub use PD0_R as PD2_R;
#[doc = "Field `PD3` reader - Interrupt pending status of line 3"]
pub use PD0_R as PD3_R;
#[doc = "Field `PD4` reader - Interrupt pending status of line 4"]
pub use PD0_R as PD4_R;
#[doc = "Field `PD5` reader - Interrupt pending status of line 5"]
pub use PD0_R as PD5_R;
#[doc = "Field `PD6` reader - Interrupt pending status of line 6"]
pub use PD0_R as PD6_R;
#[doc = "Field `PD7` reader - Interrupt pending status of line 7"]
pub use PD0_R as PD7_R;
#[doc = "Field `PD8` reader - Interrupt pending status of line 8"]
pub use PD0_R as PD8_R;
#[doc = "Field `PD9` reader - Interrupt pending status of line 9"]
pub use PD0_R as PD9_R;
#[doc = "Field `PD10` reader - Interrupt pending status of line 10"]
pub use PD0_R as PD10_R;
#[doc = "Field `PD11` reader - Interrupt pending status of line 11"]
pub use PD0_R as PD11_R;
#[doc = "Field `PD12` reader - Interrupt pending status of line 12"]
pub use PD0_R as PD12_R;
#[doc = "Field `PD13` reader - Interrupt pending status of line 13"]
pub use PD0_R as PD13_R;
#[doc = "Field `PD14` reader - Interrupt pending status of line 14"]
pub use PD0_R as PD14_R;
#[doc = "Field `PD15` reader - Interrupt pending status of line 15"]
pub use PD0_R as PD15_R;
#[doc = "Field `PD16` reader - Interrupt pending status of line 16"]
pub use PD0_R as PD16_R;
#[doc = "Field `PD17` reader - Interrupt pending status of line 17"]
pub use PD0_R as PD17_R;
#[doc = "Field `PD18` reader - Interrupt pending status of line 18"]
pub use PD0_R as PD18_R;
#[doc = "Field `PD19` reader - Interrupt pending status of line 19"]
pub use PD0_R as PD19_R;
#[doc = "Field `PD21` reader - Interrupt pending status of line 21"]
pub use PD0_R as PD21_R;
#[doc = "Field `PD22` reader - Interrupt pending status of line 22"]
pub use PD0_R as PD22_R;
#[doc = "Field `PD1` writer - Interrupt pending status of line 1"]
pub use PD0_W as PD1_W;
#[doc = "Field `PD2` writer - Interrupt pending status of line 2"]
pub use PD0_W as PD2_W;
#[doc = "Field `PD3` writer - Interrupt pending status of line 3"]
pub use PD0_W as PD3_W;
#[doc = "Field `PD4` writer - Interrupt pending status of line 4"]
pub use PD0_W as PD4_W;
#[doc = "Field `PD5` writer - Interrupt pending status of line 5"]
pub use PD0_W as PD5_W;
#[doc = "Field `PD6` writer - Interrupt pending status of line 6"]
pub use PD0_W as PD6_W;
#[doc = "Field `PD7` writer - Interrupt pending status of line 7"]
pub use PD0_W as PD7_W;
#[doc = "Field `PD8` writer - Interrupt pending status of line 8"]
pub use PD0_W as PD8_W;
#[doc = "Field `PD9` writer - Interrupt pending status of line 9"]
pub use PD0_W as PD9_W;
#[doc = "Field `PD10` writer - Interrupt pending status of line 10"]
pub use PD0_W as PD10_W;
#[doc = "Field `PD11` writer - Interrupt pending status of line 11"]
pub use PD0_W as PD11_W;
#[doc = "Field `PD12` writer - Interrupt pending status of line 12"]
pub use PD0_W as PD12_W;
#[doc = "Field `PD13` writer - Interrupt pending status of line 13"]
pub use PD0_W as PD13_W;
#[doc = "Field `PD14` writer - Interrupt pending status of line 14"]
pub use PD0_W as PD14_W;
#[doc = "Field `PD15` writer - Interrupt pending status of line 15"]
pub use PD0_W as PD15_W;
#[doc = "Field `PD16` writer - Interrupt pending status of line 16"]
pub use PD0_W as PD16_W;
#[doc = "Field `PD17` writer - Interrupt pending status of line 17"]
pub use PD0_W as PD17_W;
#[doc = "Field `PD18` writer - Interrupt pending status of line 18"]
pub use PD0_W as PD18_W;
#[doc = "Field `PD19` writer - Interrupt pending status of line 19"]
pub use PD0_W as PD19_W;
#[doc = "Field `PD21` writer - Interrupt pending status of line 21"]
pub use PD0_W as PD21_W;
#[doc = "Field `PD22` writer - Interrupt pending status of line 22"]
pub use PD0_W as PD22_W;
impl R {
    #[doc = "Bit 0 - Interrupt pending status of line 0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt pending status of line 1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt pending status of line 2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt pending status of line 3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt pending status of line 4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt pending status of line 5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt pending status of line 6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt pending status of line 7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt pending status of line 8"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt pending status of line 9"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt pending status of line 10"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt pending status of line 11"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt pending status of line 12"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt pending status of line 13"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt pending status of line 14"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt pending status of line 15"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt pending status of line 16"]
    #[inline(always)]
    pub fn pd16(&self) -> PD16_R {
        PD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt pending status of line 17"]
    #[inline(always)]
    pub fn pd17(&self) -> PD17_R {
        PD17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt pending status of line 18"]
    #[inline(always)]
    pub fn pd18(&self) -> PD18_R {
        PD18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt pending status of line 19"]
    #[inline(always)]
    pub fn pd19(&self) -> PD19_R {
        PD19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt pending status of line 21"]
    #[inline(always)]
    pub fn pd21(&self) -> PD21_R {
        PD21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt pending status of line 22"]
    #[inline(always)]
    pub fn pd22(&self) -> PD22_R {
        PD22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt pending status of line 0"]
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W {
        PD0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt pending status of line 1"]
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W {
        PD1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt pending status of line 2"]
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W {
        PD2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt pending status of line 3"]
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W {
        PD3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt pending status of line 4"]
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W {
        PD4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt pending status of line 5"]
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W {
        PD5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt pending status of line 6"]
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W {
        PD6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt pending status of line 7"]
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W {
        PD7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt pending status of line 8"]
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W {
        PD8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt pending status of line 9"]
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W {
        PD9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt pending status of line 10"]
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W {
        PD10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt pending status of line 11"]
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W {
        PD11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt pending status of line 12"]
    #[inline(always)]
    pub fn pd12(&mut self) -> PD12_W {
        PD12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt pending status of line 13"]
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W {
        PD13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt pending status of line 14"]
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W {
        PD14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt pending status of line 15"]
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W {
        PD15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt pending status of line 16"]
    #[inline(always)]
    pub fn pd16(&mut self) -> PD16_W {
        PD16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt pending status of line 17"]
    #[inline(always)]
    pub fn pd17(&mut self) -> PD17_W {
        PD17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt pending status of line 18"]
    #[inline(always)]
    pub fn pd18(&mut self) -> PD18_W {
        PD18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt pending status of line 19"]
    #[inline(always)]
    pub fn pd19(&mut self) -> PD19_W {
        PD19_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt pending status of line 21"]
    #[inline(always)]
    pub fn pd21(&mut self) -> PD21_W {
        PD21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt pending status of line 22"]
    #[inline(always)]
    pub fn pd22(&mut self) -> PD22_W {
        PD22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending register (EXTI_PD)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd](index.html) module"]
pub struct PD_SPEC;
impl crate::RegisterSpec for PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd::R](R) reader structure"]
impl crate::Readable for PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd::W](W) writer structure"]
impl crate::Writable for PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD to value 0"]
impl crate::Resettable for PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
