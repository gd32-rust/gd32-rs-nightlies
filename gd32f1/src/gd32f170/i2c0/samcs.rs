#[doc = "Register `SAMCS` reader"]
pub struct R(crate::R<SAMCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMCS` writer"]
pub struct W(crate::W<SAMCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMCS_SPEC>;
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
impl From<crate::W<SAMCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SAM_V interface enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMEN_A {
    #[doc = "0: SAM_V interface disabled"]
    DISABLED = 0,
    #[doc = "1: SAM_V interface enabled"]
    ENABLED = 1,
}
impl From<SAMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMEN` reader - SAM_V interface enable"]
pub type SAMEN_R = crate::BitReader<SAMEN_A>;
impl SAMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMEN_A {
        match self.bits {
            false => SAMEN_A::DISABLED,
            true => SAMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAMEN_A::ENABLED
    }
}
#[doc = "Field `SAMEN` writer - SAM_V interface enable"]
pub type SAMEN_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, SAMEN_A, 0>;
impl<'a> SAMEN_W<'a> {
    #[doc = "SAM_V interface disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAMEN_A::DISABLED)
    }
    #[doc = "SAM_V interface enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAMEN_A::ENABLED)
    }
}
#[doc = "SAM_V interface timeout detect enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOEN_A {
    #[doc = "0: SAM_V interface timeout detect disabled"]
    DISABLED = 0,
    #[doc = "1: SAM_V interface timeout detect enabled"]
    ENABLED = 1,
}
impl From<STOEN_A> for bool {
    #[inline(always)]
    fn from(variant: STOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOEN` reader - SAM_V interface timeout detect enable"]
pub type STOEN_R = crate::BitReader<STOEN_A>;
impl STOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOEN_A {
        match self.bits {
            false => STOEN_A::DISABLED,
            true => STOEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOEN_A::ENABLED
    }
}
#[doc = "Field `STOEN` writer - SAM_V interface timeout detect enable"]
pub type STOEN_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, STOEN_A, 1>;
impl<'a> STOEN_W<'a> {
    #[doc = "SAM_V interface timeout detect disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STOEN_A::DISABLED)
    }
    #[doc = "SAM_V interface timeout detect enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STOEN_A::ENABLED)
    }
}
#[doc = "Txframe fall interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFIE_A {
    #[doc = "0: Txframe fall interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Txframe fall interrupt enabled"]
    ENABLED = 1,
}
impl From<TFFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TFFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFIE` reader - Txframe fall interrupt enable"]
pub type TFFIE_R = crate::BitReader<TFFIE_A>;
impl TFFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFIE_A {
        match self.bits {
            false => TFFIE_A::DISABLED,
            true => TFFIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TFFIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TFFIE_A::ENABLED
    }
}
#[doc = "Field `TFFIE` writer - Txframe fall interrupt enable"]
pub type TFFIE_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, TFFIE_A, 4>;
impl<'a> TFFIE_W<'a> {
    #[doc = "Txframe fall interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TFFIE_A::DISABLED)
    }
    #[doc = "Txframe fall interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TFFIE_A::ENABLED)
    }
}
#[doc = "Txframe rise interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFRIE_A {
    #[doc = "0: Txframe rise interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Txframe rise interrupt enabled"]
    ENABLED = 1,
}
impl From<TFRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TFRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFRIE` reader - Txframe rise interrupt enable"]
pub type TFRIE_R = crate::BitReader<TFRIE_A>;
impl TFRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFRIE_A {
        match self.bits {
            false => TFRIE_A::DISABLED,
            true => TFRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TFRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TFRIE_A::ENABLED
    }
}
#[doc = "Field `TFRIE` writer - Txframe rise interrupt enable"]
pub type TFRIE_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, TFRIE_A, 5>;
impl<'a> TFRIE_W<'a> {
    #[doc = "Txframe rise interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TFRIE_A::DISABLED)
    }
    #[doc = "Txframe rise interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TFRIE_A::ENABLED)
    }
}
#[doc = "Rxframe fall interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFFIE_A {
    #[doc = "0: Rxframe fall interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Rxframe fall interrupt enabled"]
    ENABLED = 1,
}
impl From<RFFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RFFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFIE` reader - Rxframe fall interrupt enable"]
pub type RFFIE_R = crate::BitReader<RFFIE_A>;
impl RFFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFFIE_A {
        match self.bits {
            false => RFFIE_A::DISABLED,
            true => RFFIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFFIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFFIE_A::ENABLED
    }
}
#[doc = "Field `RFFIE` writer - Rxframe fall interrupt enable"]
pub type RFFIE_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, RFFIE_A, 6>;
impl<'a> RFFIE_W<'a> {
    #[doc = "Rxframe fall interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFFIE_A::DISABLED)
    }
    #[doc = "Rxframe fall interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFFIE_A::ENABLED)
    }
}
#[doc = "Rxframe rise interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFRIE_A {
    #[doc = "0: Rxframe rise interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Rxframe rise interrupt enabled"]
    ENABLED = 1,
}
impl From<RFRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RFRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFRIE` reader - Rxframe rise interrupt enable"]
pub type RFRIE_R = crate::BitReader<RFRIE_A>;
impl RFRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFRIE_A {
        match self.bits {
            false => RFRIE_A::DISABLED,
            true => RFRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFRIE_A::ENABLED
    }
}
#[doc = "Field `RFRIE` writer - Rxframe rise interrupt enable"]
pub type RFRIE_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, RFRIE_A, 7>;
impl<'a> RFRIE_W<'a> {
    #[doc = "Rxframe rise interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFRIE_A::DISABLED)
    }
    #[doc = "Rxframe rise interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFRIE_A::ENABLED)
    }
}
#[doc = "Field `TXF` reader - Level of Txframe signal"]
pub type TXF_R = crate::BitReader<bool>;
#[doc = "Field `TXF` writer - Level of Txframe signal"]
pub type TXF_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, bool, 8>;
#[doc = "Field `RXF` reader - Level of Rxframe signal"]
pub type RXF_R = crate::BitReader<bool>;
#[doc = "Field `RXF` writer - Level of Rxframe signal"]
pub type RXF_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, bool, 9>;
#[doc = "Field `TFF` reader - Txframe fall flag"]
pub type TFF_R = crate::BitReader<bool>;
#[doc = "Field `TFF` writer - Txframe fall flag"]
pub type TFF_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, bool, 12>;
#[doc = "Field `TFR` reader - Txframe rise flag"]
pub type TFR_R = crate::BitReader<bool>;
#[doc = "Field `TFR` writer - Txframe rise flag"]
pub type TFR_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, bool, 13>;
#[doc = "Field `RFF` reader - Rxframe fall flag"]
pub type RFF_R = crate::BitReader<bool>;
#[doc = "Field `RFF` writer - Rxframe fall flag"]
pub type RFF_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, bool, 14>;
#[doc = "Field `RFR` reader - Rxframe rise flag"]
pub type RFR_R = crate::BitReader<bool>;
#[doc = "Field `RFR` writer - Rxframe rise flag"]
pub type RFR_W<'a> = crate::BitWriter<'a, u32, SAMCS_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    pub fn samen(&self) -> SAMEN_R {
        SAMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    pub fn stoen(&self) -> STOEN_R {
        STOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Txframe fall interrupt enable"]
    #[inline(always)]
    pub fn tffie(&self) -> TFFIE_R {
        TFFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Txframe rise interrupt enable"]
    #[inline(always)]
    pub fn tfrie(&self) -> TFRIE_R {
        TFRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rxframe fall interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RFFIE_R {
        RFFIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rxframe rise interrupt enable"]
    #[inline(always)]
    pub fn rfrie(&self) -> RFRIE_R {
        RFRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of Txframe signal"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of Rxframe signal"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    pub fn rfr(&self) -> RFR_R {
        RFR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    pub fn samen(&mut self) -> SAMEN_W {
        SAMEN_W::new(self)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    pub fn stoen(&mut self) -> STOEN_W {
        STOEN_W::new(self)
    }
    #[doc = "Bit 4 - Txframe fall interrupt enable"]
    #[inline(always)]
    pub fn tffie(&mut self) -> TFFIE_W {
        TFFIE_W::new(self)
    }
    #[doc = "Bit 5 - Txframe rise interrupt enable"]
    #[inline(always)]
    pub fn tfrie(&mut self) -> TFRIE_W {
        TFRIE_W::new(self)
    }
    #[doc = "Bit 6 - Rxframe fall interrupt enable"]
    #[inline(always)]
    pub fn rffie(&mut self) -> RFFIE_W {
        RFFIE_W::new(self)
    }
    #[doc = "Bit 7 - Rxframe rise interrupt enable"]
    #[inline(always)]
    pub fn rfrie(&mut self) -> RFRIE_W {
        RFRIE_W::new(self)
    }
    #[doc = "Bit 8 - Level of Txframe signal"]
    #[inline(always)]
    pub fn txf(&mut self) -> TXF_W {
        TXF_W::new(self)
    }
    #[doc = "Bit 9 - Level of Rxframe signal"]
    #[inline(always)]
    pub fn rxf(&mut self) -> RXF_W {
        RXF_W::new(self)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    pub fn tff(&mut self) -> TFF_W {
        TFF_W::new(self)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    pub fn tfr(&mut self) -> TFR_W {
        TFR_W::new(self)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    pub fn rff(&mut self) -> RFF_W {
        RFF_W::new(self)
    }
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    pub fn rfr(&mut self) -> RFR_W {
        RFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAM Controland status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samcs](index.html) module"]
pub struct SAMCS_SPEC;
impl crate::RegisterSpec for SAMCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [samcs::R](R) reader structure"]
impl crate::Readable for SAMCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [samcs::W](W) writer structure"]
impl crate::Writable for SAMCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMCS to value 0"]
impl crate::Resettable for SAMCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
