#[doc = "Register `FAFIFO` reader"]
pub struct R(crate::R<FAFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAFIFO` writer"]
pub struct W(crate::W<FAFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAFIFO_SPEC>;
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
impl From<crate::W<FAFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Filter 0 associated with FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAF0_A {
    #[doc = "0: Filter associated with FIFO0"]
    FIFO0 = 0,
    #[doc = "1: Filter associated with FIFO1"]
    FIFO1 = 1,
}
impl From<FAF0_A> for bool {
    #[inline(always)]
    fn from(variant: FAF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAF0` reader - Filter 0 associated with FIFO"]
pub type FAF0_R = crate::BitReader<FAF0_A>;
impl FAF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAF0_A {
        match self.bits {
            false => FAF0_A::FIFO0,
            true => FAF0_A::FIFO1,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO0`"]
    #[inline(always)]
    pub fn is_fifo0(&self) -> bool {
        *self == FAF0_A::FIFO0
    }
    #[doc = "Checks if the value of the field is `FIFO1`"]
    #[inline(always)]
    pub fn is_fifo1(&self) -> bool {
        *self == FAF0_A::FIFO1
    }
}
#[doc = "Field `FAF0` writer - Filter 0 associated with FIFO"]
pub type FAF0_W<'a> = crate::BitWriter<'a, u32, FAFIFO_SPEC, FAF0_A, 0>;
impl<'a> FAF0_W<'a> {
    #[doc = "Filter associated with FIFO0"]
    #[inline(always)]
    pub fn fifo0(self) -> &'a mut W {
        self.variant(FAF0_A::FIFO0)
    }
    #[doc = "Filter associated with FIFO1"]
    #[inline(always)]
    pub fn fifo1(self) -> &'a mut W {
        self.variant(FAF0_A::FIFO1)
    }
}
#[doc = "Filter 1 associated with FIFO"]
pub use FAF0_A as FAF1_A;
#[doc = "Filter 2 associated with FIFO"]
pub use FAF0_A as FAF2_A;
#[doc = "Filter 3 associated with FIFO"]
pub use FAF0_A as FAF3_A;
#[doc = "Filter 4 associated with FIFO"]
pub use FAF0_A as FAF4_A;
#[doc = "Filter 5 associated with FIFO"]
pub use FAF0_A as FAF5_A;
#[doc = "Filter 6 associated with FIFO"]
pub use FAF0_A as FAF6_A;
#[doc = "Filter 7 associated with FIFO"]
pub use FAF0_A as FAF7_A;
#[doc = "Filter 8 associated with FIFO"]
pub use FAF0_A as FAF8_A;
#[doc = "Filter 9 associated with FIFO"]
pub use FAF0_A as FAF9_A;
#[doc = "Filter 10 associated with FIFO"]
pub use FAF0_A as FAF10_A;
#[doc = "Filter 11 associated with FIFO"]
pub use FAF0_A as FAF11_A;
#[doc = "Filter 12 associated with FIFO"]
pub use FAF0_A as FAF12_A;
#[doc = "Filter 13 associated with FIFO"]
pub use FAF0_A as FAF13_A;
#[doc = "Filter 14 associated with FIFO"]
pub use FAF0_A as FAF14_A;
#[doc = "Filter 15 associated with FIFO"]
pub use FAF0_A as FAF15_A;
#[doc = "Filter 16 associated with FIFO"]
pub use FAF0_A as FAF16_A;
#[doc = "Filter 17 associated with FIFO"]
pub use FAF0_A as FAF17_A;
#[doc = "Filter 18 associated with FIFO"]
pub use FAF0_A as FAF18_A;
#[doc = "Filter 19 associated with FIFO"]
pub use FAF0_A as FAF19_A;
#[doc = "Filter 20 associated with FIFO"]
pub use FAF0_A as FAF20_A;
#[doc = "Filter 21 associated with FIFO"]
pub use FAF0_A as FAF21_A;
#[doc = "Filter 22 associated with FIFO"]
pub use FAF0_A as FAF22_A;
#[doc = "Filter 23 associated with FIFO"]
pub use FAF0_A as FAF23_A;
#[doc = "Filter 24 associated with FIFO"]
pub use FAF0_A as FAF24_A;
#[doc = "Filter 25 associated with FIFO"]
pub use FAF0_A as FAF25_A;
#[doc = "Filter 26 associated with FIFO"]
pub use FAF0_A as FAF26_A;
#[doc = "Filter 27 associated with FIFO"]
pub use FAF0_A as FAF27_A;
#[doc = "Field `FAF1` reader - Filter 1 associated with FIFO"]
pub use FAF0_R as FAF1_R;
#[doc = "Field `FAF2` reader - Filter 2 associated with FIFO"]
pub use FAF0_R as FAF2_R;
#[doc = "Field `FAF3` reader - Filter 3 associated with FIFO"]
pub use FAF0_R as FAF3_R;
#[doc = "Field `FAF4` reader - Filter 4 associated with FIFO"]
pub use FAF0_R as FAF4_R;
#[doc = "Field `FAF5` reader - Filter 5 associated with FIFO"]
pub use FAF0_R as FAF5_R;
#[doc = "Field `FAF6` reader - Filter 6 associated with FIFO"]
pub use FAF0_R as FAF6_R;
#[doc = "Field `FAF7` reader - Filter 7 associated with FIFO"]
pub use FAF0_R as FAF7_R;
#[doc = "Field `FAF8` reader - Filter 8 associated with FIFO"]
pub use FAF0_R as FAF8_R;
#[doc = "Field `FAF9` reader - Filter 9 associated with FIFO"]
pub use FAF0_R as FAF9_R;
#[doc = "Field `FAF10` reader - Filter 10 associated with FIFO"]
pub use FAF0_R as FAF10_R;
#[doc = "Field `FAF11` reader - Filter 11 associated with FIFO"]
pub use FAF0_R as FAF11_R;
#[doc = "Field `FAF12` reader - Filter 12 associated with FIFO"]
pub use FAF0_R as FAF12_R;
#[doc = "Field `FAF13` reader - Filter 13 associated with FIFO"]
pub use FAF0_R as FAF13_R;
#[doc = "Field `FAF14` reader - Filter 14 associated with FIFO"]
pub use FAF0_R as FAF14_R;
#[doc = "Field `FAF15` reader - Filter 15 associated with FIFO"]
pub use FAF0_R as FAF15_R;
#[doc = "Field `FAF16` reader - Filter 16 associated with FIFO"]
pub use FAF0_R as FAF16_R;
#[doc = "Field `FAF17` reader - Filter 17 associated with FIFO"]
pub use FAF0_R as FAF17_R;
#[doc = "Field `FAF18` reader - Filter 18 associated with FIFO"]
pub use FAF0_R as FAF18_R;
#[doc = "Field `FAF19` reader - Filter 19 associated with FIFO"]
pub use FAF0_R as FAF19_R;
#[doc = "Field `FAF20` reader - Filter 20 associated with FIFO"]
pub use FAF0_R as FAF20_R;
#[doc = "Field `FAF21` reader - Filter 21 associated with FIFO"]
pub use FAF0_R as FAF21_R;
#[doc = "Field `FAF22` reader - Filter 22 associated with FIFO"]
pub use FAF0_R as FAF22_R;
#[doc = "Field `FAF23` reader - Filter 23 associated with FIFO"]
pub use FAF0_R as FAF23_R;
#[doc = "Field `FAF24` reader - Filter 24 associated with FIFO"]
pub use FAF0_R as FAF24_R;
#[doc = "Field `FAF25` reader - Filter 25 associated with FIFO"]
pub use FAF0_R as FAF25_R;
#[doc = "Field `FAF26` reader - Filter 26 associated with FIFO"]
pub use FAF0_R as FAF26_R;
#[doc = "Field `FAF27` reader - Filter 27 associated with FIFO"]
pub use FAF0_R as FAF27_R;
#[doc = "Field `FAF1` writer - Filter 1 associated with FIFO"]
pub use FAF0_W as FAF1_W;
#[doc = "Field `FAF2` writer - Filter 2 associated with FIFO"]
pub use FAF0_W as FAF2_W;
#[doc = "Field `FAF3` writer - Filter 3 associated with FIFO"]
pub use FAF0_W as FAF3_W;
#[doc = "Field `FAF4` writer - Filter 4 associated with FIFO"]
pub use FAF0_W as FAF4_W;
#[doc = "Field `FAF5` writer - Filter 5 associated with FIFO"]
pub use FAF0_W as FAF5_W;
#[doc = "Field `FAF6` writer - Filter 6 associated with FIFO"]
pub use FAF0_W as FAF6_W;
#[doc = "Field `FAF7` writer - Filter 7 associated with FIFO"]
pub use FAF0_W as FAF7_W;
#[doc = "Field `FAF8` writer - Filter 8 associated with FIFO"]
pub use FAF0_W as FAF8_W;
#[doc = "Field `FAF9` writer - Filter 9 associated with FIFO"]
pub use FAF0_W as FAF9_W;
#[doc = "Field `FAF10` writer - Filter 10 associated with FIFO"]
pub use FAF0_W as FAF10_W;
#[doc = "Field `FAF11` writer - Filter 11 associated with FIFO"]
pub use FAF0_W as FAF11_W;
#[doc = "Field `FAF12` writer - Filter 12 associated with FIFO"]
pub use FAF0_W as FAF12_W;
#[doc = "Field `FAF13` writer - Filter 13 associated with FIFO"]
pub use FAF0_W as FAF13_W;
#[doc = "Field `FAF14` writer - Filter 14 associated with FIFO"]
pub use FAF0_W as FAF14_W;
#[doc = "Field `FAF15` writer - Filter 15 associated with FIFO"]
pub use FAF0_W as FAF15_W;
#[doc = "Field `FAF16` writer - Filter 16 associated with FIFO"]
pub use FAF0_W as FAF16_W;
#[doc = "Field `FAF17` writer - Filter 17 associated with FIFO"]
pub use FAF0_W as FAF17_W;
#[doc = "Field `FAF18` writer - Filter 18 associated with FIFO"]
pub use FAF0_W as FAF18_W;
#[doc = "Field `FAF19` writer - Filter 19 associated with FIFO"]
pub use FAF0_W as FAF19_W;
#[doc = "Field `FAF20` writer - Filter 20 associated with FIFO"]
pub use FAF0_W as FAF20_W;
#[doc = "Field `FAF21` writer - Filter 21 associated with FIFO"]
pub use FAF0_W as FAF21_W;
#[doc = "Field `FAF22` writer - Filter 22 associated with FIFO"]
pub use FAF0_W as FAF22_W;
#[doc = "Field `FAF23` writer - Filter 23 associated with FIFO"]
pub use FAF0_W as FAF23_W;
#[doc = "Field `FAF24` writer - Filter 24 associated with FIFO"]
pub use FAF0_W as FAF24_W;
#[doc = "Field `FAF25` writer - Filter 25 associated with FIFO"]
pub use FAF0_W as FAF25_W;
#[doc = "Field `FAF26` writer - Filter 26 associated with FIFO"]
pub use FAF0_W as FAF26_W;
#[doc = "Field `FAF27` writer - Filter 27 associated with FIFO"]
pub use FAF0_W as FAF27_W;
impl R {
    #[doc = "Bit 0 - Filter 0 associated with FIFO"]
    #[inline(always)]
    pub fn faf0(&self) -> FAF0_R {
        FAF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter 1 associated with FIFO"]
    #[inline(always)]
    pub fn faf1(&self) -> FAF1_R {
        FAF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter 2 associated with FIFO"]
    #[inline(always)]
    pub fn faf2(&self) -> FAF2_R {
        FAF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter 3 associated with FIFO"]
    #[inline(always)]
    pub fn faf3(&self) -> FAF3_R {
        FAF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter 4 associated with FIFO"]
    #[inline(always)]
    pub fn faf4(&self) -> FAF4_R {
        FAF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter 5 associated with FIFO"]
    #[inline(always)]
    pub fn faf5(&self) -> FAF5_R {
        FAF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter 6 associated with FIFO"]
    #[inline(always)]
    pub fn faf6(&self) -> FAF6_R {
        FAF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter 7 associated with FIFO"]
    #[inline(always)]
    pub fn faf7(&self) -> FAF7_R {
        FAF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter 8 associated with FIFO"]
    #[inline(always)]
    pub fn faf8(&self) -> FAF8_R {
        FAF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter 9 associated with FIFO"]
    #[inline(always)]
    pub fn faf9(&self) -> FAF9_R {
        FAF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter 10 associated with FIFO"]
    #[inline(always)]
    pub fn faf10(&self) -> FAF10_R {
        FAF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter 11 associated with FIFO"]
    #[inline(always)]
    pub fn faf11(&self) -> FAF11_R {
        FAF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter 12 associated with FIFO"]
    #[inline(always)]
    pub fn faf12(&self) -> FAF12_R {
        FAF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter 13 associated with FIFO"]
    #[inline(always)]
    pub fn faf13(&self) -> FAF13_R {
        FAF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter 14 associated with FIFO"]
    #[inline(always)]
    pub fn faf14(&self) -> FAF14_R {
        FAF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter 15 associated with FIFO"]
    #[inline(always)]
    pub fn faf15(&self) -> FAF15_R {
        FAF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter 16 associated with FIFO"]
    #[inline(always)]
    pub fn faf16(&self) -> FAF16_R {
        FAF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter 17 associated with FIFO"]
    #[inline(always)]
    pub fn faf17(&self) -> FAF17_R {
        FAF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter 18 associated with FIFO"]
    #[inline(always)]
    pub fn faf18(&self) -> FAF18_R {
        FAF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter 19 associated with FIFO"]
    #[inline(always)]
    pub fn faf19(&self) -> FAF19_R {
        FAF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter 20 associated with FIFO"]
    #[inline(always)]
    pub fn faf20(&self) -> FAF20_R {
        FAF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter 21 associated with FIFO"]
    #[inline(always)]
    pub fn faf21(&self) -> FAF21_R {
        FAF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter 22 associated with FIFO"]
    #[inline(always)]
    pub fn faf22(&self) -> FAF22_R {
        FAF22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter 23 associated with FIFO"]
    #[inline(always)]
    pub fn faf23(&self) -> FAF23_R {
        FAF23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter 24 associated with FIFO"]
    #[inline(always)]
    pub fn faf24(&self) -> FAF24_R {
        FAF24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter 25 associated with FIFO"]
    #[inline(always)]
    pub fn faf25(&self) -> FAF25_R {
        FAF25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter 26 associated with FIFO"]
    #[inline(always)]
    pub fn faf26(&self) -> FAF26_R {
        FAF26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter 27 associated with FIFO"]
    #[inline(always)]
    pub fn faf27(&self) -> FAF27_R {
        FAF27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter 0 associated with FIFO"]
    #[inline(always)]
    pub fn faf0(&mut self) -> FAF0_W {
        FAF0_W::new(self)
    }
    #[doc = "Bit 1 - Filter 1 associated with FIFO"]
    #[inline(always)]
    pub fn faf1(&mut self) -> FAF1_W {
        FAF1_W::new(self)
    }
    #[doc = "Bit 2 - Filter 2 associated with FIFO"]
    #[inline(always)]
    pub fn faf2(&mut self) -> FAF2_W {
        FAF2_W::new(self)
    }
    #[doc = "Bit 3 - Filter 3 associated with FIFO"]
    #[inline(always)]
    pub fn faf3(&mut self) -> FAF3_W {
        FAF3_W::new(self)
    }
    #[doc = "Bit 4 - Filter 4 associated with FIFO"]
    #[inline(always)]
    pub fn faf4(&mut self) -> FAF4_W {
        FAF4_W::new(self)
    }
    #[doc = "Bit 5 - Filter 5 associated with FIFO"]
    #[inline(always)]
    pub fn faf5(&mut self) -> FAF5_W {
        FAF5_W::new(self)
    }
    #[doc = "Bit 6 - Filter 6 associated with FIFO"]
    #[inline(always)]
    pub fn faf6(&mut self) -> FAF6_W {
        FAF6_W::new(self)
    }
    #[doc = "Bit 7 - Filter 7 associated with FIFO"]
    #[inline(always)]
    pub fn faf7(&mut self) -> FAF7_W {
        FAF7_W::new(self)
    }
    #[doc = "Bit 8 - Filter 8 associated with FIFO"]
    #[inline(always)]
    pub fn faf8(&mut self) -> FAF8_W {
        FAF8_W::new(self)
    }
    #[doc = "Bit 9 - Filter 9 associated with FIFO"]
    #[inline(always)]
    pub fn faf9(&mut self) -> FAF9_W {
        FAF9_W::new(self)
    }
    #[doc = "Bit 10 - Filter 10 associated with FIFO"]
    #[inline(always)]
    pub fn faf10(&mut self) -> FAF10_W {
        FAF10_W::new(self)
    }
    #[doc = "Bit 11 - Filter 11 associated with FIFO"]
    #[inline(always)]
    pub fn faf11(&mut self) -> FAF11_W {
        FAF11_W::new(self)
    }
    #[doc = "Bit 12 - Filter 12 associated with FIFO"]
    #[inline(always)]
    pub fn faf12(&mut self) -> FAF12_W {
        FAF12_W::new(self)
    }
    #[doc = "Bit 13 - Filter 13 associated with FIFO"]
    #[inline(always)]
    pub fn faf13(&mut self) -> FAF13_W {
        FAF13_W::new(self)
    }
    #[doc = "Bit 14 - Filter 14 associated with FIFO"]
    #[inline(always)]
    pub fn faf14(&mut self) -> FAF14_W {
        FAF14_W::new(self)
    }
    #[doc = "Bit 15 - Filter 15 associated with FIFO"]
    #[inline(always)]
    pub fn faf15(&mut self) -> FAF15_W {
        FAF15_W::new(self)
    }
    #[doc = "Bit 16 - Filter 16 associated with FIFO"]
    #[inline(always)]
    pub fn faf16(&mut self) -> FAF16_W {
        FAF16_W::new(self)
    }
    #[doc = "Bit 17 - Filter 17 associated with FIFO"]
    #[inline(always)]
    pub fn faf17(&mut self) -> FAF17_W {
        FAF17_W::new(self)
    }
    #[doc = "Bit 18 - Filter 18 associated with FIFO"]
    #[inline(always)]
    pub fn faf18(&mut self) -> FAF18_W {
        FAF18_W::new(self)
    }
    #[doc = "Bit 19 - Filter 19 associated with FIFO"]
    #[inline(always)]
    pub fn faf19(&mut self) -> FAF19_W {
        FAF19_W::new(self)
    }
    #[doc = "Bit 20 - Filter 20 associated with FIFO"]
    #[inline(always)]
    pub fn faf20(&mut self) -> FAF20_W {
        FAF20_W::new(self)
    }
    #[doc = "Bit 21 - Filter 21 associated with FIFO"]
    #[inline(always)]
    pub fn faf21(&mut self) -> FAF21_W {
        FAF21_W::new(self)
    }
    #[doc = "Bit 22 - Filter 22 associated with FIFO"]
    #[inline(always)]
    pub fn faf22(&mut self) -> FAF22_W {
        FAF22_W::new(self)
    }
    #[doc = "Bit 23 - Filter 23 associated with FIFO"]
    #[inline(always)]
    pub fn faf23(&mut self) -> FAF23_W {
        FAF23_W::new(self)
    }
    #[doc = "Bit 24 - Filter 24 associated with FIFO"]
    #[inline(always)]
    pub fn faf24(&mut self) -> FAF24_W {
        FAF24_W::new(self)
    }
    #[doc = "Bit 25 - Filter 25 associated with FIFO"]
    #[inline(always)]
    pub fn faf25(&mut self) -> FAF25_W {
        FAF25_W::new(self)
    }
    #[doc = "Bit 26 - Filter 26 associated with FIFO"]
    #[inline(always)]
    pub fn faf26(&mut self) -> FAF26_W {
        FAF26_W::new(self)
    }
    #[doc = "Bit 27 - Filter 27 associated with FIFO"]
    #[inline(always)]
    pub fn faf27(&mut self) -> FAF27_W {
        FAF27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter associated FIFO register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fafifo](index.html) module"]
pub struct FAFIFO_SPEC;
impl crate::RegisterSpec for FAFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fafifo::R](R) reader structure"]
impl crate::Readable for FAFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fafifo::W](W) writer structure"]
impl crate::Writable for FAFIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FAFIFO to value 0"]
impl crate::Resettable for FAFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
