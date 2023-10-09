#[doc = "Register `ST1CAP1TRG` reader"]
pub type R = crate::R<ST1CAP1TRG_SPEC>;
#[doc = "Register `ST1CAP1TRG` writer"]
pub type W = crate::W<ST1CAP1TRG_SPEC>;
#[doc = "Field `CP1BSW` reader - Capture 1 triggered by software"]
pub type CP1BSW_R = crate::BitReader;
#[doc = "Field `CP1BSW` writer - Capture 1 triggered by software"]
pub type CP1BSW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BUP` reader - Capture 1 triggered by update event"]
pub type CP1BUP_R = crate::BitReader;
#[doc = "Field `CP1BUP` writer - Capture 1 triggered by update event"]
pub type CP1BUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BEXEV0` reader - Capture 1 triggered by external event 0"]
pub type CP1BEXEV0_R = crate::BitReader;
#[doc = "Field `CP1BEXEV0` writer - Capture 1 triggered by external event 0"]
pub type CP1BEXEV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BEXEV1` reader - Capture 1 triggered by external event 1"]
pub type CP1BEXEV1_R = crate::BitReader;
#[doc = "Field `CP1BEXEV1` writer - Capture 1 triggered by external event 1"]
pub type CP1BEXEV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BEXEV2` reader - Capture 1 triggered by external event 2"]
pub type CP1BEXEV2_R = crate::BitReader;
#[doc = "Field `CP1BEXEV2` writer - Capture 1 triggered by external event 2"]
pub type CP1BEXEV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BEXEV3` reader - Capture 1 triggered by external event 3"]
pub type CP1BEXEV3_R = crate::BitReader;
#[doc = "Field `CP1BEXEV3` writer - Capture 1 triggered by external event 3"]
pub type CP1BEXEV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BEXEV4` reader - Capture 1 triggered by external event 4"]
pub type CP1BEXEV4_R = crate::BitReader;
#[doc = "Field `CP1BEXEV4` writer - Capture 1 triggered by external event 4"]
pub type CP1BEXEV4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BEXEV5` reader - Capture 1 triggered by external event 5"]
pub type CP1BEXEV5_R = crate::BitReader;
#[doc = "Field `CP1BEXEV5` writer - Capture 1 triggered by external event 5"]
pub type CP1BEXEV5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BEXEV6` reader - Capture 1 triggered by external event 6"]
pub type CP1BEXEV6_R = crate::BitReader;
#[doc = "Field `CP1BEXEV6` writer - Capture 1 triggered by external event 6"]
pub type CP1BEXEV6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BEXEV7` reader - Capture 1 triggered by external event 7"]
pub type CP1BEXEV7_R = crate::BitReader;
#[doc = "Field `CP1BEXEV7` writer - Capture 1 triggered by external event 7"]
pub type CP1BEXEV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BEXEV8` reader - Capture 1 triggered by external event 8"]
pub type CP1BEXEV8_R = crate::BitReader;
#[doc = "Field `CP1BEXEV8` writer - Capture 1 triggered by external event 8"]
pub type CP1BEXEV8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BEXEV9` reader - Capture 1 triggered by external event 9"]
pub type CP1BEXEV9_R = crate::BitReader;
#[doc = "Field `CP1BEXEV9` writer - Capture 1 triggered by external event 9"]
pub type CP1BEXEV9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BST1A` reader - Capture 1 triggered by ST1CH0_O output inactive to active transition"]
pub type CP1BST1A_R = crate::BitReader;
#[doc = "Field `CP1BST1A` writer - Capture 1 triggered by ST1CH0_O output inactive to active transition"]
pub type CP1BST1A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BST1NA` reader - Capture 1 triggered by ST1CH0_O output active to inactive transition"]
pub type CP1BST1NA_R = crate::BitReader;
#[doc = "Field `CP1BST1NA` writer - Capture 1 triggered by ST1CH0_O output active to inactive transition"]
pub type CP1BST1NA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BST1CMP0` reader - Capture 1 triggered by compare 0 event of Slave_TIMER1"]
pub type CP1BST1CMP0_R = crate::BitReader;
#[doc = "Field `CP1BST1CMP0` writer - Capture 1 triggered by compare 0 event of Slave_TIMER1"]
pub type CP1BST1CMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1BST1CMP1` reader - Capture 1 triggered by compare 1 event of Slave_TIMER1"]
pub type CP1BST1CMP1_R = crate::BitReader;
#[doc = "Field `CP1BST1CMP1` writer - Capture 1 triggered by compare 1 event of Slave_TIMER1"]
pub type CP1BST1CMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Capture 1 triggered by software"]
    #[inline(always)]
    pub fn cp1bsw(&self) -> CP1BSW_R {
        CP1BSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture 1 triggered by update event"]
    #[inline(always)]
    pub fn cp1bup(&self) -> CP1BUP_R {
        CP1BUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture 1 triggered by external event 0"]
    #[inline(always)]
    pub fn cp1bexev0(&self) -> CP1BEXEV0_R {
        CP1BEXEV0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture 1 triggered by external event 1"]
    #[inline(always)]
    pub fn cp1bexev1(&self) -> CP1BEXEV1_R {
        CP1BEXEV1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture 1 triggered by external event 2"]
    #[inline(always)]
    pub fn cp1bexev2(&self) -> CP1BEXEV2_R {
        CP1BEXEV2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture 1 triggered by external event 3"]
    #[inline(always)]
    pub fn cp1bexev3(&self) -> CP1BEXEV3_R {
        CP1BEXEV3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture 1 triggered by external event 4"]
    #[inline(always)]
    pub fn cp1bexev4(&self) -> CP1BEXEV4_R {
        CP1BEXEV4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture 1 triggered by external event 5"]
    #[inline(always)]
    pub fn cp1bexev5(&self) -> CP1BEXEV5_R {
        CP1BEXEV5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture 1 triggered by external event 6"]
    #[inline(always)]
    pub fn cp1bexev6(&self) -> CP1BEXEV6_R {
        CP1BEXEV6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture 1 triggered by external event 7"]
    #[inline(always)]
    pub fn cp1bexev7(&self) -> CP1BEXEV7_R {
        CP1BEXEV7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture 1 triggered by external event 8"]
    #[inline(always)]
    pub fn cp1bexev8(&self) -> CP1BEXEV8_R {
        CP1BEXEV8_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture 1 triggered by external event 9"]
    #[inline(always)]
    pub fn cp1bexev9(&self) -> CP1BEXEV9_R {
        CP1BEXEV9_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture 1 triggered by ST1CH0_O output inactive to active transition"]
    #[inline(always)]
    pub fn cp1bst1a(&self) -> CP1BST1A_R {
        CP1BST1A_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture 1 triggered by ST1CH0_O output active to inactive transition"]
    #[inline(always)]
    pub fn cp1bst1na(&self) -> CP1BST1NA_R {
        CP1BST1NA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Capture 1 triggered by compare 0 event of Slave_TIMER1"]
    #[inline(always)]
    pub fn cp1bst1cmp0(&self) -> CP1BST1CMP0_R {
        CP1BST1CMP0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Capture 1 triggered by compare 1 event of Slave_TIMER1"]
    #[inline(always)]
    pub fn cp1bst1cmp1(&self) -> CP1BST1CMP1_R {
        CP1BST1CMP1_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture 1 triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bsw(&mut self) -> CP1BSW_W<ST1CAP1TRG_SPEC, 0> {
        CP1BSW_W::new(self)
    }
    #[doc = "Bit 1 - Capture 1 triggered by update event"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bup(&mut self) -> CP1BUP_W<ST1CAP1TRG_SPEC, 1> {
        CP1BUP_W::new(self)
    }
    #[doc = "Bit 2 - Capture 1 triggered by external event 0"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev0(&mut self) -> CP1BEXEV0_W<ST1CAP1TRG_SPEC, 2> {
        CP1BEXEV0_W::new(self)
    }
    #[doc = "Bit 3 - Capture 1 triggered by external event 1"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev1(&mut self) -> CP1BEXEV1_W<ST1CAP1TRG_SPEC, 3> {
        CP1BEXEV1_W::new(self)
    }
    #[doc = "Bit 4 - Capture 1 triggered by external event 2"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev2(&mut self) -> CP1BEXEV2_W<ST1CAP1TRG_SPEC, 4> {
        CP1BEXEV2_W::new(self)
    }
    #[doc = "Bit 5 - Capture 1 triggered by external event 3"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev3(&mut self) -> CP1BEXEV3_W<ST1CAP1TRG_SPEC, 5> {
        CP1BEXEV3_W::new(self)
    }
    #[doc = "Bit 6 - Capture 1 triggered by external event 4"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev4(&mut self) -> CP1BEXEV4_W<ST1CAP1TRG_SPEC, 6> {
        CP1BEXEV4_W::new(self)
    }
    #[doc = "Bit 7 - Capture 1 triggered by external event 5"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev5(&mut self) -> CP1BEXEV5_W<ST1CAP1TRG_SPEC, 7> {
        CP1BEXEV5_W::new(self)
    }
    #[doc = "Bit 8 - Capture 1 triggered by external event 6"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev6(&mut self) -> CP1BEXEV6_W<ST1CAP1TRG_SPEC, 8> {
        CP1BEXEV6_W::new(self)
    }
    #[doc = "Bit 9 - Capture 1 triggered by external event 7"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev7(&mut self) -> CP1BEXEV7_W<ST1CAP1TRG_SPEC, 9> {
        CP1BEXEV7_W::new(self)
    }
    #[doc = "Bit 10 - Capture 1 triggered by external event 8"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev8(&mut self) -> CP1BEXEV8_W<ST1CAP1TRG_SPEC, 10> {
        CP1BEXEV8_W::new(self)
    }
    #[doc = "Bit 11 - Capture 1 triggered by external event 9"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev9(&mut self) -> CP1BEXEV9_W<ST1CAP1TRG_SPEC, 11> {
        CP1BEXEV9_W::new(self)
    }
    #[doc = "Bit 16 - Capture 1 triggered by ST1CH0_O output inactive to active transition"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bst1a(&mut self) -> CP1BST1A_W<ST1CAP1TRG_SPEC, 16> {
        CP1BST1A_W::new(self)
    }
    #[doc = "Bit 17 - Capture 1 triggered by ST1CH0_O output active to inactive transition"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bst1na(&mut self) -> CP1BST1NA_W<ST1CAP1TRG_SPEC, 17> {
        CP1BST1NA_W::new(self)
    }
    #[doc = "Bit 18 - Capture 1 triggered by compare 0 event of Slave_TIMER1"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bst1cmp0(&mut self) -> CP1BST1CMP0_W<ST1CAP1TRG_SPEC, 18> {
        CP1BST1CMP0_W::new(self)
    }
    #[doc = "Bit 19 - Capture 1 triggered by compare 1 event of Slave_TIMER1"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bst1cmp1(&mut self) -> CP1BST1CMP1_W<ST1CAP1TRG_SPEC, 19> {
        CP1BST1CMP1_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx capture 1 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cap1trg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cap1trg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1CAP1TRG_SPEC;
impl crate::RegisterSpec for ST1CAP1TRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1cap1trg::R`](R) reader structure"]
impl crate::Readable for ST1CAP1TRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1cap1trg::W`](W) writer structure"]
impl crate::Writable for ST1CAP1TRG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1CAP1TRG to value 0"]
impl crate::Resettable for ST1CAP1TRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
