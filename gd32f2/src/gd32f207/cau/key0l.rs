#[doc = "Register `KEY0L` writer"]
pub struct W(crate::W<KEY0L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY0L_SPEC>;
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
impl From<crate::W<KEY0L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY0L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0L` writer - Key for DES,TDES,AES"]
pub type KEY0L_W<'a> = crate::FieldWriter<'a, u32, KEY0L_SPEC, u32, u32, 32, 0>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    pub fn key0l(&mut self) -> KEY0L_W {
        KEY0L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAU key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key0l](index.html) module"]
pub struct KEY0L_SPEC;
impl crate::RegisterSpec for KEY0L_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [key0l::W](W) writer structure"]
impl crate::Writable for KEY0L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY0L to value 0"]
impl crate::Resettable for KEY0L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
