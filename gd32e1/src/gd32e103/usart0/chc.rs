#[doc = "Register `CHC` reader"]
pub type R = crate::R<CHC_SPEC>;
#[doc = "Register `CHC` writer"]
pub type W = crate::W<CHC_SPEC>;
#[doc = "Field `HCM` reader - Hardware flow control coherence mode"]
pub type HCM_R = crate::BitReader;
#[doc = "Field `HCM` writer - Hardware flow control coherence mode"]
pub type HCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPERR` writer - Early parity error flag"]
pub type EPERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Hardware flow control coherence mode"]
    #[inline(always)]
    pub fn hcm(&self) -> HCM_R {
        HCM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware flow control coherence mode"]
    #[inline(always)]
    #[must_use]
    pub fn hcm(&mut self) -> HCM_W<CHC_SPEC, 0> {
        HCM_W::new(self)
    }
    #[doc = "Bit 8 - Early parity error flag"]
    #[inline(always)]
    #[must_use]
    pub fn eperr(&mut self) -> EPERR_W<CHC_SPEC, 8> {
        EPERR_W::new(self)
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
#[doc = "Coherence control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHC_SPEC;
impl crate::RegisterSpec for CHC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chc::R`](R) reader structure"]
impl crate::Readable for CHC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chc::W`](W) writer structure"]
impl crate::Writable for CHC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHC to value 0"]
impl crate::Resettable for CHC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
