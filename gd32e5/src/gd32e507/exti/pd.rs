#[doc = "Register `PD` reader"]
pub type R = crate::R<PD_SPEC>;
#[doc = "Register `PD` writer"]
pub type W = crate::W<PD_SPEC>;
#[doc = "Field `PD0` reader - Interrupt pending status of line 0"]
pub type PD0_R = crate::BitReader<PD0R_A>;
#[doc = "Interrupt pending status of line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD0R_A {
    #[doc = "0: No trigger request occurred"]
    NOT_PENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PD0R_A> for bool {
    #[inline(always)]
    fn from(variant: PD0R_A) -> Self {
        variant as u8 != 0
    }
}
impl PD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD0R_A {
        match self.bits {
            false => PD0R_A::NOT_PENDING,
            true => PD0R_A::PENDING,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PD0R_A::NOT_PENDING
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD0R_A::PENDING
    }
}
#[doc = "Interrupt pending status of line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD0W_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PD0W_AW> for bool {
    #[inline(always)]
    fn from(variant: PD0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD0` writer - Interrupt pending status of line 0"]
pub type PD0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PD0W_AW>;
impl<'a, REG, const O: u8> PD0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PD0W_AW::CLEAR)
    }
}
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
#[doc = "Field `PD20` reader - Interrupt pending status of line 20"]
pub use PD0_R as PD20_R;
#[doc = "Field `PD21` reader - Interrupt pending status of line 21"]
pub use PD0_R as PD21_R;
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
#[doc = "Field `PD20` writer - Interrupt pending status of line 20"]
pub use PD0_W as PD20_W;
#[doc = "Field `PD21` writer - Interrupt pending status of line 21"]
pub use PD0_W as PD21_W;
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
    #[doc = "Bit 20 - Interrupt pending status of line 20"]
    #[inline(always)]
    pub fn pd20(&self) -> PD20_R {
        PD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt pending status of line 21"]
    #[inline(always)]
    pub fn pd21(&self) -> PD21_R {
        PD21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt pending status of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PD_SPEC, 0> {
        PD0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt pending status of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PD_SPEC, 1> {
        PD1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt pending status of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PD_SPEC, 2> {
        PD2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt pending status of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PD_SPEC, 3> {
        PD3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt pending status of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<PD_SPEC, 4> {
        PD4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt pending status of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<PD_SPEC, 5> {
        PD5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt pending status of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<PD_SPEC, 6> {
        PD6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt pending status of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<PD_SPEC, 7> {
        PD7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt pending status of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<PD_SPEC, 8> {
        PD8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt pending status of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<PD_SPEC, 9> {
        PD9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt pending status of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<PD_SPEC, 10> {
        PD10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt pending status of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> PD11_W<PD_SPEC, 11> {
        PD11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt pending status of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> PD12_W<PD_SPEC, 12> {
        PD12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt pending status of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> PD13_W<PD_SPEC, 13> {
        PD13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt pending status of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<PD_SPEC, 14> {
        PD14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt pending status of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> PD15_W<PD_SPEC, 15> {
        PD15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt pending status of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn pd16(&mut self) -> PD16_W<PD_SPEC, 16> {
        PD16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt pending status of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn pd17(&mut self) -> PD17_W<PD_SPEC, 17> {
        PD17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt pending status of line 18"]
    #[inline(always)]
    #[must_use]
    pub fn pd18(&mut self) -> PD18_W<PD_SPEC, 18> {
        PD18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt pending status of line 19"]
    #[inline(always)]
    #[must_use]
    pub fn pd19(&mut self) -> PD19_W<PD_SPEC, 19> {
        PD19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt pending status of line 20"]
    #[inline(always)]
    #[must_use]
    pub fn pd20(&mut self) -> PD20_W<PD_SPEC, 20> {
        PD20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt pending status of line 21"]
    #[inline(always)]
    #[must_use]
    pub fn pd21(&mut self) -> PD21_W<PD_SPEC, 21> {
        PD21_W::new(self)
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
#[doc = "Pending register (EXTI_PD)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_SPEC;
impl crate::RegisterSpec for PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd::R`](R) reader structure"]
impl crate::Readable for PD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd::W`](W) writer structure"]
impl crate::Writable for PD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD to value 0"]
impl crate::Resettable for PD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
