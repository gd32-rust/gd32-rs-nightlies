#[doc = "Register `MSC_SCCNT` reader"]
pub struct R(crate::R<MSC_SCCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSC_SCCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSC_SCCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSC_SCCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCC` reader - Transmitted good frames after a single collision counter"]
pub type SCC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames after a single collision counter"]
    #[inline(always)]
    pub fn scc(&self) -> SCC_R {
        SCC_R::new(self.bits)
    }
}
#[doc = "Ethernet MSC transmitted good frames after a single collision counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msc_sccnt](index.html) module"]
pub struct MSC_SCCNT_SPEC;
impl crate::RegisterSpec for MSC_SCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msc_sccnt::R](R) reader structure"]
impl crate::Readable for MSC_SCCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSC_SCCNT to value 0"]
impl crate::Resettable for MSC_SCCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
