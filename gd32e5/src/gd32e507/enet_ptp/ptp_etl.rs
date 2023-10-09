#[doc = "Register `PTP_ETL` reader"]
pub type R = crate::R<PTP_ETL_SPEC>;
#[doc = "Register `PTP_ETL` writer"]
pub type W = crate::W<PTP_ETL_SPEC>;
#[doc = "Field `ETSL` reader - Expected time stamp low"]
pub type ETSL_R = crate::FieldReader<u32>;
#[doc = "Field `ETSL` writer - Expected time stamp low"]
pub type ETSL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Expected time stamp low"]
    #[inline(always)]
    pub fn etsl(&self) -> ETSL_R {
        ETSL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Expected time stamp low"]
    #[inline(always)]
    #[must_use]
    pub fn etsl(&mut self) -> ETSL_W<PTP_ETL_SPEC, 0> {
        ETSL_W::new(self)
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
#[doc = "Ethernet PTP expected time low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_etl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_etl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTP_ETL_SPEC;
impl crate::RegisterSpec for PTP_ETL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_etl::R`](R) reader structure"]
impl crate::Readable for PTP_ETL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptp_etl::W`](W) writer structure"]
impl crate::Writable for PTP_ETL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTP_ETL to value 0"]
impl crate::Resettable for PTP_ETL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
