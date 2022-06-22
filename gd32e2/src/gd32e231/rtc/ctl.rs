#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSM` reader - Backup"]
pub type DSM_R = crate::BitReader<bool>;
#[doc = "Field `DSM` writer - Backup"]
pub type DSM_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 18>;
#[doc = "Field `S1H` writer - Subtract 1 hour (winter time change)"]
pub type S1H_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 17>;
#[doc = "Field `A1H` writer - Add 1 hour (summer time change)"]
pub type A1H_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 16>;
#[doc = "Field `ALRM0IE` reader - Alarm A interrupt enable"]
pub type ALRM0IE_R = crate::BitReader<bool>;
#[doc = "Field `ALRM0IE` writer - Alarm A interrupt enable"]
pub type ALRM0IE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 12>;
#[doc = "Field `ALRM0EN` reader - Alarm A enable"]
pub type ALRM0EN_R = crate::BitReader<bool>;
#[doc = "Field `ALRM0EN` writer - Alarm A enable"]
pub type ALRM0EN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 8>;
#[doc = "Field `CS` reader - Hour format"]
pub type CS_R = crate::BitReader<bool>;
#[doc = "Field `CS` writer - Hour format"]
pub type CS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 6>;
#[doc = "Field `BPSHAD` reader - Bypass the shadow registers"]
pub type BPSHAD_R = crate::BitReader<bool>;
#[doc = "Field `BPSHAD` writer - Bypass the shadow registers"]
pub type BPSHAD_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 5>;
#[doc = "Field `REFEN` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type REFEN_R = crate::BitReader<bool>;
#[doc = "Field `REFEN` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type REFEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn dsm(&self) -> DSM_R {
        DSM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alrm0ie(&self) -> ALRM0IE_R {
        ALRM0IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrm0en(&self) -> ALRM0EN_R {
        ALRM0EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bpshad(&self) -> BPSHAD_R {
        BPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refen(&self) -> REFEN_R {
        REFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn dsm(&mut self) -> DSM_W {
        DSM_W::new(self)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub fn s1h(&mut self) -> S1H_W {
        S1H_W::new(self)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    pub fn a1h(&mut self) -> A1H_W {
        A1H_W::new(self)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alrm0ie(&mut self) -> ALRM0IE_W {
        ALRM0IE_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrm0en(&mut self) -> ALRM0EN_W {
        ALRM0EN_W::new(self)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W::new(self)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bpshad(&mut self) -> BPSHAD_W {
        BPSHAD_W::new(self)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refen(&mut self) -> REFEN_W {
        REFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
