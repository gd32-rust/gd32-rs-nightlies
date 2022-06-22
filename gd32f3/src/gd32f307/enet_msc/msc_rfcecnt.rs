#[doc = "Register `MSC_RFCECNT` reader"]
pub struct R(crate::R<MSC_RFCECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSC_RFCECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSC_RFCECNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSC_RFCECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFCER` reader - Received frames with CRC error counter"]
pub type RFCER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames with CRC error counter"]
    #[inline(always)]
    pub fn rfcer(&self) -> RFCER_R {
        RFCER_R::new(self.bits)
    }
}
#[doc = "Ethernet MSC received frames with CRC error counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msc_rfcecnt](index.html) module"]
pub struct MSC_RFCECNT_SPEC;
impl crate::RegisterSpec for MSC_RFCECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msc_rfcecnt::R](R) reader structure"]
impl crate::Readable for MSC_RFCECNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSC_RFCECNT to value 0"]
impl crate::Resettable for MSC_RFCECNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
