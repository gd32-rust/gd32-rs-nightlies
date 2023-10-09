#[doc = "Register `PTP_TSUL` reader"]
pub type R = crate::R<PTP_TSUL_SPEC>;
#[doc = "Register `PTP_TSUL` writer"]
pub type W = crate::W<PTP_TSUL_SPEC>;
#[doc = "Field `TMSUSS` reader - Time stamp update subseconds"]
pub type TMSUSS_R = crate::FieldReader<u32>;
#[doc = "Field `TMSUSS` writer - Time stamp update subseconds"]
pub type TMSUSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
#[doc = "Field `TMSUPNS` reader - Time stamp update positive or negative sign"]
pub type TMSUPNS_R = crate::BitReader;
#[doc = "Field `TMSUPNS` writer - Time stamp update positive or negative sign"]
pub type TMSUPNS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:30 - Time stamp update subseconds"]
    #[inline(always)]
    pub fn tmsuss(&self) -> TMSUSS_R {
        TMSUSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Time stamp update positive or negative sign"]
    #[inline(always)]
    pub fn tmsupns(&self) -> TMSUPNS_R {
        TMSUPNS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Time stamp update subseconds"]
    #[inline(always)]
    #[must_use]
    pub fn tmsuss(&mut self) -> TMSUSS_W<PTP_TSUL_SPEC, 0> {
        TMSUSS_W::new(self)
    }
    #[doc = "Bit 31 - Time stamp update positive or negative sign"]
    #[inline(always)]
    #[must_use]
    pub fn tmsupns(&mut self) -> TMSUPNS_W<PTP_TSUL_SPEC, 31> {
        TMSUPNS_W::new(self)
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
#[doc = "Ethernet PTP time stamp low update register (PTP_TSUL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsul::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_tsul::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTP_TSUL_SPEC;
impl crate::RegisterSpec for PTP_TSUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_tsul::R`](R) reader structure"]
impl crate::Readable for PTP_TSUL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptp_tsul::W`](W) writer structure"]
impl crate::Writable for PTP_TSUL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTP_TSUL to value 0"]
impl crate::Resettable for PTP_TSUL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
