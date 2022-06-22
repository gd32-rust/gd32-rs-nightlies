#[doc = "Register `PSC` reader"]
pub struct R(crate::R<PSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSC` writer"]
pub struct W(crate::W<PSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSC_SPEC>;
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
impl From<crate::W<PSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FACTOR_A` reader - Asynchronous prescaler factor"]
pub type FACTOR_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FACTOR_A` writer - Asynchronous prescaler factor"]
pub type FACTOR_A_W<'a> = crate::FieldWriter<'a, u32, PSC_SPEC, u8, u8, 7, 16>;
#[doc = "Field `FACTOR_S` reader - Synchronous prescaler factor"]
pub type FACTOR_S_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FACTOR_S` writer - Synchronous prescaler factor"]
pub type FACTOR_S_W<'a> = crate::FieldWriter<'a, u32, PSC_SPEC, u16, u16, 15, 0>;
impl R {
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_a(&self) -> FACTOR_A_R {
        FACTOR_A_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_s(&self) -> FACTOR_S_R {
        FACTOR_S_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_a(&mut self) -> FACTOR_A_W {
        FACTOR_A_W::new(self)
    }
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn factor_s(&mut self) -> FACTOR_S_W {
        FACTOR_S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psc](index.html) module"]
pub struct PSC_SPEC;
impl crate::RegisterSpec for PSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psc::R](R) reader structure"]
impl crate::Readable for PSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psc::W](W) writer structure"]
impl crate::Writable for PSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSC to value 0x007f_00ff"]
impl crate::Resettable for PSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x007f_00ff
    }
}
