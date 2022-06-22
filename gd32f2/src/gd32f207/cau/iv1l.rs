#[doc = "Register `IV1L` reader"]
pub struct R(crate::R<IV1L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV1L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV1L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV1L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IV1L` writer"]
pub struct W(crate::W<IV1L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV1L_SPEC>;
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
impl From<crate::W<IV1L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV1L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV1L` reader - The initialization vector for DES,TDES,AES"]
pub type IV1L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IV1L` writer - The initialization vector for DES,TDES,AES"]
pub type IV1L_W<'a> = crate::FieldWriter<'a, u32, IV1L_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv1l(&self) -> IV1L_R {
        IV1L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv1l(&mut self) -> IV1L_W {
        IV1L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAU initialization register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv1l](index.html) module"]
pub struct IV1L_SPEC;
impl crate::RegisterSpec for IV1L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iv1l::R](R) reader structure"]
impl crate::Readable for IV1L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iv1l::W](W) writer structure"]
impl crate::Writable for IV1L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IV1L to value 0"]
impl crate::Resettable for IV1L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
