#[doc = "Register `FAFIFO` reader"]
pub type R = crate::R<FAFIFO_SPEC>;
#[doc = "Register `FAFIFO` writer"]
pub type W = crate::W<FAFIFO_SPEC>;
#[doc = "Field `FAF0` reader - Filter 0 associated with FIFO"]
pub type FAF0_R = crate::BitReader;
#[doc = "Field `FAF0` writer - Filter 0 associated with FIFO"]
pub type FAF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF1` reader - Filter 1 associated with FIFO"]
pub type FAF1_R = crate::BitReader;
#[doc = "Field `FAF1` writer - Filter 1 associated with FIFO"]
pub type FAF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF2` reader - Filter 2 associated with FIFO"]
pub type FAF2_R = crate::BitReader;
#[doc = "Field `FAF2` writer - Filter 2 associated with FIFO"]
pub type FAF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF3` reader - Filter 3 associated with FIFO"]
pub type FAF3_R = crate::BitReader;
#[doc = "Field `FAF3` writer - Filter 3 associated with FIFO"]
pub type FAF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF4` reader - Filter 4 associated with FIFO"]
pub type FAF4_R = crate::BitReader;
#[doc = "Field `FAF4` writer - Filter 4 associated with FIFO"]
pub type FAF4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF5` reader - Filter 5 associated with FIFO"]
pub type FAF5_R = crate::BitReader;
#[doc = "Field `FAF5` writer - Filter 5 associated with FIFO"]
pub type FAF5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF6` reader - Filter 6 associated with FIFO"]
pub type FAF6_R = crate::BitReader;
#[doc = "Field `FAF6` writer - Filter 6 associated with FIFO"]
pub type FAF6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF7` reader - Filter 7 associated with FIFO"]
pub type FAF7_R = crate::BitReader;
#[doc = "Field `FAF7` writer - Filter 7 associated with FIFO"]
pub type FAF7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF8` reader - Filter 8 associated with FIFO"]
pub type FAF8_R = crate::BitReader;
#[doc = "Field `FAF8` writer - Filter 8 associated with FIFO"]
pub type FAF8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF9` reader - Filter 9 associated with FIFO"]
pub type FAF9_R = crate::BitReader;
#[doc = "Field `FAF9` writer - Filter 9 associated with FIFO"]
pub type FAF9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF10` reader - Filter 10 associated with FIFO"]
pub type FAF10_R = crate::BitReader;
#[doc = "Field `FAF10` writer - Filter 10 associated with FIFO"]
pub type FAF10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF11` reader - Filter 11 associated with FIFO"]
pub type FAF11_R = crate::BitReader;
#[doc = "Field `FAF11` writer - Filter 11 associated with FIFO"]
pub type FAF11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF12` reader - Filter 12 associated with FIFO"]
pub type FAF12_R = crate::BitReader;
#[doc = "Field `FAF12` writer - Filter 12 associated with FIFO"]
pub type FAF12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAF13` reader - Filter 13 associated with FIFO"]
pub type FAF13_R = crate::BitReader;
#[doc = "Field `FAF13` writer - Filter 13 associated with FIFO"]
pub type FAF13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Filter 0 associated with FIFO"]
    #[inline(always)]
    pub fn faf0(&self) -> FAF0_R {
        FAF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter 1 associated with FIFO"]
    #[inline(always)]
    pub fn faf1(&self) -> FAF1_R {
        FAF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter 2 associated with FIFO"]
    #[inline(always)]
    pub fn faf2(&self) -> FAF2_R {
        FAF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter 3 associated with FIFO"]
    #[inline(always)]
    pub fn faf3(&self) -> FAF3_R {
        FAF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter 4 associated with FIFO"]
    #[inline(always)]
    pub fn faf4(&self) -> FAF4_R {
        FAF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter 5 associated with FIFO"]
    #[inline(always)]
    pub fn faf5(&self) -> FAF5_R {
        FAF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter 6 associated with FIFO"]
    #[inline(always)]
    pub fn faf6(&self) -> FAF6_R {
        FAF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter 7 associated with FIFO"]
    #[inline(always)]
    pub fn faf7(&self) -> FAF7_R {
        FAF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter 8 associated with FIFO"]
    #[inline(always)]
    pub fn faf8(&self) -> FAF8_R {
        FAF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter 9 associated with FIFO"]
    #[inline(always)]
    pub fn faf9(&self) -> FAF9_R {
        FAF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter 10 associated with FIFO"]
    #[inline(always)]
    pub fn faf10(&self) -> FAF10_R {
        FAF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter 11 associated with FIFO"]
    #[inline(always)]
    pub fn faf11(&self) -> FAF11_R {
        FAF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter 12 associated with FIFO"]
    #[inline(always)]
    pub fn faf12(&self) -> FAF12_R {
        FAF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter 13 associated with FIFO"]
    #[inline(always)]
    pub fn faf13(&self) -> FAF13_R {
        FAF13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter 0 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf0(&mut self) -> FAF0_W<FAFIFO_SPEC, 0> {
        FAF0_W::new(self)
    }
    #[doc = "Bit 1 - Filter 1 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf1(&mut self) -> FAF1_W<FAFIFO_SPEC, 1> {
        FAF1_W::new(self)
    }
    #[doc = "Bit 2 - Filter 2 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf2(&mut self) -> FAF2_W<FAFIFO_SPEC, 2> {
        FAF2_W::new(self)
    }
    #[doc = "Bit 3 - Filter 3 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf3(&mut self) -> FAF3_W<FAFIFO_SPEC, 3> {
        FAF3_W::new(self)
    }
    #[doc = "Bit 4 - Filter 4 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf4(&mut self) -> FAF4_W<FAFIFO_SPEC, 4> {
        FAF4_W::new(self)
    }
    #[doc = "Bit 5 - Filter 5 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf5(&mut self) -> FAF5_W<FAFIFO_SPEC, 5> {
        FAF5_W::new(self)
    }
    #[doc = "Bit 6 - Filter 6 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf6(&mut self) -> FAF6_W<FAFIFO_SPEC, 6> {
        FAF6_W::new(self)
    }
    #[doc = "Bit 7 - Filter 7 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf7(&mut self) -> FAF7_W<FAFIFO_SPEC, 7> {
        FAF7_W::new(self)
    }
    #[doc = "Bit 8 - Filter 8 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf8(&mut self) -> FAF8_W<FAFIFO_SPEC, 8> {
        FAF8_W::new(self)
    }
    #[doc = "Bit 9 - Filter 9 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf9(&mut self) -> FAF9_W<FAFIFO_SPEC, 9> {
        FAF9_W::new(self)
    }
    #[doc = "Bit 10 - Filter 10 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf10(&mut self) -> FAF10_W<FAFIFO_SPEC, 10> {
        FAF10_W::new(self)
    }
    #[doc = "Bit 11 - Filter 11 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf11(&mut self) -> FAF11_W<FAFIFO_SPEC, 11> {
        FAF11_W::new(self)
    }
    #[doc = "Bit 12 - Filter 12 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf12(&mut self) -> FAF12_W<FAFIFO_SPEC, 12> {
        FAF12_W::new(self)
    }
    #[doc = "Bit 13 - Filter 13 associated with FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf13(&mut self) -> FAF13_W<FAFIFO_SPEC, 13> {
        FAF13_W::new(self)
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
#[doc = "Filter associated FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fafifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fafifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FAFIFO_SPEC;
impl crate::RegisterSpec for FAFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fafifo::R`](R) reader structure"]
impl crate::Readable for FAFIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fafifo::W`](W) writer structure"]
impl crate::Writable for FAFIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAFIFO to value 0"]
impl crate::Resettable for FAFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
