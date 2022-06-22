#[doc = "Register `SWIEV` reader"]
pub struct R(crate::R<SWIEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIEV` writer"]
pub struct W(crate::W<SWIEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIEV_SPEC>;
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
impl From<crate::W<SWIEV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt/Event software trigger on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIEV0_A {
    #[doc = "1: Generates an interrupt request"]
    PEND = 1,
}
impl From<SWIEV0_A> for bool {
    #[inline(always)]
    fn from(variant: SWIEV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWIEV0` reader - Interrupt/Event software trigger on line 0"]
pub type SWIEV0_R = crate::BitReader<SWIEV0_A>;
impl SWIEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIEV0_A> {
        match self.bits {
            true => Some(SWIEV0_A::PEND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PEND`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIEV0_A::PEND
    }
}
#[doc = "Field `SWIEV0` writer - Interrupt/Event software trigger on line 0"]
pub type SWIEV0_W<'a> = crate::BitWriter<'a, u32, SWIEV_SPEC, SWIEV0_A, 0>;
impl<'a> SWIEV0_W<'a> {
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIEV0_A::PEND)
    }
}
#[doc = "Interrupt/Event software trigger on line 1"]
pub use SWIEV0_A as SWIEV1_A;
#[doc = "Interrupt/Event software trigger on line 2"]
pub use SWIEV0_A as SWIEV2_A;
#[doc = "Interrupt/Event software trigger on line 3"]
pub use SWIEV0_A as SWIEV3_A;
#[doc = "Interrupt/Event software trigger on line 4"]
pub use SWIEV0_A as SWIEV4_A;
#[doc = "Interrupt/Event software trigger on line 5"]
pub use SWIEV0_A as SWIEV5_A;
#[doc = "Interrupt/Event software trigger on line 6"]
pub use SWIEV0_A as SWIEV6_A;
#[doc = "Interrupt/Event software trigger on line 7"]
pub use SWIEV0_A as SWIEV7_A;
#[doc = "Interrupt/Event software trigger on line 8"]
pub use SWIEV0_A as SWIEV8_A;
#[doc = "Interrupt/Event software trigger on line 9"]
pub use SWIEV0_A as SWIEV9_A;
#[doc = "Interrupt/Event software trigger on line 10"]
pub use SWIEV0_A as SWIEV10_A;
#[doc = "Interrupt/Event software trigger on line 11"]
pub use SWIEV0_A as SWIEV11_A;
#[doc = "Interrupt/Event software trigger on line 12"]
pub use SWIEV0_A as SWIEV12_A;
#[doc = "Interrupt/Event software trigger on line 13"]
pub use SWIEV0_A as SWIEV13_A;
#[doc = "Interrupt/Event software trigger on line 14"]
pub use SWIEV0_A as SWIEV14_A;
#[doc = "Interrupt/Event software trigger on line 15"]
pub use SWIEV0_A as SWIEV15_A;
#[doc = "Interrupt/Event software trigger on line 16"]
pub use SWIEV0_A as SWIEV16_A;
#[doc = "Interrupt/Event software trigger on line 17"]
pub use SWIEV0_A as SWIEV17_A;
#[doc = "Interrupt/Event software trigger on line 18"]
pub use SWIEV0_A as SWIEV18_A;
#[doc = "Interrupt/Event software trigger on line 19"]
pub use SWIEV0_A as SWIEV19_A;
#[doc = "Interrupt/Event software trigger on line 21"]
pub use SWIEV0_A as SWIEV21_A;
#[doc = "Interrupt/Event software trigger on line 22"]
pub use SWIEV0_A as SWIEV22_A;
#[doc = "Field `SWIEV1` reader - Interrupt/Event software trigger on line 1"]
pub use SWIEV0_R as SWIEV1_R;
#[doc = "Field `SWIEV2` reader - Interrupt/Event software trigger on line 2"]
pub use SWIEV0_R as SWIEV2_R;
#[doc = "Field `SWIEV3` reader - Interrupt/Event software trigger on line 3"]
pub use SWIEV0_R as SWIEV3_R;
#[doc = "Field `SWIEV4` reader - Interrupt/Event software trigger on line 4"]
pub use SWIEV0_R as SWIEV4_R;
#[doc = "Field `SWIEV5` reader - Interrupt/Event software trigger on line 5"]
pub use SWIEV0_R as SWIEV5_R;
#[doc = "Field `SWIEV6` reader - Interrupt/Event software trigger on line 6"]
pub use SWIEV0_R as SWIEV6_R;
#[doc = "Field `SWIEV7` reader - Interrupt/Event software trigger on line 7"]
pub use SWIEV0_R as SWIEV7_R;
#[doc = "Field `SWIEV8` reader - Interrupt/Event software trigger on line 8"]
pub use SWIEV0_R as SWIEV8_R;
#[doc = "Field `SWIEV9` reader - Interrupt/Event software trigger on line 9"]
pub use SWIEV0_R as SWIEV9_R;
#[doc = "Field `SWIEV10` reader - Interrupt/Event software trigger on line 10"]
pub use SWIEV0_R as SWIEV10_R;
#[doc = "Field `SWIEV11` reader - Interrupt/Event software trigger on line 11"]
pub use SWIEV0_R as SWIEV11_R;
#[doc = "Field `SWIEV12` reader - Interrupt/Event software trigger on line 12"]
pub use SWIEV0_R as SWIEV12_R;
#[doc = "Field `SWIEV13` reader - Interrupt/Event software trigger on line 13"]
pub use SWIEV0_R as SWIEV13_R;
#[doc = "Field `SWIEV14` reader - Interrupt/Event software trigger on line 14"]
pub use SWIEV0_R as SWIEV14_R;
#[doc = "Field `SWIEV15` reader - Interrupt/Event software trigger on line 15"]
pub use SWIEV0_R as SWIEV15_R;
#[doc = "Field `SWIEV16` reader - Interrupt/Event software trigger on line 16"]
pub use SWIEV0_R as SWIEV16_R;
#[doc = "Field `SWIEV17` reader - Interrupt/Event software trigger on line 17"]
pub use SWIEV0_R as SWIEV17_R;
#[doc = "Field `SWIEV18` reader - Interrupt/Event software trigger on line 18"]
pub use SWIEV0_R as SWIEV18_R;
#[doc = "Field `SWIEV19` reader - Interrupt/Event software trigger on line 19"]
pub use SWIEV0_R as SWIEV19_R;
#[doc = "Field `SWIEV21` reader - Interrupt/Event software trigger on line 21"]
pub use SWIEV0_R as SWIEV21_R;
#[doc = "Field `SWIEV22` reader - Interrupt/Event software trigger on line 22"]
pub use SWIEV0_R as SWIEV22_R;
#[doc = "Field `SWIEV1` writer - Interrupt/Event software trigger on line 1"]
pub use SWIEV0_W as SWIEV1_W;
#[doc = "Field `SWIEV2` writer - Interrupt/Event software trigger on line 2"]
pub use SWIEV0_W as SWIEV2_W;
#[doc = "Field `SWIEV3` writer - Interrupt/Event software trigger on line 3"]
pub use SWIEV0_W as SWIEV3_W;
#[doc = "Field `SWIEV4` writer - Interrupt/Event software trigger on line 4"]
pub use SWIEV0_W as SWIEV4_W;
#[doc = "Field `SWIEV5` writer - Interrupt/Event software trigger on line 5"]
pub use SWIEV0_W as SWIEV5_W;
#[doc = "Field `SWIEV6` writer - Interrupt/Event software trigger on line 6"]
pub use SWIEV0_W as SWIEV6_W;
#[doc = "Field `SWIEV7` writer - Interrupt/Event software trigger on line 7"]
pub use SWIEV0_W as SWIEV7_W;
#[doc = "Field `SWIEV8` writer - Interrupt/Event software trigger on line 8"]
pub use SWIEV0_W as SWIEV8_W;
#[doc = "Field `SWIEV9` writer - Interrupt/Event software trigger on line 9"]
pub use SWIEV0_W as SWIEV9_W;
#[doc = "Field `SWIEV10` writer - Interrupt/Event software trigger on line 10"]
pub use SWIEV0_W as SWIEV10_W;
#[doc = "Field `SWIEV11` writer - Interrupt/Event software trigger on line 11"]
pub use SWIEV0_W as SWIEV11_W;
#[doc = "Field `SWIEV12` writer - Interrupt/Event software trigger on line 12"]
pub use SWIEV0_W as SWIEV12_W;
#[doc = "Field `SWIEV13` writer - Interrupt/Event software trigger on line 13"]
pub use SWIEV0_W as SWIEV13_W;
#[doc = "Field `SWIEV14` writer - Interrupt/Event software trigger on line 14"]
pub use SWIEV0_W as SWIEV14_W;
#[doc = "Field `SWIEV15` writer - Interrupt/Event software trigger on line 15"]
pub use SWIEV0_W as SWIEV15_W;
#[doc = "Field `SWIEV16` writer - Interrupt/Event software trigger on line 16"]
pub use SWIEV0_W as SWIEV16_W;
#[doc = "Field `SWIEV17` writer - Interrupt/Event software trigger on line 17"]
pub use SWIEV0_W as SWIEV17_W;
#[doc = "Field `SWIEV18` writer - Interrupt/Event software trigger on line 18"]
pub use SWIEV0_W as SWIEV18_W;
#[doc = "Field `SWIEV19` writer - Interrupt/Event software trigger on line 19"]
pub use SWIEV0_W as SWIEV19_W;
#[doc = "Field `SWIEV21` writer - Interrupt/Event software trigger on line 21"]
pub use SWIEV0_W as SWIEV21_W;
#[doc = "Field `SWIEV22` writer - Interrupt/Event software trigger on line 22"]
pub use SWIEV0_W as SWIEV22_W;
impl R {
    #[doc = "Bit 0 - Interrupt/Event software trigger on line 0"]
    #[inline(always)]
    pub fn swiev0(&self) -> SWIEV0_R {
        SWIEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt/Event software trigger on line 1"]
    #[inline(always)]
    pub fn swiev1(&self) -> SWIEV1_R {
        SWIEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt/Event software trigger on line 2"]
    #[inline(always)]
    pub fn swiev2(&self) -> SWIEV2_R {
        SWIEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt/Event software trigger on line 3"]
    #[inline(always)]
    pub fn swiev3(&self) -> SWIEV3_R {
        SWIEV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt/Event software trigger on line 4"]
    #[inline(always)]
    pub fn swiev4(&self) -> SWIEV4_R {
        SWIEV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt/Event software trigger on line 5"]
    #[inline(always)]
    pub fn swiev5(&self) -> SWIEV5_R {
        SWIEV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt/Event software trigger on line 6"]
    #[inline(always)]
    pub fn swiev6(&self) -> SWIEV6_R {
        SWIEV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt/Event software trigger on line 7"]
    #[inline(always)]
    pub fn swiev7(&self) -> SWIEV7_R {
        SWIEV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt/Event software trigger on line 8"]
    #[inline(always)]
    pub fn swiev8(&self) -> SWIEV8_R {
        SWIEV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt/Event software trigger on line 9"]
    #[inline(always)]
    pub fn swiev9(&self) -> SWIEV9_R {
        SWIEV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt/Event software trigger on line 10"]
    #[inline(always)]
    pub fn swiev10(&self) -> SWIEV10_R {
        SWIEV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt/Event software trigger on line 11"]
    #[inline(always)]
    pub fn swiev11(&self) -> SWIEV11_R {
        SWIEV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt/Event software trigger on line 12"]
    #[inline(always)]
    pub fn swiev12(&self) -> SWIEV12_R {
        SWIEV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt/Event software trigger on line 13"]
    #[inline(always)]
    pub fn swiev13(&self) -> SWIEV13_R {
        SWIEV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt/Event software trigger on line 14"]
    #[inline(always)]
    pub fn swiev14(&self) -> SWIEV14_R {
        SWIEV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt/Event software trigger on line 15"]
    #[inline(always)]
    pub fn swiev15(&self) -> SWIEV15_R {
        SWIEV15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt/Event software trigger on line 16"]
    #[inline(always)]
    pub fn swiev16(&self) -> SWIEV16_R {
        SWIEV16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt/Event software trigger on line 17"]
    #[inline(always)]
    pub fn swiev17(&self) -> SWIEV17_R {
        SWIEV17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt/Event software trigger on line 18"]
    #[inline(always)]
    pub fn swiev18(&self) -> SWIEV18_R {
        SWIEV18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt/Event software trigger on line 19"]
    #[inline(always)]
    pub fn swiev19(&self) -> SWIEV19_R {
        SWIEV19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt/Event software trigger on line 21"]
    #[inline(always)]
    pub fn swiev21(&self) -> SWIEV21_R {
        SWIEV21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt/Event software trigger on line 22"]
    #[inline(always)]
    pub fn swiev22(&self) -> SWIEV22_R {
        SWIEV22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt/Event software trigger on line 0"]
    #[inline(always)]
    pub fn swiev0(&mut self) -> SWIEV0_W {
        SWIEV0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt/Event software trigger on line 1"]
    #[inline(always)]
    pub fn swiev1(&mut self) -> SWIEV1_W {
        SWIEV1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt/Event software trigger on line 2"]
    #[inline(always)]
    pub fn swiev2(&mut self) -> SWIEV2_W {
        SWIEV2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt/Event software trigger on line 3"]
    #[inline(always)]
    pub fn swiev3(&mut self) -> SWIEV3_W {
        SWIEV3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt/Event software trigger on line 4"]
    #[inline(always)]
    pub fn swiev4(&mut self) -> SWIEV4_W {
        SWIEV4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt/Event software trigger on line 5"]
    #[inline(always)]
    pub fn swiev5(&mut self) -> SWIEV5_W {
        SWIEV5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt/Event software trigger on line 6"]
    #[inline(always)]
    pub fn swiev6(&mut self) -> SWIEV6_W {
        SWIEV6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt/Event software trigger on line 7"]
    #[inline(always)]
    pub fn swiev7(&mut self) -> SWIEV7_W {
        SWIEV7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt/Event software trigger on line 8"]
    #[inline(always)]
    pub fn swiev8(&mut self) -> SWIEV8_W {
        SWIEV8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt/Event software trigger on line 9"]
    #[inline(always)]
    pub fn swiev9(&mut self) -> SWIEV9_W {
        SWIEV9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt/Event software trigger on line 10"]
    #[inline(always)]
    pub fn swiev10(&mut self) -> SWIEV10_W {
        SWIEV10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt/Event software trigger on line 11"]
    #[inline(always)]
    pub fn swiev11(&mut self) -> SWIEV11_W {
        SWIEV11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt/Event software trigger on line 12"]
    #[inline(always)]
    pub fn swiev12(&mut self) -> SWIEV12_W {
        SWIEV12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt/Event software trigger on line 13"]
    #[inline(always)]
    pub fn swiev13(&mut self) -> SWIEV13_W {
        SWIEV13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt/Event software trigger on line 14"]
    #[inline(always)]
    pub fn swiev14(&mut self) -> SWIEV14_W {
        SWIEV14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt/Event software trigger on line 15"]
    #[inline(always)]
    pub fn swiev15(&mut self) -> SWIEV15_W {
        SWIEV15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt/Event software trigger on line 16"]
    #[inline(always)]
    pub fn swiev16(&mut self) -> SWIEV16_W {
        SWIEV16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt/Event software trigger on line 17"]
    #[inline(always)]
    pub fn swiev17(&mut self) -> SWIEV17_W {
        SWIEV17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt/Event software trigger on line 18"]
    #[inline(always)]
    pub fn swiev18(&mut self) -> SWIEV18_W {
        SWIEV18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt/Event software trigger on line 19"]
    #[inline(always)]
    pub fn swiev19(&mut self) -> SWIEV19_W {
        SWIEV19_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt/Event software trigger on line 21"]
    #[inline(always)]
    pub fn swiev21(&mut self) -> SWIEV21_W {
        SWIEV21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt/Event software trigger on line 22"]
    #[inline(always)]
    pub fn swiev22(&mut self) -> SWIEV22_W {
        SWIEV22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software interrupt event register (EXTI_SWIEV)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swiev](index.html) module"]
pub struct SWIEV_SPEC;
impl crate::RegisterSpec for SWIEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swiev::R](R) reader structure"]
impl crate::Readable for SWIEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swiev::W](W) writer structure"]
impl crate::Writable for SWIEV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIEV to value 0"]
impl crate::Resettable for SWIEV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
