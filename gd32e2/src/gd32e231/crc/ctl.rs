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
#[doc = "reset bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "1: Resets the DATA register to IDATA, with no effect on FDATA"]
    RESET = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - reset bit"]
pub type RST_R = crate::BitReader<RST_A>;
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RST_A> {
        match self.bits {
            true => Some(RST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RST_A::RESET
    }
}
#[doc = "Field `RST` writer - reset bit"]
pub type RST_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, RST_A, 0>;
impl<'a> RST_W<'a> {
    #[doc = "Resets the DATA register to IDATA, with no effect on FDATA"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RST_A::RESET)
    }
}
#[doc = "Field `PS` reader - Size of polynomial"]
pub type PS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PS` writer - Size of polynomial"]
pub type PS_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, 3>;
#[doc = "Reverse input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REV_I_A {
    #[doc = "0: Bit order not affected"]
    NORMAL = 0,
    #[doc = "1: Bit reversal done by byte"]
    BYTE = 1,
    #[doc = "2: Bit reversal done by half-word"]
    HALFWORD = 2,
    #[doc = "3: Bit reversal done by word"]
    WORD = 3,
}
impl From<REV_I_A> for u8 {
    #[inline(always)]
    fn from(variant: REV_I_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REV_I` reader - Reverse input data"]
pub type REV_I_R = crate::FieldReader<u8, REV_I_A>;
impl REV_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV_I_A {
        match self.bits {
            0 => REV_I_A::NORMAL,
            1 => REV_I_A::BYTE,
            2 => REV_I_A::HALFWORD,
            3 => REV_I_A::WORD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REV_I_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == REV_I_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == REV_I_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == REV_I_A::WORD
    }
}
#[doc = "Field `REV_I` writer - Reverse input data"]
pub type REV_I_W<'a> = crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, REV_I_A, 2, 5>;
impl<'a> REV_I_W<'a> {
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_I_A::NORMAL)
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(REV_I_A::BYTE)
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(REV_I_A::HALFWORD)
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(REV_I_A::WORD)
    }
}
#[doc = "Reverse output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV_O_A {
    #[doc = "0: Bit order not affected"]
    NORMAL = 0,
    #[doc = "1: Bit reversed output"]
    REVERSED = 1,
}
impl From<REV_O_A> for bool {
    #[inline(always)]
    fn from(variant: REV_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV_O` reader - Reverse output data"]
pub type REV_O_R = crate::BitReader<REV_O_A>;
impl REV_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV_O_A {
        match self.bits {
            false => REV_O_A::NORMAL,
            true => REV_O_A::REVERSED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REV_O_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == REV_O_A::REVERSED
    }
}
#[doc = "Field `REV_O` writer - Reverse output data"]
pub type REV_O_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, REV_O_A, 7>;
impl<'a> REV_O_W<'a> {
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_O_A::NORMAL)
    }
    #[doc = "Bit reversed output"]
    #[inline(always)]
    pub fn reversed(self) -> &'a mut W {
        self.variant(REV_O_A::REVERSED)
    }
}
impl R {
    #[doc = "Bit 0 - reset bit"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - Size of polynomial"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    pub fn rev_i(&self) -> REV_I_R {
        REV_I_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn rev_o(&self) -> REV_O_R {
        REV_O_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reset bit"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W::new(self)
    }
    #[doc = "Bits 3:4 - Size of polynomial"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W::new(self)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    pub fn rev_i(&mut self) -> REV_I_W {
        REV_I_W::new(self)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn rev_o(&mut self) -> REV_O_W {
        REV_O_W::new(self)
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
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
