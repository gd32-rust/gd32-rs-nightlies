#[doc = "Register `STAT1` reader"]
pub type R = crate::R<STAT1_SPEC>;
#[doc = "Register `STAT1` writer"]
pub type W = crate::W<STAT1_SPEC>;
#[doc = "Field `FF0` reader - DAC_OUT0 FIFO full flag"]
pub type FF0_R = crate::BitReader;
#[doc = "Field `FE0` reader - DAC_OUT0 FIFO empty flag"]
pub type FE0_R = crate::BitReader;
#[doc = "Field `FIFOOVR0` reader - DAC_OUT0 FIFO overflow flag"]
pub type FIFOOVR0_R = crate::BitReader;
#[doc = "Field `FIFOOVR0` writer - DAC_OUT0 FIFO overflow flag"]
pub type FIFOOVR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOUDR0` reader - DAC_OUT0 FIFO underflow flag"]
pub type FIFOUDR0_R = crate::BitReader;
#[doc = "Field `FIFOUDR0` writer - DAC_OUT0 FIFO underflow flag"]
pub type FIFOUDR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFONUM0` reader - Number of data in the DAC_OUT0 FIFO"]
pub type FIFONUM0_R = crate::FieldReader;
#[doc = "Field `FF1` reader - DAC_OUT1 FIFO full flag"]
pub type FF1_R = crate::BitReader;
#[doc = "Field `FE1` reader - DAC_OUT1 FIFO empty flag"]
pub type FE1_R = crate::BitReader;
#[doc = "Field `FIFOOVR1` reader - DAC_OUT1 FIFO overflow flag"]
pub type FIFOOVR1_R = crate::BitReader;
#[doc = "Field `FIFOOVR1` writer - DAC_OUT1 FIFO overflow flag"]
pub type FIFOOVR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOUDR1` reader - DAC_OUT1 FIFO underflow flag"]
pub type FIFOUDR1_R = crate::BitReader;
#[doc = "Field `FIFOUDR1` writer - DAC_OUT1 FIFO underflow flag"]
pub type FIFOUDR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFONUM1` reader - Number of data in the DAC_OUT1 FIFO"]
pub type FIFONUM1_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - DAC_OUT0 FIFO full flag"]
    #[inline(always)]
    pub fn ff0(&self) -> FF0_R {
        FF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC_OUT0 FIFO empty flag"]
    #[inline(always)]
    pub fn fe0(&self) -> FE0_R {
        FE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC_OUT0 FIFO overflow flag"]
    #[inline(always)]
    pub fn fifoovr0(&self) -> FIFOOVR0_R {
        FIFOOVR0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC_OUT0 FIFO underflow flag"]
    #[inline(always)]
    pub fn fifoudr0(&self) -> FIFOUDR0_R {
        FIFOUDR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Number of data in the DAC_OUT0 FIFO"]
    #[inline(always)]
    pub fn fifonum0(&self) -> FIFONUM0_R {
        FIFONUM0_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 16 - DAC_OUT1 FIFO full flag"]
    #[inline(always)]
    pub fn ff1(&self) -> FF1_R {
        FF1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC_OUT1 FIFO empty flag"]
    #[inline(always)]
    pub fn fe1(&self) -> FE1_R {
        FE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC_OUT1 FIFO overflow flag"]
    #[inline(always)]
    pub fn fifoovr1(&self) -> FIFOOVR1_R {
        FIFOOVR1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC_OUT1 FIFO underflow flag"]
    #[inline(always)]
    pub fn fifoudr1(&self) -> FIFOUDR1_R {
        FIFOUDR1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Number of data in the DAC_OUT1 FIFO"]
    #[inline(always)]
    pub fn fifonum1(&self) -> FIFONUM1_R {
        FIFONUM1_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - DAC_OUT0 FIFO overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovr0(&mut self) -> FIFOOVR0_W<STAT1_SPEC, 2> {
        FIFOOVR0_W::new(self)
    }
    #[doc = "Bit 3 - DAC_OUT0 FIFO underflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn fifoudr0(&mut self) -> FIFOUDR0_W<STAT1_SPEC, 3> {
        FIFOUDR0_W::new(self)
    }
    #[doc = "Bit 18 - DAC_OUT1 FIFO overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovr1(&mut self) -> FIFOOVR1_W<STAT1_SPEC, 18> {
        FIFOOVR1_W::new(self)
    }
    #[doc = "Bit 19 - DAC_OUT1 FIFO underflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn fifoudr1(&mut self) -> FIFOUDR1_W<STAT1_SPEC, 19> {
        FIFOUDR1_W::new(self)
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
#[doc = "DAC Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT1_SPEC;
impl crate::RegisterSpec for STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat1::R`](R) reader structure"]
impl crate::Readable for STAT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat1::W`](W) writer structure"]
impl crate::Writable for STAT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT1 to value 0x02"]
impl crate::Resettable for STAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
