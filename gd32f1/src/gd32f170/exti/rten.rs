#[doc = "Register `RTEN` reader"]
pub type R = crate::R<RTEN_SPEC>;
#[doc = "Register `RTEN` writer"]
pub type W = crate::W<RTEN_SPEC>;
#[doc = "Field `RTEN0` reader - Rising edge trigger enable of line 0"]
pub type RTEN0_R = crate::BitReader<RTEN0_A>;
#[doc = "Rising edge trigger enable of line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTEN0_A {
        match self.bits {
            false => RTEN0_A::DISABLED,
            true => RTEN0_A::ENABLED,
        }
    }
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTEN0_A::DISABLED
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTEN0_A::ENABLED
    }
}
#[doc = "Field `RTEN0` writer - Rising edge trigger enable of line 0"]
pub type RTEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTEN0_A>;
impl<'a, REG, const O: u8> RTEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTEN0_A::DISABLED)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTEN0_A::ENABLED)
    }
}
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
#[doc = "Field `RTEN21` reader - Rising edge trigger enable of line 21"]
pub use RTEN0_R as RTEN21_R;
#[doc = "Field `RTEN22` reader - Rising edge trigger enable of line 22"]
pub use RTEN0_R as RTEN22_R;
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
#[doc = "Field `RTEN21` writer - Rising edge trigger enable of line 21"]
pub use RTEN0_W as RTEN21_W;
#[doc = "Field `RTEN22` writer - Rising edge trigger enable of line 22"]
pub use RTEN0_W as RTEN22_W;
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
    #[doc = "Bit 21 - Rising edge trigger enable of line 21"]
    #[inline(always)]
    pub fn rten21(&self) -> RTEN21_R {
        RTEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rising edge trigger enable of line 22"]
    #[inline(always)]
    pub fn rten22(&self) -> RTEN22_R {
        RTEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge trigger enable of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn rten0(&mut self) -> RTEN0_W<RTEN_SPEC, 0> {
        RTEN0_W::new(self)
    }
    #[doc = "Bit 1 - Rising edge trigger enable of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn rten1(&mut self) -> RTEN1_W<RTEN_SPEC, 1> {
        RTEN1_W::new(self)
    }
    #[doc = "Bit 2 - Rising edge trigger enable of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn rten2(&mut self) -> RTEN2_W<RTEN_SPEC, 2> {
        RTEN2_W::new(self)
    }
    #[doc = "Bit 3 - Rising edge trigger enable of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn rten3(&mut self) -> RTEN3_W<RTEN_SPEC, 3> {
        RTEN3_W::new(self)
    }
    #[doc = "Bit 4 - Rising edge trigger enable of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn rten4(&mut self) -> RTEN4_W<RTEN_SPEC, 4> {
        RTEN4_W::new(self)
    }
    #[doc = "Bit 5 - Rising edge trigger enable of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn rten5(&mut self) -> RTEN5_W<RTEN_SPEC, 5> {
        RTEN5_W::new(self)
    }
    #[doc = "Bit 6 - Rising edge trigger enable of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn rten6(&mut self) -> RTEN6_W<RTEN_SPEC, 6> {
        RTEN6_W::new(self)
    }
    #[doc = "Bit 7 - Rising edge trigger enable of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn rten7(&mut self) -> RTEN7_W<RTEN_SPEC, 7> {
        RTEN7_W::new(self)
    }
    #[doc = "Bit 8 - Rising edge trigger enable of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn rten8(&mut self) -> RTEN8_W<RTEN_SPEC, 8> {
        RTEN8_W::new(self)
    }
    #[doc = "Bit 9 - Rising edge trigger enable of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn rten9(&mut self) -> RTEN9_W<RTEN_SPEC, 9> {
        RTEN9_W::new(self)
    }
    #[doc = "Bit 10 - Rising edge trigger enable of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn rten10(&mut self) -> RTEN10_W<RTEN_SPEC, 10> {
        RTEN10_W::new(self)
    }
    #[doc = "Bit 11 - Rising edge trigger enable of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn rten11(&mut self) -> RTEN11_W<RTEN_SPEC, 11> {
        RTEN11_W::new(self)
    }
    #[doc = "Bit 12 - Rising edge trigger enable of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn rten12(&mut self) -> RTEN12_W<RTEN_SPEC, 12> {
        RTEN12_W::new(self)
    }
    #[doc = "Bit 13 - Rising edge trigger enable of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn rten13(&mut self) -> RTEN13_W<RTEN_SPEC, 13> {
        RTEN13_W::new(self)
    }
    #[doc = "Bit 14 - Rising edge trigger enable of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn rten14(&mut self) -> RTEN14_W<RTEN_SPEC, 14> {
        RTEN14_W::new(self)
    }
    #[doc = "Bit 15 - Rising edge trigger enable of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn rten15(&mut self) -> RTEN15_W<RTEN_SPEC, 15> {
        RTEN15_W::new(self)
    }
    #[doc = "Bit 16 - Rising edge trigger enable of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn rten16(&mut self) -> RTEN16_W<RTEN_SPEC, 16> {
        RTEN16_W::new(self)
    }
    #[doc = "Bit 17 - Rising edge trigger enable of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn rten17(&mut self) -> RTEN17_W<RTEN_SPEC, 17> {
        RTEN17_W::new(self)
    }
    #[doc = "Bit 18 - Rising edge trigger enable of line 18"]
    #[inline(always)]
    #[must_use]
    pub fn rten18(&mut self) -> RTEN18_W<RTEN_SPEC, 18> {
        RTEN18_W::new(self)
    }
    #[doc = "Bit 19 - Rising edge trigger enable of line 19"]
    #[inline(always)]
    #[must_use]
    pub fn rten19(&mut self) -> RTEN19_W<RTEN_SPEC, 19> {
        RTEN19_W::new(self)
    }
    #[doc = "Bit 21 - Rising edge trigger enable of line 21"]
    #[inline(always)]
    #[must_use]
    pub fn rten21(&mut self) -> RTEN21_W<RTEN_SPEC, 21> {
        RTEN21_W::new(self)
    }
    #[doc = "Bit 22 - Rising edge trigger enable of line 22"]
    #[inline(always)]
    #[must_use]
    pub fn rten22(&mut self) -> RTEN22_W<RTEN_SPEC, 22> {
        RTEN22_W::new(self)
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
#[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTEN_SPEC;
impl crate::RegisterSpec for RTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rten::R`](R) reader structure"]
impl crate::Readable for RTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rten::W`](W) writer structure"]
impl crate::Writable for RTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTEN to value 0"]
impl crate::Resettable for RTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
