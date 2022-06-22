#[doc = "Register `MSC_RGUFCNT` reader"]
pub struct R(crate::R<MSC_RGUFCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSC_RGUFCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSC_RGUFCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSC_RGUFCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RGUF` reader - Received good unicast frames counter"]
pub type RGUF_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Received good unicast frames counter"]
    #[inline(always)]
    pub fn rguf(&self) -> RGUF_R {
        RGUF_R::new(self.bits)
    }
}
#[doc = "MSC received good unicast frames counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msc_rgufcnt](index.html) module"]
pub struct MSC_RGUFCNT_SPEC;
impl crate::RegisterSpec for MSC_RGUFCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msc_rgufcnt::R](R) reader structure"]
impl crate::Readable for MSC_RGUFCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSC_RGUFCNT to value 0"]
impl crate::Resettable for MSC_RGUFCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
