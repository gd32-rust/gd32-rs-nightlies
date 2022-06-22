#[doc = "Register `CWSPOS` reader"]
pub struct R(crate::R<CWSPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWSPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWSPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWSPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWSPOS` writer"]
pub struct W(crate::W<CWSPOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWSPOS_SPEC>;
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
impl From<crate::W<CWSPOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWSPOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WVSP` reader - Window Vertical start position"]
pub type WVSP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WVSP` writer - Window Vertical start position"]
pub type WVSP_W<'a> = crate::FieldWriter<'a, u32, CWSPOS_SPEC, u16, u16, 13, 16>;
#[doc = "Field `WHSP` reader - Window horizontal start position"]
pub type WHSP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WHSP` writer - Window horizontal start position"]
pub type WHSP_W<'a> = crate::FieldWriter<'a, u32, CWSPOS_SPEC, u16, u16, 14, 0>;
impl R {
    #[doc = "Bits 16:28 - Window Vertical start position"]
    #[inline(always)]
    pub fn wvsp(&self) -> WVSP_R {
        WVSP_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 0:13 - Window horizontal start position"]
    #[inline(always)]
    pub fn whsp(&self) -> WHSP_R {
        WHSP_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:28 - Window Vertical start position"]
    #[inline(always)]
    pub fn wvsp(&mut self) -> WVSP_W {
        WVSP_W::new(self)
    }
    #[doc = "Bits 0:13 - Window horizontal start position"]
    #[inline(always)]
    pub fn whsp(&mut self) -> WHSP_W {
        WHSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCI Cropping window start position register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwspos](index.html) module"]
pub struct CWSPOS_SPEC;
impl crate::RegisterSpec for CWSPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwspos::R](R) reader structure"]
impl crate::Readable for CWSPOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwspos::W](W) writer structure"]
impl crate::Writable for CWSPOS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWSPOS to value 0"]
impl crate::Resettable for CWSPOS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
