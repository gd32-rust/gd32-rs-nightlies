#[doc = "Register `INTC` reader"]
pub type R = crate::R<INTC_SPEC>;
#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Field `CCTCF` reader - Clear charge-transfer complete flag"]
pub type CCTCF_R = crate::BitReader;
#[doc = "Field `CCTCF` writer - Clear charge-transfer complete flag"]
pub type CCTCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMNERR` reader - Clear max cycle number error"]
pub type CMNERR_R = crate::BitReader;
#[doc = "Field `CMNERR` writer - Clear max cycle number error"]
pub type CMNERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Clear charge-transfer complete flag"]
    #[inline(always)]
    pub fn cctcf(&self) -> CCTCF_R {
        CCTCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear max cycle number error"]
    #[inline(always)]
    pub fn cmnerr(&self) -> CMNERR_R {
        CMNERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear charge-transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn cctcf(&mut self) -> CCTCF_W<INTC_SPEC, 0> {
        CCTCF_W::new(self)
    }
    #[doc = "Bit 1 - Clear max cycle number error"]
    #[inline(always)]
    #[must_use]
    pub fn cmnerr(&mut self) -> CMNERR_W<INTC_SPEC, 1> {
        CMNERR_W::new(self)
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
#[doc = "interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc::R`](R) reader structure"]
impl crate::Readable for INTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
