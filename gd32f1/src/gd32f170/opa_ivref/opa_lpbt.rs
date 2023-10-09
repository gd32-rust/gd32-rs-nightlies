#[doc = "Register `OPA_LPBT` reader"]
pub type R = crate::R<OPA_LPBT_SPEC>;
#[doc = "Register `OPA_LPBT` writer"]
pub type W = crate::W<OPA_LPBT_SPEC>;
#[doc = "Field `OA0_TRIM_LP_LOW` reader - OPA0, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA0_TRIM_LP_LOW_R = crate::FieldReader;
#[doc = "Field `OA0_TRIM_LP_LOW` writer - OPA0, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA0_TRIM_LP_LOW_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OA0_TRIM_LP_HIGH` reader - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA0_TRIM_LP_HIGH_R = crate::FieldReader;
#[doc = "Field `OA0_TRIM_LP_HIGH` writer - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA0_TRIM_LP_HIGH_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OA1_TRIM_LP_LOW` reader - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA1_TRIM_LP_LOW_R = crate::FieldReader;
#[doc = "Field `OA1_TRIM_LP_LOW` writer - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA1_TRIM_LP_LOW_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OA1_TRIM_LP_HIGH` reader - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA1_TRIM_LP_HIGH_R = crate::FieldReader;
#[doc = "Field `OA1_TRIM_LP_HIGH` writer - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA1_TRIM_LP_HIGH_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OA2_TRIM_LP_LOW` reader - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA2_TRIM_LP_LOW_R = crate::FieldReader;
#[doc = "Field `OA2_TRIM_LP_LOW` writer - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
pub type OA2_TRIM_LP_LOW_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OA2_TRIM_LP_HIGH` reader - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA2_TRIM_LP_HIGH_R = crate::FieldReader;
#[doc = "Field `OA2_TRIM_LP_HIGH` writer - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
pub type OA2_TRIM_LP_HIGH_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
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
    #[must_use]
    pub fn oa0_trim_lp_low(&mut self) -> OA0_TRIM_LP_LOW_W<OPA_LPBT_SPEC, 0> {
        OA0_TRIM_LP_LOW_W::new(self)
    }
    #[doc = "Bits 5:9 - OPA0, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa0_trim_lp_high(&mut self) -> OA0_TRIM_LP_HIGH_W<OPA_LPBT_SPEC, 5> {
        OA0_TRIM_LP_HIGH_W::new(self)
    }
    #[doc = "Bits 10:14 - OPA1, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa1_trim_lp_low(&mut self) -> OA1_TRIM_LP_LOW_W<OPA_LPBT_SPEC, 10> {
        OA1_TRIM_LP_LOW_W::new(self)
    }
    #[doc = "Bits 15:19 - OPA1, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa1_trim_lp_high(&mut self) -> OA1_TRIM_LP_HIGH_W<OPA_LPBT_SPEC, 15> {
        OA1_TRIM_LP_HIGH_W::new(self)
    }
    #[doc = "Bits 20:24 - OPA2, low power mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa2_trim_lp_low(&mut self) -> OA2_TRIM_LP_LOW_W<OPA_LPBT_SPEC, 20> {
        OA2_TRIM_LP_LOW_W::new(self)
    }
    #[doc = "Bits 25:29 - OPA2, low power mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa2_trim_lp_high(&mut self) -> OA2_TRIM_LP_HIGH_W<OPA_LPBT_SPEC, 25> {
        OA2_TRIM_LP_HIGH_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OPA offset trimming for low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_lpbt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_lpbt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPA_LPBT_SPEC;
impl crate::RegisterSpec for OPA_LPBT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa_lpbt::R`](R) reader structure"]
impl crate::Readable for OPA_LPBT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opa_lpbt::W`](W) writer structure"]
impl crate::Writable for OPA_LPBT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPA_LPBT to value 0"]
impl crate::Resettable for OPA_LPBT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
