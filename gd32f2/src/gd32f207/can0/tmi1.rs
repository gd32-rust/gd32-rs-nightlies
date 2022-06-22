#[doc = "Register `TMI1` reader"]
pub struct R(crate::R<TMI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMI1` writer"]
pub struct W(crate::W<TMI1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMI1_SPEC>;
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
impl From<crate::W<TMI1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMI1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFID_EFID` reader - The frame identifier"]
pub type SFID_EFID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SFID_EFID` writer - The frame identifier"]
pub type SFID_EFID_W<'a> = crate::FieldWriter<'a, u32, TMI1_SPEC, u16, u16, 11, 21>;
#[doc = "Field `EFID` reader - The frame identifier"]
pub type EFID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EFID` writer - The frame identifier"]
pub type EFID_W<'a> = crate::FieldWriter<'a, u32, TMI1_SPEC, u32, u32, 18, 3>;
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FF_A {
    #[doc = "0: Standard format frame"]
    STANDARD = 0,
    #[doc = "1: Extended format frame"]
    EXTENDED = 1,
}
impl From<FF_A> for bool {
    #[inline(always)]
    fn from(variant: FF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FF` reader - Frame format"]
pub type FF_R = crate::BitReader<FF_A>;
impl FF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FF_A {
        match self.bits {
            false => FF_A::STANDARD,
            true => FF_A::EXTENDED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FF_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == FF_A::EXTENDED
    }
}
#[doc = "Field `FF` writer - Frame format"]
pub type FF_W<'a> = crate::BitWriter<'a, u32, TMI1_SPEC, FF_A, 2>;
impl<'a> FF_W<'a> {
    #[doc = "Standard format frame"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(FF_A::STANDARD)
    }
    #[doc = "Extended format frame"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(FF_A::EXTENDED)
    }
}
#[doc = "Frame type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FT_A {
    #[doc = "0: Data frame"]
    DATA = 0,
    #[doc = "1: Remote frame"]
    REMOTE = 1,
}
impl From<FT_A> for bool {
    #[inline(always)]
    fn from(variant: FT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT` reader - Frame type"]
pub type FT_R = crate::BitReader<FT_A>;
impl FT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FT_A {
        match self.bits {
            false => FT_A::DATA,
            true => FT_A::REMOTE,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == FT_A::DATA
    }
    #[doc = "Checks if the value of the field is `REMOTE`"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == FT_A::REMOTE
    }
}
#[doc = "Field `FT` writer - Frame type"]
pub type FT_W<'a> = crate::BitWriter<'a, u32, TMI1_SPEC, FT_A, 1>;
impl<'a> FT_W<'a> {
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(FT_A::DATA)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn remote(self) -> &'a mut W {
        self.variant(FT_A::REMOTE)
    }
}
#[doc = "Transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN_A {
    #[doc = "0: Transmit disabled"]
    DISABLED = 0,
    #[doc = "1: Transmit enabled"]
    ENABLED = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Transmit enable"]
pub type TEN_R = crate::BitReader<TEN_A>;
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::DISABLED,
            true => TEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN_A::ENABLED
    }
}
#[doc = "Field `TEN` writer - Transmit enable"]
pub type TEN_W<'a> = crate::BitWriter<'a, u32, TMI1_SPEC, TEN_A, 0>;
impl<'a> TEN_W<'a> {
    #[doc = "Transmit disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN_A::DISABLED)
    }
    #[doc = "Transmit enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SFID_EFID_R {
        SFID_EFID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EFID_R {
        EFID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&mut self) -> SFID_EFID_W {
        SFID_EFID_W::new(self)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&mut self) -> EFID_W {
        EFID_W::new(self)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&mut self) -> FF_W {
        FF_W::new(self)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&mut self) -> FT_W {
        FT_W::new(self)
    }
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit mailbox identifier register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmi1](index.html) module"]
pub struct TMI1_SPEC;
impl crate::RegisterSpec for TMI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmi1::R](R) reader structure"]
impl crate::Readable for TMI1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmi1::W](W) writer structure"]
impl crate::Writable for TMI1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMI1 to value 0"]
impl crate::Resettable for TMI1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
