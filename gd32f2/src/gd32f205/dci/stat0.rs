#[doc = "Register `STAT0` reader"]
pub struct R(crate::R<STAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FV` reader - FIFO Valid"]
pub type FV_R = crate::BitReader<bool>;
#[doc = "Field `VS` reader - VS line status"]
pub type VS_R = crate::BitReader<bool>;
#[doc = "Field `HS` reader - HS line status"]
pub type HS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - FIFO Valid"]
    #[inline(always)]
    pub fn fv(&self) -> FV_R {
        FV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - VS line status"]
    #[inline(always)]
    pub fn vs(&self) -> VS_R {
        VS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - HS line status"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DCI Status register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat0](index.html) module"]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat0::R](R) reader structure"]
impl crate::Readable for STAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for STAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
