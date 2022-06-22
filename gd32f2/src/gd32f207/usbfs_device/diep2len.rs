#[doc = "Register `DIEP2LEN` reader"]
pub struct R(crate::R<DIEP2LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP2LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP2LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP2LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP2LEN` writer"]
pub struct W(crate::W<DIEP2LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP2LEN_SPEC>;
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
impl From<crate::W<DIEP2LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP2LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCNT` reader - Packet count"]
pub type PCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PCNT_W<'a> = crate::FieldWriter<'a, u32, DIEP2LEN_SPEC, u16, u16, 10, 19>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TLEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TLEN_W<'a> = crate::FieldWriter<'a, u32, DIEP2LEN_SPEC, u32, u32, 19, 0>;
impl R {
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PCNT_W {
        PCNT_W::new(self)
    }
    #[doc = "Bits 0:18 - Transfer length"]
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
#[doc = "device IN endpoint-2 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2len](index.html) module"]
pub struct DIEP2LEN_SPEC;
impl crate::RegisterSpec for DIEP2LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep2len::R](R) reader structure"]
impl crate::Readable for DIEP2LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep2len::W](W) writer structure"]
impl crate::Writable for DIEP2LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEP2LEN to value 0"]
impl crate::Resettable for DIEP2LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
