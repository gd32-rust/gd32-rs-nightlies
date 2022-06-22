#[doc = "Register `IV0H` reader"]
pub struct R(crate::R<IV0H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV0H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV0H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV0H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IV0H` writer"]
pub struct W(crate::W<IV0H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV0H_SPEC>;
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
impl From<crate::W<IV0H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV0H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV0H` reader - The initialization vector for DES,TDES,AES"]
pub type IV0H_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IV0H` writer - The initialization vector for DES,TDES,AES"]
pub type IV0H_W<'a> = crate::FieldWriter<'a, u32, IV0H_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv0h(&self) -> IV0H_R {
        IV0H_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv0h(&mut self) -> IV0H_W {
        IV0H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAU initialization register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv0h](index.html) module"]
pub struct IV0H_SPEC;
impl crate::RegisterSpec for IV0H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iv0h::R](R) reader structure"]
impl crate::Readable for IV0H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iv0h::W](W) writer structure"]
impl crate::Writable for IV0H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IV0H to value 0"]
impl crate::Resettable for IV0H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
