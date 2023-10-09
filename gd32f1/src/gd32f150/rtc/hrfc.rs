#[doc = "Register `HRFC` reader"]
pub type R = crate::R<HRFC_SPEC>;
#[doc = "Register `HRFC` writer"]
pub type W = crate::W<HRFC_SPEC>;
#[doc = "Field `CMSK` reader - Calibration mask number"]
pub type CMSK_R = crate::FieldReader<u16>;
#[doc = "Field `CMSK` writer - Calibration mask number"]
pub type CMSK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `CWND16` reader - Frequency compensation window 16 second selected"]
pub type CWND16_R = crate::BitReader;
#[doc = "Field `CWND16` writer - Frequency compensation window 16 second selected"]
pub type CWND16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CWND8` reader - Frequency compensation window 8 second selected"]
pub type CWND8_R = crate::BitReader;
#[doc = "Field `CWND8` writer - Frequency compensation window 8 second selected"]
pub type CWND8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FREQI` reader - Increase RTC frequency by 488.5ppm"]
pub type FREQI_R = crate::BitReader;
#[doc = "Field `FREQI` writer - Increase RTC frequency by 488.5ppm"]
pub type FREQI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:8 - Calibration mask number"]
    #[inline(always)]
    pub fn cmsk(&self) -> CMSK_R {
        CMSK_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Frequency compensation window 16 second selected"]
    #[inline(always)]
    pub fn cwnd16(&self) -> CWND16_R {
        CWND16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Frequency compensation window 8 second selected"]
    #[inline(always)]
    pub fn cwnd8(&self) -> CWND8_R {
        CWND8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase RTC frequency by 488.5ppm"]
    #[inline(always)]
    pub fn freqi(&self) -> FREQI_R {
        FREQI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration mask number"]
    #[inline(always)]
    #[must_use]
    pub fn cmsk(&mut self) -> CMSK_W<HRFC_SPEC, 0> {
        CMSK_W::new(self)
    }
    #[doc = "Bit 13 - Frequency compensation window 16 second selected"]
    #[inline(always)]
    #[must_use]
    pub fn cwnd16(&mut self) -> CWND16_W<HRFC_SPEC, 13> {
        CWND16_W::new(self)
    }
    #[doc = "Bit 14 - Frequency compensation window 8 second selected"]
    #[inline(always)]
    #[must_use]
    pub fn cwnd8(&mut self) -> CWND8_W<HRFC_SPEC, 14> {
        CWND8_W::new(self)
    }
    #[doc = "Bit 15 - Increase RTC frequency by 488.5ppm"]
    #[inline(always)]
    #[must_use]
    pub fn freqi(&mut self) -> FREQI_W<HRFC_SPEC, 15> {
        FREQI_W::new(self)
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
#[doc = "High resolution frequency compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRFC_SPEC;
impl crate::RegisterSpec for HRFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrfc::R`](R) reader structure"]
impl crate::Readable for HRFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hrfc::W`](W) writer structure"]
impl crate::Writable for HRFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRFC to value 0"]
impl crate::Resettable for HRFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
