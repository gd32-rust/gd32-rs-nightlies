#[doc = "Register `TPCTL` reader"]
pub struct R(crate::R<TPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPCTL` writer"]
pub struct W(crate::W<TPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPCTL_SPEC>;
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
impl From<crate::W<TPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPAL` reader - TAMPER pin active level"]
pub type TPAL_R = crate::BitReader<bool>;
#[doc = "Field `TPAL` writer - TAMPER pin active level"]
pub type TPAL_W<'a> = crate::BitWriter<'a, u32, TPCTL_SPEC, bool, 1>;
#[doc = "Field `TPEN` reader - TAMPER detection enable"]
pub type TPEN_R = crate::BitReader<bool>;
#[doc = "Field `TPEN` writer - TAMPER detection enable"]
pub type TPEN_W<'a> = crate::BitWriter<'a, u32, TPCTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TAMPER detection enable"]
    #[inline(always)]
    pub fn tpen(&self) -> TPEN_R {
        TPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    pub fn tpal(&mut self) -> TPAL_W {
        TPAL_W::new(self)
    }
    #[doc = "Bit 0 - TAMPER detection enable"]
    #[inline(always)]
    pub fn tpen(&mut self) -> TPEN_W {
        TPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper pin control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpctl](index.html) module"]
pub struct TPCTL_SPEC;
impl crate::RegisterSpec for TPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpctl::R](R) reader structure"]
impl crate::Readable for TPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpctl::W](W) writer structure"]
impl crate::Writable for TPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPCTL to value 0"]
impl crate::Resettable for TPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
