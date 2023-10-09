#[doc = "Register `DMAUPMTR` reader"]
pub type R = crate::R<DMAUPMTR_SPEC>;
#[doc = "Register `DMAUPMTR` writer"]
pub type W = crate::W<DMAUPMTR_SPEC>;
#[doc = "Field `MTCTL0` reader - SHRTIMER_MTCTL0 update by DMA mode"]
pub type MTCTL0_R = crate::BitReader;
#[doc = "Field `MTCTL0` writer - SHRTIMER_MTCTL0 update by DMA mode"]
pub type MTCTL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTINTC` reader - SHRTIMER_MTINTC update by DMA mode"]
pub type MTINTC_R = crate::BitReader;
#[doc = "Field `MTINTC` writer - SHRTIMER_MTINTC update by DMA mode"]
pub type MTINTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTDMAINTEN` reader - SHRTIMER_MTDMAINTEN update by DMA mode"]
pub type MTDMAINTEN_R = crate::BitReader;
#[doc = "Field `MTDMAINTEN` writer - SHRTIMER_MTDMAINTEN update by DMA mode"]
pub type MTDMAINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCNT` reader - SHRTIMER_MTCNT update by DMA mode"]
pub type MTCNT_R = crate::BitReader;
#[doc = "Field `MTCNT` writer - SHRTIMER_MTCNT update by DMA mode"]
pub type MTCNT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCAR` reader - SHRTIMER_MTCAR update by DMA mode"]
pub type MTCAR_R = crate::BitReader;
#[doc = "Field `MTCAR` writer - SHRTIMER_MTCAR update by DMA mode"]
pub type MTCAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCREP` reader - SHRTIMER_MTCAR update by DMA mode"]
pub type MTCREP_R = crate::BitReader;
#[doc = "Field `MTCREP` writer - SHRTIMER_MTCAR update by DMA mode"]
pub type MTCREP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP0V` reader - SHRTIMER_MTCMP0V update by DMA mode"]
pub type MTCMP0V_R = crate::BitReader;
#[doc = "Field `MTCMP0V` writer - SHRTIMER_MTCMP0V update by DMA mode"]
pub type MTCMP0V_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP1V` reader - SHRTIMER_MTCMP1V update by DMA mode"]
pub type MTCMP1V_R = crate::BitReader;
#[doc = "Field `MTCMP1V` writer - SHRTIMER_MTCMP1V update by DMA mode"]
pub type MTCMP1V_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP2V` reader - SHRTIMER_MTCMP2V update by DMA mode"]
pub type MTCMP2V_R = crate::BitReader;
#[doc = "Field `MTCMP2V` writer - SHRTIMER_MTCMP2V update by DMA mode"]
pub type MTCMP2V_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP3V` reader - SHRTIMER_MTCMP3V update by DMA mode"]
pub type MTCMP3V_R = crate::BitReader;
#[doc = "Field `MTCMP3V` writer - SHRTIMER_MTCMP3V update by DMA mode"]
pub type MTCMP3V_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTACTL` reader - SHRTIMER_MTACTL update by DMA mode"]
pub type MTACTL_R = crate::BitReader;
#[doc = "Field `MTACTL` writer - SHRTIMER_MTACTL update by DMA mode"]
pub type MTACTL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SHRTIMER_MTCTL0 update by DMA mode"]
    #[inline(always)]
    pub fn mtctl0(&self) -> MTCTL0_R {
        MTCTL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SHRTIMER_MTINTC update by DMA mode"]
    #[inline(always)]
    pub fn mtintc(&self) -> MTINTC_R {
        MTINTC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SHRTIMER_MTDMAINTEN update by DMA mode"]
    #[inline(always)]
    pub fn mtdmainten(&self) -> MTDMAINTEN_R {
        MTDMAINTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SHRTIMER_MTCNT update by DMA mode"]
    #[inline(always)]
    pub fn mtcnt(&self) -> MTCNT_R {
        MTCNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SHRTIMER_MTCAR update by DMA mode"]
    #[inline(always)]
    pub fn mtcar(&self) -> MTCAR_R {
        MTCAR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SHRTIMER_MTCAR update by DMA mode"]
    #[inline(always)]
    pub fn mtcrep(&self) -> MTCREP_R {
        MTCREP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SHRTIMER_MTCMP0V update by DMA mode"]
    #[inline(always)]
    pub fn mtcmp0v(&self) -> MTCMP0V_R {
        MTCMP0V_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SHRTIMER_MTCMP1V update by DMA mode"]
    #[inline(always)]
    pub fn mtcmp1v(&self) -> MTCMP1V_R {
        MTCMP1V_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SHRTIMER_MTCMP2V update by DMA mode"]
    #[inline(always)]
    pub fn mtcmp2v(&self) -> MTCMP2V_R {
        MTCMP2V_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SHRTIMER_MTCMP3V update by DMA mode"]
    #[inline(always)]
    pub fn mtcmp3v(&self) -> MTCMP3V_R {
        MTCMP3V_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - SHRTIMER_MTACTL update by DMA mode"]
    #[inline(always)]
    pub fn mtactl(&self) -> MTACTL_R {
        MTACTL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHRTIMER_MTCTL0 update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtctl0(&mut self) -> MTCTL0_W<DMAUPMTR_SPEC, 0> {
        MTCTL0_W::new(self)
    }
    #[doc = "Bit 1 - SHRTIMER_MTINTC update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtintc(&mut self) -> MTINTC_W<DMAUPMTR_SPEC, 1> {
        MTINTC_W::new(self)
    }
    #[doc = "Bit 2 - SHRTIMER_MTDMAINTEN update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtdmainten(&mut self) -> MTDMAINTEN_W<DMAUPMTR_SPEC, 2> {
        MTDMAINTEN_W::new(self)
    }
    #[doc = "Bit 3 - SHRTIMER_MTCNT update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtcnt(&mut self) -> MTCNT_W<DMAUPMTR_SPEC, 3> {
        MTCNT_W::new(self)
    }
    #[doc = "Bit 4 - SHRTIMER_MTCAR update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtcar(&mut self) -> MTCAR_W<DMAUPMTR_SPEC, 4> {
        MTCAR_W::new(self)
    }
    #[doc = "Bit 5 - SHRTIMER_MTCAR update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtcrep(&mut self) -> MTCREP_W<DMAUPMTR_SPEC, 5> {
        MTCREP_W::new(self)
    }
    #[doc = "Bit 6 - SHRTIMER_MTCMP0V update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp0v(&mut self) -> MTCMP0V_W<DMAUPMTR_SPEC, 6> {
        MTCMP0V_W::new(self)
    }
    #[doc = "Bit 7 - SHRTIMER_MTCMP1V update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp1v(&mut self) -> MTCMP1V_W<DMAUPMTR_SPEC, 7> {
        MTCMP1V_W::new(self)
    }
    #[doc = "Bit 8 - SHRTIMER_MTCMP2V update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp2v(&mut self) -> MTCMP2V_W<DMAUPMTR_SPEC, 8> {
        MTCMP2V_W::new(self)
    }
    #[doc = "Bit 9 - SHRTIMER_MTCMP3V update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp3v(&mut self) -> MTCMP3V_W<DMAUPMTR_SPEC, 9> {
        MTCMP3V_W::new(self)
    }
    #[doc = "Bit 31 - SHRTIMER_MTACTL update by DMA mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtactl(&mut self) -> MTACTL_W<DMAUPMTR_SPEC, 31> {
        MTACTL_W::new(self)
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
#[doc = "SHRTIMER DMA update Master_TIMER register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaupmtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaupmtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAUPMTR_SPEC;
impl crate::RegisterSpec for DMAUPMTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaupmtr::R`](R) reader structure"]
impl crate::Readable for DMAUPMTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaupmtr::W`](W) writer structure"]
impl crate::Writable for DMAUPMTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAUPMTR to value 0"]
impl crate::Resettable for DMAUPMTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
