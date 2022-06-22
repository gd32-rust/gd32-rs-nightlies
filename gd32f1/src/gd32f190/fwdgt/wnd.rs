#[doc = "Register `WND` reader"]
pub struct R(crate::R<WND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WND` reader - Watchdog counter window value"]
pub type WND_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&self) -> WND_R {
        WND_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Window register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wnd](index.html) module"]
pub struct WND_SPEC;
impl crate::RegisterSpec for WND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wnd::R](R) reader structure"]
impl crate::Readable for WND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WND to value 0"]
impl crate::Resettable for WND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
