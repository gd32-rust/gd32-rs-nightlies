#[doc = "Register `OMODE` reader"]
pub type R = crate::R<OMODE_SPEC>;
#[doc = "Register `OMODE` writer"]
pub type W = crate::W<OMODE_SPEC>;
#[doc = "Field `OM0` reader - Pin 0 output mode"]
pub type OM0_R = crate::BitReader<OM0_A>;
#[doc = "Pin 0 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OM0_A {
    #[doc = "0: Output push-pull (reset state)"]
    PUSH_PULL = 0,
    #[doc = "1: Output open-drain"]
    OPEN_DRAIN = 1,
}
impl From<OM0_A> for bool {
    #[inline(always)]
    fn from(variant: OM0_A) -> Self {
        variant as u8 != 0
    }
}
impl OM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OM0_A {
        match self.bits {
            false => OM0_A::PUSH_PULL,
            true => OM0_A::OPEN_DRAIN,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OM0_A::PUSH_PULL
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OM0_A::OPEN_DRAIN
    }
}
#[doc = "Field `OM0` writer - Pin 0 output mode"]
pub type OM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OM0_A>;
impl<'a, REG, const O: u8> OM0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(OM0_A::PUSH_PULL)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(OM0_A::OPEN_DRAIN)
    }
}
#[doc = "Field `OM1` reader - Pin 1 output mode"]
pub use OM0_R as OM1_R;
#[doc = "Field `OM2` reader - Pin 2 output mode"]
pub use OM0_R as OM2_R;
#[doc = "Field `OM3` reader - Pin 3 output mode"]
pub use OM0_R as OM3_R;
#[doc = "Field `OM4` reader - Pin 4 output mode"]
pub use OM0_R as OM4_R;
#[doc = "Field `OM5` reader - Pin 5 output mode"]
pub use OM0_R as OM5_R;
#[doc = "Field `OM6` reader - Pin 6 output mode"]
pub use OM0_R as OM6_R;
#[doc = "Field `OM7` reader - Pin 7 output mode"]
pub use OM0_R as OM7_R;
#[doc = "Field `OM8` reader - Pin 8 output mode"]
pub use OM0_R as OM8_R;
#[doc = "Field `OM9` reader - Pin 9 output mode"]
pub use OM0_R as OM9_R;
#[doc = "Field `OM10` reader - Pin 10 output mode"]
pub use OM0_R as OM10_R;
#[doc = "Field `OM11` reader - Pin 11 output mode"]
pub use OM0_R as OM11_R;
#[doc = "Field `OM12` reader - Pin 12 output mode"]
pub use OM0_R as OM12_R;
#[doc = "Field `OM13` reader - Pin 13 output mode"]
pub use OM0_R as OM13_R;
#[doc = "Field `OM14` reader - Pin 14 output mode"]
pub use OM0_R as OM14_R;
#[doc = "Field `OM15` reader - Pin 15 output mode"]
pub use OM0_R as OM15_R;
#[doc = "Field `OM1` writer - Pin 1 output mode"]
pub use OM0_W as OM1_W;
#[doc = "Field `OM2` writer - Pin 2 output mode"]
pub use OM0_W as OM2_W;
#[doc = "Field `OM3` writer - Pin 3 output mode"]
pub use OM0_W as OM3_W;
#[doc = "Field `OM4` writer - Pin 4 output mode"]
pub use OM0_W as OM4_W;
#[doc = "Field `OM5` writer - Pin 5 output mode"]
pub use OM0_W as OM5_W;
#[doc = "Field `OM6` writer - Pin 6 output mode"]
pub use OM0_W as OM6_W;
#[doc = "Field `OM7` writer - Pin 7 output mode"]
pub use OM0_W as OM7_W;
#[doc = "Field `OM8` writer - Pin 8 output mode"]
pub use OM0_W as OM8_W;
#[doc = "Field `OM9` writer - Pin 9 output mode"]
pub use OM0_W as OM9_W;
#[doc = "Field `OM10` writer - Pin 10 output mode"]
pub use OM0_W as OM10_W;
#[doc = "Field `OM11` writer - Pin 11 output mode"]
pub use OM0_W as OM11_W;
#[doc = "Field `OM12` writer - Pin 12 output mode"]
pub use OM0_W as OM12_W;
#[doc = "Field `OM13` writer - Pin 13 output mode"]
pub use OM0_W as OM13_W;
#[doc = "Field `OM14` writer - Pin 14 output mode"]
pub use OM0_W as OM14_W;
#[doc = "Field `OM15` writer - Pin 15 output mode"]
pub use OM0_W as OM15_W;
impl R {
    #[doc = "Bit 0 - Pin 0 output mode"]
    #[inline(always)]
    pub fn om0(&self) -> OM0_R {
        OM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1 output mode"]
    #[inline(always)]
    pub fn om1(&self) -> OM1_R {
        OM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2 output mode"]
    #[inline(always)]
    pub fn om2(&self) -> OM2_R {
        OM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3 output mode"]
    #[inline(always)]
    pub fn om3(&self) -> OM3_R {
        OM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4 output mode"]
    #[inline(always)]
    pub fn om4(&self) -> OM4_R {
        OM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5 output mode"]
    #[inline(always)]
    pub fn om5(&self) -> OM5_R {
        OM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6 output mode"]
    #[inline(always)]
    pub fn om6(&self) -> OM6_R {
        OM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7 output mode"]
    #[inline(always)]
    pub fn om7(&self) -> OM7_R {
        OM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8 output mode"]
    #[inline(always)]
    pub fn om8(&self) -> OM8_R {
        OM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9 output mode"]
    #[inline(always)]
    pub fn om9(&self) -> OM9_R {
        OM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10 output mode"]
    #[inline(always)]
    pub fn om10(&self) -> OM10_R {
        OM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11 output mode"]
    #[inline(always)]
    pub fn om11(&self) -> OM11_R {
        OM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12 output mode"]
    #[inline(always)]
    pub fn om12(&self) -> OM12_R {
        OM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13 output mode"]
    #[inline(always)]
    pub fn om13(&self) -> OM13_R {
        OM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14 output mode"]
    #[inline(always)]
    pub fn om14(&self) -> OM14_R {
        OM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15 output mode"]
    #[inline(always)]
    pub fn om15(&self) -> OM15_R {
        OM15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin 0 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om0(&mut self) -> OM0_W<OMODE_SPEC, 0> {
        OM0_W::new(self)
    }
    #[doc = "Bit 1 - Pin 1 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om1(&mut self) -> OM1_W<OMODE_SPEC, 1> {
        OM1_W::new(self)
    }
    #[doc = "Bit 2 - Pin 2 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om2(&mut self) -> OM2_W<OMODE_SPEC, 2> {
        OM2_W::new(self)
    }
    #[doc = "Bit 3 - Pin 3 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om3(&mut self) -> OM3_W<OMODE_SPEC, 3> {
        OM3_W::new(self)
    }
    #[doc = "Bit 4 - Pin 4 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om4(&mut self) -> OM4_W<OMODE_SPEC, 4> {
        OM4_W::new(self)
    }
    #[doc = "Bit 5 - Pin 5 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om5(&mut self) -> OM5_W<OMODE_SPEC, 5> {
        OM5_W::new(self)
    }
    #[doc = "Bit 6 - Pin 6 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om6(&mut self) -> OM6_W<OMODE_SPEC, 6> {
        OM6_W::new(self)
    }
    #[doc = "Bit 7 - Pin 7 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om7(&mut self) -> OM7_W<OMODE_SPEC, 7> {
        OM7_W::new(self)
    }
    #[doc = "Bit 8 - Pin 8 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om8(&mut self) -> OM8_W<OMODE_SPEC, 8> {
        OM8_W::new(self)
    }
    #[doc = "Bit 9 - Pin 9 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om9(&mut self) -> OM9_W<OMODE_SPEC, 9> {
        OM9_W::new(self)
    }
    #[doc = "Bit 10 - Pin 10 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om10(&mut self) -> OM10_W<OMODE_SPEC, 10> {
        OM10_W::new(self)
    }
    #[doc = "Bit 11 - Pin 11 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om11(&mut self) -> OM11_W<OMODE_SPEC, 11> {
        OM11_W::new(self)
    }
    #[doc = "Bit 12 - Pin 12 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om12(&mut self) -> OM12_W<OMODE_SPEC, 12> {
        OM12_W::new(self)
    }
    #[doc = "Bit 13 - Pin 13 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om13(&mut self) -> OM13_W<OMODE_SPEC, 13> {
        OM13_W::new(self)
    }
    #[doc = "Bit 14 - Pin 14 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om14(&mut self) -> OM14_W<OMODE_SPEC, 14> {
        OM14_W::new(self)
    }
    #[doc = "Bit 15 - Pin 15 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om15(&mut self) -> OM15_W<OMODE_SPEC, 15> {
        OM15_W::new(self)
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
#[doc = "GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OMODE_SPEC;
impl crate::RegisterSpec for OMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`omode::R`](R) reader structure"]
impl crate::Readable for OMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`omode::W`](W) writer structure"]
impl crate::Writable for OMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OMODE to value 0"]
impl crate::Resettable for OMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
