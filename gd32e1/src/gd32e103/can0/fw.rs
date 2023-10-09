#[doc = "Register `FW` reader"]
pub type R = crate::R<FW_SPEC>;
#[doc = "Register `FW` writer"]
pub type W = crate::W<FW_SPEC>;
#[doc = "Field `FW0` reader - Filter working"]
pub type FW0_R = crate::BitReader<FW0_A>;
#[doc = "Filter working\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl FW0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FW0_A {
        match self.bits {
            false => FW0_A::DISABLED,
            true => FW0_A::ENABLED,
        }
    }
    #[doc = "Filter working disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FW0_A::DISABLED
    }
    #[doc = "Filter working enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FW0_A::ENABLED
    }
}
#[doc = "Field `FW0` writer - Filter working"]
pub type FW0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FW0_A>;
impl<'a, REG, const O: u8> FW0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter working disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FW0_A::DISABLED)
    }
    #[doc = "Filter working enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FW0_A::ENABLED)
    }
}
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
    #[must_use]
    pub fn fw0(&mut self) -> FW0_W<FW_SPEC, 0> {
        FW0_W::new(self)
    }
    #[doc = "Bit 1 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw1(&mut self) -> FW1_W<FW_SPEC, 1> {
        FW1_W::new(self)
    }
    #[doc = "Bit 2 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw2(&mut self) -> FW2_W<FW_SPEC, 2> {
        FW2_W::new(self)
    }
    #[doc = "Bit 3 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw3(&mut self) -> FW3_W<FW_SPEC, 3> {
        FW3_W::new(self)
    }
    #[doc = "Bit 4 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw4(&mut self) -> FW4_W<FW_SPEC, 4> {
        FW4_W::new(self)
    }
    #[doc = "Bit 5 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw5(&mut self) -> FW5_W<FW_SPEC, 5> {
        FW5_W::new(self)
    }
    #[doc = "Bit 6 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw6(&mut self) -> FW6_W<FW_SPEC, 6> {
        FW6_W::new(self)
    }
    #[doc = "Bit 7 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw7(&mut self) -> FW7_W<FW_SPEC, 7> {
        FW7_W::new(self)
    }
    #[doc = "Bit 8 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw8(&mut self) -> FW8_W<FW_SPEC, 8> {
        FW8_W::new(self)
    }
    #[doc = "Bit 9 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw9(&mut self) -> FW9_W<FW_SPEC, 9> {
        FW9_W::new(self)
    }
    #[doc = "Bit 10 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw10(&mut self) -> FW10_W<FW_SPEC, 10> {
        FW10_W::new(self)
    }
    #[doc = "Bit 11 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw11(&mut self) -> FW11_W<FW_SPEC, 11> {
        FW11_W::new(self)
    }
    #[doc = "Bit 12 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw12(&mut self) -> FW12_W<FW_SPEC, 12> {
        FW12_W::new(self)
    }
    #[doc = "Bit 13 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw13(&mut self) -> FW13_W<FW_SPEC, 13> {
        FW13_W::new(self)
    }
    #[doc = "Bit 14 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw14(&mut self) -> FW14_W<FW_SPEC, 14> {
        FW14_W::new(self)
    }
    #[doc = "Bit 15 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw15(&mut self) -> FW15_W<FW_SPEC, 15> {
        FW15_W::new(self)
    }
    #[doc = "Bit 16 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw16(&mut self) -> FW16_W<FW_SPEC, 16> {
        FW16_W::new(self)
    }
    #[doc = "Bit 17 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw17(&mut self) -> FW17_W<FW_SPEC, 17> {
        FW17_W::new(self)
    }
    #[doc = "Bit 18 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw18(&mut self) -> FW18_W<FW_SPEC, 18> {
        FW18_W::new(self)
    }
    #[doc = "Bit 19 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw19(&mut self) -> FW19_W<FW_SPEC, 19> {
        FW19_W::new(self)
    }
    #[doc = "Bit 20 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw20(&mut self) -> FW20_W<FW_SPEC, 20> {
        FW20_W::new(self)
    }
    #[doc = "Bit 21 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw21(&mut self) -> FW21_W<FW_SPEC, 21> {
        FW21_W::new(self)
    }
    #[doc = "Bit 22 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw22(&mut self) -> FW22_W<FW_SPEC, 22> {
        FW22_W::new(self)
    }
    #[doc = "Bit 23 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw23(&mut self) -> FW23_W<FW_SPEC, 23> {
        FW23_W::new(self)
    }
    #[doc = "Bit 24 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw24(&mut self) -> FW24_W<FW_SPEC, 24> {
        FW24_W::new(self)
    }
    #[doc = "Bit 25 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw25(&mut self) -> FW25_W<FW_SPEC, 25> {
        FW25_W::new(self)
    }
    #[doc = "Bit 26 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw26(&mut self) -> FW26_W<FW_SPEC, 26> {
        FW26_W::new(self)
    }
    #[doc = "Bit 27 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw27(&mut self) -> FW27_W<FW_SPEC, 27> {
        FW27_W::new(self)
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
#[doc = "Filter working register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FW_SPEC;
impl crate::RegisterSpec for FW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw::R`](R) reader structure"]
impl crate::Readable for FW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fw::W`](W) writer structure"]
impl crate::Writable for FW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FW to value 0"]
impl crate::Resettable for FW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
