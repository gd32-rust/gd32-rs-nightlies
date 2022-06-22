#[doc = "Register `IV0L` reader"]
pub struct R(crate::R<IV0L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV0L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV0L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV0L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IV0L` writer"]
pub struct W(crate::W<IV0L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV0L_SPEC>;
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
impl From<crate::W<IV0L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV0L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV0L` reader - The initialization vector for DES,TDES,AES"]
pub type IV0L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IV0L` writer - The initialization vector for DES,TDES,AES"]
pub type IV0L_W<'a> = crate::FieldWriter<'a, u32, IV0L_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv0l(&self) -> IV0L_R {
        IV0L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv0l(&mut self) -> IV0L_W {
        IV0L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAU initialization register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv0l](index.html) module"]
pub struct IV0L_SPEC;
impl crate::RegisterSpec for IV0L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iv0l::R](R) reader structure"]
impl crate::Readable for IV0L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iv0l::W](W) writer structure"]
impl crate::Writable for IV0L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IV0L to value 0"]
impl crate::Resettable for IV0L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
