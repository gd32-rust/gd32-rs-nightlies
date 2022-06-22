#[doc = "Register `TPCTL0` reader"]
pub struct R(crate::R<TPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPCTL0` writer"]
pub struct W(crate::W<TPCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPCTL0_SPEC>;
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
impl From<crate::W<TPCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPAL0` reader - TAMPER0 pin active level"]
pub type TPAL0_R = crate::BitReader<bool>;
#[doc = "Field `TPAL0` writer - TAMPER0 pin active level"]
pub type TPAL0_W<'a> = crate::BitWriter<'a, u32, TPCTL0_SPEC, bool, 1>;
#[doc = "Field `TPEN0` reader - TAMPER0 detection enable"]
pub type TPEN0_R = crate::BitReader<bool>;
#[doc = "Field `TPEN0` writer - TAMPER0 detection enable"]
pub type TPEN0_W<'a> = crate::BitWriter<'a, u32, TPCTL0_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 1 - TAMPER0 pin active level"]
    #[inline(always)]
    pub fn tpal0(&self) -> TPAL0_R {
        TPAL0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TAMPER0 detection enable"]
    #[inline(always)]
    pub fn tpen0(&self) -> TPEN0_R {
        TPEN0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TAMPER0 pin active level"]
    #[inline(always)]
    pub fn tpal0(&mut self) -> TPAL0_W {
        TPAL0_W::new(self)
    }
    #[doc = "Bit 0 - TAMPER0 detection enable"]
    #[inline(always)]
    pub fn tpen0(&mut self) -> TPEN0_W {
        TPEN0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper pin control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpctl0](index.html) module"]
pub struct TPCTL0_SPEC;
impl crate::RegisterSpec for TPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpctl0::R](R) reader structure"]
impl crate::Readable for TPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpctl0::W](W) writer structure"]
impl crate::Writable for TPCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPCTL0 to value 0"]
impl crate::Resettable for TPCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
