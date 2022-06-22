#[doc = "Register `KEY2H` writer"]
pub struct W(crate::W<KEY2H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY2H_SPEC>;
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
impl From<crate::W<KEY2H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY2H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY2H` writer - Key for DES,TDES,AES"]
pub type KEY2H_W<'a> = crate::FieldWriter<'a, u32, KEY2H_SPEC, u32, u32, 32, 0>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    pub fn key2h(&mut self) -> KEY2H_W {
        KEY2H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAU key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2h](index.html) module"]
pub struct KEY2H_SPEC;
impl crate::RegisterSpec for KEY2H_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [key2h::W](W) writer structure"]
impl crate::Writable for KEY2H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY2H to value 0"]
impl crate::Resettable for KEY2H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
