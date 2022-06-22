#[doc = "Register `FW` reader"]
pub struct R(crate::R<FW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FW` writer"]
pub struct W(crate::W<FW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FW_SPEC>;
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
impl From<crate::W<FW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Filter working\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FW0_A {
    #[doc = "0: Filter working disabled"]
    DISABLED = 0,
    #[doc = "1: Filter working enabled"]
    ENABLED = 1,
}
impl From<FW0_A> for bool {
    #[inline(always)]
    fn from(variant: FW0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FW0` reader - Filter working"]
pub type FW0_R = crate::BitReader<FW0_A>;
impl FW0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FW0_A {
        match self.bits {
            false => FW0_A::DISABLED,
            true => FW0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FW0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FW0_A::ENABLED
    }
}
#[doc = "Field `FW0` writer - Filter working"]
pub type FW0_W<'a> = crate::BitWriter<'a, u32, FW_SPEC, FW0_A, 0>;
impl<'a> FW0_W<'a> {
    #[doc = "Filter working disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FW0_A::DISABLED)
    }
    #[doc = "Filter working enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FW0_A::ENABLED)
    }
}
#[doc = "Filter working"]
pub use FW0_A as FW1_A;
#[doc = "Filter working"]
pub use FW0_A as FW2_A;
#[doc = "Filter working"]
pub use FW0_A as FW3_A;
#[doc = "Filter working"]
pub use FW0_A as FW4_A;
#[doc = "Filter working"]
pub use FW0_A as FW5_A;
#[doc = "Filter working"]
pub use FW0_A as FW6_A;
#[doc = "Filter working"]
pub use FW0_A as FW7_A;
#[doc = "Filter working"]
pub use FW0_A as FW8_A;
#[doc = "Filter working"]
pub use FW0_A as FW9_A;
#[doc = "Filter working"]
pub use FW0_A as FW10_A;
#[doc = "Filter working"]
pub use FW0_A as FW11_A;
#[doc = "Filter working"]
pub use FW0_A as FW12_A;
#[doc = "Filter working"]
pub use FW0_A as FW13_A;
#[doc = "Filter working"]
pub use FW0_A as FW14_A;
#[doc = "Filter working"]
pub use FW0_A as FW15_A;
#[doc = "Filter working"]
pub use FW0_A as FW16_A;
#[doc = "Filter working"]
pub use FW0_A as FW17_A;
#[doc = "Filter working"]
pub use FW0_A as FW18_A;
#[doc = "Filter working"]
pub use FW0_A as FW19_A;
#[doc = "Filter working"]
pub use FW0_A as FW20_A;
#[doc = "Filter working"]
pub use FW0_A as FW21_A;
#[doc = "Filter working"]
pub use FW0_A as FW22_A;
#[doc = "Filter working"]
pub use FW0_A as FW23_A;
#[doc = "Filter working"]
pub use FW0_A as FW24_A;
#[doc = "Filter working"]
pub use FW0_A as FW25_A;
#[doc = "Filter working"]
pub use FW0_A as FW26_A;
#[doc = "Filter working"]
pub use FW0_A as FW27_A;
#[doc = "Field `FW1` reader - Filter working"]
pub use FW0_R as FW1_R;
#[doc = "Field `FW2` reader - Filter working"]
pub use FW0_R as FW2_R;
#[doc = "Field `FW3` reader - Filter working"]
pub use FW0_R as FW3_R;
#[doc = "Field `FW4` reader - Filter working"]
pub use FW0_R as FW4_R;
#[doc = "Field `FW5` reader - Filter working"]
pub use FW0_R as FW5_R;
#[doc = "Field `FW6` reader - Filter working"]
pub use FW0_R as FW6_R;
#[doc = "Field `FW7` reader - Filter working"]
pub use FW0_R as FW7_R;
#[doc = "Field `FW8` reader - Filter working"]
pub use FW0_R as FW8_R;
#[doc = "Field `FW9` reader - Filter working"]
pub use FW0_R as FW9_R;
#[doc = "Field `FW10` reader - Filter working"]
pub use FW0_R as FW10_R;
#[doc = "Field `FW11` reader - Filter working"]
pub use FW0_R as FW11_R;
#[doc = "Field `FW12` reader - Filter working"]
pub use FW0_R as FW12_R;
#[doc = "Field `FW13` reader - Filter working"]
pub use FW0_R as FW13_R;
#[doc = "Field `FW14` reader - Filter working"]
pub use FW0_R as FW14_R;
#[doc = "Field `FW15` reader - Filter working"]
pub use FW0_R as FW15_R;
#[doc = "Field `FW16` reader - Filter working"]
pub use FW0_R as FW16_R;
#[doc = "Field `FW17` reader - Filter working"]
pub use FW0_R as FW17_R;
#[doc = "Field `FW18` reader - Filter working"]
pub use FW0_R as FW18_R;
#[doc = "Field `FW19` reader - Filter working"]
pub use FW0_R as FW19_R;
#[doc = "Field `FW20` reader - Filter working"]
pub use FW0_R as FW20_R;
#[doc = "Field `FW21` reader - Filter working"]
pub use FW0_R as FW21_R;
#[doc = "Field `FW22` reader - Filter working"]
pub use FW0_R as FW22_R;
#[doc = "Field `FW23` reader - Filter working"]
pub use FW0_R as FW23_R;
#[doc = "Field `FW24` reader - Filter working"]
pub use FW0_R as FW24_R;
#[doc = "Field `FW25` reader - Filter working"]
pub use FW0_R as FW25_R;
#[doc = "Field `FW26` reader - Filter working"]
pub use FW0_R as FW26_R;
#[doc = "Field `FW27` reader - Filter working"]
pub use FW0_R as FW27_R;
#[doc = "Field `FW1` writer - Filter working"]
pub use FW0_W as FW1_W;
#[doc = "Field `FW2` writer - Filter working"]
pub use FW0_W as FW2_W;
#[doc = "Field `FW3` writer - Filter working"]
pub use FW0_W as FW3_W;
#[doc = "Field `FW4` writer - Filter working"]
pub use FW0_W as FW4_W;
#[doc = "Field `FW5` writer - Filter working"]
pub use FW0_W as FW5_W;
#[doc = "Field `FW6` writer - Filter working"]
pub use FW0_W as FW6_W;
#[doc = "Field `FW7` writer - Filter working"]
pub use FW0_W as FW7_W;
#[doc = "Field `FW8` writer - Filter working"]
pub use FW0_W as FW8_W;
#[doc = "Field `FW9` writer - Filter working"]
pub use FW0_W as FW9_W;
#[doc = "Field `FW10` writer - Filter working"]
pub use FW0_W as FW10_W;
#[doc = "Field `FW11` writer - Filter working"]
pub use FW0_W as FW11_W;
#[doc = "Field `FW12` writer - Filter working"]
pub use FW0_W as FW12_W;
#[doc = "Field `FW13` writer - Filter working"]
pub use FW0_W as FW13_W;
#[doc = "Field `FW14` writer - Filter working"]
pub use FW0_W as FW14_W;
#[doc = "Field `FW15` writer - Filter working"]
pub use FW0_W as FW15_W;
#[doc = "Field `FW16` writer - Filter working"]
pub use FW0_W as FW16_W;
#[doc = "Field `FW17` writer - Filter working"]
pub use FW0_W as FW17_W;
#[doc = "Field `FW18` writer - Filter working"]
pub use FW0_W as FW18_W;
#[doc = "Field `FW19` writer - Filter working"]
pub use FW0_W as FW19_W;
#[doc = "Field `FW20` writer - Filter working"]
pub use FW0_W as FW20_W;
#[doc = "Field `FW21` writer - Filter working"]
pub use FW0_W as FW21_W;
#[doc = "Field `FW22` writer - Filter working"]
pub use FW0_W as FW22_W;
#[doc = "Field `FW23` writer - Filter working"]
pub use FW0_W as FW23_W;
#[doc = "Field `FW24` writer - Filter working"]
pub use FW0_W as FW24_W;
#[doc = "Field `FW25` writer - Filter working"]
pub use FW0_W as FW25_W;
#[doc = "Field `FW26` writer - Filter working"]
pub use FW0_W as FW26_W;
#[doc = "Field `FW27` writer - Filter working"]
pub use FW0_W as FW27_W;
impl R {
    #[doc = "Bit 0 - Filter working"]
    #[inline(always)]
    pub fn fw0(&self) -> FW0_R {
        FW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter working"]
    #[inline(always)]
    pub fn fw1(&self) -> FW1_R {
        FW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter working"]
    #[inline(always)]
    pub fn fw2(&self) -> FW2_R {
        FW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter working"]
    #[inline(always)]
    pub fn fw3(&self) -> FW3_R {
        FW3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter working"]
    #[inline(always)]
    pub fn fw4(&self) -> FW4_R {
        FW4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter working"]
    #[inline(always)]
    pub fn fw5(&self) -> FW5_R {
        FW5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter working"]
    #[inline(always)]
    pub fn fw6(&self) -> FW6_R {
        FW6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter working"]
    #[inline(always)]
    pub fn fw7(&self) -> FW7_R {
        FW7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter working"]
    #[inline(always)]
    pub fn fw8(&self) -> FW8_R {
        FW8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter working"]
    #[inline(always)]
    pub fn fw9(&self) -> FW9_R {
        FW9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter working"]
    #[inline(always)]
    pub fn fw10(&self) -> FW10_R {
        FW10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter working"]
    #[inline(always)]
    pub fn fw11(&self) -> FW11_R {
        FW11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter working"]
    #[inline(always)]
    pub fn fw12(&self) -> FW12_R {
        FW12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter working"]
    #[inline(always)]
    pub fn fw13(&self) -> FW13_R {
        FW13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter working"]
    #[inline(always)]
    pub fn fw14(&self) -> FW14_R {
        FW14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter working"]
    #[inline(always)]
    pub fn fw15(&self) -> FW15_R {
        FW15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter working"]
    #[inline(always)]
    pub fn fw16(&self) -> FW16_R {
        FW16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter working"]
    #[inline(always)]
    pub fn fw17(&self) -> FW17_R {
        FW17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter working"]
    #[inline(always)]
    pub fn fw18(&self) -> FW18_R {
        FW18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter working"]
    #[inline(always)]
    pub fn fw19(&self) -> FW19_R {
        FW19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter working"]
    #[inline(always)]
    pub fn fw20(&self) -> FW20_R {
        FW20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter working"]
    #[inline(always)]
    pub fn fw21(&self) -> FW21_R {
        FW21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter working"]
    #[inline(always)]
    pub fn fw22(&self) -> FW22_R {
        FW22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter working"]
    #[inline(always)]
    pub fn fw23(&self) -> FW23_R {
        FW23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter working"]
    #[inline(always)]
    pub fn fw24(&self) -> FW24_R {
        FW24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter working"]
    #[inline(always)]
    pub fn fw25(&self) -> FW25_R {
        FW25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter working"]
    #[inline(always)]
    pub fn fw26(&self) -> FW26_R {
        FW26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter working"]
    #[inline(always)]
    pub fn fw27(&self) -> FW27_R {
        FW27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter working"]
    #[inline(always)]
    pub fn fw0(&mut self) -> FW0_W {
        FW0_W::new(self)
    }
    #[doc = "Bit 1 - Filter working"]
    #[inline(always)]
    pub fn fw1(&mut self) -> FW1_W {
        FW1_W::new(self)
    }
    #[doc = "Bit 2 - Filter working"]
    #[inline(always)]
    pub fn fw2(&mut self) -> FW2_W {
        FW2_W::new(self)
    }
    #[doc = "Bit 3 - Filter working"]
    #[inline(always)]
    pub fn fw3(&mut self) -> FW3_W {
        FW3_W::new(self)
    }
    #[doc = "Bit 4 - Filter working"]
    #[inline(always)]
    pub fn fw4(&mut self) -> FW4_W {
        FW4_W::new(self)
    }
    #[doc = "Bit 5 - Filter working"]
    #[inline(always)]
    pub fn fw5(&mut self) -> FW5_W {
        FW5_W::new(self)
    }
    #[doc = "Bit 6 - Filter working"]
    #[inline(always)]
    pub fn fw6(&mut self) -> FW6_W {
        FW6_W::new(self)
    }
    #[doc = "Bit 7 - Filter working"]
    #[inline(always)]
    pub fn fw7(&mut self) -> FW7_W {
        FW7_W::new(self)
    }
    #[doc = "Bit 8 - Filter working"]
    #[inline(always)]
    pub fn fw8(&mut self) -> FW8_W {
        FW8_W::new(self)
    }
    #[doc = "Bit 9 - Filter working"]
    #[inline(always)]
    pub fn fw9(&mut self) -> FW9_W {
        FW9_W::new(self)
    }
    #[doc = "Bit 10 - Filter working"]
    #[inline(always)]
    pub fn fw10(&mut self) -> FW10_W {
        FW10_W::new(self)
    }
    #[doc = "Bit 11 - Filter working"]
    #[inline(always)]
    pub fn fw11(&mut self) -> FW11_W {
        FW11_W::new(self)
    }
    #[doc = "Bit 12 - Filter working"]
    #[inline(always)]
    pub fn fw12(&mut self) -> FW12_W {
        FW12_W::new(self)
    }
    #[doc = "Bit 13 - Filter working"]
    #[inline(always)]
    pub fn fw13(&mut self) -> FW13_W {
        FW13_W::new(self)
    }
    #[doc = "Bit 14 - Filter working"]
    #[inline(always)]
    pub fn fw14(&mut self) -> FW14_W {
        FW14_W::new(self)
    }
    #[doc = "Bit 15 - Filter working"]
    #[inline(always)]
    pub fn fw15(&mut self) -> FW15_W {
        FW15_W::new(self)
    }
    #[doc = "Bit 16 - Filter working"]
    #[inline(always)]
    pub fn fw16(&mut self) -> FW16_W {
        FW16_W::new(self)
    }
    #[doc = "Bit 17 - Filter working"]
    #[inline(always)]
    pub fn fw17(&mut self) -> FW17_W {
        FW17_W::new(self)
    }
    #[doc = "Bit 18 - Filter working"]
    #[inline(always)]
    pub fn fw18(&mut self) -> FW18_W {
        FW18_W::new(self)
    }
    #[doc = "Bit 19 - Filter working"]
    #[inline(always)]
    pub fn fw19(&mut self) -> FW19_W {
        FW19_W::new(self)
    }
    #[doc = "Bit 20 - Filter working"]
    #[inline(always)]
    pub fn fw20(&mut self) -> FW20_W {
        FW20_W::new(self)
    }
    #[doc = "Bit 21 - Filter working"]
    #[inline(always)]
    pub fn fw21(&mut self) -> FW21_W {
        FW21_W::new(self)
    }
    #[doc = "Bit 22 - Filter working"]
    #[inline(always)]
    pub fn fw22(&mut self) -> FW22_W {
        FW22_W::new(self)
    }
    #[doc = "Bit 23 - Filter working"]
    #[inline(always)]
    pub fn fw23(&mut self) -> FW23_W {
        FW23_W::new(self)
    }
    #[doc = "Bit 24 - Filter working"]
    #[inline(always)]
    pub fn fw24(&mut self) -> FW24_W {
        FW24_W::new(self)
    }
    #[doc = "Bit 25 - Filter working"]
    #[inline(always)]
    pub fn fw25(&mut self) -> FW25_W {
        FW25_W::new(self)
    }
    #[doc = "Bit 26 - Filter working"]
    #[inline(always)]
    pub fn fw26(&mut self) -> FW26_W {
        FW26_W::new(self)
    }
    #[doc = "Bit 27 - Filter working"]
    #[inline(always)]
    pub fn fw27(&mut self) -> FW27_W {
        FW27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter working register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fw](index.html) module"]
pub struct FW_SPEC;
impl crate::RegisterSpec for FW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fw::R](R) reader structure"]
impl crate::Readable for FW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fw::W](W) writer structure"]
impl crate::Writable for FW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FW to value 0"]
impl crate::Resettable for FW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
