#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRM0WF` reader - Alarm 0 configuration can be write flag"]
pub type ALRM0WF_R = crate::BitReader<bool>;
#[doc = "Field `SOPF` reader - Shift function operation pending flag"]
pub type SOPF_R = crate::BitReader<bool>;
#[doc = "Field `SOPF` writer - Shift function operation pending flag"]
pub type SOPF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 3>;
#[doc = "Field `YCM` reader - Year configuration mark"]
pub type YCM_R = crate::BitReader<bool>;
#[doc = "Field `RSYNF` reader - Register synchronization flag"]
pub type RSYNF_R = crate::BitReader<bool>;
#[doc = "Field `RSYNF` writer - Register synchronization flag"]
pub type RSYNF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 5>;
#[doc = "Field `INITF` reader - Initialization state flag"]
pub type INITF_R = crate::BitReader<bool>;
#[doc = "Field `INITM` reader - enter initialization mode"]
pub type INITM_R = crate::BitReader<bool>;
#[doc = "Field `INITM` writer - enter initialization mode"]
pub type INITM_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 7>;
#[doc = "Field `ALRM0F` reader - Alarm-0 occurs flag"]
pub type ALRM0F_R = crate::BitReader<bool>;
#[doc = "Field `ALRM0F` writer - Alarm-0 occurs flag"]
pub type ALRM0F_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 8>;
#[doc = "Field `TSF` reader - Time-stamp flag"]
pub type TSF_R = crate::BitReader<bool>;
#[doc = "Field `TSF` writer - Time-stamp flag"]
pub type TSF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 11>;
#[doc = "Field `TSOVRF` reader - Time-stamp overflow flag"]
pub type TSOVRF_R = crate::BitReader<bool>;
#[doc = "Field `TSOVRF` writer - Time-stamp overflow flag"]
pub type TSOVRF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 12>;
#[doc = "Field `TP0F` reader - RTC_TAMP0 detected flag"]
pub type TP0F_R = crate::BitReader<bool>;
#[doc = "Field `TP0F` writer - RTC_TAMP0 detected flag"]
pub type TP0F_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 13>;
#[doc = "Field `TP1F` reader - RTC_TAMP1 detected flag"]
pub type TP1F_R = crate::BitReader<bool>;
#[doc = "Field `TP1F` writer - RTC_TAMP1 detected flag"]
pub type TP1F_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 14>;
#[doc = "Field `SCPF` reader - Smooth calibration pending flag"]
pub type SCPF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Alarm 0 configuration can be write flag"]
    #[inline(always)]
    pub fn alrm0wf(&self) -> ALRM0WF_R {
        ALRM0WF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Shift function operation pending flag"]
    #[inline(always)]
    pub fn sopf(&self) -> SOPF_R {
        SOPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Year configuration mark"]
    #[inline(always)]
    pub fn ycm(&self) -> YCM_R {
        YCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Register synchronization flag"]
    #[inline(always)]
    pub fn rsynf(&self) -> RSYNF_R {
        RSYNF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization state flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enter initialization mode"]
    #[inline(always)]
    pub fn initm(&self) -> INITM_R {
        INITM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm-0 occurs flag"]
    #[inline(always)]
    pub fn alrm0f(&self) -> ALRM0F_R {
        ALRM0F_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovrf(&self) -> TSOVRF_R {
        TSOVRF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RTC_TAMP0 detected flag"]
    #[inline(always)]
    pub fn tp0f(&self) -> TP0F_R {
        TP0F_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC_TAMP1 detected flag"]
    #[inline(always)]
    pub fn tp1f(&self) -> TP1F_R {
        TP1F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Smooth calibration pending flag"]
    #[inline(always)]
    pub fn scpf(&self) -> SCPF_R {
        SCPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shift function operation pending flag"]
    #[inline(always)]
    pub fn sopf(&mut self) -> SOPF_W {
        SOPF_W::new(self)
    }
    #[doc = "Bit 5 - Register synchronization flag"]
    #[inline(always)]
    pub fn rsynf(&mut self) -> RSYNF_W {
        RSYNF_W::new(self)
    }
    #[doc = "Bit 7 - enter initialization mode"]
    #[inline(always)]
    pub fn initm(&mut self) -> INITM_W {
        INITM_W::new(self)
    }
    #[doc = "Bit 8 - Alarm-0 occurs flag"]
    #[inline(always)]
    pub fn alrm0f(&mut self) -> ALRM0F_W {
        ALRM0F_W::new(self)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W::new(self)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovrf(&mut self) -> TSOVRF_W {
        TSOVRF_W::new(self)
    }
    #[doc = "Bit 13 - RTC_TAMP0 detected flag"]
    #[inline(always)]
    pub fn tp0f(&mut self) -> TP0F_W {
        TP0F_W::new(self)
    }
    #[doc = "Bit 14 - RTC_TAMP1 detected flag"]
    #[inline(always)]
    pub fn tp1f(&mut self) -> TP1F_W {
        TP1F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0x07"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
