#[doc = "Register `MSC_TGFCNT` reader"]
pub struct R(crate::R<MSC_TGFCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSC_TGFCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSC_TGFCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSC_TGFCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TGF` reader - Transmitted good frames counter"]
pub type TGF_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames counter"]
    #[inline(always)]
    pub fn tgf(&self) -> TGF_R {
        TGF_R::new(self.bits)
    }
}
#[doc = "Ethernet MSC transmitted good frames counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msc_tgfcnt](index.html) module"]
pub struct MSC_TGFCNT_SPEC;
impl crate::RegisterSpec for MSC_TGFCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msc_tgfcnt::R](R) reader structure"]
impl crate::Readable for MSC_TGFCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSC_TGFCNT to value 0"]
impl crate::Resettable for MSC_TGFCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
