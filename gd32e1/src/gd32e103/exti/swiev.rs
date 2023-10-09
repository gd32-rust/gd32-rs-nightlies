#[doc = "Register `SWIEV` reader"]
pub type R = crate::R<SWIEV_SPEC>;
#[doc = "Register `SWIEV` writer"]
pub type W = crate::W<SWIEV_SPEC>;
#[doc = "Field `SWIEV0` reader - Interrupt/Event software trigger on line 0"]
pub type SWIEV0_R = crate::BitReader<SWIEV0W_A>;
#[doc = "Interrupt/Event software trigger on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWIEV0W_A {
    #[doc = "1: Generates an interrupt request"]
    PEND = 1,
}
impl From<SWIEV0W_A> for bool {
    #[inline(always)]
    fn from(variant: SWIEV0W_A) -> Self {
        variant as u8 != 0
    }
}
impl SWIEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIEV0W_A> {
        match self.bits {
            true => Some(SWIEV0W_A::PEND),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIEV0W_A::PEND
    }
}
#[doc = "Field `SWIEV0` writer - Interrupt/Event software trigger on line 0"]
pub type SWIEV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWIEV0W_A>;
impl<'a, REG, const O: u8> SWIEV0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWIEV0W_A::PEND)
    }
}
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
}
impl W {
    #[doc = "Bit 0 - Interrupt/Event software trigger on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn swiev0(&mut self) -> SWIEV0_W<SWIEV_SPEC, 0> {
        SWIEV0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt/Event software trigger on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn swiev1(&mut self) -> SWIEV1_W<SWIEV_SPEC, 1> {
        SWIEV1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt/Event software trigger on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn swiev2(&mut self) -> SWIEV2_W<SWIEV_SPEC, 2> {
        SWIEV2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt/Event software trigger on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn swiev3(&mut self) -> SWIEV3_W<SWIEV_SPEC, 3> {
        SWIEV3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt/Event software trigger on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn swiev4(&mut self) -> SWIEV4_W<SWIEV_SPEC, 4> {
        SWIEV4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt/Event software trigger on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn swiev5(&mut self) -> SWIEV5_W<SWIEV_SPEC, 5> {
        SWIEV5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt/Event software trigger on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn swiev6(&mut self) -> SWIEV6_W<SWIEV_SPEC, 6> {
        SWIEV6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt/Event software trigger on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn swiev7(&mut self) -> SWIEV7_W<SWIEV_SPEC, 7> {
        SWIEV7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt/Event software trigger on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn swiev8(&mut self) -> SWIEV8_W<SWIEV_SPEC, 8> {
        SWIEV8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt/Event software trigger on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn swiev9(&mut self) -> SWIEV9_W<SWIEV_SPEC, 9> {
        SWIEV9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt/Event software trigger on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn swiev10(&mut self) -> SWIEV10_W<SWIEV_SPEC, 10> {
        SWIEV10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt/Event software trigger on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn swiev11(&mut self) -> SWIEV11_W<SWIEV_SPEC, 11> {
        SWIEV11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt/Event software trigger on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn swiev12(&mut self) -> SWIEV12_W<SWIEV_SPEC, 12> {
        SWIEV12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt/Event software trigger on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn swiev13(&mut self) -> SWIEV13_W<SWIEV_SPEC, 13> {
        SWIEV13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt/Event software trigger on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn swiev14(&mut self) -> SWIEV14_W<SWIEV_SPEC, 14> {
        SWIEV14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt/Event software trigger on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn swiev15(&mut self) -> SWIEV15_W<SWIEV_SPEC, 15> {
        SWIEV15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt/Event software trigger on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn swiev16(&mut self) -> SWIEV16_W<SWIEV_SPEC, 16> {
        SWIEV16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt/Event software trigger on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn swiev17(&mut self) -> SWIEV17_W<SWIEV_SPEC, 17> {
        SWIEV17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt/Event software trigger on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn swiev18(&mut self) -> SWIEV18_W<SWIEV_SPEC, 18> {
        SWIEV18_W::new(self)
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
#[doc = "Software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swiev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swiev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIEV_SPEC;
impl crate::RegisterSpec for SWIEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swiev::R`](R) reader structure"]
impl crate::Readable for SWIEV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swiev::W`](W) writer structure"]
impl crate::Writable for SWIEV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWIEV to value 0"]
impl crate::Resettable for SWIEV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
