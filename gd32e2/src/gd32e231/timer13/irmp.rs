#[doc = "Register `IRMP` reader"]
pub struct R(crate::R<IRMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRMP` writer"]
pub struct W(crate::W<IRMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRMP_SPEC>;
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
impl From<crate::W<IRMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CI0_RMP` reader - Timer input 0 remap"]
pub type CI0_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CI0_RMP` writer - Timer input 0 remap"]
pub type CI0_RMP_W<'a> = crate::FieldWriter<'a, u32, IRMP_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bits 0:1 - Timer input 0 remap"]
    #[inline(always)]
    pub fn ci0_rmp(&self) -> CI0_RMP_R {
        CI0_RMP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer input 0 remap"]
    #[inline(always)]
    pub fn ci0_rmp(&mut self) -> CI0_RMP_W {
        CI0_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel input remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irmp](index.html) module"]
pub struct IRMP_SPEC;
impl crate::RegisterSpec for IRMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irmp::R](R) reader structure"]
impl crate::Readable for IRMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irmp::W](W) writer structure"]
impl crate::Writable for IRMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRMP to value 0"]
impl crate::Resettable for IRMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
