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
#[doc = "Register `WND` writer"]
pub struct W(crate::W<WND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WND` reader - Watchdog counter window value"]
pub type WND_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WND` writer - Watchdog counter window value"]
pub type WND_W<'a> = crate::FieldWriterSafe<'a, u32, WND_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&self) -> WND_R {
        WND_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&mut self) -> WND_W {
        WND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wnd](index.html) module"]
pub struct WND_SPEC;
impl crate::RegisterSpec for WND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wnd::R](R) reader structure"]
impl crate::Readable for WND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wnd::W](W) writer structure"]
impl crate::Writable for WND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WND to value 0x0fff"]
impl crate::Resettable for WND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
