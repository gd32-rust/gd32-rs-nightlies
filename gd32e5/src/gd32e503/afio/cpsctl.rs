#[doc = "Register `CPSCTL` reader"]
pub type R = crate::R<CPSCTL_SPEC>;
#[doc = "Register `CPSCTL` writer"]
pub type W = crate::W<CPSCTL_SPEC>;
#[doc = "Field `CPS_EN` reader - I/O compensation cell enable"]
pub type CPS_EN_R = crate::BitReader;
#[doc = "Field `CPS_EN` writer - I/O compensation cell enable"]
pub type CPS_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPS_RDY` reader - I/O compensation cell is really or not"]
pub type CPS_RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I/O compensation cell enable"]
    #[inline(always)]
    pub fn cps_en(&self) -> CPS_EN_R {
        CPS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - I/O compensation cell is really or not"]
    #[inline(always)]
    pub fn cps_rdy(&self) -> CPS_RDY_R {
        CPS_RDY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O compensation cell enable"]
    #[inline(always)]
    #[must_use]
    pub fn cps_en(&mut self) -> CPS_EN_W<CPSCTL_SPEC, 0> {
        CPS_EN_W::new(self)
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
#[doc = "IO compensation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPSCTL_SPEC;
impl crate::RegisterSpec for CPSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsctl::R`](R) reader structure"]
impl crate::Readable for CPSCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpsctl::W`](W) writer structure"]
impl crate::Writable for CPSCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPSCTL to value 0"]
impl crate::Resettable for CPSCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
