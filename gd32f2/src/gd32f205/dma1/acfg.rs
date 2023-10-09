#[doc = "Register `ACFG` reader"]
pub type R = crate::R<ACFG_SPEC>;
#[doc = "Register `ACFG` writer"]
pub type W = crate::W<ACFG_SPEC>;
#[doc = "Field `FD_CH5EN` reader - Enable bit for channel 5 Full_Data transfer mode"]
pub type FD_CH5EN_R = crate::BitReader;
#[doc = "Field `FD_CH5EN` writer - Enable bit for channel 5 Full_Data transfer mode"]
pub type FD_CH5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 5 - Enable bit for channel 5 Full_Data transfer mode"]
    #[inline(always)]
    pub fn fd_ch5en(&self) -> FD_CH5EN_R {
        FD_CH5EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Enable bit for channel 5 Full_Data transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn fd_ch5en(&mut self) -> FD_CH5EN_W<ACFG_SPEC, 5> {
        FD_CH5EN_W::new(self)
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
#[doc = "DMA additional configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACFG_SPEC;
impl crate::RegisterSpec for ACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acfg::R`](R) reader structure"]
impl crate::Readable for ACFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acfg::W`](W) writer structure"]
impl crate::Writable for ACFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACFG to value 0"]
impl crate::Resettable for ACFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
