#[doc = "Register `RTEN` reader"]
pub struct R(crate::R<RTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTEN` writer"]
pub struct W(crate::W<RTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTEN_SPEC>;
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
impl From<crate::W<RTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Rising edge trigger enable of line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTEN0_A {
    #[doc = "0: Rising edge trigger is disabled"]
    DISABLED = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    ENABLED = 1,
}
impl From<RTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: RTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTEN0` reader - Rising edge trigger enable of line 0"]
pub type RTEN0_R = crate::BitReader<RTEN0_A>;
impl RTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTEN0_A {
        match self.bits {
            false => RTEN0_A::DISABLED,
            true => RTEN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTEN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTEN0_A::ENABLED
    }
}
#[doc = "Field `RTEN0` writer - Rising edge trigger enable of line 0"]
pub type RTEN0_W<'a> = crate::BitWriter<'a, u32, RTEN_SPEC, RTEN0_A, 0>;
impl<'a> RTEN0_W<'a> {
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTEN0_A::DISABLED)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTEN0_A::ENABLED)
    }
}
#[doc = "Rising edge trigger enable of line 1"]
pub use RTEN0_A as RTEN1_A;
#[doc = "Rising edge trigger enable of line 2"]
pub use RTEN0_A as RTEN2_A;
#[doc = "Rising edge trigger enable of line 3"]
pub use RTEN0_A as RTEN3_A;
#[doc = "Rising edge trigger enable of line 4"]
pub use RTEN0_A as RTEN4_A;
#[doc = "Rising edge trigger enable of line 5"]
pub use RTEN0_A as RTEN5_A;
#[doc = "Rising edge trigger enable of line 6"]
pub use RTEN0_A as RTEN6_A;
#[doc = "Rising edge trigger enable of line 7"]
pub use RTEN0_A as RTEN7_A;
#[doc = "Rising edge trigger enable of line 8"]
pub use RTEN0_A as RTEN8_A;
#[doc = "Rising edge trigger enable of line 9"]
pub use RTEN0_A as RTEN9_A;
#[doc = "Rising edge trigger enable of line 10"]
pub use RTEN0_A as RTEN10_A;
#[doc = "Rising edge trigger enable of line 11"]
pub use RTEN0_A as RTEN11_A;
#[doc = "Rising edge trigger enable of line 12"]
pub use RTEN0_A as RTEN12_A;
#[doc = "Rising edge trigger enable of line 13"]
pub use RTEN0_A as RTEN13_A;
#[doc = "Rising edge trigger enable of line 14"]
pub use RTEN0_A as RTEN14_A;
#[doc = "Rising edge trigger enable of line 15"]
pub use RTEN0_A as RTEN15_A;
#[doc = "Rising edge trigger enable of line 16"]
pub use RTEN0_A as RTEN16_A;
#[doc = "Rising edge trigger enable of line 17"]
pub use RTEN0_A as RTEN17_A;
#[doc = "Rising edge trigger enable of line 18"]
pub use RTEN0_A as RTEN18_A;
#[doc = "Rising edge trigger enable of line 19"]
pub use RTEN0_A as RTEN19_A;
#[doc = "Field `RTEN1` reader - Rising edge trigger enable of line 1"]
pub use RTEN0_R as RTEN1_R;
#[doc = "Field `RTEN2` reader - Rising edge trigger enable of line 2"]
pub use RTEN0_R as RTEN2_R;
#[doc = "Field `RTEN3` reader - Rising edge trigger enable of line 3"]
pub use RTEN0_R as RTEN3_R;
#[doc = "Field `RTEN4` reader - Rising edge trigger enable of line 4"]
pub use RTEN0_R as RTEN4_R;
#[doc = "Field `RTEN5` reader - Rising edge trigger enable of line 5"]
pub use RTEN0_R as RTEN5_R;
#[doc = "Field `RTEN6` reader - Rising edge trigger enable of line 6"]
pub use RTEN0_R as RTEN6_R;
#[doc = "Field `RTEN7` reader - Rising edge trigger enable of line 7"]
pub use RTEN0_R as RTEN7_R;
#[doc = "Field `RTEN8` reader - Rising edge trigger enable of line 8"]
pub use RTEN0_R as RTEN8_R;
#[doc = "Field `RTEN9` reader - Rising edge trigger enable of line 9"]
pub use RTEN0_R as RTEN9_R;
#[doc = "Field `RTEN10` reader - Rising edge trigger enable of line 10"]
pub use RTEN0_R as RTEN10_R;
#[doc = "Field `RTEN11` reader - Rising edge trigger enable of line 11"]
pub use RTEN0_R as RTEN11_R;
#[doc = "Field `RTEN12` reader - Rising edge trigger enable of line 12"]
pub use RTEN0_R as RTEN12_R;
#[doc = "Field `RTEN13` reader - Rising edge trigger enable of line 13"]
pub use RTEN0_R as RTEN13_R;
#[doc = "Field `RTEN14` reader - Rising edge trigger enable of line 14"]
pub use RTEN0_R as RTEN14_R;
#[doc = "Field `RTEN15` reader - Rising edge trigger enable of line 15"]
pub use RTEN0_R as RTEN15_R;
#[doc = "Field `RTEN16` reader - Rising edge trigger enable of line 16"]
pub use RTEN0_R as RTEN16_R;
#[doc = "Field `RTEN17` reader - Rising edge trigger enable of line 17"]
pub use RTEN0_R as RTEN17_R;
#[doc = "Field `RTEN18` reader - Rising edge trigger enable of line 18"]
pub use RTEN0_R as RTEN18_R;
#[doc = "Field `RTEN19` reader - Rising edge trigger enable of line 19"]
pub use RTEN0_R as RTEN19_R;
#[doc = "Field `RTEN1` writer - Rising edge trigger enable of line 1"]
pub use RTEN0_W as RTEN1_W;
#[doc = "Field `RTEN2` writer - Rising edge trigger enable of line 2"]
pub use RTEN0_W as RTEN2_W;
#[doc = "Field `RTEN3` writer - Rising edge trigger enable of line 3"]
pub use RTEN0_W as RTEN3_W;
#[doc = "Field `RTEN4` writer - Rising edge trigger enable of line 4"]
pub use RTEN0_W as RTEN4_W;
#[doc = "Field `RTEN5` writer - Rising edge trigger enable of line 5"]
pub use RTEN0_W as RTEN5_W;
#[doc = "Field `RTEN6` writer - Rising edge trigger enable of line 6"]
pub use RTEN0_W as RTEN6_W;
#[doc = "Field `RTEN7` writer - Rising edge trigger enable of line 7"]
pub use RTEN0_W as RTEN7_W;
#[doc = "Field `RTEN8` writer - Rising edge trigger enable of line 8"]
pub use RTEN0_W as RTEN8_W;
#[doc = "Field `RTEN9` writer - Rising edge trigger enable of line 9"]
pub use RTEN0_W as RTEN9_W;
#[doc = "Field `RTEN10` writer - Rising edge trigger enable of line 10"]
pub use RTEN0_W as RTEN10_W;
#[doc = "Field `RTEN11` writer - Rising edge trigger enable of line 11"]
pub use RTEN0_W as RTEN11_W;
#[doc = "Field `RTEN12` writer - Rising edge trigger enable of line 12"]
pub use RTEN0_W as RTEN12_W;
#[doc = "Field `RTEN13` writer - Rising edge trigger enable of line 13"]
pub use RTEN0_W as RTEN13_W;
#[doc = "Field `RTEN14` writer - Rising edge trigger enable of line 14"]
pub use RTEN0_W as RTEN14_W;
#[doc = "Field `RTEN15` writer - Rising edge trigger enable of line 15"]
pub use RTEN0_W as RTEN15_W;
#[doc = "Field `RTEN16` writer - Rising edge trigger enable of line 16"]
pub use RTEN0_W as RTEN16_W;
#[doc = "Field `RTEN17` writer - Rising edge trigger enable of line 17"]
pub use RTEN0_W as RTEN17_W;
#[doc = "Field `RTEN18` writer - Rising edge trigger enable of line 18"]
pub use RTEN0_W as RTEN18_W;
#[doc = "Field `RTEN19` writer - Rising edge trigger enable of line 19"]
pub use RTEN0_W as RTEN19_W;
impl R {
    #[doc = "Bit 0 - Rising edge trigger enable of line 0"]
    #[inline(always)]
    pub fn rten0(&self) -> RTEN0_R {
        RTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising edge trigger enable of line 1"]
    #[inline(always)]
    pub fn rten1(&self) -> RTEN1_R {
        RTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising edge trigger enable of line 2"]
    #[inline(always)]
    pub fn rten2(&self) -> RTEN2_R {
        RTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising edge trigger enable of line 3"]
    #[inline(always)]
    pub fn rten3(&self) -> RTEN3_R {
        RTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising edge trigger enable of line 4"]
    #[inline(always)]
    pub fn rten4(&self) -> RTEN4_R {
        RTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising edge trigger enable of line 5"]
    #[inline(always)]
    pub fn rten5(&self) -> RTEN5_R {
        RTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising edge trigger enable of line 6"]
    #[inline(always)]
    pub fn rten6(&self) -> RTEN6_R {
        RTEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising edge trigger enable of line 7"]
    #[inline(always)]
    pub fn rten7(&self) -> RTEN7_R {
        RTEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising edge trigger enable of line 8"]
    #[inline(always)]
    pub fn rten8(&self) -> RTEN8_R {
        RTEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising edge trigger enable of line 9"]
    #[inline(always)]
    pub fn rten9(&self) -> RTEN9_R {
        RTEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising edge trigger enable of line 10"]
    #[inline(always)]
    pub fn rten10(&self) -> RTEN10_R {
        RTEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising edge trigger enable of line 11"]
    #[inline(always)]
    pub fn rten11(&self) -> RTEN11_R {
        RTEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising edge trigger enable of line 12"]
    #[inline(always)]
    pub fn rten12(&self) -> RTEN12_R {
        RTEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising edge trigger enable of line 13"]
    #[inline(always)]
    pub fn rten13(&self) -> RTEN13_R {
        RTEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising edge trigger enable of line 14"]
    #[inline(always)]
    pub fn rten14(&self) -> RTEN14_R {
        RTEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising edge trigger enable of line 15"]
    #[inline(always)]
    pub fn rten15(&self) -> RTEN15_R {
        RTEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising edge trigger enable of line 16"]
    #[inline(always)]
    pub fn rten16(&self) -> RTEN16_R {
        RTEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising edge trigger enable of line 17"]
    #[inline(always)]
    pub fn rten17(&self) -> RTEN17_R {
        RTEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising edge trigger enable of line 18"]
    #[inline(always)]
    pub fn rten18(&self) -> RTEN18_R {
        RTEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising edge trigger enable of line 19"]
    #[inline(always)]
    pub fn rten19(&self) -> RTEN19_R {
        RTEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge trigger enable of line 0"]
    #[inline(always)]
    pub fn rten0(&mut self) -> RTEN0_W {
        RTEN0_W::new(self)
    }
    #[doc = "Bit 1 - Rising edge trigger enable of line 1"]
    #[inline(always)]
    pub fn rten1(&mut self) -> RTEN1_W {
        RTEN1_W::new(self)
    }
    #[doc = "Bit 2 - Rising edge trigger enable of line 2"]
    #[inline(always)]
    pub fn rten2(&mut self) -> RTEN2_W {
        RTEN2_W::new(self)
    }
    #[doc = "Bit 3 - Rising edge trigger enable of line 3"]
    #[inline(always)]
    pub fn rten3(&mut self) -> RTEN3_W {
        RTEN3_W::new(self)
    }
    #[doc = "Bit 4 - Rising edge trigger enable of line 4"]
    #[inline(always)]
    pub fn rten4(&mut self) -> RTEN4_W {
        RTEN4_W::new(self)
    }
    #[doc = "Bit 5 - Rising edge trigger enable of line 5"]
    #[inline(always)]
    pub fn rten5(&mut self) -> RTEN5_W {
        RTEN5_W::new(self)
    }
    #[doc = "Bit 6 - Rising edge trigger enable of line 6"]
    #[inline(always)]
    pub fn rten6(&mut self) -> RTEN6_W {
        RTEN6_W::new(self)
    }
    #[doc = "Bit 7 - Rising edge trigger enable of line 7"]
    #[inline(always)]
    pub fn rten7(&mut self) -> RTEN7_W {
        RTEN7_W::new(self)
    }
    #[doc = "Bit 8 - Rising edge trigger enable of line 8"]
    #[inline(always)]
    pub fn rten8(&mut self) -> RTEN8_W {
        RTEN8_W::new(self)
    }
    #[doc = "Bit 9 - Rising edge trigger enable of line 9"]
    #[inline(always)]
    pub fn rten9(&mut self) -> RTEN9_W {
        RTEN9_W::new(self)
    }
    #[doc = "Bit 10 - Rising edge trigger enable of line 10"]
    #[inline(always)]
    pub fn rten10(&mut self) -> RTEN10_W {
        RTEN10_W::new(self)
    }
    #[doc = "Bit 11 - Rising edge trigger enable of line 11"]
    #[inline(always)]
    pub fn rten11(&mut self) -> RTEN11_W {
        RTEN11_W::new(self)
    }
    #[doc = "Bit 12 - Rising edge trigger enable of line 12"]
    #[inline(always)]
    pub fn rten12(&mut self) -> RTEN12_W {
        RTEN12_W::new(self)
    }
    #[doc = "Bit 13 - Rising edge trigger enable of line 13"]
    #[inline(always)]
    pub fn rten13(&mut self) -> RTEN13_W {
        RTEN13_W::new(self)
    }
    #[doc = "Bit 14 - Rising edge trigger enable of line 14"]
    #[inline(always)]
    pub fn rten14(&mut self) -> RTEN14_W {
        RTEN14_W::new(self)
    }
    #[doc = "Bit 15 - Rising edge trigger enable of line 15"]
    #[inline(always)]
    pub fn rten15(&mut self) -> RTEN15_W {
        RTEN15_W::new(self)
    }
    #[doc = "Bit 16 - Rising edge trigger enable of line 16"]
    #[inline(always)]
    pub fn rten16(&mut self) -> RTEN16_W {
        RTEN16_W::new(self)
    }
    #[doc = "Bit 17 - Rising edge trigger enable of line 17"]
    #[inline(always)]
    pub fn rten17(&mut self) -> RTEN17_W {
        RTEN17_W::new(self)
    }
    #[doc = "Bit 18 - Rising edge trigger enable of line 18"]
    #[inline(always)]
    pub fn rten18(&mut self) -> RTEN18_W {
        RTEN18_W::new(self)
    }
    #[doc = "Bit 19 - Rising edge trigger enable of line 19"]
    #[inline(always)]
    pub fn rten19(&mut self) -> RTEN19_W {
        RTEN19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rten](index.html) module"]
pub struct RTEN_SPEC;
impl crate::RegisterSpec for RTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rten::R](R) reader structure"]
impl crate::Readable for RTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rten::W](W) writer structure"]
impl crate::Writable for RTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTEN to value 0"]
impl crate::Resettable for RTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
