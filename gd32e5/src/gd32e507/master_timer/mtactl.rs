#[doc = "Register `MTACTL` reader"]
pub type R = crate::R<MTACTL_SPEC>;
#[doc = "Register `MTACTL` writer"]
pub type W = crate::W<MTACTL_SPEC>;
#[doc = "Field `CNTCKDIV_3` reader - Counter clock division"]
pub type CNTCKDIV_3_R = crate::BitReader;
#[doc = "Field `CNTCKDIV_3` writer - Counter clock division"]
pub type CNTCKDIV_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - Counter clock division"]
    #[inline(always)]
    pub fn cntckdiv_3(&self) -> CNTCKDIV_3_R {
        CNTCKDIV_3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Counter clock division"]
    #[inline(always)]
    #[must_use]
    pub fn cntckdiv_3(&mut self) -> CNTCKDIV_3_W<MTACTL_SPEC, 3> {
        CNTCKDIV_3_W::new(self)
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
#[doc = "SHRTIMER Master_TIMER additional control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtactl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtactl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTACTL_SPEC;
impl crate::RegisterSpec for MTACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtactl::R`](R) reader structure"]
impl crate::Readable for MTACTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtactl::W`](W) writer structure"]
impl crate::Writable for MTACTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTACTL to value 0"]
impl crate::Resettable for MTACTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
