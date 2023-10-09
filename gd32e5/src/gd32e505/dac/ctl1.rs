#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `FIFOEN0` reader - DAC_OUT0 data FIFO enable"]
pub type FIFOEN0_R = crate::BitReader;
#[doc = "Field `FIFOEN0` writer - DAC_OUT0 data FIFO enable"]
pub type FIFOEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOOVRIE0` reader - DAC_OUT0 FIFO overflow interrupt enable"]
pub type FIFOOVRIE0_R = crate::BitReader;
#[doc = "Field `FIFOOVRIE0` writer - DAC_OUT0 FIFO overflow interrupt enable"]
pub type FIFOOVRIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOUDRIE0` reader - DAC_OUT0 FIFO underflow interrupt enable"]
pub type FIFOUDRIE0_R = crate::BitReader;
#[doc = "Field `FIFOUDRIE0` writer - DAC_OUT0 FIFO underflow interrupt enable"]
pub type FIFOUDRIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOEN1` reader - DAC_OUT1 data FIFO enable"]
pub type FIFOEN1_R = crate::BitReader;
#[doc = "Field `FIFOEN1` writer - DAC_OUT1 data FIFO enable"]
pub type FIFOEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOOVRIE1` reader - DAC_OUT1 FIFO overflow interrupt enable"]
pub type FIFOOVRIE1_R = crate::BitReader;
#[doc = "Field `FIFOOVRIE1` writer - DAC_OUT1 FIFO overflow interrupt enable"]
pub type FIFOOVRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOUDRIE1` reader - DAC_OUT1 FIFO underflow interrupt enable"]
pub type FIFOUDRIE1_R = crate::BitReader;
#[doc = "Field `FIFOUDRIE1` writer - DAC_OUT1 FIFO underflow interrupt enable"]
pub type FIFOUDRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DAC_OUT0 data FIFO enable"]
    #[inline(always)]
    pub fn fifoen0(&self) -> FIFOEN0_R {
        FIFOEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC_OUT0 FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn fifoovrie0(&self) -> FIFOOVRIE0_R {
        FIFOOVRIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC_OUT0 FIFO underflow interrupt enable"]
    #[inline(always)]
    pub fn fifoudrie0(&self) -> FIFOUDRIE0_R {
        FIFOUDRIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC_OUT1 data FIFO enable"]
    #[inline(always)]
    pub fn fifoen1(&self) -> FIFOEN1_R {
        FIFOEN1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC_OUT1 FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn fifoovrie1(&self) -> FIFOOVRIE1_R {
        FIFOOVRIE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC_OUT1 FIFO underflow interrupt enable"]
    #[inline(always)]
    pub fn fifoudrie1(&self) -> FIFOUDRIE1_R {
        FIFOUDRIE1_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC_OUT0 data FIFO enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen0(&mut self) -> FIFOEN0_W<CTL1_SPEC, 0> {
        FIFOEN0_W::new(self)
    }
    #[doc = "Bit 1 - DAC_OUT0 FIFO overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovrie0(&mut self) -> FIFOOVRIE0_W<CTL1_SPEC, 1> {
        FIFOOVRIE0_W::new(self)
    }
    #[doc = "Bit 2 - DAC_OUT0 FIFO underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoudrie0(&mut self) -> FIFOUDRIE0_W<CTL1_SPEC, 2> {
        FIFOUDRIE0_W::new(self)
    }
    #[doc = "Bit 16 - DAC_OUT1 data FIFO enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen1(&mut self) -> FIFOEN1_W<CTL1_SPEC, 16> {
        FIFOEN1_W::new(self)
    }
    #[doc = "Bit 17 - DAC_OUT1 FIFO overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovrie1(&mut self) -> FIFOOVRIE1_W<CTL1_SPEC, 17> {
        FIFOOVRIE1_W::new(self)
    }
    #[doc = "Bit 18 - DAC_OUT1 FIFO underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoudrie1(&mut self) -> FIFOUDRIE1_W<CTL1_SPEC, 18> {
        FIFOUDRIE1_W::new(self)
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
#[doc = "DAC Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
