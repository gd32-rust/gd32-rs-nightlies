#[doc = "Register `DATA` reader"]
pub struct R(crate::R<DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DT3` reader - Pixel Data 3"]
pub type DT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT2` reader - Pixel Data 2"]
pub type DT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT1` reader - Pixel Data 1"]
pub type DT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT0` reader - Pixel Data 0"]
pub type DT0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - Pixel Data 3"]
    #[inline(always)]
    pub fn dt3(&self) -> DT3_R {
        DT3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pixel Data 2"]
    #[inline(always)]
    pub fn dt2(&self) -> DT2_R {
        DT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pixel Data 1"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Pixel Data 0"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DCI DATA register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data::R](R) reader structure"]
impl crate::Readable for DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
