#[doc = "Register `ST1CAP0TRG` reader"]
pub type R = crate::R<ST1CAP0TRG_SPEC>;
#[doc = "Register `ST1CAP0TRG` writer"]
pub type W = crate::W<ST1CAP0TRG_SPEC>;
#[doc = "Field `CP0BSW` reader - Capture 0 triggered by software"]
pub type CP0BSW_R = crate::BitReader;
#[doc = "Field `CP0BSW` writer - Capture 0 triggered by software"]
pub type CP0BSW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BUP` reader - Capture 0 triggered by update event"]
pub type CP0BUP_R = crate::BitReader;
#[doc = "Field `CP0BUP` writer - Capture 0 triggered by update event"]
pub type CP0BUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BEXEV0` reader - Capture 0 triggered by external event 0"]
pub type CP0BEXEV0_R = crate::BitReader;
#[doc = "Field `CP0BEXEV0` writer - Capture 0 triggered by external event 0"]
pub type CP0BEXEV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BEXEV1` reader - Capture 0 triggered by external event 1"]
pub type CP0BEXEV1_R = crate::BitReader;
#[doc = "Field `CP0BEXEV1` writer - Capture 0 triggered by external event 1"]
pub type CP0BEXEV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BEXEV2` reader - Capture 0 triggered by external event 2"]
pub type CP0BEXEV2_R = crate::BitReader;
#[doc = "Field `CP0BEXEV2` writer - Capture 0 triggered by external event 2"]
pub type CP0BEXEV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BEXEV3` reader - Capture 0 triggered by external event 3"]
pub type CP0BEXEV3_R = crate::BitReader;
#[doc = "Field `CP0BEXEV3` writer - Capture 0 triggered by external event 3"]
pub type CP0BEXEV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BEXEV4` reader - Capture 0 triggered by external event 4"]
pub type CP0BEXEV4_R = crate::BitReader;
#[doc = "Field `CP0BEXEV4` writer - Capture 0 triggered by external event 4"]
pub type CP0BEXEV4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BEXEV5` reader - Capture 0 triggered by external event 5"]
pub type CP0BEXEV5_R = crate::BitReader;
#[doc = "Field `CP0BEXEV5` writer - Capture 0 triggered by external event 5"]
pub type CP0BEXEV5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BEXEV6` reader - Capture 0 triggered by external event 6"]
pub type CP0BEXEV6_R = crate::BitReader;
#[doc = "Field `CP0BEXEV6` writer - Capture 0 triggered by external event 6"]
pub type CP0BEXEV6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BEXEV7` reader - Capture 0 triggered by external event 7"]
pub type CP0BEXEV7_R = crate::BitReader;
#[doc = "Field `CP0BEXEV7` writer - Capture 0 triggered by external event 7"]
pub type CP0BEXEV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BEXEV8` reader - Capture 0 triggered by external event 8"]
pub type CP0BEXEV8_R = crate::BitReader;
#[doc = "Field `CP0BEXEV8` writer - Capture 0 triggered by external event 8"]
pub type CP0BEXEV8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BEXEV9` reader - Capture 0 triggered by external event 9"]
pub type CP0BEXEV9_R = crate::BitReader;
#[doc = "Field `CP0BEXEV9` writer - Capture 0 triggered by external event 9"]
pub type CP0BEXEV9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BST1A` reader - Capture 0 triggered by ST1CH0_O output inactive to active transition"]
pub type CP0BST1A_R = crate::BitReader;
#[doc = "Field `CP0BST1A` writer - Capture 0 triggered by ST1CH0_O output inactive to active transition"]
pub type CP0BST1A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BST1NA` reader - Capture 0 triggered by ST1CH0_O output active to inactive transition"]
pub type CP0BST1NA_R = crate::BitReader;
#[doc = "Field `CP0BST1NA` writer - Capture 0 triggered by ST1CH0_O output active to inactive transition"]
pub type CP0BST1NA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BST1CMP0` reader - Capture 0 triggered by compare 0 event of Slave_TIMER1"]
pub type CP0BST1CMP0_R = crate::BitReader;
#[doc = "Field `CP0BST1CMP0` writer - Capture 0 triggered by compare 0 event of Slave_TIMER1"]
pub type CP0BST1CMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0BST1CMP1` reader - Capture 0 triggered by compare 1 event of Slave_TIMER1"]
pub type CP0BST1CMP1_R = crate::BitReader;
#[doc = "Field `CP0BST1CMP1` writer - Capture 0 triggered by compare 1 event of Slave_TIMER1"]
pub type CP0BST1CMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Capture 0 triggered by software"]
    #[inline(always)]
    pub fn cp0bsw(&self) -> CP0BSW_R {
        CP0BSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture 0 triggered by update event"]
    #[inline(always)]
    pub fn cp0bup(&self) -> CP0BUP_R {
        CP0BUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture 0 triggered by external event 0"]
    #[inline(always)]
    pub fn cp0bexev0(&self) -> CP0BEXEV0_R {
        CP0BEXEV0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture 0 triggered by external event 1"]
    #[inline(always)]
    pub fn cp0bexev1(&self) -> CP0BEXEV1_R {
        CP0BEXEV1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture 0 triggered by external event 2"]
    #[inline(always)]
    pub fn cp0bexev2(&self) -> CP0BEXEV2_R {
        CP0BEXEV2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture 0 triggered by external event 3"]
    #[inline(always)]
    pub fn cp0bexev3(&self) -> CP0BEXEV3_R {
        CP0BEXEV3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture 0 triggered by external event 4"]
    #[inline(always)]
    pub fn cp0bexev4(&self) -> CP0BEXEV4_R {
        CP0BEXEV4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture 0 triggered by external event 5"]
    #[inline(always)]
    pub fn cp0bexev5(&self) -> CP0BEXEV5_R {
        CP0BEXEV5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture 0 triggered by external event 6"]
    #[inline(always)]
    pub fn cp0bexev6(&self) -> CP0BEXEV6_R {
        CP0BEXEV6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture 0 triggered by external event 7"]
    #[inline(always)]
    pub fn cp0bexev7(&self) -> CP0BEXEV7_R {
        CP0BEXEV7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture 0 triggered by external event 8"]
    #[inline(always)]
    pub fn cp0bexev8(&self) -> CP0BEXEV8_R {
        CP0BEXEV8_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture 0 triggered by external event 9"]
    #[inline(always)]
    pub fn cp0bexev9(&self) -> CP0BEXEV9_R {
        CP0BEXEV9_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture 0 triggered by ST1CH0_O output inactive to active transition"]
    #[inline(always)]
    pub fn cp0bst1a(&self) -> CP0BST1A_R {
        CP0BST1A_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture 0 triggered by ST1CH0_O output active to inactive transition"]
    #[inline(always)]
    pub fn cp0bst1na(&self) -> CP0BST1NA_R {
        CP0BST1NA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Capture 0 triggered by compare 0 event of Slave_TIMER1"]
    #[inline(always)]
    pub fn cp0bst1cmp0(&self) -> CP0BST1CMP0_R {
        CP0BST1CMP0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Capture 0 triggered by compare 1 event of Slave_TIMER1"]
    #[inline(always)]
    pub fn cp0bst1cmp1(&self) -> CP0BST1CMP1_R {
        CP0BST1CMP1_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture 0 triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bsw(&mut self) -> CP0BSW_W<ST1CAP0TRG_SPEC, 0> {
        CP0BSW_W::new(self)
    }
    #[doc = "Bit 1 - Capture 0 triggered by update event"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bup(&mut self) -> CP0BUP_W<ST1CAP0TRG_SPEC, 1> {
        CP0BUP_W::new(self)
    }
    #[doc = "Bit 2 - Capture 0 triggered by external event 0"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev0(&mut self) -> CP0BEXEV0_W<ST1CAP0TRG_SPEC, 2> {
        CP0BEXEV0_W::new(self)
    }
    #[doc = "Bit 3 - Capture 0 triggered by external event 1"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev1(&mut self) -> CP0BEXEV1_W<ST1CAP0TRG_SPEC, 3> {
        CP0BEXEV1_W::new(self)
    }
    #[doc = "Bit 4 - Capture 0 triggered by external event 2"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev2(&mut self) -> CP0BEXEV2_W<ST1CAP0TRG_SPEC, 4> {
        CP0BEXEV2_W::new(self)
    }
    #[doc = "Bit 5 - Capture 0 triggered by external event 3"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev3(&mut self) -> CP0BEXEV3_W<ST1CAP0TRG_SPEC, 5> {
        CP0BEXEV3_W::new(self)
    }
    #[doc = "Bit 6 - Capture 0 triggered by external event 4"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev4(&mut self) -> CP0BEXEV4_W<ST1CAP0TRG_SPEC, 6> {
        CP0BEXEV4_W::new(self)
    }
    #[doc = "Bit 7 - Capture 0 triggered by external event 5"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev5(&mut self) -> CP0BEXEV5_W<ST1CAP0TRG_SPEC, 7> {
        CP0BEXEV5_W::new(self)
    }
    #[doc = "Bit 8 - Capture 0 triggered by external event 6"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev6(&mut self) -> CP0BEXEV6_W<ST1CAP0TRG_SPEC, 8> {
        CP0BEXEV6_W::new(self)
    }
    #[doc = "Bit 9 - Capture 0 triggered by external event 7"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev7(&mut self) -> CP0BEXEV7_W<ST1CAP0TRG_SPEC, 9> {
        CP0BEXEV7_W::new(self)
    }
    #[doc = "Bit 10 - Capture 0 triggered by external event 8"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev8(&mut self) -> CP0BEXEV8_W<ST1CAP0TRG_SPEC, 10> {
        CP0BEXEV8_W::new(self)
    }
    #[doc = "Bit 11 - Capture 0 triggered by external event 9"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bexev9(&mut self) -> CP0BEXEV9_W<ST1CAP0TRG_SPEC, 11> {
        CP0BEXEV9_W::new(self)
    }
    #[doc = "Bit 16 - Capture 0 triggered by ST1CH0_O output inactive to active transition"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bst1a(&mut self) -> CP0BST1A_W<ST1CAP0TRG_SPEC, 16> {
        CP0BST1A_W::new(self)
    }
    #[doc = "Bit 17 - Capture 0 triggered by ST1CH0_O output active to inactive transition"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bst1na(&mut self) -> CP0BST1NA_W<ST1CAP0TRG_SPEC, 17> {
        CP0BST1NA_W::new(self)
    }
    #[doc = "Bit 18 - Capture 0 triggered by compare 0 event of Slave_TIMER1"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bst1cmp0(&mut self) -> CP0BST1CMP0_W<ST1CAP0TRG_SPEC, 18> {
        CP0BST1CMP0_W::new(self)
    }
    #[doc = "Bit 19 - Capture 0 triggered by compare 1 event of Slave_TIMER1"]
    #[inline(always)]
    #[must_use]
    pub fn cp0bst1cmp1(&mut self) -> CP0BST1CMP1_W<ST1CAP0TRG_SPEC, 19> {
        CP0BST1CMP1_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx capture 0 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cap0trg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cap0trg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1CAP0TRG_SPEC;
impl crate::RegisterSpec for ST1CAP0TRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1cap0trg::R`](R) reader structure"]
impl crate::Readable for ST1CAP0TRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1cap0trg::W`](W) writer structure"]
impl crate::Writable for ST1CAP0TRG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1CAP0TRG to value 0"]
impl crate::Resettable for ST1CAP0TRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
