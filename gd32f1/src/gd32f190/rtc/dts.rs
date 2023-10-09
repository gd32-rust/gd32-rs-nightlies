#[doc = "Register `DTS` reader"]
pub type R = crate::R<DTS_SPEC>;
#[doc = "Field `DAYU` reader - Date units in BCD code"]
pub type DAYU_R = crate::FieldReader;
#[doc = "Field `DAYT` reader - Date tens in BCD code"]
pub type DAYT_R = crate::FieldReader;
#[doc = "Field `MONU` reader - Month units in BCD code"]
pub type MONU_R = crate::FieldReader;
#[doc = "Field `MONT` reader - Month tens in BCD code"]
pub type MONT_R = crate::BitReader;
#[doc = "Field `DOW` reader - Week day units"]
pub type DOW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Date units in BCD code"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD code"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units in BCD code"]
    #[inline(always)]
    pub fn monu(&self) -> MONU_R {
        MONU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD code"]
    #[inline(always)]
    pub fn mont(&self) -> MONT_R {
        MONT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits >> 13) & 7) as u8)
    }
}
#[doc = "Date of time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTS_SPEC;
impl crate::RegisterSpec for DTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dts::R`](R) reader structure"]
impl crate::Readable for DTS_SPEC {}
#[doc = "`reset()` method sets DTS to value 0"]
impl crate::Resettable for DTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
