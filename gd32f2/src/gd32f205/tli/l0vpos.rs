#[doc = "Register `L0VPOS` reader"]
pub struct R(crate::R<L0VPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L0VPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L0VPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L0VPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L0VPOS` writer"]
pub struct W(crate::W<L0VPOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L0VPOS_SPEC>;
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
impl From<crate::W<L0VPOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L0VPOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WTP` reader - Window top position"]
pub type WTP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WTP` writer - Window top position"]
pub type WTP_W<'a> = crate::FieldWriter<'a, u32, L0VPOS_SPEC, u16, u16, 12, 0>;
#[doc = "Field `WBP` reader - Window bottom position"]
pub type WBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WBP` writer - Window bottom position"]
pub type WBP_W<'a> = crate::FieldWriter<'a, u32, L0VPOS_SPEC, u16, u16, 12, 16>;
impl R {
    #[doc = "Bits 0:11 - Window top position"]
    #[inline(always)]
    pub fn wtp(&self) -> WTP_R {
        WTP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window bottom position"]
    #[inline(always)]
    pub fn wbp(&self) -> WBP_R {
        WBP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window top position"]
    #[inline(always)]
    pub fn wtp(&mut self) -> WTP_W {
        WTP_W::new(self)
    }
    #[doc = "Bits 16:27 - Window bottom position"]
    #[inline(always)]
    pub fn wbp(&mut self) -> WBP_W {
        WBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 0 vertical position parameters register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l0vpos](index.html) module"]
pub struct L0VPOS_SPEC;
impl crate::RegisterSpec for L0VPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l0vpos::R](R) reader structure"]
impl crate::Readable for L0VPOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l0vpos::W](W) writer structure"]
impl crate::Writable for L0VPOS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L0VPOS to value 0"]
impl crate::Resettable for L0VPOS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
