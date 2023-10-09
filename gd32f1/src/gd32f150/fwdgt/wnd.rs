#[doc = "Register `WND` reader"]
pub type R = crate::R<WND_SPEC>;
#[doc = "Field `WND` reader - Watchdog counter window value"]
pub type WND_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&self) -> WND_R {
        WND_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wnd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WND_SPEC;
impl crate::RegisterSpec for WND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wnd::R`](R) reader structure"]
impl crate::Readable for WND_SPEC {}
#[doc = "`reset()` method sets WND to value 0"]
impl crate::Resettable for WND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
