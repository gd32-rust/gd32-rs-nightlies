#[doc = "Register `PWRCTL` reader"]
pub struct R(crate::R<PWRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCTL` writer"]
pub struct W(crate::W<PWRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCTL_SPEC>;
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
impl From<crate::W<PWRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRCTL` reader - SDIO power control bits"]
pub type PWRCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRCTL` writer - SDIO power control bits"]
pub type PWRCTL_W<'a> = crate::FieldWriter<'a, u32, PWRCTL_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bits 0:1 - SDIO power control bits"]
    #[inline(always)]
    pub fn pwrctl(&self) -> PWRCTL_R {
        PWRCTL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDIO power control bits"]
    #[inline(always)]
    pub fn pwrctl(&mut self) -> PWRCTL_W {
        PWRCTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctl](index.html) module"]
pub struct PWRCTL_SPEC;
impl crate::RegisterSpec for PWRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrctl::R](R) reader structure"]
impl crate::Readable for PWRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrctl::W](W) writer structure"]
impl crate::Writable for PWRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRCTL to value 0"]
impl crate::Resettable for PWRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
