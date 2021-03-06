#[doc = "Register `BT` reader"]
pub struct R(crate::R<BT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT` writer"]
pub struct W(crate::W<BT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT_SPEC>;
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
impl From<crate::W<BT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Silent communication mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCMOD_A {
    #[doc = "0: Normal operation"]
    NORMAL = 0,
    #[doc = "1: Silent Mode"]
    SILENT = 1,
}
impl From<SCMOD_A> for bool {
    #[inline(always)]
    fn from(variant: SCMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCMOD` reader - Silent communication mode"]
pub type SCMOD_R = crate::BitReader<SCMOD_A>;
impl SCMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCMOD_A {
        match self.bits {
            false => SCMOD_A::NORMAL,
            true => SCMOD_A::SILENT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SCMOD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SILENT`"]
    #[inline(always)]
    pub fn is_silent(&self) -> bool {
        *self == SCMOD_A::SILENT
    }
}
#[doc = "Field `SCMOD` writer - Silent communication mode"]
pub type SCMOD_W<'a> = crate::BitWriter<'a, u32, BT_SPEC, SCMOD_A, 31>;
impl<'a> SCMOD_W<'a> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SCMOD_A::NORMAL)
    }
    #[doc = "Silent Mode"]
    #[inline(always)]
    pub fn silent(self) -> &'a mut W {
        self.variant(SCMOD_A::SILENT)
    }
}
#[doc = "Loopback communication mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCMOD_A {
    #[doc = "0: Loop Back Mode disabled"]
    DISABLED = 0,
    #[doc = "1: Loop Back Mode enabled"]
    ENABLED = 1,
}
impl From<LCMOD_A> for bool {
    #[inline(always)]
    fn from(variant: LCMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCMOD` reader - Loopback communication mode"]
pub type LCMOD_R = crate::BitReader<LCMOD_A>;
impl LCMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCMOD_A {
        match self.bits {
            false => LCMOD_A::DISABLED,
            true => LCMOD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LCMOD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LCMOD_A::ENABLED
    }
}
#[doc = "Field `LCMOD` writer - Loopback communication mode"]
pub type LCMOD_W<'a> = crate::BitWriter<'a, u32, BT_SPEC, LCMOD_A, 30>;
impl<'a> LCMOD_W<'a> {
    #[doc = "Loop Back Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LCMOD_A::DISABLED)
    }
    #[doc = "Loop Back Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LCMOD_A::ENABLED)
    }
}
#[doc = "Field `SJW` reader - Resynchronization jump width"]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - Resynchronization jump width"]
pub type SJW_W<'a> = crate::FieldWriterSafe<'a, u32, BT_SPEC, u8, u8, 2, 24>;
#[doc = "Field `BS2` reader - Bit segment 2"]
pub type BS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BS2` writer - Bit segment 2"]
pub type BS2_W<'a> = crate::FieldWriter<'a, u32, BT_SPEC, u8, u8, 3, 20>;
#[doc = "Field `BS1` reader - Bit segment 1"]
pub type BS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BS1` writer - Bit segment 1"]
pub type BS1_W<'a> = crate::FieldWriter<'a, u32, BT_SPEC, u8, u8, 4, 16>;
#[doc = "Field `BAUDPSC` reader - Baud rate prescaler"]
pub type BAUDPSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BAUDPSC` writer - Baud rate prescaler"]
pub type BAUDPSC_W<'a> = crate::FieldWriter<'a, u32, BT_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bit 31 - Silent communication mode"]
    #[inline(always)]
    pub fn scmod(&self) -> SCMOD_R {
        SCMOD_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Loopback communication mode"]
    #[inline(always)]
    pub fn lcmod(&self) -> LCMOD_R {
        LCMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    pub fn bs2(&self) -> BS2_R {
        BS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    pub fn bs1(&self) -> BS1_R {
        BS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    pub fn baudpsc(&self) -> BAUDPSC_R {
        BAUDPSC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Silent communication mode"]
    #[inline(always)]
    pub fn scmod(&mut self) -> SCMOD_W {
        SCMOD_W::new(self)
    }
    #[doc = "Bit 30 - Loopback communication mode"]
    #[inline(always)]
    pub fn lcmod(&mut self) -> LCMOD_W {
        LCMOD_W::new(self)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W::new(self)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    pub fn bs2(&mut self) -> BS2_W {
        BS2_W::new(self)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    pub fn bs1(&mut self) -> BS1_W {
        BS1_W::new(self)
    }
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    pub fn baudpsc(&mut self) -> BAUDPSC_W {
        BAUDPSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt](index.html) module"]
pub struct BT_SPEC;
impl crate::RegisterSpec for BT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt::R](R) reader structure"]
impl crate::Readable for BT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt::W](W) writer structure"]
impl crate::Writable for BT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BT to value 0x1230_0000"]
impl crate::Resettable for BT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1230_0000
    }
}
