#[doc = "Register `OVSAMPCTL` reader"]
pub struct R(crate::R<OVSAMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVSAMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVSAMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVSAMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVSAMPCTL` writer"]
pub struct W(crate::W<OVSAMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVSAMPCTL_SPEC>;
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
impl From<crate::W<OVSAMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVSAMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Triggered Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOVS_A {
    #[doc = "0: All oversampled conversions are done consecutively"]
    CONSECUTIVE = 0,
    #[doc = "1: Each oversampled conversion needs a trigger"]
    INDIVIDUAL = 1,
}
impl From<TOVS_A> for bool {
    #[inline(always)]
    fn from(variant: TOVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOVS` reader - Triggered Oversampling"]
pub type TOVS_R = crate::BitReader<TOVS_A>;
impl TOVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOVS_A {
        match self.bits {
            false => TOVS_A::CONSECUTIVE,
            true => TOVS_A::INDIVIDUAL,
        }
    }
    #[doc = "Checks if the value of the field is `CONSECUTIVE`"]
    #[inline(always)]
    pub fn is_consecutive(&self) -> bool {
        *self == TOVS_A::CONSECUTIVE
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL`"]
    #[inline(always)]
    pub fn is_individual(&self) -> bool {
        *self == TOVS_A::INDIVIDUAL
    }
}
#[doc = "Field `TOVS` writer - Triggered Oversampling"]
pub type TOVS_W<'a> = crate::BitWriter<'a, u32, OVSAMPCTL_SPEC, TOVS_A, 9>;
impl<'a> TOVS_W<'a> {
    #[doc = "All oversampled conversions are done consecutively"]
    #[inline(always)]
    pub fn consecutive(self) -> &'a mut W {
        self.variant(TOVS_A::CONSECUTIVE)
    }
    #[doc = "Each oversampled conversion needs a trigger"]
    #[inline(always)]
    pub fn individual(self) -> &'a mut W {
        self.variant(TOVS_A::INDIVIDUAL)
    }
}
#[doc = "Field `OVSS` reader - Oversampling shift"]
pub type OVSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVSS` writer - Oversampling shift"]
pub type OVSS_W<'a> = crate::FieldWriter<'a, u32, OVSAMPCTL_SPEC, u8, u8, 4, 5>;
#[doc = "Oversampling ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVSR_A {
    #[doc = "0: 2x"]
    TIMES2 = 0,
    #[doc = "1: 4x"]
    TIMES4 = 1,
    #[doc = "2: 8x"]
    TIMES8 = 2,
    #[doc = "3: 16x"]
    TIMES16 = 3,
    #[doc = "4: 32x"]
    TIMES32 = 4,
    #[doc = "5: 64x"]
    TIMES64 = 5,
    #[doc = "6: 128x"]
    TIMES128 = 6,
    #[doc = "7: 256x"]
    TIMES256 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OVSR` reader - Oversampling ratio"]
pub type OVSR_R = crate::FieldReader<u8, OVSR_A>;
impl OVSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::TIMES2,
            1 => OVSR_A::TIMES4,
            2 => OVSR_A::TIMES8,
            3 => OVSR_A::TIMES16,
            4 => OVSR_A::TIMES32,
            5 => OVSR_A::TIMES64,
            6 => OVSR_A::TIMES128,
            7 => OVSR_A::TIMES256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMES2`"]
    #[inline(always)]
    pub fn is_times2(&self) -> bool {
        *self == OVSR_A::TIMES2
    }
    #[doc = "Checks if the value of the field is `TIMES4`"]
    #[inline(always)]
    pub fn is_times4(&self) -> bool {
        *self == OVSR_A::TIMES4
    }
    #[doc = "Checks if the value of the field is `TIMES8`"]
    #[inline(always)]
    pub fn is_times8(&self) -> bool {
        *self == OVSR_A::TIMES8
    }
    #[doc = "Checks if the value of the field is `TIMES16`"]
    #[inline(always)]
    pub fn is_times16(&self) -> bool {
        *self == OVSR_A::TIMES16
    }
    #[doc = "Checks if the value of the field is `TIMES32`"]
    #[inline(always)]
    pub fn is_times32(&self) -> bool {
        *self == OVSR_A::TIMES32
    }
    #[doc = "Checks if the value of the field is `TIMES64`"]
    #[inline(always)]
    pub fn is_times64(&self) -> bool {
        *self == OVSR_A::TIMES64
    }
    #[doc = "Checks if the value of the field is `TIMES128`"]
    #[inline(always)]
    pub fn is_times128(&self) -> bool {
        *self == OVSR_A::TIMES128
    }
    #[doc = "Checks if the value of the field is `TIMES256`"]
    #[inline(always)]
    pub fn is_times256(&self) -> bool {
        *self == OVSR_A::TIMES256
    }
}
#[doc = "Field `OVSR` writer - Oversampling ratio"]
pub type OVSR_W<'a> = crate::FieldWriterSafe<'a, u32, OVSAMPCTL_SPEC, u8, OVSR_A, 3, 2>;
impl<'a> OVSR_W<'a> {
    #[doc = "2x"]
    #[inline(always)]
    pub fn times2(self) -> &'a mut W {
        self.variant(OVSR_A::TIMES2)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn times4(self) -> &'a mut W {
        self.variant(OVSR_A::TIMES4)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn times8(self) -> &'a mut W {
        self.variant(OVSR_A::TIMES8)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn times16(self) -> &'a mut W {
        self.variant(OVSR_A::TIMES16)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn times32(self) -> &'a mut W {
        self.variant(OVSR_A::TIMES32)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn times64(self) -> &'a mut W {
        self.variant(OVSR_A::TIMES64)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn times128(self) -> &'a mut W {
        self.variant(OVSR_A::TIMES128)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn times256(self) -> &'a mut W {
        self.variant(OVSR_A::TIMES256)
    }
}
#[doc = "Oversampler Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSEN_A {
    #[doc = "0: Oversampling disabled"]
    DISABLED = 0,
    #[doc = "1: Oversampling enabled"]
    ENABLED = 1,
}
impl From<OVSEN_A> for bool {
    #[inline(always)]
    fn from(variant: OVSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVSEN` reader - Oversampler Enable"]
pub type OVSEN_R = crate::BitReader<OVSEN_A>;
impl OVSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSEN_A {
        match self.bits {
            false => OVSEN_A::DISABLED,
            true => OVSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVSEN_A::ENABLED
    }
}
#[doc = "Field `OVSEN` writer - Oversampler Enable"]
pub type OVSEN_W<'a> = crate::BitWriter<'a, u32, OVSAMPCTL_SPEC, OVSEN_A, 0>;
impl<'a> OVSEN_W<'a> {
    #[doc = "Oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVSEN_A::DISABLED)
    }
    #[doc = "Oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVSEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 0 - Oversampler Enable"]
    #[inline(always)]
    pub fn ovsen(&self) -> OVSEN_R {
        OVSEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W {
        TOVS_W::new(self)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W {
        OVSS_W::new(self)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W {
        OVSR_W::new(self)
    }
    #[doc = "Bit 0 - Oversampler Enable"]
    #[inline(always)]
    pub fn ovsen(&mut self) -> OVSEN_W {
        OVSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "oversample control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovsampctl](index.html) module"]
pub struct OVSAMPCTL_SPEC;
impl crate::RegisterSpec for OVSAMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ovsampctl::R](R) reader structure"]
impl crate::Readable for OVSAMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ovsampctl::W](W) writer structure"]
impl crate::Writable for OVSAMPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVSAMPCTL to value 0"]
impl crate::Resettable for OVSAMPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
