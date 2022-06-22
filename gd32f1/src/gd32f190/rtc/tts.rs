#[doc = "Register `TTS` reader"]
pub struct R(crate::R<TTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PM` reader - AM/PM mark"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `HRT` reader - Hour tens in BCD code"]
pub type HRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HRU` reader - Hour units in BCD code"]
pub type HRU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNT` reader - Minute tens in BCD code"]
pub type MNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNU` reader - Minute units in BCD code"]
pub type MNU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCT` reader - Second tens in BCD code"]
pub type SCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCU` reader - Second units in BCD code"]
pub type SCU_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 22 - AM/PM mark"]
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
    #[doc = "Bits 12:14 - Minute tens in BCD code"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD code"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
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
#[doc = "Time of time stamp register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tts](index.html) module"]
pub struct TTS_SPEC;
impl crate::RegisterSpec for TTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tts::R](R) reader structure"]
impl crate::Readable for TTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TTS to value 0"]
impl crate::Resettable for TTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
