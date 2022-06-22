#[doc = "Register `OPA_BT` reader"]
pub struct R(crate::R<OPA_BT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPA_BT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPA_BT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPA_BT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPA_BT` writer"]
pub struct W(crate::W<OPA_BT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPA_BT_SPEC>;
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
impl From<crate::W<OPA_BT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPA_BT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA0_TRIM_LOW` reader - OPA0, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA0_TRIM_LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA0_TRIM_LOW` writer - OPA0, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA0_TRIM_LOW_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_BT_SPEC, u8, u8, 5, 0>;
#[doc = "Field `OA0_TRIM_HIGH` reader - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA0_TRIM_HIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA0_TRIM_HIGH` writer - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA0_TRIM_HIGH_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_BT_SPEC, u8, u8, 5, 5>;
#[doc = "Field `OA1_TRIM_LOW` reader - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA1_TRIM_LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA1_TRIM_LOW` writer - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA1_TRIM_LOW_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_BT_SPEC, u8, u8, 5, 10>;
#[doc = "Field `OA1_TRIM_HIGH` reader - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA1_TRIM_HIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA1_TRIM_HIGH` writer - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA1_TRIM_HIGH_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_BT_SPEC, u8, u8, 5, 15>;
#[doc = "Field `OA2_TRIM_LOW` reader - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA2_TRIM_LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA2_TRIM_LOW` writer - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA2_TRIM_LOW_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_BT_SPEC, u8, u8, 5, 20>;
#[doc = "Field `OA2_TRIM_HIGH` reader - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA2_TRIM_HIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA2_TRIM_HIGH` writer - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA2_TRIM_HIGH_W<'a> = crate::FieldWriterSafe<'a, u32, OPA_BT_SPEC, u8, u8, 5, 25>;
#[doc = "user programmed trimming value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OT_USER_A {
    #[doc = "0: Trim with factory-default values"]
    FACTORY = 0,
    #[doc = "1: Trim with user-programmed values"]
    USER = 1,
}
impl From<OT_USER_A> for bool {
    #[inline(always)]
    fn from(variant: OT_USER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT_USER` reader - user programmed trimming value"]
pub type OT_USER_R = crate::BitReader<OT_USER_A>;
impl OT_USER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OT_USER_A {
        match self.bits {
            false => OT_USER_A::FACTORY,
            true => OT_USER_A::USER,
        }
    }
    #[doc = "Checks if the value of the field is `FACTORY`"]
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        *self == OT_USER_A::FACTORY
    }
    #[doc = "Checks if the value of the field is `USER`"]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == OT_USER_A::USER
    }
}
#[doc = "Field `OT_USER` writer - user programmed trimming value"]
pub type OT_USER_W<'a> = crate::BitWriter<'a, u32, OPA_BT_SPEC, OT_USER_A, 31>;
impl<'a> OT_USER_W<'a> {
    #[doc = "Trim with factory-default values"]
    #[inline(always)]
    pub fn factory(self) -> &'a mut W {
        self.variant(OT_USER_A::FACTORY)
    }
    #[doc = "Trim with user-programmed values"]
    #[inline(always)]
    pub fn user(self) -> &'a mut W {
        self.variant(OT_USER_A::USER)
    }
}
impl R {
    #[doc = "Bits 0:4 - OPA0, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_low(&self) -> OA0_TRIM_LOW_R {
        OA0_TRIM_LOW_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_high(&self) -> OA0_TRIM_HIGH_R {
        OA0_TRIM_HIGH_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_low(&self) -> OA1_TRIM_LOW_R {
        OA1_TRIM_LOW_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_high(&self) -> OA1_TRIM_HIGH_R {
        OA1_TRIM_HIGH_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_low(&self) -> OA2_TRIM_LOW_R {
        OA2_TRIM_LOW_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_high(&self) -> OA2_TRIM_HIGH_R {
        OA2_TRIM_HIGH_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - user programmed trimming value"]
    #[inline(always)]
    pub fn ot_user(&self) -> OT_USER_R {
        OT_USER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - OPA0, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_low(&mut self) -> OA0_TRIM_LOW_W {
        OA0_TRIM_LOW_W::new(self)
    }
    #[doc = "Bits 5:9 - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa0_trim_high(&mut self) -> OA0_TRIM_HIGH_W {
        OA0_TRIM_HIGH_W::new(self)
    }
    #[doc = "Bits 10:14 - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_low(&mut self) -> OA1_TRIM_LOW_W {
        OA1_TRIM_LOW_W::new(self)
    }
    #[doc = "Bits 15:19 - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa1_trim_high(&mut self) -> OA1_TRIM_HIGH_W {
        OA1_TRIM_HIGH_W::new(self)
    }
    #[doc = "Bits 20:24 - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_low(&mut self) -> OA2_TRIM_LOW_W {
        OA2_TRIM_LOW_W::new(self)
    }
    #[doc = "Bits 25:29 - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    pub fn oa2_trim_high(&mut self) -> OA2_TRIM_HIGH_W {
        OA2_TRIM_HIGH_W::new(self)
    }
    #[doc = "Bit 31 - user programmed trimming value"]
    #[inline(always)]
    pub fn ot_user(&mut self) -> OT_USER_W {
        OT_USER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPA offset trimming for normal mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa_bt](index.html) module"]
pub struct OPA_BT_SPEC;
impl crate::RegisterSpec for OPA_BT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opa_bt::R](R) reader structure"]
impl crate::Readable for OPA_BT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opa_bt::W](W) writer structure"]
impl crate::Writable for OPA_BT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPA_BT to value 0"]
impl crate::Resettable for OPA_BT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
