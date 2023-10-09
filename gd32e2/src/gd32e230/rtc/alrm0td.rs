#[doc = "Register `ALRM0TD` reader"]
pub type R = crate::R<ALRM0TD_SPEC>;
#[doc = "Register `ALRM0TD` writer"]
pub type W = crate::W<ALRM0TD_SPEC>;
#[doc = "Field `SCU` reader - Second units in BCD format."]
pub type SCU_R = crate::FieldReader;
#[doc = "Field `SCU` writer - Second units in BCD format."]
pub type SCU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SCT` reader - Second tens in BCD format."]
pub type SCT_R = crate::FieldReader;
#[doc = "Field `SCT` writer - Second tens in BCD format."]
pub type SCT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MSKS` reader - Alarm seconds mask"]
pub type MSKS_R = crate::BitReader;
#[doc = "Field `MSKS` writer - Alarm seconds mask"]
pub type MSKS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MNU` reader - Minute units in BCD format."]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format."]
pub type MNU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MNT` reader - Minute tens in BCD format."]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format."]
pub type MNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MSKM` reader - Alarm minutes mask"]
pub type MSKM_R = crate::BitReader;
#[doc = "Field `MSKM` writer - Alarm minutes mask"]
pub type MSKM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HRU` reader - Hour units in BCD format."]
pub type HRU_R = crate::FieldReader;
#[doc = "Field `HRU` writer - Hour units in BCD format."]
pub type HRU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `HRT` reader - Hour tens in BCD format."]
pub type HRT_R = crate::FieldReader;
#[doc = "Field `HRT` writer - Hour tens in BCD format."]
pub type HRT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSKH` reader - Alarm hours mask"]
pub type MSKH_R = crate::BitReader;
#[doc = "Field `MSKH` writer - Alarm hours mask"]
pub type MSKH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAYU` reader - Date units or day in BCD format."]
pub type DAYU_R = crate::FieldReader;
#[doc = "Field `DAYU` writer - Date units or day in BCD format."]
pub type DAYU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DAYT` reader - Date tens in BCD format."]
pub type DAYT_R = crate::FieldReader;
#[doc = "Field `DAYT` writer - Date tens in BCD format."]
pub type DAYT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DOWS` reader - Week day selection"]
pub type DOWS_R = crate::BitReader;
#[doc = "Field `DOWS` writer - Week day selection"]
pub type DOWS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSKD` reader - Alarm date mask"]
pub type MSKD_R = crate::BitReader;
#[doc = "Field `MSKD` writer - Alarm date mask"]
pub type MSKD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn scu(&self) -> SCU_R {
        SCU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm seconds mask"]
    #[inline(always)]
    pub fn msks(&self) -> MSKS_R {
        MSKS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes mask"]
    #[inline(always)]
    pub fn mskm(&self) -> MSKM_R {
        MSKM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    pub fn hru(&self) -> HRU_R {
        HRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    pub fn hrt(&self) -> HRT_R {
        HRT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm hours mask"]
    #[inline(always)]
    pub fn mskh(&self) -> MSKH_R {
        MSKH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format."]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format."]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn dows(&self) -> DOWS_R {
        DOWS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm date mask"]
    #[inline(always)]
    pub fn mskd(&self) -> MSKD_R {
        MSKD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn scu(&mut self) -> SCU_W<ALRM0TD_SPEC, 0> {
        SCU_W::new(self)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn sct(&mut self) -> SCT_W<ALRM0TD_SPEC, 4> {
        SCT_W::new(self)
    }
    #[doc = "Bit 7 - Alarm seconds mask"]
    #[inline(always)]
    #[must_use]
    pub fn msks(&mut self) -> MSKS_W<ALRM0TD_SPEC, 7> {
        MSKS_W::new(self)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MNU_W<ALRM0TD_SPEC, 8> {
        MNU_W::new(self)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MNT_W<ALRM0TD_SPEC, 12> {
        MNT_W::new(self)
    }
    #[doc = "Bit 15 - Alarm minutes mask"]
    #[inline(always)]
    #[must_use]
    pub fn mskm(&mut self) -> MSKM_W<ALRM0TD_SPEC, 15> {
        MSKM_W::new(self)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn hru(&mut self) -> HRU_W<ALRM0TD_SPEC, 16> {
        HRU_W::new(self)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn hrt(&mut self) -> HRT_W<ALRM0TD_SPEC, 20> {
        HRT_W::new(self)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<ALRM0TD_SPEC, 22> {
        PM_W::new(self)
    }
    #[doc = "Bit 23 - Alarm hours mask"]
    #[inline(always)]
    #[must_use]
    pub fn mskh(&mut self) -> MSKH_W<ALRM0TD_SPEC, 23> {
        MSKH_W::new(self)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn dayu(&mut self) -> DAYU_W<ALRM0TD_SPEC, 24> {
        DAYU_W::new(self)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn dayt(&mut self) -> DAYT_W<ALRM0TD_SPEC, 28> {
        DAYT_W::new(self)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    #[must_use]
    pub fn dows(&mut self) -> DOWS_W<ALRM0TD_SPEC, 30> {
        DOWS_W::new(self)
    }
    #[doc = "Bit 31 - Alarm date mask"]
    #[inline(always)]
    #[must_use]
    pub fn mskd(&mut self) -> MSKD_W<ALRM0TD_SPEC, 31> {
        MSKD_W::new(self)
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
#[doc = "alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrm0td::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrm0td::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRM0TD_SPEC;
impl crate::RegisterSpec for ALRM0TD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrm0td::R`](R) reader structure"]
impl crate::Readable for ALRM0TD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alrm0td::W`](W) writer structure"]
impl crate::Writable for ALRM0TD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRM0TD to value 0"]
impl crate::Resettable for ALRM0TD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
