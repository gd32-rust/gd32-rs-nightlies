#[doc = "Register `WS` reader"]
pub struct R(crate::R<WS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WS` writer"]
pub struct W(crate::W<WS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WS_SPEC>;
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
impl From<crate::W<WS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "wait state counter register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WSCNT_A {
    #[doc = "0: 0 wait states added"]
    WS0 = 0,
    #[doc = "1: 1 wait state added"]
    WS1 = 1,
    #[doc = "2: 2 wait states added"]
    WS2 = 2,
}
impl From<WSCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: WSCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WSCNT` reader - wait state counter register"]
pub type WSCNT_R = crate::FieldReader<u8, WSCNT_A>;
impl WSCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WSCNT_A> {
        match self.bits {
            0 => Some(WSCNT_A::WS0),
            1 => Some(WSCNT_A::WS1),
            2 => Some(WSCNT_A::WS2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == WSCNT_A::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == WSCNT_A::WS1
    }
    #[doc = "Checks if the value of the field is `WS2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == WSCNT_A::WS2
    }
}
#[doc = "Field `WSCNT` writer - wait state counter register"]
pub type WSCNT_W<'a> = crate::FieldWriter<'a, u32, WS_SPEC, u8, WSCNT_A, 3, 0>;
impl<'a> WSCNT_W<'a> {
    #[doc = "0 wait states added"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(WSCNT_A::WS0)
    }
    #[doc = "1 wait state added"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(WSCNT_A::WS1)
    }
    #[doc = "2 wait states added"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(WSCNT_A::WS2)
    }
}
#[doc = "Field `PFEN` reader - Pre-fetch enable"]
pub type PFEN_R = crate::BitReader<bool>;
#[doc = "Field `PFEN` writer - Pre-fetch enable"]
pub type PFEN_W<'a> = crate::BitWriter<'a, u32, WS_SPEC, bool, 4>;
#[doc = "Field `PGW` reader - Program width to flash memory"]
pub type PGW_R = crate::BitReader<bool>;
#[doc = "Field `PGW` writer - Program width to flash memory"]
pub type PGW_W<'a> = crate::BitWriter<'a, u32, WS_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&self) -> WSCNT_R {
        WSCNT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Pre-fetch enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PFEN_R {
        PFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 15 - Program width to flash memory"]
    #[inline(always)]
    pub fn pgw(&self) -> PGW_R {
        PGW_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&mut self) -> WSCNT_W {
        WSCNT_W::new(self)
    }
    #[doc = "Bit 4 - Pre-fetch enable"]
    #[inline(always)]
    pub fn pfen(&mut self) -> PFEN_W {
        PFEN_W::new(self)
    }
    #[doc = "Bit 15 - Program width to flash memory"]
    #[inline(always)]
    pub fn pgw(&mut self) -> PGW_W {
        PGW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ws](index.html) module"]
pub struct WS_SPEC;
impl crate::RegisterSpec for WS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ws::R](R) reader structure"]
impl crate::Readable for WS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ws::W](W) writer structure"]
impl crate::Writable for WS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WS to value 0"]
impl crate::Resettable for WS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
