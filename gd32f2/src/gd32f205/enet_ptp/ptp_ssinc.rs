#[doc = "Register `PTP_SSINC` reader"]
pub type R = crate::R<PTP_SSINC_SPEC>;
#[doc = "Register `PTP_SSINC` writer"]
pub type W = crate::W<PTP_SSINC_SPEC>;
#[doc = "Field `STMSSI` reader - System time subsecond increment"]
pub type STMSSI_R = crate::FieldReader;
#[doc = "Field `STMSSI` writer - System time subsecond increment"]
pub type STMSSI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - System time subsecond increment"]
    #[inline(always)]
    pub fn stmssi(&self) -> STMSSI_R {
        STMSSI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System time subsecond increment"]
    #[inline(always)]
    #[must_use]
    pub fn stmssi(&mut self) -> STMSSI_W<PTP_SSINC_SPEC, 0> {
        STMSSI_W::new(self)
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
#[doc = "Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_ssinc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_ssinc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTP_SSINC_SPEC;
impl crate::RegisterSpec for PTP_SSINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_ssinc::R`](R) reader structure"]
impl crate::Readable for PTP_SSINC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptp_ssinc::W`](W) writer structure"]
impl crate::Writable for PTP_SSINC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTP_SSINC to value 0"]
impl crate::Resettable for PTP_SSINC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
