#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `ALRM0WF` reader - Alarm A write flag"]
pub type ALRM0WF_R = crate::BitReader;
#[doc = "Field `SOPF` reader - Shift operation pending"]
pub type SOPF_R = crate::BitReader;
#[doc = "Field `YCM` reader - Initialization status flag"]
pub type YCM_R = crate::BitReader;
#[doc = "Field `RSYNF` reader - Registers synchronization flag"]
pub type RSYNF_R = crate::BitReader;
#[doc = "Field `RSYNF` writer - Registers synchronization flag"]
pub type RSYNF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INITF` reader - Initialization flag"]
pub type INITF_R = crate::BitReader;
#[doc = "Field `INITM` reader - Initialization mode"]
pub type INITM_R = crate::BitReader;
#[doc = "Field `INITM` writer - Initialization mode"]
pub type INITM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALRM0F` reader - Alarm A flag"]
pub type ALRM0F_R = crate::BitReader;
#[doc = "Field `ALRM0F` writer - Alarm A flag"]
pub type ALRM0F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TP1F` reader - RTC_TAMP1 detection flag"]
pub type TP1F_R = crate::BitReader;
#[doc = "Field `TP1F` writer - RTC_TAMP1 detection flag"]
pub type TP1F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCPF` reader - Recalibration pending Flag"]
pub type SCPF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrm0wf(&self) -> ALRM0WF_R {
        ALRM0WF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn sopf(&self) -> SOPF_R {
        SOPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn ycm(&self) -> YCM_R {
        YCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsynf(&self) -> RSYNF_R {
        RSYNF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn initm(&self) -> INITM_R {
        INITM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alrm0f(&self) -> ALRM0F_R {
        ALRM0F_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC_TAMP1 detection flag"]
    #[inline(always)]
    pub fn tp1f(&self) -> TP1F_R {
        TP1F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn scpf(&self) -> SCPF_R {
        SCPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsynf(&mut self) -> RSYNF_W<STAT_SPEC, 5> {
        RSYNF_W::new(self)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    #[must_use]
    pub fn initm(&mut self) -> INITM_W<STAT_SPEC, 7> {
        INITM_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrm0f(&mut self) -> ALRM0F_W<STAT_SPEC, 8> {
        ALRM0F_W::new(self)
    }
    #[doc = "Bit 14 - RTC_TAMP1 detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tp1f(&mut self) -> TP1F_W<STAT_SPEC, 14> {
        TP1F_W::new(self)
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
#[doc = "initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0x07"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
