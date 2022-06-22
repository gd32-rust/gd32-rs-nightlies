#[doc = "Register `TMP2` reader"]
pub struct R(crate::R<TMP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMP2` writer"]
pub struct W(crate::W<TMP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMP2_SPEC>;
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
impl From<crate::W<TMP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS` reader - Time stamp"]
pub type TS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TS` writer - Time stamp"]
pub type TS_W<'a> = crate::FieldWriterSafe<'a, u32, TMP2_SPEC, u16, u16, 16, 16>;
#[doc = "Time stamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEN_A {
    #[doc = "0: Timestamp disabled"]
    DISABLED = 0,
    #[doc = "1: Timestamp enabled"]
    ENABLED = 1,
}
impl From<TSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEN` reader - Time stamp enable"]
pub type TSEN_R = crate::BitReader<TSEN_A>;
impl TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEN_A {
        match self.bits {
            false => TSEN_A::DISABLED,
            true => TSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSEN_A::ENABLED
    }
}
#[doc = "Field `TSEN` writer - Time stamp enable"]
pub type TSEN_W<'a> = crate::BitWriter<'a, u32, TMP2_SPEC, TSEN_A, 8>;
impl<'a> TSEN_W<'a> {
    #[doc = "Timestamp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSEN_A::DISABLED)
    }
    #[doc = "Timestamp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSEN_A::ENABLED)
    }
}
#[doc = "Field `DLENC` reader - Data length code"]
pub type DLENC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLENC` writer - Data length code"]
pub type DLENC_W<'a> = crate::FieldWriterSafe<'a, u32, TMP2_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 8 - Time stamp enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlenc(&self) -> DLENC_R {
        DLENC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W::new(self)
    }
    #[doc = "Bit 8 - Time stamp enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W::new(self)
    }
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlenc(&mut self) -> DLENC_W {
        DLENC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit mailbox property register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmp2](index.html) module"]
pub struct TMP2_SPEC;
impl crate::RegisterSpec for TMP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmp2::R](R) reader structure"]
impl crate::Readable for TMP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmp2::W](W) writer structure"]
impl crate::Writable for TMP2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMP2 to value 0"]
impl crate::Resettable for TMP2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
