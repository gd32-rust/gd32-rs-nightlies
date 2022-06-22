#[doc = "Register `IRMP` reader"]
pub struct R(crate::R<IRMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRMP` writer"]
pub struct W(crate::W<IRMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRMP_SPEC>;
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
impl From<crate::W<IRMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 input remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CI0_RMP_A {
    #[doc = "0: Channel 0 input is connected to GPIO"]
    GPIO = 0,
    #[doc = "1: Channel 0 input is connected to RTCCLK"]
    RTCCLK = 1,
    #[doc = "2: Channel 0 input is connected to HXTAL / 32"]
    HXTAL_32 = 2,
    #[doc = "3: Channel 0 input is connected to CKOUTSEL"]
    CKOUTSEL = 3,
}
impl From<CI0_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: CI0_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CI0_RMP` reader - Channel 0 input remap"]
pub type CI0_RMP_R = crate::FieldReader<u8, CI0_RMP_A>;
impl CI0_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CI0_RMP_A {
        match self.bits {
            0 => CI0_RMP_A::GPIO,
            1 => CI0_RMP_A::RTCCLK,
            2 => CI0_RMP_A::HXTAL_32,
            3 => CI0_RMP_A::CKOUTSEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == CI0_RMP_A::GPIO
    }
    #[doc = "Checks if the value of the field is `RTCCLK`"]
    #[inline(always)]
    pub fn is_rtcclk(&self) -> bool {
        *self == CI0_RMP_A::RTCCLK
    }
    #[doc = "Checks if the value of the field is `HXTAL_32`"]
    #[inline(always)]
    pub fn is_hxtal_32(&self) -> bool {
        *self == CI0_RMP_A::HXTAL_32
    }
    #[doc = "Checks if the value of the field is `CKOUTSEL`"]
    #[inline(always)]
    pub fn is_ckoutsel(&self) -> bool {
        *self == CI0_RMP_A::CKOUTSEL
    }
}
#[doc = "Field `CI0_RMP` writer - Channel 0 input remap"]
pub type CI0_RMP_W<'a> = crate::FieldWriterSafe<'a, u16, IRMP_SPEC, u8, CI0_RMP_A, 2, 0>;
impl<'a> CI0_RMP_W<'a> {
    #[doc = "Channel 0 input is connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(CI0_RMP_A::GPIO)
    }
    #[doc = "Channel 0 input is connected to RTCCLK"]
    #[inline(always)]
    pub fn rtcclk(self) -> &'a mut W {
        self.variant(CI0_RMP_A::RTCCLK)
    }
    #[doc = "Channel 0 input is connected to HXTAL / 32"]
    #[inline(always)]
    pub fn hxtal_32(self) -> &'a mut W {
        self.variant(CI0_RMP_A::HXTAL_32)
    }
    #[doc = "Channel 0 input is connected to CKOUTSEL"]
    #[inline(always)]
    pub fn ckoutsel(self) -> &'a mut W {
        self.variant(CI0_RMP_A::CKOUTSEL)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 0 input remap"]
    #[inline(always)]
    pub fn ci0_rmp(&self) -> CI0_RMP_R {
        CI0_RMP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 input remap"]
    #[inline(always)]
    pub fn ci0_rmp(&mut self) -> CI0_RMP_W {
        CI0_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel input remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irmp](index.html) module"]
pub struct IRMP_SPEC;
impl crate::RegisterSpec for IRMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [irmp::R](R) reader structure"]
impl crate::Readable for IRMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irmp::W](W) writer structure"]
impl crate::Writable for IRMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRMP to value 0"]
impl crate::Resettable for IRMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
