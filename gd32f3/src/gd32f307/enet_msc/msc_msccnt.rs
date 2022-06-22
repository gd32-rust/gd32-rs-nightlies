#[doc = "Register `MSC_MSCCNT` reader"]
pub struct R(crate::R<MSC_MSCCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSC_MSCCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSC_MSCCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSC_MSCCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSCC` reader - Transmitted good frames after more than a single collision counter"]
pub type MSCC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames after more than a single collision counter"]
    #[inline(always)]
    pub fn mscc(&self) -> MSCC_R {
        MSCC_R::new(self.bits)
    }
}
#[doc = "Ethernet MSC transmitted good frames after more than a single collision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msc_msccnt](index.html) module"]
pub struct MSC_MSCCNT_SPEC;
impl crate::RegisterSpec for MSC_MSCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msc_msccnt::R](R) reader structure"]
impl crate::Readable for MSC_MSCCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSC_MSCCNT to value 0"]
impl crate::Resettable for MSC_MSCCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
