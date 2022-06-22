#[doc = "Register `L0HPOS` reader"]
pub struct R(crate::R<L0HPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L0HPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L0HPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L0HPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L0HPOS` writer"]
pub struct W(crate::W<L0HPOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L0HPOS_SPEC>;
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
impl From<crate::W<L0HPOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L0HPOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLP` reader - Window left position"]
pub type WLP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WLP` writer - Window left position"]
pub type WLP_W<'a> = crate::FieldWriter<'a, u32, L0HPOS_SPEC, u16, u16, 12, 0>;
#[doc = "Field `WRP` reader - Window right position"]
pub type WRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WRP` writer - Window right position"]
pub type WRP_W<'a> = crate::FieldWriter<'a, u32, L0HPOS_SPEC, u16, u16, 12, 16>;
impl R {
    #[doc = "Bits 0:11 - Window left position"]
    #[inline(always)]
    pub fn wlp(&self) -> WLP_R {
        WLP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window right position"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window left position"]
    #[inline(always)]
    pub fn wlp(&mut self) -> WLP_W {
        WLP_W::new(self)
    }
    #[doc = "Bits 16:27 - Window right position"]
    #[inline(always)]
    pub fn wrp(&mut self) -> WRP_W {
        WRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 0 horizontal position parameters register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l0hpos](index.html) module"]
pub struct L0HPOS_SPEC;
impl crate::RegisterSpec for L0HPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l0hpos::R](R) reader structure"]
impl crate::Readable for L0HPOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l0hpos::W](W) writer structure"]
impl crate::Writable for L0HPOS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L0HPOS to value 0"]
impl crate::Resettable for L0HPOS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
