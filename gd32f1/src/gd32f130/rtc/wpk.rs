#[doc = "Register `WPK` writer"]
pub struct W(crate::W<WPK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPK_SPEC>;
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
impl From<crate::W<WPK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WPK` writer - Write protection key"]
pub type WPK_W<'a> = crate::FieldWriter<'a, u32, WPK_SPEC, u8, u8, 8, 0>;
impl W {
    #[doc = "Bits 0:7 - Write protection key"]
    #[inline(always)]
    pub fn wpk(&mut self) -> WPK_W {
        WPK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write protection key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpk](index.html) module"]
pub struct WPK_SPEC;
impl crate::RegisterSpec for WPK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wpk::W](W) writer structure"]
impl crate::Writable for WPK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPK to value 0"]
impl crate::Resettable for WPK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
