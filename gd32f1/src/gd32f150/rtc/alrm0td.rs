#[doc = "Register `ALRM0TD` reader"]
pub struct R(crate::R<ALRM0TD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRM0TD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRM0TD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRM0TD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALRM0TD` writer"]
pub struct W(crate::W<ALRM0TD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRM0TD_SPEC>;
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
impl From<crate::W<ALRM0TD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRM0TD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSKD` reader - Alarm date mask bit"]
pub type MSKD_R = crate::BitReader<bool>;
#[doc = "Field `MSKD` writer - Alarm date mask bit"]
pub type MSKD_W<'a> = crate::BitWriter<'a, u32, ALRM0TD_SPEC, bool, 31>;
#[doc = "Field `DOWS` reader - Day of the week selected"]
pub type DOWS_R = crate::BitReader<bool>;
#[doc = "Field `DOWS` writer - Day of the week selected"]
pub type DOWS_W<'a> = crate::BitWriter<'a, u32, ALRM0TD_SPEC, bool, 30>;
#[doc = "Field `DAYT` reader - Date tens in BCD code"]
pub type DAYT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYT` writer - Date tens in BCD code"]
pub type DAYT_W<'a> = crate::FieldWriter<'a, u32, ALRM0TD_SPEC, u8, u8, 2, 28>;
#[doc = "Field `DAYU` reader - Date units or week day in BCD code"]
pub type DAYU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYU` writer - Date units or week day in BCD code"]
pub type DAYU_W<'a> = crate::FieldWriter<'a, u32, ALRM0TD_SPEC, u8, u8, 4, 24>;
#[doc = "Field `MSKH` reader - Alarm hour mask bit"]
pub type MSKH_R = crate::BitReader<bool>;
#[doc = "Field `MSKH` writer - Alarm hour mask bit"]
pub type MSKH_W<'a> = crate::BitWriter<'a, u32, ALRM0TD_SPEC, bool, 23>;
#[doc = "Field `PM` reader - AM/PM flag"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - AM/PM flag"]
pub type PM_W<'a> = crate::BitWriter<'a, u32, ALRM0TD_SPEC, bool, 22>;
#[doc = "Field `HRT` reader - Hour tens in BCD code"]
pub type HRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HRT` writer - Hour tens in BCD code"]
pub type HRT_W<'a> = crate::FieldWriter<'a, u32, ALRM0TD_SPEC, u8, u8, 2, 20>;
#[doc = "Field `HRU` reader - Hour units in BCD code"]
pub type HRU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HRU` writer - Hour units in BCD code"]
pub type HRU_W<'a> = crate::FieldWriter<'a, u32, ALRM0TD_SPEC, u8, u8, 4, 16>;
#[doc = "Field `MSKM` reader - Alarm minutes mask bit"]
pub type MSKM_R = crate::BitReader<bool>;
#[doc = "Field `MSKM` writer - Alarm minutes mask bit"]
pub type MSKM_W<'a> = crate::BitWriter<'a, u32, ALRM0TD_SPEC, bool, 15>;
#[doc = "Field `MNT` reader - Minutes tens in BCD code"]
pub type MNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNT` writer - Minutes tens in BCD code"]
pub type MNT_W<'a> = crate::FieldWriter<'a, u32, ALRM0TD_SPEC, u8, u8, 3, 12>;
#[doc = "Field `MNU` reader - Minutes units in BCD code"]
pub type MNU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNU` writer - Minutes units in BCD code"]
pub type MNU_W<'a> = crate::FieldWriter<'a, u32, ALRM0TD_SPEC, u8, u8, 4, 8>;
#[doc = "Field `MSKS` reader - Alarm second mask bit"]
pub type MSKS_R = crate::BitReader<bool>;
#[doc = "Field `MSKS` writer - Alarm second mask bit"]
pub type MSKS_W<'a> = crate::BitWriter<'a, u32, ALRM0TD_SPEC, bool, 7>;
#[doc = "Field `SCT` reader - Second tens in BCD code"]
pub type SCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCT` writer - Second tens in BCD code"]
pub type SCT_W<'a> = crate::FieldWriter<'a, u32, ALRM0TD_SPEC, u8, u8, 3, 4>;
#[doc = "Field `SCU` reader - Second units in BCD code"]
pub type SCU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCU` writer - Second units in BCD code"]
pub type SCU_W<'a> = crate::FieldWriter<'a, u32, ALRM0TD_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bit 31 - Alarm date mask bit"]
    #[inline(always)]
    pub fn mskd(&self) -> MSKD_R {
        MSKD_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Day of the week selected"]
    #[inline(always)]
    pub fn dows(&self) -> DOWS_R {
        DOWS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Date tens in BCD code"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Date units or week day in BCD code"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Alarm hour mask bit"]
    #[inline(always)]
    pub fn mskh(&self) -> MSKH_R {
        MSKH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - AM/PM flag"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD code"]
    #[inline(always)]
    pub fn hrt(&self) -> HRT_R {
        HRT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD code"]
    #[inline(always)]
    pub fn hru(&self) -> HRU_R {
        HRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes mask bit"]
    #[inline(always)]
    pub fn mskm(&self) -> MSKM_R {
        MSKM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Minutes tens in BCD code"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minutes units in BCD code"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Alarm second mask bit"]
    #[inline(always)]
    pub fn msks(&self) -> MSKS_R {
        MSKS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD code"]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:3 - Second units in BCD code"]
    #[inline(always)]
    pub fn scu(&self) -> SCU_R {
        SCU_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Alarm date mask bit"]
    #[inline(always)]
    pub fn mskd(&mut self) -> MSKD_W {
        MSKD_W::new(self)
    }
    #[doc = "Bit 30 - Day of the week selected"]
    #[inline(always)]
    pub fn dows(&mut self) -> DOWS_W {
        DOWS_W::new(self)
    }
    #[doc = "Bits 28:29 - Date tens in BCD code"]
    #[inline(always)]
    pub fn dayt(&mut self) -> DAYT_W {
        DAYT_W::new(self)
    }
    #[doc = "Bits 24:27 - Date units or week day in BCD code"]
    #[inline(always)]
    pub fn dayu(&mut self) -> DAYU_W {
        DAYU_W::new(self)
    }
    #[doc = "Bit 23 - Alarm hour mask bit"]
    #[inline(always)]
    pub fn mskh(&mut self) -> MSKH_W {
        MSKH_W::new(self)
    }
    #[doc = "Bit 22 - AM/PM flag"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W::new(self)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD code"]
    #[inline(always)]
    pub fn hrt(&mut self) -> HRT_W {
        HRT_W::new(self)
    }
    #[doc = "Bits 16:19 - Hour units in BCD code"]
    #[inline(always)]
    pub fn hru(&mut self) -> HRU_W {
        HRU_W::new(self)
    }
    #[doc = "Bit 15 - Alarm minutes mask bit"]
    #[inline(always)]
    pub fn mskm(&mut self) -> MSKM_W {
        MSKM_W::new(self)
    }
    #[doc = "Bits 12:14 - Minutes tens in BCD code"]
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W {
        MNT_W::new(self)
    }
    #[doc = "Bits 8:11 - Minutes units in BCD code"]
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W {
        MNU_W::new(self)
    }
    #[doc = "Bit 7 - Alarm second mask bit"]
    #[inline(always)]
    pub fn msks(&mut self) -> MSKS_W {
        MSKS_W::new(self)
    }
    #[doc = "Bits 4:6 - Second tens in BCD code"]
    #[inline(always)]
    pub fn sct(&mut self) -> SCT_W {
        SCT_W::new(self)
    }
    #[doc = "Bits 0:3 - Second units in BCD code"]
    #[inline(always)]
    pub fn scu(&mut self) -> SCU_W {
        SCU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm 0 Time and date register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrm0td](index.html) module"]
pub struct ALRM0TD_SPEC;
impl crate::RegisterSpec for ALRM0TD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alrm0td::R](R) reader structure"]
impl crate::Readable for ALRM0TD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alrm0td::W](W) writer structure"]
impl crate::Writable for ALRM0TD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALRM0TD to value 0"]
impl crate::Resettable for ALRM0TD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
