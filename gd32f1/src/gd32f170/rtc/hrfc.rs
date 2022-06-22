#[doc = "Register `HRFC` reader"]
pub struct R(crate::R<HRFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRFC` writer"]
pub struct W(crate::W<HRFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRFC_SPEC>;
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
impl From<crate::W<HRFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQI` reader - Increase RTC frequency by 488.5ppm"]
pub type FREQI_R = crate::BitReader<bool>;
#[doc = "Field `FREQI` writer - Increase RTC frequency by 488.5ppm"]
pub type FREQI_W<'a> = crate::BitWriter<'a, u32, HRFC_SPEC, bool, 15>;
#[doc = "Field `CWND8` reader - Frequency compensation window 8 second selected"]
pub type CWND8_R = crate::BitReader<bool>;
#[doc = "Field `CWND8` writer - Frequency compensation window 8 second selected"]
pub type CWND8_W<'a> = crate::BitWriter<'a, u32, HRFC_SPEC, bool, 14>;
#[doc = "Field `CWND16` reader - Frequency compensation window 16 second selected"]
pub type CWND16_R = crate::BitReader<bool>;
#[doc = "Field `CWND16` writer - Frequency compensation window 16 second selected"]
pub type CWND16_W<'a> = crate::BitWriter<'a, u32, HRFC_SPEC, bool, 13>;
#[doc = "Field `CMSK` reader - Calibration mask number"]
pub type CMSK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMSK` writer - Calibration mask number"]
pub type CMSK_W<'a> = crate::FieldWriter<'a, u32, HRFC_SPEC, u16, u16, 9, 0>;
impl R {
    #[doc = "Bit 15 - Increase RTC frequency by 488.5ppm"]
    #[inline(always)]
    pub fn freqi(&self) -> FREQI_R {
        FREQI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Frequency compensation window 8 second selected"]
    #[inline(always)]
    pub fn cwnd8(&self) -> CWND8_R {
        CWND8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Frequency compensation window 16 second selected"]
    #[inline(always)]
    pub fn cwnd16(&self) -> CWND16_R {
        CWND16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 0:8 - Calibration mask number"]
    #[inline(always)]
    pub fn cmsk(&self) -> CMSK_R {
        CMSK_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Increase RTC frequency by 488.5ppm"]
    #[inline(always)]
    pub fn freqi(&mut self) -> FREQI_W {
        FREQI_W::new(self)
    }
    #[doc = "Bit 14 - Frequency compensation window 8 second selected"]
    #[inline(always)]
    pub fn cwnd8(&mut self) -> CWND8_W {
        CWND8_W::new(self)
    }
    #[doc = "Bit 13 - Frequency compensation window 16 second selected"]
    #[inline(always)]
    pub fn cwnd16(&mut self) -> CWND16_W {
        CWND16_W::new(self)
    }
    #[doc = "Bits 0:8 - Calibration mask number"]
    #[inline(always)]
    pub fn cmsk(&mut self) -> CMSK_W {
        CMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High resolution frequency compensation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrfc](index.html) module"]
pub struct HRFC_SPEC;
impl crate::RegisterSpec for HRFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrfc::R](R) reader structure"]
impl crate::Readable for HRFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrfc::W](W) writer structure"]
impl crate::Writable for HRFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HRFC to value 0"]
impl crate::Resettable for HRFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
