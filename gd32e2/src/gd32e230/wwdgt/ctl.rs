#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Activation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDGTEN_A {
    #[doc = "0: Watchdog disabled"]
    DISABLED = 0,
    #[doc = "1: Watchdog enabled"]
    ENABLED = 1,
}
impl From<WDGTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDGTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDGTEN` reader - Activation bit"]
pub type WDGTEN_R = crate::BitReader<WDGTEN_A>;
impl WDGTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDGTEN_A {
        match self.bits {
            false => WDGTEN_A::DISABLED,
            true => WDGTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDGTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDGTEN_A::ENABLED
    }
}
#[doc = "Field `WDGTEN` writer - Activation bit"]
pub type WDGTEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, WDGTEN_A, 7>;
impl<'a> WDGTEN_W<'a> {
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDGTEN_A::DISABLED)
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDGTEN_A::ENABLED)
    }
}
#[doc = "Field `CNT` reader - 7-bit counter"]
pub type CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` writer - 7-bit counter"]
pub type CNT_W<'a> = crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdgten(&self) -> WDGTEN_R {
        WDGTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:6 - 7-bit counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdgten(&mut self) -> WDGTEN_W {
        WDGTEN_W::new(self)
    }
    #[doc = "Bits 0:6 - 7-bit counter"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0x7f"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
