#[doc = "Register `PTP_PPSCTL` reader"]
pub type R = crate::R<PTP_PPSCTL_SPEC>;
#[doc = "Register `PTP_PPSCTL` writer"]
pub type W = crate::W<PTP_PPSCTL_SPEC>;
#[doc = "Field `PPSOFC` reader - PPS output frequency configure"]
pub type PPSOFC_R = crate::FieldReader;
#[doc = "Field `PPSOFC` writer - PPS output frequency configure"]
pub type PPSOFC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PPS output frequency configure"]
    #[inline(always)]
    pub fn ppsofc(&self) -> PPSOFC_R {
        PPSOFC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PPS output frequency configure"]
    #[inline(always)]
    #[must_use]
    pub fn ppsofc(&mut self) -> PPSOFC_W<PTP_PPSCTL_SPEC, 0> {
        PPSOFC_W::new(self)
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
#[doc = "Ethernet PTP PPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_ppsctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_ppsctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTP_PPSCTL_SPEC;
impl crate::RegisterSpec for PTP_PPSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_ppsctl::R`](R) reader structure"]
impl crate::Readable for PTP_PPSCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptp_ppsctl::W`](W) writer structure"]
impl crate::Writable for PTP_PPSCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTP_PPSCTL to value 0"]
impl crate::Resettable for PTP_PPSCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
