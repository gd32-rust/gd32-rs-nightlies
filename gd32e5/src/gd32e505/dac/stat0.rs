#[doc = "Register `STAT0` reader"]
pub type R = crate::R<STAT0_SPEC>;
#[doc = "Register `STAT0` writer"]
pub type W = crate::W<STAT0_SPEC>;
#[doc = "Field `DDUDR0` reader - DAC_OUT0 DMA underrun flag"]
pub type DDUDR0_R = crate::BitReader;
#[doc = "Field `DDUDR0` writer - DAC_OUT0 DMA underrun flag"]
pub type DDUDR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDUDR1` reader - DAC_OUT1 DMA underrun flag"]
pub type DDUDR1_R = crate::BitReader;
#[doc = "Field `DDUDR1` writer - DAC_OUT1 DMA underrun flag"]
pub type DDUDR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 13 - DAC_OUT0 DMA underrun flag"]
    #[inline(always)]
    pub fn ddudr0(&self) -> DDUDR0_R {
        DDUDR0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC_OUT1 DMA underrun flag"]
    #[inline(always)]
    pub fn ddudr1(&self) -> DDUDR1_R {
        DDUDR1_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC_OUT0 DMA underrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn ddudr0(&mut self) -> DDUDR0_W<STAT0_SPEC, 13> {
        DDUDR0_W::new(self)
    }
    #[doc = "Bit 29 - DAC_OUT1 DMA underrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn ddudr1(&mut self) -> DDUDR1_W<STAT0_SPEC, 29> {
        DDUDR1_W::new(self)
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
#[doc = "DAC Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for STAT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat0::W`](W) writer structure"]
impl crate::Writable for STAT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for STAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
