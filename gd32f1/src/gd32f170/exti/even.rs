#[doc = "Register `EVEN` reader"]
pub type R = crate::R<EVEN_SPEC>;
#[doc = "Register `EVEN` writer"]
pub type W = crate::W<EVEN_SPEC>;
#[doc = "Field `EVEN0` reader - Event enable on line 0"]
pub type EVEN0_R = crate::BitReader<EVEN0_A>;
#[doc = "Event enable on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EVEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVEN0_A {
        match self.bits {
            false => EVEN0_A::MASKED,
            true => EVEN0_A::UNMASKED,
        }
    }
    #[doc = "Event from line is disabled"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EVEN0_A::MASKED
    }
    #[doc = "Event from line is enabled"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EVEN0_A::UNMASKED
    }
}
#[doc = "Field `EVEN0` writer - Event enable on line 0"]
pub type EVEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EVEN0_A>;
impl<'a, REG, const O: u8> EVEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event from line is disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EVEN0_A::MASKED)
    }
    #[doc = "Event from line is enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(EVEN0_A::UNMASKED)
    }
}
#[doc = "Field `EVEN1` reader - Event enable on line 1"]
pub use EVEN0_R as EVEN1_R;
#[doc = "Field `EVEN2` reader - Event enable on line 2"]
pub use EVEN0_R as EVEN2_R;
#[doc = "Field `EVEN3` reader - Event enable on line 3"]
pub use EVEN0_R as EVEN3_R;
#[doc = "Field `EVEN4` reader - Event enable on line 4"]
pub use EVEN0_R as EVEN4_R;
#[doc = "Field `EVEN5` reader - Event enable on line 5"]
pub use EVEN0_R as EVEN5_R;
#[doc = "Field `EVEN6` reader - Event enable on line 6"]
pub use EVEN0_R as EVEN6_R;
#[doc = "Field `EVEN7` reader - Event enable on line 7"]
pub use EVEN0_R as EVEN7_R;
#[doc = "Field `EVEN8` reader - Event enable on line 8"]
pub use EVEN0_R as EVEN8_R;
#[doc = "Field `EVEN9` reader - Event enable on line 9"]
pub use EVEN0_R as EVEN9_R;
#[doc = "Field `EVEN10` reader - Event enable on line 10"]
pub use EVEN0_R as EVEN10_R;
#[doc = "Field `EVEN11` reader - Event enable on line 11"]
pub use EVEN0_R as EVEN11_R;
#[doc = "Field `EVEN12` reader - Event enable on line 12"]
pub use EVEN0_R as EVEN12_R;
#[doc = "Field `EVEN13` reader - Event enable on line 13"]
pub use EVEN0_R as EVEN13_R;
#[doc = "Field `EVEN14` reader - Event enable on line 14"]
pub use EVEN0_R as EVEN14_R;
#[doc = "Field `EVEN15` reader - Event enable on line 15"]
pub use EVEN0_R as EVEN15_R;
#[doc = "Field `EVEN16` reader - Event enable on line 16"]
pub use EVEN0_R as EVEN16_R;
#[doc = "Field `EVEN17` reader - Event enable on line 17"]
pub use EVEN0_R as EVEN17_R;
#[doc = "Field `EVEN18` reader - Event enable on line 18"]
pub use EVEN0_R as EVEN18_R;
#[doc = "Field `EVEN19` reader - Event enable on line 19"]
pub use EVEN0_R as EVEN19_R;
#[doc = "Field `EVEN20` reader - Event enable on line 20"]
pub use EVEN0_R as EVEN20_R;
#[doc = "Field `EVEN21` reader - Event enable on line 21"]
pub use EVEN0_R as EVEN21_R;
#[doc = "Field `EVEN22` reader - Event enable on line 22"]
pub use EVEN0_R as EVEN22_R;
#[doc = "Field `EVEN23` reader - Event enable on line 23"]
pub use EVEN0_R as EVEN23_R;
#[doc = "Field `EVEN24` reader - Event enable on line 24"]
pub use EVEN0_R as EVEN24_R;
#[doc = "Field `EVEN25` reader - Event enable on line 25"]
pub use EVEN0_R as EVEN25_R;
#[doc = "Field `EVEN26` reader - Event enable on line 26"]
pub use EVEN0_R as EVEN26_R;
#[doc = "Field `EVEN27` reader - Event enable on line 27"]
pub use EVEN0_R as EVEN27_R;
#[doc = "Field `EVEN1` writer - Event enable on line 1"]
pub use EVEN0_W as EVEN1_W;
#[doc = "Field `EVEN2` writer - Event enable on line 2"]
pub use EVEN0_W as EVEN2_W;
#[doc = "Field `EVEN3` writer - Event enable on line 3"]
pub use EVEN0_W as EVEN3_W;
#[doc = "Field `EVEN4` writer - Event enable on line 4"]
pub use EVEN0_W as EVEN4_W;
#[doc = "Field `EVEN5` writer - Event enable on line 5"]
pub use EVEN0_W as EVEN5_W;
#[doc = "Field `EVEN6` writer - Event enable on line 6"]
pub use EVEN0_W as EVEN6_W;
#[doc = "Field `EVEN7` writer - Event enable on line 7"]
pub use EVEN0_W as EVEN7_W;
#[doc = "Field `EVEN8` writer - Event enable on line 8"]
pub use EVEN0_W as EVEN8_W;
#[doc = "Field `EVEN9` writer - Event enable on line 9"]
pub use EVEN0_W as EVEN9_W;
#[doc = "Field `EVEN10` writer - Event enable on line 10"]
pub use EVEN0_W as EVEN10_W;
#[doc = "Field `EVEN11` writer - Event enable on line 11"]
pub use EVEN0_W as EVEN11_W;
#[doc = "Field `EVEN12` writer - Event enable on line 12"]
pub use EVEN0_W as EVEN12_W;
#[doc = "Field `EVEN13` writer - Event enable on line 13"]
pub use EVEN0_W as EVEN13_W;
#[doc = "Field `EVEN14` writer - Event enable on line 14"]
pub use EVEN0_W as EVEN14_W;
#[doc = "Field `EVEN15` writer - Event enable on line 15"]
pub use EVEN0_W as EVEN15_W;
#[doc = "Field `EVEN16` writer - Event enable on line 16"]
pub use EVEN0_W as EVEN16_W;
#[doc = "Field `EVEN17` writer - Event enable on line 17"]
pub use EVEN0_W as EVEN17_W;
#[doc = "Field `EVEN18` writer - Event enable on line 18"]
pub use EVEN0_W as EVEN18_W;
#[doc = "Field `EVEN19` writer - Event enable on line 19"]
pub use EVEN0_W as EVEN19_W;
#[doc = "Field `EVEN20` writer - Event enable on line 20"]
pub use EVEN0_W as EVEN20_W;
#[doc = "Field `EVEN21` writer - Event enable on line 21"]
pub use EVEN0_W as EVEN21_W;
#[doc = "Field `EVEN22` writer - Event enable on line 22"]
pub use EVEN0_W as EVEN22_W;
#[doc = "Field `EVEN23` writer - Event enable on line 23"]
pub use EVEN0_W as EVEN23_W;
#[doc = "Field `EVEN24` writer - Event enable on line 24"]
pub use EVEN0_W as EVEN24_W;
#[doc = "Field `EVEN25` writer - Event enable on line 25"]
pub use EVEN0_W as EVEN25_W;
#[doc = "Field `EVEN26` writer - Event enable on line 26"]
pub use EVEN0_W as EVEN26_W;
#[doc = "Field `EVEN27` writer - Event enable on line 27"]
pub use EVEN0_W as EVEN27_W;
impl R {
    #[doc = "Bit 0 - Event enable on line 0"]
    #[inline(always)]
    pub fn even0(&self) -> EVEN0_R {
        EVEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event enable on line 1"]
    #[inline(always)]
    pub fn even1(&self) -> EVEN1_R {
        EVEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event enable on line 2"]
    #[inline(always)]
    pub fn even2(&self) -> EVEN2_R {
        EVEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event enable on line 3"]
    #[inline(always)]
    pub fn even3(&self) -> EVEN3_R {
        EVEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event enable on line 4"]
    #[inline(always)]
    pub fn even4(&self) -> EVEN4_R {
        EVEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event enable on line 5"]
    #[inline(always)]
    pub fn even5(&self) -> EVEN5_R {
        EVEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event enable on line 6"]
    #[inline(always)]
    pub fn even6(&self) -> EVEN6_R {
        EVEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event enable on line 7"]
    #[inline(always)]
    pub fn even7(&self) -> EVEN7_R {
        EVEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event enable on line 8"]
    #[inline(always)]
    pub fn even8(&self) -> EVEN8_R {
        EVEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event enable on line 9"]
    #[inline(always)]
    pub fn even9(&self) -> EVEN9_R {
        EVEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event enable on line 10"]
    #[inline(always)]
    pub fn even10(&self) -> EVEN10_R {
        EVEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event enable on line 11"]
    #[inline(always)]
    pub fn even11(&self) -> EVEN11_R {
        EVEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event enable on line 12"]
    #[inline(always)]
    pub fn even12(&self) -> EVEN12_R {
        EVEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event enable on line 13"]
    #[inline(always)]
    pub fn even13(&self) -> EVEN13_R {
        EVEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event enable on line 14"]
    #[inline(always)]
    pub fn even14(&self) -> EVEN14_R {
        EVEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event enable on line 15"]
    #[inline(always)]
    pub fn even15(&self) -> EVEN15_R {
        EVEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event enable on line 16"]
    #[inline(always)]
    pub fn even16(&self) -> EVEN16_R {
        EVEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event enable on line 17"]
    #[inline(always)]
    pub fn even17(&self) -> EVEN17_R {
        EVEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event enable on line 18"]
    #[inline(always)]
    pub fn even18(&self) -> EVEN18_R {
        EVEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Event enable on line 19"]
    #[inline(always)]
    pub fn even19(&self) -> EVEN19_R {
        EVEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Event enable on line 20"]
    #[inline(always)]
    pub fn even20(&self) -> EVEN20_R {
        EVEN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Event enable on line 21"]
    #[inline(always)]
    pub fn even21(&self) -> EVEN21_R {
        EVEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Event enable on line 22"]
    #[inline(always)]
    pub fn even22(&self) -> EVEN22_R {
        EVEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Event enable on line 23"]
    #[inline(always)]
    pub fn even23(&self) -> EVEN23_R {
        EVEN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Event enable on line 24"]
    #[inline(always)]
    pub fn even24(&self) -> EVEN24_R {
        EVEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Event enable on line 25"]
    #[inline(always)]
    pub fn even25(&self) -> EVEN25_R {
        EVEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Event enable on line 26"]
    #[inline(always)]
    pub fn even26(&self) -> EVEN26_R {
        EVEN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Event enable on line 27"]
    #[inline(always)]
    pub fn even27(&self) -> EVEN27_R {
        EVEN27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event enable on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn even0(&mut self) -> EVEN0_W<EVEN_SPEC, 0> {
        EVEN0_W::new(self)
    }
    #[doc = "Bit 1 - Event enable on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn even1(&mut self) -> EVEN1_W<EVEN_SPEC, 1> {
        EVEN1_W::new(self)
    }
    #[doc = "Bit 2 - Event enable on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn even2(&mut self) -> EVEN2_W<EVEN_SPEC, 2> {
        EVEN2_W::new(self)
    }
    #[doc = "Bit 3 - Event enable on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn even3(&mut self) -> EVEN3_W<EVEN_SPEC, 3> {
        EVEN3_W::new(self)
    }
    #[doc = "Bit 4 - Event enable on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn even4(&mut self) -> EVEN4_W<EVEN_SPEC, 4> {
        EVEN4_W::new(self)
    }
    #[doc = "Bit 5 - Event enable on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn even5(&mut self) -> EVEN5_W<EVEN_SPEC, 5> {
        EVEN5_W::new(self)
    }
    #[doc = "Bit 6 - Event enable on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn even6(&mut self) -> EVEN6_W<EVEN_SPEC, 6> {
        EVEN6_W::new(self)
    }
    #[doc = "Bit 7 - Event enable on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn even7(&mut self) -> EVEN7_W<EVEN_SPEC, 7> {
        EVEN7_W::new(self)
    }
    #[doc = "Bit 8 - Event enable on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn even8(&mut self) -> EVEN8_W<EVEN_SPEC, 8> {
        EVEN8_W::new(self)
    }
    #[doc = "Bit 9 - Event enable on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn even9(&mut self) -> EVEN9_W<EVEN_SPEC, 9> {
        EVEN9_W::new(self)
    }
    #[doc = "Bit 10 - Event enable on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn even10(&mut self) -> EVEN10_W<EVEN_SPEC, 10> {
        EVEN10_W::new(self)
    }
    #[doc = "Bit 11 - Event enable on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn even11(&mut self) -> EVEN11_W<EVEN_SPEC, 11> {
        EVEN11_W::new(self)
    }
    #[doc = "Bit 12 - Event enable on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn even12(&mut self) -> EVEN12_W<EVEN_SPEC, 12> {
        EVEN12_W::new(self)
    }
    #[doc = "Bit 13 - Event enable on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn even13(&mut self) -> EVEN13_W<EVEN_SPEC, 13> {
        EVEN13_W::new(self)
    }
    #[doc = "Bit 14 - Event enable on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn even14(&mut self) -> EVEN14_W<EVEN_SPEC, 14> {
        EVEN14_W::new(self)
    }
    #[doc = "Bit 15 - Event enable on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn even15(&mut self) -> EVEN15_W<EVEN_SPEC, 15> {
        EVEN15_W::new(self)
    }
    #[doc = "Bit 16 - Event enable on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn even16(&mut self) -> EVEN16_W<EVEN_SPEC, 16> {
        EVEN16_W::new(self)
    }
    #[doc = "Bit 17 - Event enable on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn even17(&mut self) -> EVEN17_W<EVEN_SPEC, 17> {
        EVEN17_W::new(self)
    }
    #[doc = "Bit 18 - Event enable on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn even18(&mut self) -> EVEN18_W<EVEN_SPEC, 18> {
        EVEN18_W::new(self)
    }
    #[doc = "Bit 19 - Event enable on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn even19(&mut self) -> EVEN19_W<EVEN_SPEC, 19> {
        EVEN19_W::new(self)
    }
    #[doc = "Bit 20 - Event enable on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn even20(&mut self) -> EVEN20_W<EVEN_SPEC, 20> {
        EVEN20_W::new(self)
    }
    #[doc = "Bit 21 - Event enable on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn even21(&mut self) -> EVEN21_W<EVEN_SPEC, 21> {
        EVEN21_W::new(self)
    }
    #[doc = "Bit 22 - Event enable on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn even22(&mut self) -> EVEN22_W<EVEN_SPEC, 22> {
        EVEN22_W::new(self)
    }
    #[doc = "Bit 23 - Event enable on line 23"]
    #[inline(always)]
    #[must_use]
    pub fn even23(&mut self) -> EVEN23_W<EVEN_SPEC, 23> {
        EVEN23_W::new(self)
    }
    #[doc = "Bit 24 - Event enable on line 24"]
    #[inline(always)]
    #[must_use]
    pub fn even24(&mut self) -> EVEN24_W<EVEN_SPEC, 24> {
        EVEN24_W::new(self)
    }
    #[doc = "Bit 25 - Event enable on line 25"]
    #[inline(always)]
    #[must_use]
    pub fn even25(&mut self) -> EVEN25_W<EVEN_SPEC, 25> {
        EVEN25_W::new(self)
    }
    #[doc = "Bit 26 - Event enable on line 26"]
    #[inline(always)]
    #[must_use]
    pub fn even26(&mut self) -> EVEN26_W<EVEN_SPEC, 26> {
        EVEN26_W::new(self)
    }
    #[doc = "Bit 27 - Event enable on line 27"]
    #[inline(always)]
    #[must_use]
    pub fn even27(&mut self) -> EVEN27_W<EVEN_SPEC, 27> {
        EVEN27_W::new(self)
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
#[doc = "Event enable register (EXTI_EVEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`even::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`even::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVEN_SPEC;
impl crate::RegisterSpec for EVEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`even::R`](R) reader structure"]
impl crate::Readable for EVEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`even::W`](W) writer structure"]
impl crate::Writable for EVEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVEN to value 0"]
impl crate::Resettable for EVEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
