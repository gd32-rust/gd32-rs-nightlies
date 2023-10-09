#[doc = "Register `FMCFG` reader"]
pub type R = crate::R<FMCFG_SPEC>;
#[doc = "Register `FMCFG` writer"]
pub type W = crate::W<FMCFG_SPEC>;
#[doc = "Field `FMOD0` reader - Filter mode"]
pub type FMOD0_R = crate::BitReader<FMOD0_A>;
#[doc = "Filter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMOD0_A {
    #[doc = "0: Mask mode"]
    MASK = 0,
    #[doc = "1: List mode"]
    LIST = 1,
}
impl From<FMOD0_A> for bool {
    #[inline(always)]
    fn from(variant: FMOD0_A) -> Self {
        variant as u8 != 0
    }
}
impl FMOD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMOD0_A {
        match self.bits {
            false => FMOD0_A::MASK,
            true => FMOD0_A::LIST,
        }
    }
    #[doc = "Mask mode"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == FMOD0_A::MASK
    }
    #[doc = "List mode"]
    #[inline(always)]
    pub fn is_list(&self) -> bool {
        *self == FMOD0_A::LIST
    }
}
#[doc = "Field `FMOD0` writer - Filter mode"]
pub type FMOD0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FMOD0_A>;
impl<'a, REG, const O: u8> FMOD0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask mode"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(FMOD0_A::MASK)
    }
    #[doc = "List mode"]
    #[inline(always)]
    pub fn list(self) -> &'a mut crate::W<REG> {
        self.variant(FMOD0_A::LIST)
    }
}
#[doc = "Field `FMOD1` reader - Filter mode"]
pub use FMOD0_R as FMOD1_R;
#[doc = "Field `FMOD2` reader - Filter mode"]
pub use FMOD0_R as FMOD2_R;
#[doc = "Field `FMOD3` reader - Filter mode"]
pub use FMOD0_R as FMOD3_R;
#[doc = "Field `FMOD4` reader - Filter mode"]
pub use FMOD0_R as FMOD4_R;
#[doc = "Field `FMOD5` reader - Filter mode"]
pub use FMOD0_R as FMOD5_R;
#[doc = "Field `FMOD6` reader - Filter mode"]
pub use FMOD0_R as FMOD6_R;
#[doc = "Field `FMOD7` reader - Filter mode"]
pub use FMOD0_R as FMOD7_R;
#[doc = "Field `FMOD8` reader - Filter mode"]
pub use FMOD0_R as FMOD8_R;
#[doc = "Field `FMOD9` reader - Filter mode"]
pub use FMOD0_R as FMOD9_R;
#[doc = "Field `FMOD10` reader - Filter mode"]
pub use FMOD0_R as FMOD10_R;
#[doc = "Field `FMOD11` reader - Filter mode"]
pub use FMOD0_R as FMOD11_R;
#[doc = "Field `FMOD12` reader - Filter mode"]
pub use FMOD0_R as FMOD12_R;
#[doc = "Field `FMOD13` reader - Filter mode"]
pub use FMOD0_R as FMOD13_R;
#[doc = "Field `FMOD14` reader - Filter mode"]
pub use FMOD0_R as FMOD14_R;
#[doc = "Field `FMOD15` reader - Filter mode"]
pub use FMOD0_R as FMOD15_R;
#[doc = "Field `FMOD16` reader - Filter mode"]
pub use FMOD0_R as FMOD16_R;
#[doc = "Field `FMOD17` reader - Filter mode"]
pub use FMOD0_R as FMOD17_R;
#[doc = "Field `FMOD18` reader - Filter mode"]
pub use FMOD0_R as FMOD18_R;
#[doc = "Field `FMOD19` reader - Filter mode"]
pub use FMOD0_R as FMOD19_R;
#[doc = "Field `FMOD20` reader - Filter mode"]
pub use FMOD0_R as FMOD20_R;
#[doc = "Field `FMOD21` reader - Filter mode"]
pub use FMOD0_R as FMOD21_R;
#[doc = "Field `FMOD22` reader - Filter mode"]
pub use FMOD0_R as FMOD22_R;
#[doc = "Field `FMOD23` reader - Filter mode"]
pub use FMOD0_R as FMOD23_R;
#[doc = "Field `FMOD24` reader - Filter mode"]
pub use FMOD0_R as FMOD24_R;
#[doc = "Field `FMOD25` reader - Filter mode"]
pub use FMOD0_R as FMOD25_R;
#[doc = "Field `FMOD26` reader - Filter mode"]
pub use FMOD0_R as FMOD26_R;
#[doc = "Field `FMOD27` reader - Filter mode"]
pub use FMOD0_R as FMOD27_R;
#[doc = "Field `FMOD1` writer - Filter mode"]
pub use FMOD0_W as FMOD1_W;
#[doc = "Field `FMOD2` writer - Filter mode"]
pub use FMOD0_W as FMOD2_W;
#[doc = "Field `FMOD3` writer - Filter mode"]
pub use FMOD0_W as FMOD3_W;
#[doc = "Field `FMOD4` writer - Filter mode"]
pub use FMOD0_W as FMOD4_W;
#[doc = "Field `FMOD5` writer - Filter mode"]
pub use FMOD0_W as FMOD5_W;
#[doc = "Field `FMOD6` writer - Filter mode"]
pub use FMOD0_W as FMOD6_W;
#[doc = "Field `FMOD7` writer - Filter mode"]
pub use FMOD0_W as FMOD7_W;
#[doc = "Field `FMOD8` writer - Filter mode"]
pub use FMOD0_W as FMOD8_W;
#[doc = "Field `FMOD9` writer - Filter mode"]
pub use FMOD0_W as FMOD9_W;
#[doc = "Field `FMOD10` writer - Filter mode"]
pub use FMOD0_W as FMOD10_W;
#[doc = "Field `FMOD11` writer - Filter mode"]
pub use FMOD0_W as FMOD11_W;
#[doc = "Field `FMOD12` writer - Filter mode"]
pub use FMOD0_W as FMOD12_W;
#[doc = "Field `FMOD13` writer - Filter mode"]
pub use FMOD0_W as FMOD13_W;
#[doc = "Field `FMOD14` writer - Filter mode"]
pub use FMOD0_W as FMOD14_W;
#[doc = "Field `FMOD15` writer - Filter mode"]
pub use FMOD0_W as FMOD15_W;
#[doc = "Field `FMOD16` writer - Filter mode"]
pub use FMOD0_W as FMOD16_W;
#[doc = "Field `FMOD17` writer - Filter mode"]
pub use FMOD0_W as FMOD17_W;
#[doc = "Field `FMOD18` writer - Filter mode"]
pub use FMOD0_W as FMOD18_W;
#[doc = "Field `FMOD19` writer - Filter mode"]
pub use FMOD0_W as FMOD19_W;
#[doc = "Field `FMOD20` writer - Filter mode"]
pub use FMOD0_W as FMOD20_W;
#[doc = "Field `FMOD21` writer - Filter mode"]
pub use FMOD0_W as FMOD21_W;
#[doc = "Field `FMOD22` writer - Filter mode"]
pub use FMOD0_W as FMOD22_W;
#[doc = "Field `FMOD23` writer - Filter mode"]
pub use FMOD0_W as FMOD23_W;
#[doc = "Field `FMOD24` writer - Filter mode"]
pub use FMOD0_W as FMOD24_W;
#[doc = "Field `FMOD25` writer - Filter mode"]
pub use FMOD0_W as FMOD25_W;
#[doc = "Field `FMOD26` writer - Filter mode"]
pub use FMOD0_W as FMOD26_W;
#[doc = "Field `FMOD27` writer - Filter mode"]
pub use FMOD0_W as FMOD27_W;
impl R {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fmod0(&self) -> FMOD0_R {
        FMOD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fmod1(&self) -> FMOD1_R {
        FMOD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fmod2(&self) -> FMOD2_R {
        FMOD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fmod3(&self) -> FMOD3_R {
        FMOD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fmod4(&self) -> FMOD4_R {
        FMOD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fmod5(&self) -> FMOD5_R {
        FMOD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fmod6(&self) -> FMOD6_R {
        FMOD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fmod7(&self) -> FMOD7_R {
        FMOD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fmod8(&self) -> FMOD8_R {
        FMOD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fmod9(&self) -> FMOD9_R {
        FMOD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fmod10(&self) -> FMOD10_R {
        FMOD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fmod11(&self) -> FMOD11_R {
        FMOD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fmod12(&self) -> FMOD12_R {
        FMOD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fmod13(&self) -> FMOD13_R {
        FMOD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter mode"]
    #[inline(always)]
    pub fn fmod14(&self) -> FMOD14_R {
        FMOD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter mode"]
    #[inline(always)]
    pub fn fmod15(&self) -> FMOD15_R {
        FMOD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter mode"]
    #[inline(always)]
    pub fn fmod16(&self) -> FMOD16_R {
        FMOD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter mode"]
    #[inline(always)]
    pub fn fmod17(&self) -> FMOD17_R {
        FMOD17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter mode"]
    #[inline(always)]
    pub fn fmod18(&self) -> FMOD18_R {
        FMOD18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter mode"]
    #[inline(always)]
    pub fn fmod19(&self) -> FMOD19_R {
        FMOD19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter mode"]
    #[inline(always)]
    pub fn fmod20(&self) -> FMOD20_R {
        FMOD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter mode"]
    #[inline(always)]
    pub fn fmod21(&self) -> FMOD21_R {
        FMOD21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter mode"]
    #[inline(always)]
    pub fn fmod22(&self) -> FMOD22_R {
        FMOD22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter mode"]
    #[inline(always)]
    pub fn fmod23(&self) -> FMOD23_R {
        FMOD23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter mode"]
    #[inline(always)]
    pub fn fmod24(&self) -> FMOD24_R {
        FMOD24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter mode"]
    #[inline(always)]
    pub fn fmod25(&self) -> FMOD25_R {
        FMOD25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter mode"]
    #[inline(always)]
    pub fn fmod26(&self) -> FMOD26_R {
        FMOD26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter mode"]
    #[inline(always)]
    pub fn fmod27(&self) -> FMOD27_R {
        FMOD27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod0(&mut self) -> FMOD0_W<FMCFG_SPEC, 0> {
        FMOD0_W::new(self)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod1(&mut self) -> FMOD1_W<FMCFG_SPEC, 1> {
        FMOD1_W::new(self)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod2(&mut self) -> FMOD2_W<FMCFG_SPEC, 2> {
        FMOD2_W::new(self)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod3(&mut self) -> FMOD3_W<FMCFG_SPEC, 3> {
        FMOD3_W::new(self)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod4(&mut self) -> FMOD4_W<FMCFG_SPEC, 4> {
        FMOD4_W::new(self)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod5(&mut self) -> FMOD5_W<FMCFG_SPEC, 5> {
        FMOD5_W::new(self)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod6(&mut self) -> FMOD6_W<FMCFG_SPEC, 6> {
        FMOD6_W::new(self)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod7(&mut self) -> FMOD7_W<FMCFG_SPEC, 7> {
        FMOD7_W::new(self)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod8(&mut self) -> FMOD8_W<FMCFG_SPEC, 8> {
        FMOD8_W::new(self)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod9(&mut self) -> FMOD9_W<FMCFG_SPEC, 9> {
        FMOD9_W::new(self)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod10(&mut self) -> FMOD10_W<FMCFG_SPEC, 10> {
        FMOD10_W::new(self)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod11(&mut self) -> FMOD11_W<FMCFG_SPEC, 11> {
        FMOD11_W::new(self)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod12(&mut self) -> FMOD12_W<FMCFG_SPEC, 12> {
        FMOD12_W::new(self)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod13(&mut self) -> FMOD13_W<FMCFG_SPEC, 13> {
        FMOD13_W::new(self)
    }
    #[doc = "Bit 14 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod14(&mut self) -> FMOD14_W<FMCFG_SPEC, 14> {
        FMOD14_W::new(self)
    }
    #[doc = "Bit 15 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod15(&mut self) -> FMOD15_W<FMCFG_SPEC, 15> {
        FMOD15_W::new(self)
    }
    #[doc = "Bit 16 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod16(&mut self) -> FMOD16_W<FMCFG_SPEC, 16> {
        FMOD16_W::new(self)
    }
    #[doc = "Bit 17 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod17(&mut self) -> FMOD17_W<FMCFG_SPEC, 17> {
        FMOD17_W::new(self)
    }
    #[doc = "Bit 18 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod18(&mut self) -> FMOD18_W<FMCFG_SPEC, 18> {
        FMOD18_W::new(self)
    }
    #[doc = "Bit 19 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod19(&mut self) -> FMOD19_W<FMCFG_SPEC, 19> {
        FMOD19_W::new(self)
    }
    #[doc = "Bit 20 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod20(&mut self) -> FMOD20_W<FMCFG_SPEC, 20> {
        FMOD20_W::new(self)
    }
    #[doc = "Bit 21 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod21(&mut self) -> FMOD21_W<FMCFG_SPEC, 21> {
        FMOD21_W::new(self)
    }
    #[doc = "Bit 22 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod22(&mut self) -> FMOD22_W<FMCFG_SPEC, 22> {
        FMOD22_W::new(self)
    }
    #[doc = "Bit 23 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod23(&mut self) -> FMOD23_W<FMCFG_SPEC, 23> {
        FMOD23_W::new(self)
    }
    #[doc = "Bit 24 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod24(&mut self) -> FMOD24_W<FMCFG_SPEC, 24> {
        FMOD24_W::new(self)
    }
    #[doc = "Bit 25 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod25(&mut self) -> FMOD25_W<FMCFG_SPEC, 25> {
        FMOD25_W::new(self)
    }
    #[doc = "Bit 26 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod26(&mut self) -> FMOD26_W<FMCFG_SPEC, 26> {
        FMOD26_W::new(self)
    }
    #[doc = "Bit 27 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod27(&mut self) -> FMOD27_W<FMCFG_SPEC, 27> {
        FMOD27_W::new(self)
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
#[doc = "Filter mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMCFG_SPEC;
impl crate::RegisterSpec for FMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmcfg::R`](R) reader structure"]
impl crate::Readable for FMCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmcfg::W`](W) writer structure"]
impl crate::Writable for FMCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMCFG to value 0"]
impl crate::Resettable for FMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
