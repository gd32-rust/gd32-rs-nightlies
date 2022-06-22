#[doc = "Register `FCTL` reader"]
pub struct R(crate::R<FCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTL` writer"]
pub struct W(crate::W<FCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTL_SPEC>;
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
impl From<crate::W<FCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HBC1F` reader - Header bank of CAN1 filter"]
pub type HBC1F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HBC1F` writer - Header bank of CAN1 filter"]
pub type HBC1F_W<'a> = crate::FieldWriter<'a, u32, FCTL_SPEC, u8, u8, 6, 8>;
#[doc = "Filter lock disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLD_A {
    #[doc = "0: Filter lock disabled"]
    DISABLED = 0,
    #[doc = "1: Filter lock enabled"]
    ENABLED = 1,
}
impl From<FLD_A> for bool {
    #[inline(always)]
    fn from(variant: FLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLD` reader - Filter lock disable"]
pub type FLD_R = crate::BitReader<FLD_A>;
impl FLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLD_A {
        match self.bits {
            false => FLD_A::DISABLED,
            true => FLD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLD_A::ENABLED
    }
}
#[doc = "Field `FLD` writer - Filter lock disable"]
pub type FLD_W<'a> = crate::BitWriter<'a, u32, FCTL_SPEC, FLD_A, 0>;
impl<'a> FLD_W<'a> {
    #[doc = "Filter lock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLD_A::DISABLED)
    }
    #[doc = "Filter lock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLD_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 8:13 - Header bank of CAN1 filter"]
    #[inline(always)]
    pub fn hbc1f(&self) -> HBC1F_R {
        HBC1F_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 0 - Filter lock disable"]
    #[inline(always)]
    pub fn fld(&self) -> FLD_R {
        FLD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - Header bank of CAN1 filter"]
    #[inline(always)]
    pub fn hbc1f(&mut self) -> HBC1F_W {
        HBC1F_W::new(self)
    }
    #[doc = "Bit 0 - Filter lock disable"]
    #[inline(always)]
    pub fn fld(&mut self) -> FLD_W {
        FLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl](index.html) module"]
pub struct FCTL_SPEC;
impl crate::RegisterSpec for FCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fctl::R](R) reader structure"]
impl crate::Readable for FCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctl::W](W) writer structure"]
impl crate::Writable for FCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCTL to value 0x2a1c_0e01"]
impl crate::Resettable for FCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2a1c_0e01
    }
}
