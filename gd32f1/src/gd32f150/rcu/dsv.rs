#[doc = "Register `DSV` reader"]
pub struct R(crate::R<DSV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSV` writer"]
pub struct W(crate::W<DSV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSV_SPEC>;
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
impl From<crate::W<DSV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Deep-sleep mode voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSLPVS_A {
    #[doc = "0: The core voltage is 1.2 V in deep-sleep mode"]
    V1_2 = 0,
    #[doc = "1: The core voltage is 1.1 V in deep-sleep mode"]
    V1_1 = 1,
    #[doc = "2: The core voltage is 1.0 V in deep-sleep mode"]
    V1_0 = 2,
    #[doc = "3: The core voltage is 0.9 V in deep-sleep mode"]
    V0_9 = 3,
}
impl From<DSLPVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DSLPVS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DSLPVS` reader - Deep-sleep mode voltage select"]
pub type DSLPVS_R = crate::FieldReader<u8, DSLPVS_A>;
impl DSLPVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSLPVS_A> {
        match self.bits {
            0 => Some(DSLPVS_A::V1_2),
            1 => Some(DSLPVS_A::V1_1),
            2 => Some(DSLPVS_A::V1_0),
            3 => Some(DSLPVS_A::V0_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `V1_2`"]
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        *self == DSLPVS_A::V1_2
    }
    #[doc = "Checks if the value of the field is `V1_1`"]
    #[inline(always)]
    pub fn is_v1_1(&self) -> bool {
        *self == DSLPVS_A::V1_1
    }
    #[doc = "Checks if the value of the field is `V1_0`"]
    #[inline(always)]
    pub fn is_v1_0(&self) -> bool {
        *self == DSLPVS_A::V1_0
    }
    #[doc = "Checks if the value of the field is `V0_9`"]
    #[inline(always)]
    pub fn is_v0_9(&self) -> bool {
        *self == DSLPVS_A::V0_9
    }
}
#[doc = "Field `DSLPVS` writer - Deep-sleep mode voltage select"]
pub type DSLPVS_W<'a> = crate::FieldWriter<'a, u32, DSV_SPEC, u8, DSLPVS_A, 3, 0>;
impl<'a> DSLPVS_W<'a> {
    #[doc = "The core voltage is 1.2 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut W {
        self.variant(DSLPVS_A::V1_2)
    }
    #[doc = "The core voltage is 1.1 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v1_1(self) -> &'a mut W {
        self.variant(DSLPVS_A::V1_1)
    }
    #[doc = "The core voltage is 1.0 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v1_0(self) -> &'a mut W {
        self.variant(DSLPVS_A::V1_0)
    }
    #[doc = "The core voltage is 0.9 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v0_9(self) -> &'a mut W {
        self.variant(DSLPVS_A::V0_9)
    }
}
impl R {
    #[doc = "Bits 0:2 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&self) -> DSLPVS_R {
        DSLPVS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&mut self) -> DSLPVS_W {
        DSLPVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep-sleep mode voltage register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsv](index.html) module"]
pub struct DSV_SPEC;
impl crate::RegisterSpec for DSV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsv::R](R) reader structure"]
impl crate::Readable for DSV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsv::W](W) writer structure"]
impl crate::Writable for DSV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSV to value 0"]
impl crate::Resettable for DSV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
