#[doc = "Register `L1VPOS` reader"]
pub struct R(crate::R<L1VPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1VPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1VPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1VPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1VPOS` writer"]
pub struct W(crate::W<L1VPOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1VPOS_SPEC>;
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
impl From<crate::W<L1VPOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1VPOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WTP` reader - Window top position"]
pub type WTP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WTP` writer - Window top position"]
pub type WTP_W<'a> = crate::FieldWriter<'a, u32, L1VPOS_SPEC, u16, u16, 12, 0>;
#[doc = "Field `WBP` reader - Window bottom position"]
pub type WBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WBP` writer - Window bottom position"]
pub type WBP_W<'a> = crate::FieldWriter<'a, u32, L1VPOS_SPEC, u16, u16, 12, 16>;
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
#[doc = "Layer 1 vertical position parameters register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1vpos](index.html) module"]
pub struct L1VPOS_SPEC;
impl crate::RegisterSpec for L1VPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1vpos::R](R) reader structure"]
impl crate::Readable for L1VPOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1vpos::W](W) writer structure"]
impl crate::Writable for L1VPOS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L1VPOS to value 0"]
impl crate::Resettable for L1VPOS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
