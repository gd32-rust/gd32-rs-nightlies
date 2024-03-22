#[doc = "Register `EVEN` reader"]
pub type R = crate::R<EvenSpec>;
#[doc = "Register `EVEN` writer"]
pub type W = crate::W<EvenSpec>;
#[doc = "Event enable on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Even0 {
    #[doc = "0: Event from line is disabled"]
    Masked = 0,
    #[doc = "1: Event from line is enabled"]
    Unmasked = 1,
}
impl From<Even0> for bool {
    #[inline(always)]
    fn from(variant: Even0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVEN0` reader - Event enable on line 0"]
pub type Even0R = crate::BitReader<Even0>;
impl Even0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Even0 {
        match self.bits {
            false => Even0::Masked,
            true => Even0::Unmasked,
        }
    }
    #[doc = "Event from line is disabled"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Even0::Masked
    }
    #[doc = "Event from line is enabled"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Even0::Unmasked
    }
}
#[doc = "Field `EVEN0` writer - Event enable on line 0"]
pub type Even0W<'a, REG> = crate::BitWriter<'a, REG, Even0>;
impl<'a, REG> Even0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event from line is disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Even0::Masked)
    }
    #[doc = "Event from line is enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Even0::Unmasked)
    }
}
#[doc = "Field `EVEN1` reader - Event enable on line 1"]
pub use Even0R as Even1R;
#[doc = "Field `EVEN2` reader - Event enable on line 2"]
pub use Even0R as Even2R;
#[doc = "Field `EVEN3` reader - Event enable on line 3"]
pub use Even0R as Even3R;
#[doc = "Field `EVEN4` reader - Event enable on line 4"]
pub use Even0R as Even4R;
#[doc = "Field `EVEN5` reader - Event enable on line 5"]
pub use Even0R as Even5R;
#[doc = "Field `EVEN6` reader - Event enable on line 6"]
pub use Even0R as Even6R;
#[doc = "Field `EVEN7` reader - Event enable on line 7"]
pub use Even0R as Even7R;
#[doc = "Field `EVEN8` reader - Event enable on line 8"]
pub use Even0R as Even8R;
#[doc = "Field `EVEN9` reader - Event enable on line 9"]
pub use Even0R as Even9R;
#[doc = "Field `EVEN10` reader - Event enable on line 10"]
pub use Even0R as Even10R;
#[doc = "Field `EVEN11` reader - Event enable on line 11"]
pub use Even0R as Even11R;
#[doc = "Field `EVEN12` reader - Event enable on line 12"]
pub use Even0R as Even12R;
#[doc = "Field `EVEN13` reader - Event enable on line 13"]
pub use Even0R as Even13R;
#[doc = "Field `EVEN14` reader - Event enable on line 14"]
pub use Even0R as Even14R;
#[doc = "Field `EVEN15` reader - Event enable on line 15"]
pub use Even0R as Even15R;
#[doc = "Field `EVEN16` reader - Event enable on line 16"]
pub use Even0R as Even16R;
#[doc = "Field `EVEN17` reader - Event enable on line 17"]
pub use Even0R as Even17R;
#[doc = "Field `EVEN18` reader - Event enable on line 18"]
pub use Even0R as Even18R;
#[doc = "Field `EVEN19` reader - Event enable on line 19"]
pub use Even0R as Even19R;
#[doc = "Field `EVEN20` reader - Event enable on line 20"]
pub use Even0R as Even20R;
#[doc = "Field `EVEN21` reader - Event enable on line 21"]
pub use Even0R as Even21R;
#[doc = "Field `EVEN22` reader - Event enable on line 22"]
pub use Even0R as Even22R;
#[doc = "Field `EVEN23` reader - Event enable on line 23"]
pub use Even0R as Even23R;
#[doc = "Field `EVEN24` reader - Event enable on line 24"]
pub use Even0R as Even24R;
#[doc = "Field `EVEN25` reader - Event enable on line 25"]
pub use Even0R as Even25R;
#[doc = "Field `EVEN26` reader - Event enable on line 26"]
pub use Even0R as Even26R;
#[doc = "Field `EVEN27` reader - Event enable on line 27"]
pub use Even0R as Even27R;
#[doc = "Field `EVEN1` writer - Event enable on line 1"]
pub use Even0W as Even1W;
#[doc = "Field `EVEN2` writer - Event enable on line 2"]
pub use Even0W as Even2W;
#[doc = "Field `EVEN3` writer - Event enable on line 3"]
pub use Even0W as Even3W;
#[doc = "Field `EVEN4` writer - Event enable on line 4"]
pub use Even0W as Even4W;
#[doc = "Field `EVEN5` writer - Event enable on line 5"]
pub use Even0W as Even5W;
#[doc = "Field `EVEN6` writer - Event enable on line 6"]
pub use Even0W as Even6W;
#[doc = "Field `EVEN7` writer - Event enable on line 7"]
pub use Even0W as Even7W;
#[doc = "Field `EVEN8` writer - Event enable on line 8"]
pub use Even0W as Even8W;
#[doc = "Field `EVEN9` writer - Event enable on line 9"]
pub use Even0W as Even9W;
#[doc = "Field `EVEN10` writer - Event enable on line 10"]
pub use Even0W as Even10W;
#[doc = "Field `EVEN11` writer - Event enable on line 11"]
pub use Even0W as Even11W;
#[doc = "Field `EVEN12` writer - Event enable on line 12"]
pub use Even0W as Even12W;
#[doc = "Field `EVEN13` writer - Event enable on line 13"]
pub use Even0W as Even13W;
#[doc = "Field `EVEN14` writer - Event enable on line 14"]
pub use Even0W as Even14W;
#[doc = "Field `EVEN15` writer - Event enable on line 15"]
pub use Even0W as Even15W;
#[doc = "Field `EVEN16` writer - Event enable on line 16"]
pub use Even0W as Even16W;
#[doc = "Field `EVEN17` writer - Event enable on line 17"]
pub use Even0W as Even17W;
#[doc = "Field `EVEN18` writer - Event enable on line 18"]
pub use Even0W as Even18W;
#[doc = "Field `EVEN19` writer - Event enable on line 19"]
pub use Even0W as Even19W;
#[doc = "Field `EVEN20` writer - Event enable on line 20"]
pub use Even0W as Even20W;
#[doc = "Field `EVEN21` writer - Event enable on line 21"]
pub use Even0W as Even21W;
#[doc = "Field `EVEN22` writer - Event enable on line 22"]
pub use Even0W as Even22W;
#[doc = "Field `EVEN23` writer - Event enable on line 23"]
pub use Even0W as Even23W;
#[doc = "Field `EVEN24` writer - Event enable on line 24"]
pub use Even0W as Even24W;
#[doc = "Field `EVEN25` writer - Event enable on line 25"]
pub use Even0W as Even25W;
#[doc = "Field `EVEN26` writer - Event enable on line 26"]
pub use Even0W as Even26W;
#[doc = "Field `EVEN27` writer - Event enable on line 27"]
pub use Even0W as Even27W;
impl R {
    #[doc = "Bit 0 - Event enable on line 0"]
    #[inline(always)]
    pub fn even0(&self) -> Even0R {
        Even0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event enable on line 1"]
    #[inline(always)]
    pub fn even1(&self) -> Even1R {
        Even1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event enable on line 2"]
    #[inline(always)]
    pub fn even2(&self) -> Even2R {
        Even2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event enable on line 3"]
    #[inline(always)]
    pub fn even3(&self) -> Even3R {
        Even3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event enable on line 4"]
    #[inline(always)]
    pub fn even4(&self) -> Even4R {
        Even4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event enable on line 5"]
    #[inline(always)]
    pub fn even5(&self) -> Even5R {
        Even5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event enable on line 6"]
    #[inline(always)]
    pub fn even6(&self) -> Even6R {
        Even6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event enable on line 7"]
    #[inline(always)]
    pub fn even7(&self) -> Even7R {
        Even7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event enable on line 8"]
    #[inline(always)]
    pub fn even8(&self) -> Even8R {
        Even8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event enable on line 9"]
    #[inline(always)]
    pub fn even9(&self) -> Even9R {
        Even9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event enable on line 10"]
    #[inline(always)]
    pub fn even10(&self) -> Even10R {
        Even10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event enable on line 11"]
    #[inline(always)]
    pub fn even11(&self) -> Even11R {
        Even11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event enable on line 12"]
    #[inline(always)]
    pub fn even12(&self) -> Even12R {
        Even12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event enable on line 13"]
    #[inline(always)]
    pub fn even13(&self) -> Even13R {
        Even13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event enable on line 14"]
    #[inline(always)]
    pub fn even14(&self) -> Even14R {
        Even14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event enable on line 15"]
    #[inline(always)]
    pub fn even15(&self) -> Even15R {
        Even15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event enable on line 16"]
    #[inline(always)]
    pub fn even16(&self) -> Even16R {
        Even16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event enable on line 17"]
    #[inline(always)]
    pub fn even17(&self) -> Even17R {
        Even17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event enable on line 18"]
    #[inline(always)]
    pub fn even18(&self) -> Even18R {
        Even18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Event enable on line 19"]
    #[inline(always)]
    pub fn even19(&self) -> Even19R {
        Even19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Event enable on line 20"]
    #[inline(always)]
    pub fn even20(&self) -> Even20R {
        Even20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Event enable on line 21"]
    #[inline(always)]
    pub fn even21(&self) -> Even21R {
        Even21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Event enable on line 22"]
    #[inline(always)]
    pub fn even22(&self) -> Even22R {
        Even22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Event enable on line 23"]
    #[inline(always)]
    pub fn even23(&self) -> Even23R {
        Even23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Event enable on line 24"]
    #[inline(always)]
    pub fn even24(&self) -> Even24R {
        Even24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Event enable on line 25"]
    #[inline(always)]
    pub fn even25(&self) -> Even25R {
        Even25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Event enable on line 26"]
    #[inline(always)]
    pub fn even26(&self) -> Even26R {
        Even26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Event enable on line 27"]
    #[inline(always)]
    pub fn even27(&self) -> Even27R {
        Even27R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event enable on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn even0(&mut self) -> Even0W<EvenSpec> {
        Even0W::new(self, 0)
    }
    #[doc = "Bit 1 - Event enable on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn even1(&mut self) -> Even1W<EvenSpec> {
        Even1W::new(self, 1)
    }
    #[doc = "Bit 2 - Event enable on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn even2(&mut self) -> Even2W<EvenSpec> {
        Even2W::new(self, 2)
    }
    #[doc = "Bit 3 - Event enable on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn even3(&mut self) -> Even3W<EvenSpec> {
        Even3W::new(self, 3)
    }
    #[doc = "Bit 4 - Event enable on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn even4(&mut self) -> Even4W<EvenSpec> {
        Even4W::new(self, 4)
    }
    #[doc = "Bit 5 - Event enable on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn even5(&mut self) -> Even5W<EvenSpec> {
        Even5W::new(self, 5)
    }
    #[doc = "Bit 6 - Event enable on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn even6(&mut self) -> Even6W<EvenSpec> {
        Even6W::new(self, 6)
    }
    #[doc = "Bit 7 - Event enable on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn even7(&mut self) -> Even7W<EvenSpec> {
        Even7W::new(self, 7)
    }
    #[doc = "Bit 8 - Event enable on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn even8(&mut self) -> Even8W<EvenSpec> {
        Even8W::new(self, 8)
    }
    #[doc = "Bit 9 - Event enable on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn even9(&mut self) -> Even9W<EvenSpec> {
        Even9W::new(self, 9)
    }
    #[doc = "Bit 10 - Event enable on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn even10(&mut self) -> Even10W<EvenSpec> {
        Even10W::new(self, 10)
    }
    #[doc = "Bit 11 - Event enable on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn even11(&mut self) -> Even11W<EvenSpec> {
        Even11W::new(self, 11)
    }
    #[doc = "Bit 12 - Event enable on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn even12(&mut self) -> Even12W<EvenSpec> {
        Even12W::new(self, 12)
    }
    #[doc = "Bit 13 - Event enable on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn even13(&mut self) -> Even13W<EvenSpec> {
        Even13W::new(self, 13)
    }
    #[doc = "Bit 14 - Event enable on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn even14(&mut self) -> Even14W<EvenSpec> {
        Even14W::new(self, 14)
    }
    #[doc = "Bit 15 - Event enable on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn even15(&mut self) -> Even15W<EvenSpec> {
        Even15W::new(self, 15)
    }
    #[doc = "Bit 16 - Event enable on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn even16(&mut self) -> Even16W<EvenSpec> {
        Even16W::new(self, 16)
    }
    #[doc = "Bit 17 - Event enable on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn even17(&mut self) -> Even17W<EvenSpec> {
        Even17W::new(self, 17)
    }
    #[doc = "Bit 18 - Event enable on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn even18(&mut self) -> Even18W<EvenSpec> {
        Even18W::new(self, 18)
    }
    #[doc = "Bit 19 - Event enable on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn even19(&mut self) -> Even19W<EvenSpec> {
        Even19W::new(self, 19)
    }
    #[doc = "Bit 20 - Event enable on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn even20(&mut self) -> Even20W<EvenSpec> {
        Even20W::new(self, 20)
    }
    #[doc = "Bit 21 - Event enable on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn even21(&mut self) -> Even21W<EvenSpec> {
        Even21W::new(self, 21)
    }
    #[doc = "Bit 22 - Event enable on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn even22(&mut self) -> Even22W<EvenSpec> {
        Even22W::new(self, 22)
    }
    #[doc = "Bit 23 - Event enable on line 23"]
    #[inline(always)]
    #[must_use]
    pub fn even23(&mut self) -> Even23W<EvenSpec> {
        Even23W::new(self, 23)
    }
    #[doc = "Bit 24 - Event enable on line 24"]
    #[inline(always)]
    #[must_use]
    pub fn even24(&mut self) -> Even24W<EvenSpec> {
        Even24W::new(self, 24)
    }
    #[doc = "Bit 25 - Event enable on line 25"]
    #[inline(always)]
    #[must_use]
    pub fn even25(&mut self) -> Even25W<EvenSpec> {
        Even25W::new(self, 25)
    }
    #[doc = "Bit 26 - Event enable on line 26"]
    #[inline(always)]
    #[must_use]
    pub fn even26(&mut self) -> Even26W<EvenSpec> {
        Even26W::new(self, 26)
    }
    #[doc = "Bit 27 - Event enable on line 27"]
    #[inline(always)]
    #[must_use]
    pub fn even27(&mut self) -> Even27W<EvenSpec> {
        Even27W::new(self, 27)
    }
}
#[doc = "Event enable register (EXTI_EVEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`even::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`even::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvenSpec;
impl crate::RegisterSpec for EvenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`even::R`](R) reader structure"]
impl crate::Readable for EvenSpec {}
#[doc = "`write(|w| ..)` method takes [`even::W`](W) writer structure"]
impl crate::Writable for EvenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVEN to value 0"]
impl crate::Resettable for EvenSpec {
    const RESET_VALUE: u32 = 0;
}
