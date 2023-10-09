#[doc = "Register `GLPMCFG` reader"]
pub type R = crate::R<GLPMCFG_SPEC>;
#[doc = "Register `GLPMCFG` writer"]
pub type W = crate::W<GLPMCFG_SPEC>;
#[doc = "Field `LPMEN` reader - LPM enable"]
pub type LPMEN_R = crate::BitReader;
#[doc = "Field `LPMEN` writer - LPM enable"]
pub type LPMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACKLPM` reader - ACK in LPM transaction enable"]
pub type ACKLPM_R = crate::BitReader;
#[doc = "Field `ACKLPM` writer - ACK in LPM transaction enable"]
pub type ACKLPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BESL` reader - Best effort service latency"]
pub type BESL_R = crate::FieldReader;
#[doc = "Field `BESL` writer - Best effort service latency"]
pub type BESL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `REW` reader - RemoteWake value"]
pub type REW_R = crate::BitReader;
#[doc = "Field `REW` writer - RemoteWake value"]
pub type REW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSEN` reader - Shallow sleep enable"]
pub type SSEN_R = crate::BitReader;
#[doc = "Field `SSEN` writer - Shallow sleep enable"]
pub type SSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BESLTH` reader - BESL threshold"]
pub type BESLTH_R = crate::FieldReader;
#[doc = "Field `BESLTH` writer - BESL threshold"]
pub type BESLTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DSEN` reader - Deep sleep enable"]
pub type DSEN_R = crate::BitReader;
#[doc = "Field `DSEN` writer - Deep sleep enable"]
pub type DSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPMRSP` reader - Response of LPM"]
pub type LPMRSP_R = crate::FieldReader;
#[doc = "Field `LPMRSP` writer - Response of LPM"]
pub type LPMRSP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPMSLPS` reader - Sleep status"]
pub type LPMSLPS_R = crate::BitReader;
#[doc = "Field `LPMSLPS` writer - Sleep status"]
pub type LPMSLPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSOK` reader - Resume can be sent after sleep state"]
pub type RSOK_R = crate::BitReader;
#[doc = "Field `RSOK` writer - Resume can be sent after sleep state"]
pub type RSOK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPMCHI` reader - Channel number index when send LPM transaction"]
pub type LPMCHI_R = crate::FieldReader;
#[doc = "Field `LPMCHI` writer - Channel number index when send LPM transaction"]
pub type LPMCHI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `LPMRC` reader - LPM retry count"]
pub type LPMRC_R = crate::FieldReader;
#[doc = "Field `LPMRC` writer - LPM retry count"]
pub type LPMRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `LPMSND` reader - Send LPM transaction"]
pub type LPMSND_R = crate::BitReader;
#[doc = "Field `LPMSND` writer - Send LPM transaction"]
pub type LPMSND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPMRCS` reader - LPM retry count status"]
pub type LPMRCS_R = crate::FieldReader;
#[doc = "Field `LPMRCS` writer - LPM retry count status"]
pub type LPMRCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BESLEN` reader - LPM Errata selection enable"]
pub type BESLEN_R = crate::BitReader;
#[doc = "Field `BESLEN` writer - LPM Errata selection enable"]
pub type BESLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - LPM enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACK in LPM transaction enable"]
    #[inline(always)]
    pub fn acklpm(&self) -> ACKLPM_R {
        ACKLPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Best effort service latency"]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - RemoteWake value"]
    #[inline(always)]
    pub fn rew(&self) -> REW_R {
        REW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Shallow sleep enable"]
    #[inline(always)]
    pub fn ssen(&self) -> SSEN_R {
        SSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - BESL threshold"]
    #[inline(always)]
    pub fn beslth(&self) -> BESLTH_R {
        BESLTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Deep sleep enable"]
    #[inline(always)]
    pub fn dsen(&self) -> DSEN_R {
        DSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Response of LPM"]
    #[inline(always)]
    pub fn lpmrsp(&self) -> LPMRSP_R {
        LPMRSP_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Sleep status"]
    #[inline(always)]
    pub fn lpmslps(&self) -> LPMSLPS_R {
        LPMSLPS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Resume can be sent after sleep state"]
    #[inline(always)]
    pub fn rsok(&self) -> RSOK_R {
        RSOK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - Channel number index when send LPM transaction"]
    #[inline(always)]
    pub fn lpmchi(&self) -> LPMCHI_R {
        LPMCHI_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:23 - LPM retry count"]
    #[inline(always)]
    pub fn lpmrc(&self) -> LPMRC_R {
        LPMRC_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Send LPM transaction"]
    #[inline(always)]
    pub fn lpmsnd(&self) -> LPMSND_R {
        LPMSND_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - LPM retry count status"]
    #[inline(always)]
    pub fn lpmrcs(&self) -> LPMRCS_R {
        LPMRCS_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - LPM Errata selection enable"]
    #[inline(always)]
    pub fn beslen(&self) -> BESLEN_R {
        BESLEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LPMEN_W<GLPMCFG_SPEC, 0> {
        LPMEN_W::new(self)
    }
    #[doc = "Bit 1 - ACK in LPM transaction enable"]
    #[inline(always)]
    #[must_use]
    pub fn acklpm(&mut self) -> ACKLPM_W<GLPMCFG_SPEC, 1> {
        ACKLPM_W::new(self)
    }
    #[doc = "Bits 2:5 - Best effort service latency"]
    #[inline(always)]
    #[must_use]
    pub fn besl(&mut self) -> BESL_W<GLPMCFG_SPEC, 2> {
        BESL_W::new(self)
    }
    #[doc = "Bit 6 - RemoteWake value"]
    #[inline(always)]
    #[must_use]
    pub fn rew(&mut self) -> REW_W<GLPMCFG_SPEC, 6> {
        REW_W::new(self)
    }
    #[doc = "Bit 7 - Shallow sleep enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssen(&mut self) -> SSEN_W<GLPMCFG_SPEC, 7> {
        SSEN_W::new(self)
    }
    #[doc = "Bits 8:11 - BESL threshold"]
    #[inline(always)]
    #[must_use]
    pub fn beslth(&mut self) -> BESLTH_W<GLPMCFG_SPEC, 8> {
        BESLTH_W::new(self)
    }
    #[doc = "Bit 12 - Deep sleep enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsen(&mut self) -> DSEN_W<GLPMCFG_SPEC, 12> {
        DSEN_W::new(self)
    }
    #[doc = "Bits 13:14 - Response of LPM"]
    #[inline(always)]
    #[must_use]
    pub fn lpmrsp(&mut self) -> LPMRSP_W<GLPMCFG_SPEC, 13> {
        LPMRSP_W::new(self)
    }
    #[doc = "Bit 15 - Sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn lpmslps(&mut self) -> LPMSLPS_W<GLPMCFG_SPEC, 15> {
        LPMSLPS_W::new(self)
    }
    #[doc = "Bit 16 - Resume can be sent after sleep state"]
    #[inline(always)]
    #[must_use]
    pub fn rsok(&mut self) -> RSOK_W<GLPMCFG_SPEC, 16> {
        RSOK_W::new(self)
    }
    #[doc = "Bits 17:20 - Channel number index when send LPM transaction"]
    #[inline(always)]
    #[must_use]
    pub fn lpmchi(&mut self) -> LPMCHI_W<GLPMCFG_SPEC, 17> {
        LPMCHI_W::new(self)
    }
    #[doc = "Bits 21:23 - LPM retry count"]
    #[inline(always)]
    #[must_use]
    pub fn lpmrc(&mut self) -> LPMRC_W<GLPMCFG_SPEC, 21> {
        LPMRC_W::new(self)
    }
    #[doc = "Bit 24 - Send LPM transaction"]
    #[inline(always)]
    #[must_use]
    pub fn lpmsnd(&mut self) -> LPMSND_W<GLPMCFG_SPEC, 24> {
        LPMSND_W::new(self)
    }
    #[doc = "Bits 25:27 - LPM retry count status"]
    #[inline(always)]
    #[must_use]
    pub fn lpmrcs(&mut self) -> LPMRCS_W<GLPMCFG_SPEC, 25> {
        LPMRCS_W::new(self)
    }
    #[doc = "Bit 28 - LPM Errata selection enable"]
    #[inline(always)]
    #[must_use]
    pub fn beslen(&mut self) -> BESLEN_W<GLPMCFG_SPEC, 28> {
        BESLEN_W::new(self)
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
#[doc = "Global core LPM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glpmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glpmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLPMCFG_SPEC;
impl crate::RegisterSpec for GLPMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glpmcfg::R`](R) reader structure"]
impl crate::Readable for GLPMCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`glpmcfg::W`](W) writer structure"]
impl crate::Writable for GLPMCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLPMCFG to value 0"]
impl crate::Resettable for GLPMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
