#[doc = "Register `PTP_TSADDEND` reader"]
pub type R = crate::R<PTP_TSADDEND_SPEC>;
#[doc = "Register `PTP_TSADDEND` writer"]
pub type W = crate::W<PTP_TSADDEND_SPEC>;
#[doc = "Field `TMSA` reader - Time stamp addend"]
pub type TMSA_R = crate::FieldReader<u32>;
#[doc = "Field `TMSA` writer - Time stamp addend"]
pub type TMSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Time stamp addend"]
    #[inline(always)]
    pub fn tmsa(&self) -> TMSA_R {
        TMSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp addend"]
    #[inline(always)]
    #[must_use]
    pub fn tmsa(&mut self) -> TMSA_W<PTP_TSADDEND_SPEC, 0> {
        TMSA_W::new(self)
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
#[doc = "Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsaddend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_tsaddend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTP_TSADDEND_SPEC;
impl crate::RegisterSpec for PTP_TSADDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_tsaddend::R`](R) reader structure"]
impl crate::Readable for PTP_TSADDEND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptp_tsaddend::W`](W) writer structure"]
impl crate::Writable for PTP_TSADDEND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTP_TSADDEND to value 0"]
impl crate::Resettable for PTP_TSADDEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
