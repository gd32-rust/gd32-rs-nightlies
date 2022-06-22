#[doc = "Register `MSC_RFAECNT` reader"]
pub struct R(crate::R<MSC_RFAECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSC_RFAECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSC_RFAECNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSC_RFAECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFAER` reader - Received frames with alignment error counter"]
pub type RFAER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames with alignment error counter"]
    #[inline(always)]
    pub fn rfaer(&self) -> RFAER_R {
        RFAER_R::new(self.bits)
    }
}
#[doc = "Ethernet MSC received frames with alignment error counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msc_rfaecnt](index.html) module"]
pub struct MSC_RFAECNT_SPEC;
impl crate::RegisterSpec for MSC_RFAECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msc_rfaecnt::R](R) reader structure"]
impl crate::Readable for MSC_RFAECNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSC_RFAECNT to value 0"]
impl crate::Resettable for MSC_RFAECNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
