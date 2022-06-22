#[doc = "Register `DOEP0LEN` reader"]
pub struct R(crate::R<DOEP0LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP0LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP0LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP0LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP0LEN` writer"]
pub struct W(crate::W<DOEP0LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP0LEN_SPEC>;
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
impl From<crate::W<DOEP0LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP0LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPCNT` reader - SETUP packet count"]
pub type STPCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STPCNT` writer - SETUP packet count"]
pub type STPCNT_W<'a> = crate::FieldWriter<'a, u32, DOEP0LEN_SPEC, u8, u8, 2, 29>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PCNT_R = crate::BitReader<bool>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PCNT_W<'a> = crate::BitWriter<'a, u32, DOEP0LEN_SPEC, bool, 19>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TLEN_W<'a> = crate::FieldWriter<'a, u32, DOEP0LEN_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    pub fn stpcnt(&self) -> STPCNT_R {
        STPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 0:6 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    pub fn stpcnt(&mut self) -> STPCNT_W {
        STPCNT_W::new(self)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PCNT_W {
        PCNT_W::new(self)
    }
    #[doc = "Bits 0:6 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TLEN_W {
        TLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device OUT endpoint-0 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0len](index.html) module"]
pub struct DOEP0LEN_SPEC;
impl crate::RegisterSpec for DOEP0LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep0len::R](R) reader structure"]
impl crate::Readable for DOEP0LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep0len::W](W) writer structure"]
impl crate::Writable for DOEP0LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEP0LEN to value 0"]
impl crate::Resettable for DOEP0LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
