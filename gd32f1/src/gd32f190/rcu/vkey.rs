#[doc = "Register `VKEY` reader"]
pub struct R(crate::R<VKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VKEY` writer"]
pub struct W(crate::W<VKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VKEY_SPEC>;
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
impl From<crate::W<VKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The key of RCU_PDVSEL and RCU_DSV register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum KEY_A {
    #[doc = "439041101: Allow PDVSEL and DSV to be written"]
    ENABLE = 439041101,
}
impl From<KEY_A> for u32 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` reader - The key of RCU_PDVSEL and RCU_DSV register"]
pub type KEY_R = crate::FieldReader<u32, KEY_A>;
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            439041101 => Some(KEY_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == KEY_A::ENABLE
    }
}
#[doc = "Field `KEY` writer - The key of RCU_PDVSEL and RCU_DSV register"]
pub type KEY_W<'a> = crate::FieldWriter<'a, u32, VKEY_SPEC, u32, KEY_A, 32, 0>;
impl<'a> KEY_W<'a> {
    #[doc = "Allow PDVSEL and DSV to be written"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(KEY_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:31 - The key of RCU_PDVSEL and RCU_DSV register"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The key of RCU_PDVSEL and RCU_DSV register"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage key register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vkey](index.html) module"]
pub struct VKEY_SPEC;
impl crate::RegisterSpec for VKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vkey::R](R) reader structure"]
impl crate::Readable for VKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vkey::W](W) writer structure"]
impl crate::Writable for VKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VKEY to value 0"]
impl crate::Resettable for VKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
