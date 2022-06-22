#[doc = "Register `TPCTL1` reader"]
pub struct R(crate::R<TPCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPCTL1` writer"]
pub struct W(crate::W<TPCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPCTL1_SPEC>;
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
impl From<crate::W<TPCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPM1` reader - The first waveform detection enable"]
pub type TPM1_R = crate::BitReader<bool>;
#[doc = "Field `TPM1` writer - The first waveform detection enable"]
pub type TPM1_W<'a> = crate::BitWriter<'a, u32, TPCTL1_SPEC, bool, 15>;
#[doc = "Field `TPM2` reader - The second waveform detection enable"]
pub type TPM2_R = crate::BitReader<bool>;
#[doc = "Field `TPM2` writer - The second waveform detection enable"]
pub type TPM2_W<'a> = crate::BitWriter<'a, u32, TPCTL1_SPEC, bool, 14>;
#[doc = "Field `TPAL1` reader - TAMPER1 pin active level"]
pub type TPAL1_R = crate::BitReader<bool>;
#[doc = "Field `TPAL1` writer - TAMPER1 pin active level"]
pub type TPAL1_W<'a> = crate::BitWriter<'a, u32, TPCTL1_SPEC, bool, 9>;
#[doc = "Field `TPEN1` reader - TAMPER1 detection enable"]
pub type TPEN1_R = crate::BitReader<bool>;
#[doc = "Field `TPEN1` writer - TAMPER1 detection enable"]
pub type TPEN1_W<'a> = crate::BitWriter<'a, u32, TPCTL1_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 15 - The first waveform detection enable"]
    #[inline(always)]
    pub fn tpm1(&self) -> TPM1_R {
        TPM1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - The second waveform detection enable"]
    #[inline(always)]
    pub fn tpm2(&self) -> TPM2_R {
        TPM2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 9 - TAMPER1 pin active level"]
    #[inline(always)]
    pub fn tpal1(&self) -> TPAL1_R {
        TPAL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - TAMPER1 detection enable"]
    #[inline(always)]
    pub fn tpen1(&self) -> TPEN1_R {
        TPEN1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - The first waveform detection enable"]
    #[inline(always)]
    pub fn tpm1(&mut self) -> TPM1_W {
        TPM1_W::new(self)
    }
    #[doc = "Bit 14 - The second waveform detection enable"]
    #[inline(always)]
    pub fn tpm2(&mut self) -> TPM2_W {
        TPM2_W::new(self)
    }
    #[doc = "Bit 9 - TAMPER1 pin active level"]
    #[inline(always)]
    pub fn tpal1(&mut self) -> TPAL1_W {
        TPAL1_W::new(self)
    }
    #[doc = "Bit 8 - TAMPER1 detection enable"]
    #[inline(always)]
    pub fn tpen1(&mut self) -> TPEN1_W {
        TPEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper pin control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpctl1](index.html) module"]
pub struct TPCTL1_SPEC;
impl crate::RegisterSpec for TPCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpctl1::R](R) reader structure"]
impl crate::Readable for TPCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpctl1::W](W) writer structure"]
impl crate::Writable for TPCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPCTL1 to value 0"]
impl crate::Resettable for TPCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
