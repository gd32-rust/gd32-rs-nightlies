#[doc = "Register `OPA_BT` reader"]
pub type R = crate::R<OPA_BT_SPEC>;
#[doc = "Register `OPA_BT` writer"]
pub type W = crate::W<OPA_BT_SPEC>;
#[doc = "Field `OA0_TRIM_LOW` reader - OPA0, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA0_TRIM_LOW_R = crate::FieldReader;
#[doc = "Field `OA0_TRIM_LOW` writer - OPA0, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA0_TRIM_LOW_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OA0_TRIM_HIGH` reader - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA0_TRIM_HIGH_R = crate::FieldReader;
#[doc = "Field `OA0_TRIM_HIGH` writer - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA0_TRIM_HIGH_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OA1_TRIM_LOW` reader - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA1_TRIM_LOW_R = crate::FieldReader;
#[doc = "Field `OA1_TRIM_LOW` writer - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA1_TRIM_LOW_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OA1_TRIM_HIGH` reader - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA1_TRIM_HIGH_R = crate::FieldReader;
#[doc = "Field `OA1_TRIM_HIGH` writer - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA1_TRIM_HIGH_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OA2_TRIM_LOW` reader - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA2_TRIM_LOW_R = crate::FieldReader;
#[doc = "Field `OA2_TRIM_LOW` writer - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
pub type OA2_TRIM_LOW_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OA2_TRIM_HIGH` reader - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA2_TRIM_HIGH_R = crate::FieldReader;
#[doc = "Field `OA2_TRIM_HIGH` writer - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
pub type OA2_TRIM_HIGH_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `OT_USER` reader - user programmed trimming value"]
pub type OT_USER_R = crate::BitReader<OT_USER_A>;
#[doc = "user programmed trimming value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl OT_USER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OT_USER_A {
        match self.bits {
            false => OT_USER_A::FACTORY,
            true => OT_USER_A::USER,
        }
    }
    #[doc = "Trim with factory-default values"]
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        *self == OT_USER_A::FACTORY
    }
    #[doc = "Trim with user-programmed values"]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == OT_USER_A::USER
    }
}
#[doc = "Field `OT_USER` writer - user programmed trimming value"]
pub type OT_USER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OT_USER_A>;
impl<'a, REG, const O: u8> OT_USER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trim with factory-default values"]
    #[inline(always)]
    pub fn factory(self) -> &'a mut crate::W<REG> {
        self.variant(OT_USER_A::FACTORY)
    }
    #[doc = "Trim with user-programmed values"]
    #[inline(always)]
    pub fn user(self) -> &'a mut crate::W<REG> {
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
    #[must_use]
    pub fn oa0_trim_low(&mut self) -> OA0_TRIM_LOW_W<OPA_BT_SPEC, 0> {
        OA0_TRIM_LOW_W::new(self)
    }
    #[doc = "Bits 5:9 - OPA0, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa0_trim_high(&mut self) -> OA0_TRIM_HIGH_W<OPA_BT_SPEC, 5> {
        OA0_TRIM_HIGH_W::new(self)
    }
    #[doc = "Bits 10:14 - OPA1, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa1_trim_low(&mut self) -> OA1_TRIM_LOW_W<OPA_BT_SPEC, 10> {
        OA1_TRIM_LOW_W::new(self)
    }
    #[doc = "Bits 15:19 - OPA1, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa1_trim_high(&mut self) -> OA1_TRIM_HIGH_W<OPA_BT_SPEC, 15> {
        OA1_TRIM_HIGH_W::new(self)
    }
    #[doc = "Bits 20:24 - OPA2, normal mode 5-bit offset trim value for PMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa2_trim_low(&mut self) -> OA2_TRIM_LOW_W<OPA_BT_SPEC, 20> {
        OA2_TRIM_LOW_W::new(self)
    }
    #[doc = "Bits 25:29 - OPA2, normal mode 5-bit offset trim value for NMOS pairs"]
    #[inline(always)]
    #[must_use]
    pub fn oa2_trim_high(&mut self) -> OA2_TRIM_HIGH_W<OPA_BT_SPEC, 25> {
        OA2_TRIM_HIGH_W::new(self)
    }
    #[doc = "Bit 31 - user programmed trimming value"]
    #[inline(always)]
    #[must_use]
    pub fn ot_user(&mut self) -> OT_USER_W<OPA_BT_SPEC, 31> {
        OT_USER_W::new(self)
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
#[doc = "OPA offset trimming for normal mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_bt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_bt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPA_BT_SPEC;
impl crate::RegisterSpec for OPA_BT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa_bt::R`](R) reader structure"]
impl crate::Readable for OPA_BT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opa_bt::W`](W) writer structure"]
impl crate::Writable for OPA_BT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPA_BT to value 0"]
impl crate::Resettable for OPA_BT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
