#[doc = "Register `ST0CAP0TRG` reader"]
pub type R = crate::R<St0cap0trgSpec>;
#[doc = "Register `ST0CAP0TRG` writer"]
pub type W = crate::W<St0cap0trgSpec>;
#[doc = "Field `CP0BSW` reader - Capture 0 triggered by software"]
pub type Cp0bswR = crate::BitReader;
#[doc = "Field `CP0BSW` writer - Capture 0 triggered by software"]
pub type Cp0bswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BUP` reader - Capture 0 triggered by update event"]
pub type Cp0bupR = crate::BitReader;
#[doc = "Field `CP0BUP` writer - Capture 0 triggered by update event"]
pub type Cp0bupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BEXEV0` reader - Capture 0 triggered by external event 0"]
pub type Cp0bexev0R = crate::BitReader;
#[doc = "Field `CP0BEXEV0` writer - Capture 0 triggered by external event 0"]
pub type Cp0bexev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BEXEV1` reader - Capture 0 triggered by external event 1"]
pub type Cp0bexev1R = crate::BitReader;
#[doc = "Field `CP0BEXEV1` writer - Capture 0 triggered by external event 1"]
pub type Cp0bexev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BEXEV2` reader - Capture 0 triggered by external event 2"]
pub type Cp0bexev2R = crate::BitReader;
#[doc = "Field `CP0BEXEV2` writer - Capture 0 triggered by external event 2"]
pub type Cp0bexev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BEXEV3` reader - Capture 0 triggered by external event 3"]
pub type Cp0bexev3R = crate::BitReader;
#[doc = "Field `CP0BEXEV3` writer - Capture 0 triggered by external event 3"]
pub type Cp0bexev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BEXEV4` reader - Capture 0 triggered by external event 4"]
pub type Cp0bexev4R = crate::BitReader;
#[doc = "Field `CP0BEXEV4` writer - Capture 0 triggered by external event 4"]
pub type Cp0bexev4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BEXEV5` reader - Capture 0 triggered by external event 5"]
pub type Cp0bexev5R = crate::BitReader;
#[doc = "Field `CP0BEXEV5` writer - Capture 0 triggered by external event 5"]
pub type Cp0bexev5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BEXEV6` reader - Capture 0 triggered by external event 6"]
pub type Cp0bexev6R = crate::BitReader;
#[doc = "Field `CP0BEXEV6` writer - Capture 0 triggered by external event 6"]
pub type Cp0bexev6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BEXEV7` reader - Capture 0 triggered by external event 7"]
pub type Cp0bexev7R = crate::BitReader;
#[doc = "Field `CP0BEXEV7` writer - Capture 0 triggered by external event 7"]
pub type Cp0bexev7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BEXEV8` reader - Capture 0 triggered by external event 8"]
pub type Cp0bexev8R = crate::BitReader;
#[doc = "Field `CP0BEXEV8` writer - Capture 0 triggered by external event 8"]
pub type Cp0bexev8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BEXEV9` reader - Capture 0 triggered by external event 9"]
pub type Cp0bexev9R = crate::BitReader;
#[doc = "Field `CP0BEXEV9` writer - Capture 0 triggered by external event 9"]
pub type Cp0bexev9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BST0A` reader - Capture 0 triggered by ST0CH0_O output inactive to active transition"]
pub type Cp0bst0aR = crate::BitReader;
#[doc = "Field `CP0BST0A` writer - Capture 0 triggered by ST0CH0_O output inactive to active transition"]
pub type Cp0bst0aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BST0NA` reader - Capture 0 triggered by ST0CH0_O output active to inactive transition"]
pub type Cp0bst0naR = crate::BitReader;
#[doc = "Field `CP0BST0NA` writer - Capture 0 triggered by ST0CH0_O output active to inactive transition"]
pub type Cp0bst0naW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BST0CMP0` reader - Capture 0 triggered by compare 0 event of Slave_TIMER0"]
pub type Cp0bst0cmp0R = crate::BitReader;
#[doc = "Field `CP0BST0CMP0` writer - Capture 0 triggered by compare 0 event of Slave_TIMER0"]
pub type Cp0bst0cmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0BST0CMP1` reader - Capture 0 triggered by compare 1 event of Slave_TIMER0"]
pub type Cp0bst0cmp1R = crate::BitReader;
#[doc = "Field `CP0BST0CMP1` writer - Capture 0 triggered by compare 1 event of Slave_TIMER0"]
pub type Cp0bst0cmp1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture 0 triggered by software"]
    #[inline(always)]
    pub fn cp0bsw(&self) -> Cp0bswR {
        Cp0bswR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture 0 triggered by update event"]
    #[inline(always)]
    pub fn cp0bup(&self) -> Cp0bupR {
        Cp0bupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture 0 triggered by external event 0"]
    #[inline(always)]
    pub fn cp0bexev0(&self) -> Cp0bexev0R {
        Cp0bexev0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture 0 triggered by external event 1"]
    #[inline(always)]
    pub fn cp0bexev1(&self) -> Cp0bexev1R {
        Cp0bexev1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture 0 triggered by external event 2"]
    #[inline(always)]
    pub fn cp0bexev2(&self) -> Cp0bexev2R {
        Cp0bexev2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture 0 triggered by external event 3"]
    #[inline(always)]
    pub fn cp0bexev3(&self) -> Cp0bexev3R {
        Cp0bexev3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture 0 triggered by external event 4"]
    #[inline(always)]
    pub fn cp0bexev4(&self) -> Cp0bexev4R {
        Cp0bexev4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture 0 triggered by external event 5"]
    #[inline(always)]
    pub fn cp0bexev5(&self) -> Cp0bexev5R {
        Cp0bexev5R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture 0 triggered by external event 6"]
    #[inline(always)]
    pub fn cp0bexev6(&self) -> Cp0bexev6R {
        Cp0bexev6R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture 0 triggered by external event 7"]
    #[inline(always)]
    pub fn cp0bexev7(&self) -> Cp0bexev7R {
        Cp0bexev7R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture 0 triggered by external event 8"]
    #[inline(always)]
    pub fn cp0bexev8(&self) -> Cp0bexev8R {
        Cp0bexev8R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture 0 triggered by external event 9"]
    #[inline(always)]
    pub fn cp0bexev9(&self) -> Cp0bexev9R {
        Cp0bexev9R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture 0 triggered by ST0CH0_O output inactive to active transition"]
    #[inline(always)]
    pub fn cp0bst0a(&self) -> Cp0bst0aR {
        Cp0bst0aR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture 0 triggered by ST0CH0_O output active to inactive transition"]
    #[inline(always)]
    pub fn cp0bst0na(&self) -> Cp0bst0naR {
        Cp0bst0naR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Capture 0 triggered by compare 0 event of Slave_TIMER0"]
    #[inline(always)]
    pub fn cp0bst0cmp0(&self) -> Cp0bst0cmp0R {
        Cp0bst0cmp0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Capture 0 triggered by compare 1 event of Slave_TIMER0"]
    #[inline(always)]
    pub fn cp0bst0cmp1(&self) -> Cp0bst0cmp1R {
        Cp0bst0cmp1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture 0 triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bsw(&mut self) -> Cp0bswW<St0cap0trgSpec> {
        Cp0bswW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture 0 triggered by update event"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bup(&mut self) -> Cp0bupW<St0cap0trgSpec> {
        Cp0bupW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture 0 triggered by external event 0"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev0(&mut self) -> Cp0bexev0W<St0cap0trgSpec> {
        Cp0bexev0W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture 0 triggered by external event 1"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev1(&mut self) -> Cp0bexev1W<St0cap0trgSpec> {
        Cp0bexev1W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture 0 triggered by external event 2"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev2(&mut self) -> Cp0bexev2W<St0cap0trgSpec> {
        Cp0bexev2W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture 0 triggered by external event 3"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev3(&mut self) -> Cp0bexev3W<St0cap0trgSpec> {
        Cp0bexev3W::new(self, 5)
    }
    #[doc = "Bit 6 - Capture 0 triggered by external event 4"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev4(&mut self) -> Cp0bexev4W<St0cap0trgSpec> {
        Cp0bexev4W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture 0 triggered by external event 5"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev5(&mut self) -> Cp0bexev5W<St0cap0trgSpec> {
        Cp0bexev5W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture 0 triggered by external event 6"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev6(&mut self) -> Cp0bexev6W<St0cap0trgSpec> {
        Cp0bexev6W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture 0 triggered by external event 7"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev7(&mut self) -> Cp0bexev7W<St0cap0trgSpec> {
        Cp0bexev7W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture 0 triggered by external event 8"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev8(&mut self) -> Cp0bexev8W<St0cap0trgSpec> {
        Cp0bexev8W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture 0 triggered by external event 9"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev9(&mut self) -> Cp0bexev9W<St0cap0trgSpec> {
        Cp0bexev9W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture 0 triggered by ST0CH0_O output inactive to active transition"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bst0a(&mut self) -> Cp0bst0aW<St0cap0trgSpec> {
        Cp0bst0aW::new(self, 12)
    }
    #[doc = "Bit 13 - Capture 0 triggered by ST0CH0_O output active to inactive transition"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bst0na(&mut self) -> Cp0bst0naW<St0cap0trgSpec> {
        Cp0bst0naW::new(self, 13)
    }
    #[doc = "Bit 14 - Capture 0 triggered by compare 0 event of Slave_TIMER0"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bst0cmp0(&mut self) -> Cp0bst0cmp0W<St0cap0trgSpec> {
        Cp0bst0cmp0W::new(self, 14)
    }
    #[doc = "Bit 15 - Capture 0 triggered by compare 1 event of Slave_TIMER0"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bst0cmp1(&mut self) -> Cp0bst0cmp1W<St0cap0trgSpec> {
        Cp0bst0cmp1W::new(self, 15)
    }
}
#[doc = "SHRTIMER Slave_TIMER0 capture 0 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cap0trg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cap0trg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St0cap0trgSpec;
impl crate::RegisterSpec for St0cap0trgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0cap0trg::R`](R) reader structure"]
impl crate::Readable for St0cap0trgSpec {}
#[doc = "`write(|w| ..)` method takes [`st0cap0trg::W`](W) writer structure"]
impl crate::Writable for St0cap0trgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST0CAP0TRG to value 0"]
impl crate::Resettable for St0cap0trgSpec {
    const RESET_VALUE: u32 = 0;
}
