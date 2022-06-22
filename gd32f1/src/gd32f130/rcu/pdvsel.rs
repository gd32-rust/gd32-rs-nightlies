#[doc = "Register `PDVSEL` reader"]
pub struct R(crate::R<PDVSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDVSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDVSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDVSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDVSEL` writer"]
pub struct W(crate::W<PDVSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDVSEL_SPEC>;
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
impl From<crate::W<PDVSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDVSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power down voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDRVS_A {
    #[doc = "0: The power down voltage is 2.6 V"]
    V2_6 = 0,
    #[doc = "1: The power down voltage is 1.8 V"]
    V1_8 = 1,
}
impl From<PDRVS_A> for bool {
    #[inline(always)]
    fn from(variant: PDRVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDRVS` reader - Power down voltage select"]
pub type PDRVS_R = crate::BitReader<PDRVS_A>;
impl PDRVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDRVS_A {
        match self.bits {
            false => PDRVS_A::V2_6,
            true => PDRVS_A::V1_8,
        }
    }
    #[doc = "Checks if the value of the field is `V2_6`"]
    #[inline(always)]
    pub fn is_v2_6(&self) -> bool {
        *self == PDRVS_A::V2_6
    }
    #[doc = "Checks if the value of the field is `V1_8`"]
    #[inline(always)]
    pub fn is_v1_8(&self) -> bool {
        *self == PDRVS_A::V1_8
    }
}
#[doc = "Field `PDRVS` writer - Power down voltage select"]
pub type PDRVS_W<'a> = crate::BitWriter<'a, u32, PDVSEL_SPEC, PDRVS_A, 0>;
impl<'a> PDRVS_W<'a> {
    #[doc = "The power down voltage is 2.6 V"]
    #[inline(always)]
    pub fn v2_6(self) -> &'a mut W {
        self.variant(PDRVS_A::V2_6)
    }
    #[doc = "The power down voltage is 1.8 V"]
    #[inline(always)]
    pub fn v1_8(self) -> &'a mut W {
        self.variant(PDRVS_A::V1_8)
    }
}
impl R {
    #[doc = "Bit 0 - Power down voltage select"]
    #[inline(always)]
    pub fn pdrvs(&self) -> PDRVS_R {
        PDRVS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down voltage select"]
    #[inline(always)]
    pub fn pdrvs(&mut self) -> PDRVS_W {
        PDRVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power down voltage select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdvsel](index.html) module"]
pub struct PDVSEL_SPEC;
impl crate::RegisterSpec for PDVSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdvsel::R](R) reader structure"]
impl crate::Readable for PDVSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdvsel::W](W) writer structure"]
impl crate::Writable for PDVSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDVSEL to value 0"]
impl crate::Resettable for PDVSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
