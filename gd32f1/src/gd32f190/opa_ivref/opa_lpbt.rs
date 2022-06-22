#[doc = "Register `OPA_LPBT` reader"]
pub struct R(crate::R<OPA_LPBT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPA_LPBT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPA_LPBT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPA_LPBT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPA_LPBT` writer"]
pub struct W(crate::W<OPA_LPBT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPA_LPBT_SPEC>;
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
impl From<crate::W<OPA_LPBT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPA_LPBT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA0_TRIM_LP_LOW` reader - OPA0, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA0_TRIM_LP_LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA0_TRIM_LP_LOW` writer - OPA0, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA0_TRIM_LP_LOW_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_LPBT_SPEC, u8, u8, 5, 0>;
#[doc = "Field `OA0_TRIM_LP_HIGH` reader - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA0_TRIM_LP_HIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA0_TRIM_LP_HIGH` writer - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA0_TRIM_LP_HIGH_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_LPBT_SPEC, u8, u8, 5, 5>;
#[doc = "Field `OA1_TRIM_LP_LOW` reader - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA1_TRIM_LP_LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA1_TRIM_LP_LOW` writer - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA1_TRIM_LP_LOW_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_LPBT_SPEC, u8, u8, 5, 10>;
#[doc = "Field `OA1_TRIM_LP_HIGH` reader - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA1_TRIM_LP_HIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA1_TRIM_LP_HIGH` writer - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA1_TRIM_LP_HIGH_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_LPBT_SPEC, u8, u8, 5, 15>;
#[doc = "Field `OA2_TRIM_LP_LOW` reader - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA2_TRIM_LP_LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA2_TRIM_LP_LOW` writer - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA2_TRIM_LP_LOW_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_LPBT_SPEC, u8, u8, 5, 20>;
#[doc = "Field `OA2_TRIM_LP_HIGH` reader - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA2_TRIM_LP_HIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA2_TRIM_LP_HIGH` writer - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA2_TRIM_LP_HIGH_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_LPBT_SPEC, u8, u8, 5, 25>;
impl R {
    #[doc = "Bits 0:4 - OPA0, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_lp_low(&self) -> OA0_TRIM_LP_LOW_R {
        OA0_TRIM_LP_LOW_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_lp_high(&self) -> OA0_TRIM_LP_HIGH_R {
        OA0_TRIM_LP_HIGH_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_lp_low(&self) -> OA1_TRIM_LP_LOW_R {
        OA1_TRIM_LP_LOW_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_lp_high(&self) -> OA1_TRIM_LP_HIGH_R {
        OA1_TRIM_LP_HIGH_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_lp_low(&self) -> OA2_TRIM_LP_LOW_R {
        OA2_TRIM_LP_LOW_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_lp_high(&self) -> OA2_TRIM_LP_HIGH_R {
        OA2_TRIM_LP_HIGH_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - OPA0, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_lp_low(&mut self) -> OA0_TRIM_LP_LOW_W {
        OA0_TRIM_LP_LOW_W::new(self)
    }
    #[doc = "Bits 5:9 - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_lp_high(&mut self) -> OA0_TRIM_LP_HIGH_W {
        OA0_TRIM_LP_HIGH_W::new(self)
    }
    #[doc = "Bits 10:14 - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_lp_low(&mut self) -> OA1_TRIM_LP_LOW_W {
        OA1_TRIM_LP_LOW_W::new(self)
    }
    #[doc = "Bits 15:19 - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_lp_high(&mut self) -> OA1_TRIM_LP_HIGH_W {
        OA1_TRIM_LP_HIGH_W::new(self)
    }
    #[doc = "Bits 20:24 - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_lp_low(&mut self) -> OA2_TRIM_LP_LOW_W {
        OA2_TRIM_LP_LOW_W::new(self)
    }
    #[doc = "Bits 25:29 - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_lp_high(&mut self) -> OA2_TRIM_LP_HIGH_W {
        OA2_TRIM_LP_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPA offset trimming for low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa_lpbt](index.html) module"]
pub struct OPA_LPBT_SPEC;
impl crate::RegisterSpec for OPA_LPBT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opa_lpbt::R](R) reader structure"]
impl crate::Readable for OPA_LPBT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opa_lpbt::W](W) writer structure"]
impl crate::Writable for OPA_LPBT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPA_LPBT to value 0"]
impl crate::Resettable for OPA_LPBT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
