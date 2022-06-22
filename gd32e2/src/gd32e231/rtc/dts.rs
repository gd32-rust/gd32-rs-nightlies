#[doc = "Register `DTS` reader"]
pub struct R(crate::R<DTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOW` reader - Week day units"]
pub type DOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MONT` reader - Month tens in BCD code"]
pub type MONT_R = crate::BitReader<bool>;
#[doc = "Field `MONU` reader - Month units in BCD code"]
pub type MONU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYT` reader - Date tens in BCD code"]
pub type DAYT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYU` reader - Date units in BCD code"]
pub type DAYU_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD code"]
    #[inline(always)]
    pub fn mont(&self) -> MONT_R {
        MONT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Month units in BCD code"]
    #[inline(always)]
    pub fn monu(&self) -> MONU_R {
        MONU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - Date tens in BCD code"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 0:4 - Date units in BCD code"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Date of time stamp register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts](index.html) module"]
pub struct DTS_SPEC;
impl crate::RegisterSpec for DTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts::R](R) reader structure"]
impl crate::Readable for DTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTS to value 0"]
impl crate::Resettable for DTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
