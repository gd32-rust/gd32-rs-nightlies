#[doc = "Register `FTEN` reader"]
pub type R = crate::R<FTEN_SPEC>;
#[doc = "Register `FTEN` writer"]
pub type W = crate::W<FTEN_SPEC>;
#[doc = "Field `FTEN0` reader - Falling edge trigger enable of line 0"]
pub type FTEN0_R = crate::BitReader<FTEN0_A>;
#[doc = "Falling edge trigger enable of line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTEN0_A {
    #[doc = "0: Falling edge trigger is disabled"]
    DISABLED = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    ENABLED = 1,
}
impl From<FTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: FTEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl FTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTEN0_A {
        match self.bits {
            false => FTEN0_A::DISABLED,
            true => FTEN0_A::ENABLED,
        }
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FTEN0_A::DISABLED
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FTEN0_A::ENABLED
    }
}
#[doc = "Field `FTEN0` writer - Falling edge trigger enable of line 0"]
pub type FTEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FTEN0_A>;
impl<'a, REG, const O: u8> FTEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FTEN0_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FTEN0_A::ENABLED)
    }
}
#[doc = "Field `FTEN1` reader - Falling edge trigger enable of line 1"]
pub use FTEN0_R as FTEN1_R;
#[doc = "Field `FTEN2` reader - Falling edge trigger enable of line 2"]
pub use FTEN0_R as FTEN2_R;
#[doc = "Field `FTEN3` reader - Falling edge trigger enable of line 3"]
pub use FTEN0_R as FTEN3_R;
#[doc = "Field `FTEN4` reader - Falling edge trigger enable of line 4"]
pub use FTEN0_R as FTEN4_R;
#[doc = "Field `FTEN5` reader - Falling edge trigger enable of line 5"]
pub use FTEN0_R as FTEN5_R;
#[doc = "Field `FTEN6` reader - Falling edge trigger enable of line 6"]
pub use FTEN0_R as FTEN6_R;
#[doc = "Field `FTEN7` reader - Falling edge trigger enable of line 7"]
pub use FTEN0_R as FTEN7_R;
#[doc = "Field `FTEN8` reader - Falling edge trigger enable of line 8"]
pub use FTEN0_R as FTEN8_R;
#[doc = "Field `FTEN9` reader - Falling edge trigger enable of line 9"]
pub use FTEN0_R as FTEN9_R;
#[doc = "Field `FTEN10` reader - Falling edge trigger enable of line 10"]
pub use FTEN0_R as FTEN10_R;
#[doc = "Field `FTEN11` reader - Falling edge trigger enable of line 11"]
pub use FTEN0_R as FTEN11_R;
#[doc = "Field `FTEN12` reader - Falling edge trigger enable of line 12"]
pub use FTEN0_R as FTEN12_R;
#[doc = "Field `FTEN13` reader - Falling edge trigger enable of line 13"]
pub use FTEN0_R as FTEN13_R;
#[doc = "Field `FTEN14` reader - Falling edge trigger enable of line 14"]
pub use FTEN0_R as FTEN14_R;
#[doc = "Field `FTEN15` reader - Falling edge trigger enable of line 15"]
pub use FTEN0_R as FTEN15_R;
#[doc = "Field `FTEN16` reader - Falling edge trigger enable of line 16"]
pub use FTEN0_R as FTEN16_R;
#[doc = "Field `FTEN17` reader - Falling edge trigger enable of line 17"]
pub use FTEN0_R as FTEN17_R;
#[doc = "Field `FTEN18` reader - Falling edge trigger enable of line 18"]
pub use FTEN0_R as FTEN18_R;
#[doc = "Field `FTEN19` reader - Falling edge trigger enable of line 19"]
pub use FTEN0_R as FTEN19_R;
#[doc = "Field `FTEN21` reader - Falling edge trigger enable of line 21"]
pub use FTEN0_R as FTEN21_R;
#[doc = "Field `FTEN22` reader - Falling edge trigger enable of line 22"]
pub use FTEN0_R as FTEN22_R;
#[doc = "Field `FTEN1` writer - Falling edge trigger enable of line 1"]
pub use FTEN0_W as FTEN1_W;
#[doc = "Field `FTEN2` writer - Falling edge trigger enable of line 2"]
pub use FTEN0_W as FTEN2_W;
#[doc = "Field `FTEN3` writer - Falling edge trigger enable of line 3"]
pub use FTEN0_W as FTEN3_W;
#[doc = "Field `FTEN4` writer - Falling edge trigger enable of line 4"]
pub use FTEN0_W as FTEN4_W;
#[doc = "Field `FTEN5` writer - Falling edge trigger enable of line 5"]
pub use FTEN0_W as FTEN5_W;
#[doc = "Field `FTEN6` writer - Falling edge trigger enable of line 6"]
pub use FTEN0_W as FTEN6_W;
#[doc = "Field `FTEN7` writer - Falling edge trigger enable of line 7"]
pub use FTEN0_W as FTEN7_W;
#[doc = "Field `FTEN8` writer - Falling edge trigger enable of line 8"]
pub use FTEN0_W as FTEN8_W;
#[doc = "Field `FTEN9` writer - Falling edge trigger enable of line 9"]
pub use FTEN0_W as FTEN9_W;
#[doc = "Field `FTEN10` writer - Falling edge trigger enable of line 10"]
pub use FTEN0_W as FTEN10_W;
#[doc = "Field `FTEN11` writer - Falling edge trigger enable of line 11"]
pub use FTEN0_W as FTEN11_W;
#[doc = "Field `FTEN12` writer - Falling edge trigger enable of line 12"]
pub use FTEN0_W as FTEN12_W;
#[doc = "Field `FTEN13` writer - Falling edge trigger enable of line 13"]
pub use FTEN0_W as FTEN13_W;
#[doc = "Field `FTEN14` writer - Falling edge trigger enable of line 14"]
pub use FTEN0_W as FTEN14_W;
#[doc = "Field `FTEN15` writer - Falling edge trigger enable of line 15"]
pub use FTEN0_W as FTEN15_W;
#[doc = "Field `FTEN16` writer - Falling edge trigger enable of line 16"]
pub use FTEN0_W as FTEN16_W;
#[doc = "Field `FTEN17` writer - Falling edge trigger enable of line 17"]
pub use FTEN0_W as FTEN17_W;
#[doc = "Field `FTEN18` writer - Falling edge trigger enable of line 18"]
pub use FTEN0_W as FTEN18_W;
#[doc = "Field `FTEN19` writer - Falling edge trigger enable of line 19"]
pub use FTEN0_W as FTEN19_W;
#[doc = "Field `FTEN21` writer - Falling edge trigger enable of line 21"]
pub use FTEN0_W as FTEN21_W;
#[doc = "Field `FTEN22` writer - Falling edge trigger enable of line 22"]
pub use FTEN0_W as FTEN22_W;
impl R {
    #[doc = "Bit 0 - Falling edge trigger enable of line 0"]
    #[inline(always)]
    pub fn ften0(&self) -> FTEN0_R {
        FTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge trigger enable of line 1"]
    #[inline(always)]
    pub fn ften1(&self) -> FTEN1_R {
        FTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge trigger enable of line 2"]
    #[inline(always)]
    pub fn ften2(&self) -> FTEN2_R {
        FTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling edge trigger enable of line 3"]
    #[inline(always)]
    pub fn ften3(&self) -> FTEN3_R {
        FTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling edge trigger enable of line 4"]
    #[inline(always)]
    pub fn ften4(&self) -> FTEN4_R {
        FTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling edge trigger enable of line 5"]
    #[inline(always)]
    pub fn ften5(&self) -> FTEN5_R {
        FTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling edge trigger enable of line 6"]
    #[inline(always)]
    pub fn ften6(&self) -> FTEN6_R {
        FTEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling edge trigger enable of line 7"]
    #[inline(always)]
    pub fn ften7(&self) -> FTEN7_R {
        FTEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling edge trigger enable of line 8"]
    #[inline(always)]
    pub fn ften8(&self) -> FTEN8_R {
        FTEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling edge trigger enable of line 9"]
    #[inline(always)]
    pub fn ften9(&self) -> FTEN9_R {
        FTEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling edge trigger enable of line 10"]
    #[inline(always)]
    pub fn ften10(&self) -> FTEN10_R {
        FTEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling edge trigger enable of line 11"]
    #[inline(always)]
    pub fn ften11(&self) -> FTEN11_R {
        FTEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling edge trigger enable of line 12"]
    #[inline(always)]
    pub fn ften12(&self) -> FTEN12_R {
        FTEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling edge trigger enable of line 13"]
    #[inline(always)]
    pub fn ften13(&self) -> FTEN13_R {
        FTEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling edge trigger enable of line 14"]
    #[inline(always)]
    pub fn ften14(&self) -> FTEN14_R {
        FTEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling edge trigger enable of line 15"]
    #[inline(always)]
    pub fn ften15(&self) -> FTEN15_R {
        FTEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling edge trigger enable of line 16"]
    #[inline(always)]
    pub fn ften16(&self) -> FTEN16_R {
        FTEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling edge trigger enable of line 17"]
    #[inline(always)]
    pub fn ften17(&self) -> FTEN17_R {
        FTEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling edge trigger enable of line 18"]
    #[inline(always)]
    pub fn ften18(&self) -> FTEN18_R {
        FTEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Falling edge trigger enable of line 19"]
    #[inline(always)]
    pub fn ften19(&self) -> FTEN19_R {
        FTEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling edge trigger enable of line 21"]
    #[inline(always)]
    pub fn ften21(&self) -> FTEN21_R {
        FTEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Falling edge trigger enable of line 22"]
    #[inline(always)]
    pub fn ften22(&self) -> FTEN22_R {
        FTEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling edge trigger enable of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn ften0(&mut self) -> FTEN0_W<FTEN_SPEC, 0> {
        FTEN0_W::new(self)
    }
    #[doc = "Bit 1 - Falling edge trigger enable of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn ften1(&mut self) -> FTEN1_W<FTEN_SPEC, 1> {
        FTEN1_W::new(self)
    }
    #[doc = "Bit 2 - Falling edge trigger enable of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn ften2(&mut self) -> FTEN2_W<FTEN_SPEC, 2> {
        FTEN2_W::new(self)
    }
    #[doc = "Bit 3 - Falling edge trigger enable of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn ften3(&mut self) -> FTEN3_W<FTEN_SPEC, 3> {
        FTEN3_W::new(self)
    }
    #[doc = "Bit 4 - Falling edge trigger enable of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn ften4(&mut self) -> FTEN4_W<FTEN_SPEC, 4> {
        FTEN4_W::new(self)
    }
    #[doc = "Bit 5 - Falling edge trigger enable of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn ften5(&mut self) -> FTEN5_W<FTEN_SPEC, 5> {
        FTEN5_W::new(self)
    }
    #[doc = "Bit 6 - Falling edge trigger enable of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn ften6(&mut self) -> FTEN6_W<FTEN_SPEC, 6> {
        FTEN6_W::new(self)
    }
    #[doc = "Bit 7 - Falling edge trigger enable of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn ften7(&mut self) -> FTEN7_W<FTEN_SPEC, 7> {
        FTEN7_W::new(self)
    }
    #[doc = "Bit 8 - Falling edge trigger enable of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn ften8(&mut self) -> FTEN8_W<FTEN_SPEC, 8> {
        FTEN8_W::new(self)
    }
    #[doc = "Bit 9 - Falling edge trigger enable of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn ften9(&mut self) -> FTEN9_W<FTEN_SPEC, 9> {
        FTEN9_W::new(self)
    }
    #[doc = "Bit 10 - Falling edge trigger enable of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn ften10(&mut self) -> FTEN10_W<FTEN_SPEC, 10> {
        FTEN10_W::new(self)
    }
    #[doc = "Bit 11 - Falling edge trigger enable of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn ften11(&mut self) -> FTEN11_W<FTEN_SPEC, 11> {
        FTEN11_W::new(self)
    }
    #[doc = "Bit 12 - Falling edge trigger enable of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn ften12(&mut self) -> FTEN12_W<FTEN_SPEC, 12> {
        FTEN12_W::new(self)
    }
    #[doc = "Bit 13 - Falling edge trigger enable of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn ften13(&mut self) -> FTEN13_W<FTEN_SPEC, 13> {
        FTEN13_W::new(self)
    }
    #[doc = "Bit 14 - Falling edge trigger enable of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn ften14(&mut self) -> FTEN14_W<FTEN_SPEC, 14> {
        FTEN14_W::new(self)
    }
    #[doc = "Bit 15 - Falling edge trigger enable of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn ften15(&mut self) -> FTEN15_W<FTEN_SPEC, 15> {
        FTEN15_W::new(self)
    }
    #[doc = "Bit 16 - Falling edge trigger enable of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn ften16(&mut self) -> FTEN16_W<FTEN_SPEC, 16> {
        FTEN16_W::new(self)
    }
    #[doc = "Bit 17 - Falling edge trigger enable of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn ften17(&mut self) -> FTEN17_W<FTEN_SPEC, 17> {
        FTEN17_W::new(self)
    }
    #[doc = "Bit 18 - Falling edge trigger enable of line 18"]
    #[inline(always)]
    #[must_use]
    pub fn ften18(&mut self) -> FTEN18_W<FTEN_SPEC, 18> {
        FTEN18_W::new(self)
    }
    #[doc = "Bit 19 - Falling edge trigger enable of line 19"]
    #[inline(always)]
    #[must_use]
    pub fn ften19(&mut self) -> FTEN19_W<FTEN_SPEC, 19> {
        FTEN19_W::new(self)
    }
    #[doc = "Bit 21 - Falling edge trigger enable of line 21"]
    #[inline(always)]
    #[must_use]
    pub fn ften21(&mut self) -> FTEN21_W<FTEN_SPEC, 21> {
        FTEN21_W::new(self)
    }
    #[doc = "Bit 22 - Falling edge trigger enable of line 22"]
    #[inline(always)]
    #[must_use]
    pub fn ften22(&mut self) -> FTEN22_W<FTEN_SPEC, 22> {
        FTEN22_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ften::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ften::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTEN_SPEC;
impl crate::RegisterSpec for FTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ften::R`](R) reader structure"]
impl crate::Readable for FTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ften::W`](W) writer structure"]
impl crate::Writable for FTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTEN to value 0"]
impl crate::Resettable for FTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
