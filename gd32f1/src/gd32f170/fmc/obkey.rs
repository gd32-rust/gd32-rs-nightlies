#[doc = "Register `OBKEY` writer"]
pub struct W(crate::W<OBKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OBKEY_SPEC>;
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
impl From<crate::W<OBKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OBKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OBKEY` writer - FMC_CTL option byte operation unlock registers"]
pub type OBKEY_W<'a> = crate::FieldWriterSafe<'a, u32, OBKEY_SPEC, u32, u32, 32, 0>;
impl W {
    #[doc = "Bits 0:31 - FMC_CTL option byte operation unlock registers"]
    #[inline(always)]
    pub fn obkey(&mut self) -> OBKEY_W {
        OBKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Flash option byte unlock key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obkey](index.html) module"]
pub struct OBKEY_SPEC;
impl crate::RegisterSpec for OBKEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [obkey::W](W) writer structure"]
impl crate::Writable for OBKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OBKEY to value 0"]
impl crate::Resettable for OBKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
