#[doc = "Register `EVEN` reader"]
pub struct R(crate::R<EVEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVEN` writer"]
pub struct W(crate::W<EVEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVEN_SPEC>;
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
impl From<crate::W<EVEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable Event on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVEN0_A {
    #[doc = "0: Event from line is disabled"]
    MASKED = 0,
    #[doc = "1: Event from line is enabled"]
    UNMASKED = 1,
}
impl From<EVEN0_A> for bool {
    #[inline(always)]
    fn from(variant: EVEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVEN0` reader - Enable Event on line 0"]
pub type EVEN0_R = crate::BitReader<EVEN0_A>;
impl EVEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVEN0_A {
        match self.bits {
            false => EVEN0_A::MASKED,
            true => EVEN0_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EVEN0_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EVEN0_A::UNMASKED
    }
}
#[doc = "Field `EVEN0` writer - Enable Event on line 0"]
pub type EVEN0_W<'a> = crate::BitWriter<'a, u32, EVEN_SPEC, EVEN0_A, 0>;
impl<'a> EVEN0_W<'a> {
    #[doc = "Event from line is disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EVEN0_A::MASKED)
    }
    #[doc = "Event from line is enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EVEN0_A::UNMASKED)
    }
}
#[doc = "Enable Event on line 1"]
pub use EVEN0_A as EVEN1_A;
#[doc = "Enable Event on line 2"]
pub use EVEN0_A as EVEN2_A;
#[doc = "Enable Event on line 3"]
pub use EVEN0_A as EVEN3_A;
#[doc = "Enable Event on line 4"]
pub use EVEN0_A as EVEN4_A;
#[doc = "Enable Event on line 5"]
pub use EVEN0_A as EVEN5_A;
#[doc = "Enable Event on line 6"]
pub use EVEN0_A as EVEN6_A;
#[doc = "Enable Event on line 7"]
pub use EVEN0_A as EVEN7_A;
#[doc = "Enable Event on line 8"]
pub use EVEN0_A as EVEN8_A;
#[doc = "Enable Event on line 9"]
pub use EVEN0_A as EVEN9_A;
#[doc = "Enable Event on line 10"]
pub use EVEN0_A as EVEN10_A;
#[doc = "Enable Event on line 11"]
pub use EVEN0_A as EVEN11_A;
#[doc = "Enable Event on line 12"]
pub use EVEN0_A as EVEN12_A;
#[doc = "Enable Event on line 13"]
pub use EVEN0_A as EVEN13_A;
#[doc = "Enable Event on line 14"]
pub use EVEN0_A as EVEN14_A;
#[doc = "Enable Event on line 15"]
pub use EVEN0_A as EVEN15_A;
#[doc = "Enable Event on line 16"]
pub use EVEN0_A as EVEN16_A;
#[doc = "Enable Event on line 17"]
pub use EVEN0_A as EVEN17_A;
#[doc = "Enable Event on line 18"]
pub use EVEN0_A as EVEN18_A;
#[doc = "Enable Event on line 19"]
pub use EVEN0_A as EVEN19_A;
#[doc = "Enable Event on line 20"]
pub use EVEN0_A as EVEN20_A;
#[doc = "Enable Event on line 21"]
pub use EVEN0_A as EVEN21_A;
#[doc = "Enable Event on line 22"]
pub use EVEN0_A as EVEN22_A;
#[doc = "Enable Event on line 23"]
pub use EVEN0_A as EVEN23_A;
#[doc = "Enable Event on line 24"]
pub use EVEN0_A as EVEN24_A;
#[doc = "Enable Event on line 25"]
pub use EVEN0_A as EVEN25_A;
#[doc = "Enable Event on line 26"]
pub use EVEN0_A as EVEN26_A;
#[doc = "Enable Event on line 27"]
pub use EVEN0_A as EVEN27_A;
#[doc = "Field `EVEN1` reader - Enable Event on line 1"]
pub use EVEN0_R as EVEN1_R;
#[doc = "Field `EVEN2` reader - Enable Event on line 2"]
pub use EVEN0_R as EVEN2_R;
#[doc = "Field `EVEN3` reader - Enable Event on line 3"]
pub use EVEN0_R as EVEN3_R;
#[doc = "Field `EVEN4` reader - Enable Event on line 4"]
pub use EVEN0_R as EVEN4_R;
#[doc = "Field `EVEN5` reader - Enable Event on line 5"]
pub use EVEN0_R as EVEN5_R;
#[doc = "Field `EVEN6` reader - Enable Event on line 6"]
pub use EVEN0_R as EVEN6_R;
#[doc = "Field `EVEN7` reader - Enable Event on line 7"]
pub use EVEN0_R as EVEN7_R;
#[doc = "Field `EVEN8` reader - Enable Event on line 8"]
pub use EVEN0_R as EVEN8_R;
#[doc = "Field `EVEN9` reader - Enable Event on line 9"]
pub use EVEN0_R as EVEN9_R;
#[doc = "Field `EVEN10` reader - Enable Event on line 10"]
pub use EVEN0_R as EVEN10_R;
#[doc = "Field `EVEN11` reader - Enable Event on line 11"]
pub use EVEN0_R as EVEN11_R;
#[doc = "Field `EVEN12` reader - Enable Event on line 12"]
pub use EVEN0_R as EVEN12_R;
#[doc = "Field `EVEN13` reader - Enable Event on line 13"]
pub use EVEN0_R as EVEN13_R;
#[doc = "Field `EVEN14` reader - Enable Event on line 14"]
pub use EVEN0_R as EVEN14_R;
#[doc = "Field `EVEN15` reader - Enable Event on line 15"]
pub use EVEN0_R as EVEN15_R;
#[doc = "Field `EVEN16` reader - Enable Event on line 16"]
pub use EVEN0_R as EVEN16_R;
#[doc = "Field `EVEN17` reader - Enable Event on line 17"]
pub use EVEN0_R as EVEN17_R;
#[doc = "Field `EVEN18` reader - Enable Event on line 18"]
pub use EVEN0_R as EVEN18_R;
#[doc = "Field `EVEN19` reader - Enable Event on line 19"]
pub use EVEN0_R as EVEN19_R;
#[doc = "Field `EVEN20` reader - Enable Event on line 20"]
pub use EVEN0_R as EVEN20_R;
#[doc = "Field `EVEN21` reader - Enable Event on line 21"]
pub use EVEN0_R as EVEN21_R;
#[doc = "Field `EVEN22` reader - Enable Event on line 22"]
pub use EVEN0_R as EVEN22_R;
#[doc = "Field `EVEN23` reader - Enable Event on line 23"]
pub use EVEN0_R as EVEN23_R;
#[doc = "Field `EVEN24` reader - Enable Event on line 24"]
pub use EVEN0_R as EVEN24_R;
#[doc = "Field `EVEN25` reader - Enable Event on line 25"]
pub use EVEN0_R as EVEN25_R;
#[doc = "Field `EVEN26` reader - Enable Event on line 26"]
pub use EVEN0_R as EVEN26_R;
#[doc = "Field `EVEN27` reader - Enable Event on line 27"]
pub use EVEN0_R as EVEN27_R;
#[doc = "Field `EVEN1` writer - Enable Event on line 1"]
pub use EVEN0_W as EVEN1_W;
#[doc = "Field `EVEN2` writer - Enable Event on line 2"]
pub use EVEN0_W as EVEN2_W;
#[doc = "Field `EVEN3` writer - Enable Event on line 3"]
pub use EVEN0_W as EVEN3_W;
#[doc = "Field `EVEN4` writer - Enable Event on line 4"]
pub use EVEN0_W as EVEN4_W;
#[doc = "Field `EVEN5` writer - Enable Event on line 5"]
pub use EVEN0_W as EVEN5_W;
#[doc = "Field `EVEN6` writer - Enable Event on line 6"]
pub use EVEN0_W as EVEN6_W;
#[doc = "Field `EVEN7` writer - Enable Event on line 7"]
pub use EVEN0_W as EVEN7_W;
#[doc = "Field `EVEN8` writer - Enable Event on line 8"]
pub use EVEN0_W as EVEN8_W;
#[doc = "Field `EVEN9` writer - Enable Event on line 9"]
pub use EVEN0_W as EVEN9_W;
#[doc = "Field `EVEN10` writer - Enable Event on line 10"]
pub use EVEN0_W as EVEN10_W;
#[doc = "Field `EVEN11` writer - Enable Event on line 11"]
pub use EVEN0_W as EVEN11_W;
#[doc = "Field `EVEN12` writer - Enable Event on line 12"]
pub use EVEN0_W as EVEN12_W;
#[doc = "Field `EVEN13` writer - Enable Event on line 13"]
pub use EVEN0_W as EVEN13_W;
#[doc = "Field `EVEN14` writer - Enable Event on line 14"]
pub use EVEN0_W as EVEN14_W;
#[doc = "Field `EVEN15` writer - Enable Event on line 15"]
pub use EVEN0_W as EVEN15_W;
#[doc = "Field `EVEN16` writer - Enable Event on line 16"]
pub use EVEN0_W as EVEN16_W;
#[doc = "Field `EVEN17` writer - Enable Event on line 17"]
pub use EVEN0_W as EVEN17_W;
#[doc = "Field `EVEN18` writer - Enable Event on line 18"]
pub use EVEN0_W as EVEN18_W;
#[doc = "Field `EVEN19` writer - Enable Event on line 19"]
pub use EVEN0_W as EVEN19_W;
#[doc = "Field `EVEN20` writer - Enable Event on line 20"]
pub use EVEN0_W as EVEN20_W;
#[doc = "Field `EVEN21` writer - Enable Event on line 21"]
pub use EVEN0_W as EVEN21_W;
#[doc = "Field `EVEN22` writer - Enable Event on line 22"]
pub use EVEN0_W as EVEN22_W;
#[doc = "Field `EVEN23` writer - Enable Event on line 23"]
pub use EVEN0_W as EVEN23_W;
#[doc = "Field `EVEN24` writer - Enable Event on line 24"]
pub use EVEN0_W as EVEN24_W;
#[doc = "Field `EVEN25` writer - Enable Event on line 25"]
pub use EVEN0_W as EVEN25_W;
#[doc = "Field `EVEN26` writer - Enable Event on line 26"]
pub use EVEN0_W as EVEN26_W;
#[doc = "Field `EVEN27` writer - Enable Event on line 27"]
pub use EVEN0_W as EVEN27_W;
impl R {
    #[doc = "Bit 0 - Enable Event on line 0"]
    #[inline(always)]
    pub fn even0(&self) -> EVEN0_R {
        EVEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Event on line 1"]
    #[inline(always)]
    pub fn even1(&self) -> EVEN1_R {
        EVEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Event on line 2"]
    #[inline(always)]
    pub fn even2(&self) -> EVEN2_R {
        EVEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Event on line 3"]
    #[inline(always)]
    pub fn even3(&self) -> EVEN3_R {
        EVEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Event on line 4"]
    #[inline(always)]
    pub fn even4(&self) -> EVEN4_R {
        EVEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Event on line 5"]
    #[inline(always)]
    pub fn even5(&self) -> EVEN5_R {
        EVEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Event on line 6"]
    #[inline(always)]
    pub fn even6(&self) -> EVEN6_R {
        EVEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Event on line 7"]
    #[inline(always)]
    pub fn even7(&self) -> EVEN7_R {
        EVEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Event on line 8"]
    #[inline(always)]
    pub fn even8(&self) -> EVEN8_R {
        EVEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Event on line 9"]
    #[inline(always)]
    pub fn even9(&self) -> EVEN9_R {
        EVEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Event on line 10"]
    #[inline(always)]
    pub fn even10(&self) -> EVEN10_R {
        EVEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Event on line 11"]
    #[inline(always)]
    pub fn even11(&self) -> EVEN11_R {
        EVEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Event on line 12"]
    #[inline(always)]
    pub fn even12(&self) -> EVEN12_R {
        EVEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Event on line 13"]
    #[inline(always)]
    pub fn even13(&self) -> EVEN13_R {
        EVEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Event on line 14"]
    #[inline(always)]
    pub fn even14(&self) -> EVEN14_R {
        EVEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Event on line 15"]
    #[inline(always)]
    pub fn even15(&self) -> EVEN15_R {
        EVEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Event on line 16"]
    #[inline(always)]
    pub fn even16(&self) -> EVEN16_R {
        EVEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Event on line 17"]
    #[inline(always)]
    pub fn even17(&self) -> EVEN17_R {
        EVEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Event on line 18"]
    #[inline(always)]
    pub fn even18(&self) -> EVEN18_R {
        EVEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Event on line 19"]
    #[inline(always)]
    pub fn even19(&self) -> EVEN19_R {
        EVEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Event on line 20"]
    #[inline(always)]
    pub fn even20(&self) -> EVEN20_R {
        EVEN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Event on line 21"]
    #[inline(always)]
    pub fn even21(&self) -> EVEN21_R {
        EVEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Event on line 22"]
    #[inline(always)]
    pub fn even22(&self) -> EVEN22_R {
        EVEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Event on line 23"]
    #[inline(always)]
    pub fn even23(&self) -> EVEN23_R {
        EVEN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Event on line 24"]
    #[inline(always)]
    pub fn even24(&self) -> EVEN24_R {
        EVEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Event on line 25"]
    #[inline(always)]
    pub fn even25(&self) -> EVEN25_R {
        EVEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Event on line 26"]
    #[inline(always)]
    pub fn even26(&self) -> EVEN26_R {
        EVEN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Event on line 27"]
    #[inline(always)]
    pub fn even27(&self) -> EVEN27_R {
        EVEN27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Event on line 0"]
    #[inline(always)]
    pub fn even0(&mut self) -> EVEN0_W {
        EVEN0_W::new(self)
    }
    #[doc = "Bit 1 - Enable Event on line 1"]
    #[inline(always)]
    pub fn even1(&mut self) -> EVEN1_W {
        EVEN1_W::new(self)
    }
    #[doc = "Bit 2 - Enable Event on line 2"]
    #[inline(always)]
    pub fn even2(&mut self) -> EVEN2_W {
        EVEN2_W::new(self)
    }
    #[doc = "Bit 3 - Enable Event on line 3"]
    #[inline(always)]
    pub fn even3(&mut self) -> EVEN3_W {
        EVEN3_W::new(self)
    }
    #[doc = "Bit 4 - Enable Event on line 4"]
    #[inline(always)]
    pub fn even4(&mut self) -> EVEN4_W {
        EVEN4_W::new(self)
    }
    #[doc = "Bit 5 - Enable Event on line 5"]
    #[inline(always)]
    pub fn even5(&mut self) -> EVEN5_W {
        EVEN5_W::new(self)
    }
    #[doc = "Bit 6 - Enable Event on line 6"]
    #[inline(always)]
    pub fn even6(&mut self) -> EVEN6_W {
        EVEN6_W::new(self)
    }
    #[doc = "Bit 7 - Enable Event on line 7"]
    #[inline(always)]
    pub fn even7(&mut self) -> EVEN7_W {
        EVEN7_W::new(self)
    }
    #[doc = "Bit 8 - Enable Event on line 8"]
    #[inline(always)]
    pub fn even8(&mut self) -> EVEN8_W {
        EVEN8_W::new(self)
    }
    #[doc = "Bit 9 - Enable Event on line 9"]
    #[inline(always)]
    pub fn even9(&mut self) -> EVEN9_W {
        EVEN9_W::new(self)
    }
    #[doc = "Bit 10 - Enable Event on line 10"]
    #[inline(always)]
    pub fn even10(&mut self) -> EVEN10_W {
        EVEN10_W::new(self)
    }
    #[doc = "Bit 11 - Enable Event on line 11"]
    #[inline(always)]
    pub fn even11(&mut self) -> EVEN11_W {
        EVEN11_W::new(self)
    }
    #[doc = "Bit 12 - Enable Event on line 12"]
    #[inline(always)]
    pub fn even12(&mut self) -> EVEN12_W {
        EVEN12_W::new(self)
    }
    #[doc = "Bit 13 - Enable Event on line 13"]
    #[inline(always)]
    pub fn even13(&mut self) -> EVEN13_W {
        EVEN13_W::new(self)
    }
    #[doc = "Bit 14 - Enable Event on line 14"]
    #[inline(always)]
    pub fn even14(&mut self) -> EVEN14_W {
        EVEN14_W::new(self)
    }
    #[doc = "Bit 15 - Enable Event on line 15"]
    #[inline(always)]
    pub fn even15(&mut self) -> EVEN15_W {
        EVEN15_W::new(self)
    }
    #[doc = "Bit 16 - Enable Event on line 16"]
    #[inline(always)]
    pub fn even16(&mut self) -> EVEN16_W {
        EVEN16_W::new(self)
    }
    #[doc = "Bit 17 - Enable Event on line 17"]
    #[inline(always)]
    pub fn even17(&mut self) -> EVEN17_W {
        EVEN17_W::new(self)
    }
    #[doc = "Bit 18 - Enable Event on line 18"]
    #[inline(always)]
    pub fn even18(&mut self) -> EVEN18_W {
        EVEN18_W::new(self)
    }
    #[doc = "Bit 19 - Enable Event on line 19"]
    #[inline(always)]
    pub fn even19(&mut self) -> EVEN19_W {
        EVEN19_W::new(self)
    }
    #[doc = "Bit 20 - Enable Event on line 20"]
    #[inline(always)]
    pub fn even20(&mut self) -> EVEN20_W {
        EVEN20_W::new(self)
    }
    #[doc = "Bit 21 - Enable Event on line 21"]
    #[inline(always)]
    pub fn even21(&mut self) -> EVEN21_W {
        EVEN21_W::new(self)
    }
    #[doc = "Bit 22 - Enable Event on line 22"]
    #[inline(always)]
    pub fn even22(&mut self) -> EVEN22_W {
        EVEN22_W::new(self)
    }
    #[doc = "Bit 23 - Enable Event on line 23"]
    #[inline(always)]
    pub fn even23(&mut self) -> EVEN23_W {
        EVEN23_W::new(self)
    }
    #[doc = "Bit 24 - Enable Event on line 24"]
    #[inline(always)]
    pub fn even24(&mut self) -> EVEN24_W {
        EVEN24_W::new(self)
    }
    #[doc = "Bit 25 - Enable Event on line 25"]
    #[inline(always)]
    pub fn even25(&mut self) -> EVEN25_W {
        EVEN25_W::new(self)
    }
    #[doc = "Bit 26 - Enable Event on line 26"]
    #[inline(always)]
    pub fn even26(&mut self) -> EVEN26_W {
        EVEN26_W::new(self)
    }
    #[doc = "Bit 27 - Enable Event on line 27"]
    #[inline(always)]
    pub fn even27(&mut self) -> EVEN27_W {
        EVEN27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event enable register (EXTI_EVEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [even](index.html) module"]
pub struct EVEN_SPEC;
impl crate::RegisterSpec for EVEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [even::R](R) reader structure"]
impl crate::Readable for EVEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [even::W](W) writer structure"]
impl crate::Writable for EVEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVEN to value 0"]
impl crate::Resettable for EVEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
