#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `TSEG` reader - Time-stamp event active edge"]
pub type TSEG_R = crate::BitReader;
#[doc = "Field `TSEG` writer - Time-stamp event active edge"]
pub type TSEG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REFEN` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type REFEN_R = crate::BitReader;
#[doc = "Field `REFEN` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type REFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BPSHAD` reader - Bypass the shadow registers"]
pub type BPSHAD_R = crate::BitReader;
#[doc = "Field `BPSHAD` writer - Bypass the shadow registers"]
pub type BPSHAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS` reader - Hour format"]
pub type CS_R = crate::BitReader;
#[doc = "Field `CS` writer - Hour format"]
pub type CS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALRM0EN` reader - Alarm A enable"]
pub type ALRM0EN_R = crate::BitReader;
#[doc = "Field `ALRM0EN` writer - Alarm A enable"]
pub type ALRM0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSEN` reader - timestamp enable"]
pub type TSEN_R = crate::BitReader;
#[doc = "Field `TSEN` writer - timestamp enable"]
pub type TSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALRM0IE` reader - Alarm A interrupt enable"]
pub type ALRM0IE_R = crate::BitReader;
#[doc = "Field `ALRM0IE` writer - Alarm A interrupt enable"]
pub type ALRM0IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSIE` reader - Time-stamp interrupt enable"]
pub type TSIE_R = crate::BitReader;
#[doc = "Field `TSIE` writer - Time-stamp interrupt enable"]
pub type TSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `A1H` writer - Add 1 hour (summer time change)"]
pub type A1H_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S1H` writer - Subtract 1 hour (winter time change)"]
pub type S1H_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSM` reader - Backup"]
pub type DSM_R = crate::BitReader;
#[doc = "Field `DSM` writer - Backup"]
pub type DSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COS` reader - Calibration output selection"]
pub type COS_R = crate::BitReader;
#[doc = "Field `COS` writer - Calibration output selection"]
pub type COS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPOL` reader - Output polarity"]
pub type OPOL_R = crate::BitReader;
#[doc = "Field `OPOL` writer - Output polarity"]
pub type OPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OS` reader - Output selection"]
pub type OS_R = crate::FieldReader;
#[doc = "Field `OS` writer - Output selection"]
pub type OS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `COEN` reader - Calibration output enable"]
pub type COEN_R = crate::BitReader;
#[doc = "Field `COEN` writer - Calibration output enable"]
pub type COEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    pub fn tseg(&self) -> TSEG_R {
        TSEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refen(&self) -> REFEN_R {
        REFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bpshad(&self) -> BPSHAD_R {
        BPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrm0en(&self) -> ALRM0EN_R {
        ALRM0EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alrm0ie(&self) -> ALRM0IE_R {
        ALRM0IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn dsm(&self) -> DSM_R {
        DSM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn cos(&self) -> COS_R {
        COS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn opol(&self) -> OPOL_R {
        OPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn os(&self) -> OS_R {
        OS_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coen(&self) -> COEN_R {
        COEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    #[must_use]
    pub fn tseg(&mut self) -> TSEG_W<CTL_SPEC, 3> {
        TSEG_W::new(self)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    #[must_use]
    pub fn refen(&mut self) -> REFEN_W<CTL_SPEC, 4> {
        REFEN_W::new(self)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn bpshad(&mut self) -> BPSHAD_W<CTL_SPEC, 5> {
        BPSHAD_W::new(self)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<CTL_SPEC, 6> {
        CS_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrm0en(&mut self) -> ALRM0EN_W<CTL_SPEC, 8> {
        ALRM0EN_W::new(self)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CTL_SPEC, 11> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrm0ie(&mut self) -> ALRM0IE_W<CTL_SPEC, 12> {
        ALRM0IE_W::new(self)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<CTL_SPEC, 15> {
        TSIE_W::new(self)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    #[must_use]
    pub fn a1h(&mut self) -> A1H_W<CTL_SPEC, 16> {
        A1H_W::new(self)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    #[must_use]
    pub fn s1h(&mut self) -> S1H_W<CTL_SPEC, 17> {
        S1H_W::new(self)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    #[must_use]
    pub fn dsm(&mut self) -> DSM_W<CTL_SPEC, 18> {
        DSM_W::new(self)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cos(&mut self) -> COS_W<CTL_SPEC, 19> {
        COS_W::new(self)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn opol(&mut self) -> OPOL_W<CTL_SPEC, 20> {
        OPOL_W::new(self)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    #[must_use]
    pub fn os(&mut self) -> OS_W<CTL_SPEC, 21> {
        OS_W::new(self)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn coen(&mut self) -> COEN_W<CTL_SPEC, 23> {
        COEN_W::new(self)
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
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
