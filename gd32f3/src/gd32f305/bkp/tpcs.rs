#[doc = "Register `TPCS` reader"]
pub struct R(crate::R<TPCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPCS` writer"]
pub struct W(crate::W<TPCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPCS_SPEC>;
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
impl From<crate::W<TPCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIF` reader - Tamper interrupt flag"]
pub type TIF_R = crate::BitReader<bool>;
#[doc = "Field `TIF` writer - Tamper interrupt flag"]
pub type TIF_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 9>;
#[doc = "Field `TEF` reader - Tamper event flag"]
pub type TEF_R = crate::BitReader<bool>;
#[doc = "Field `TEF` writer - Tamper event flag"]
pub type TEF_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 8>;
#[doc = "Field `TPIE` reader - Tamper interrupt enable"]
pub type TPIE_R = crate::BitReader<bool>;
#[doc = "Field `TPIE` writer - Tamper interrupt enable"]
pub type TPIE_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 2>;
#[doc = "Field `TIR` reader - Tamper interrupt reset"]
pub type TIR_R = crate::BitReader<bool>;
#[doc = "Field `TIR` writer - Tamper interrupt reset"]
pub type TIR_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 1>;
#[doc = "Field `TER` reader - Tamper event reset"]
pub type TER_R = crate::BitReader<bool>;
#[doc = "Field `TER` writer - Tamper event reset"]
pub type TER_W<'a> = crate::BitWriter<'a, u32, TPCS_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper interrupt reset"]
    #[inline(always)]
    pub fn tir(&self) -> TIR_R {
        TIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Tamper event reset"]
    #[inline(always)]
    pub fn ter(&self) -> TER_R {
        TER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W::new(self)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    pub fn tef(&mut self) -> TEF_W {
        TEF_W::new(self)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TPIE_W {
        TPIE_W::new(self)
    }
    #[doc = "Bit 1 - Tamper interrupt reset"]
    #[inline(always)]
    pub fn tir(&mut self) -> TIR_W {
        TIR_W::new(self)
    }
    #[doc = "Bit 0 - Tamper event reset"]
    #[inline(always)]
    pub fn ter(&mut self) -> TER_W {
        TER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpcs](index.html) module"]
pub struct TPCS_SPEC;
impl crate::RegisterSpec for TPCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpcs::R](R) reader structure"]
impl crate::Readable for TPCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpcs::W](W) writer structure"]
impl crate::Writable for TPCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPCS to value 0"]
impl crate::Resettable for TPCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
