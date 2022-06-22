#[doc = "Register `PTP_TSUL` reader"]
pub struct R(crate::R<PTP_TSUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_TSUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_TSUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_TSUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTP_TSUL` writer"]
pub struct W(crate::W<PTP_TSUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTP_TSUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PTP_TSUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTP_TSUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMSUSS` reader - Time stamp update subseconds"]
pub type TMSUSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMSUSS` writer - Time stamp update subseconds"]
pub type TMSUSS_W<'a> = crate::FieldWriter<'a, u32, PTP_TSUL_SPEC, u32, u32, 31, 0>;
#[doc = "Field `TMSUPNS` reader - Time stamp update positive or negative sign"]
pub type TMSUPNS_R = crate::BitReader<bool>;
#[doc = "Field `TMSUPNS` writer - Time stamp update positive or negative sign"]
pub type TMSUPNS_W<'a> = crate::BitWriter<'a, u32, PTP_TSUL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:30 - Time stamp update subseconds"]
    #[inline(always)]
    pub fn tmsuss(&self) -> TMSUSS_R {
        TMSUSS_R::new((self.bits & 0x7fff_ffff) as u32)
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
    pub fn tmsuss(&mut self) -> TMSUSS_W {
        TMSUSS_W::new(self)
    }
    #[doc = "Bit 31 - Time stamp update positive or negative sign"]
    #[inline(always)]
    pub fn tmsupns(&mut self) -> TMSUPNS_W {
        TMSUPNS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP time stamp low update register (PTP_TSUL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_tsul](index.html) module"]
pub struct PTP_TSUL_SPEC;
impl crate::RegisterSpec for PTP_TSUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_tsul::R](R) reader structure"]
impl crate::Readable for PTP_TSUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptp_tsul::W](W) writer structure"]
impl crate::Writable for PTP_TSUL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTP_TSUL to value 0"]
impl crate::Resettable for PTP_TSUL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
