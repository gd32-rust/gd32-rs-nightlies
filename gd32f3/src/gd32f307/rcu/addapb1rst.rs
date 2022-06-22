#[doc = "Register `ADDAPB1RST` reader"]
pub struct R(crate::R<ADDAPB1RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDAPB1RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDAPB1RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDAPB1RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDAPB1RST` writer"]
pub struct W(crate::W<ADDAPB1RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDAPB1RST_SPEC>;
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
impl From<crate::W<ADDAPB1RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDAPB1RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTCRST` reader - CTC reset"]
pub type CTCRST_R = crate::BitReader<bool>;
#[doc = "Field `CTCRST` writer - CTC reset"]
pub type CTCRST_W<'a> = crate::BitWriter<'a, u32, ADDAPB1RST_SPEC, bool, 27>;
impl R {
    #[doc = "Bit 27 - CTC reset"]
    #[inline(always)]
    pub fn ctcrst(&self) -> CTCRST_R {
        CTCRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC reset"]
    #[inline(always)]
    pub fn ctcrst(&mut self) -> CTCRST_W {
        CTCRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 additional reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addapb1rst](index.html) module"]
pub struct ADDAPB1RST_SPEC;
impl crate::RegisterSpec for ADDAPB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addapb1rst::R](R) reader structure"]
impl crate::Readable for ADDAPB1RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addapb1rst::W](W) writer structure"]
impl crate::Writable for ADDAPB1RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDAPB1RST to value 0"]
impl crate::Resettable for ADDAPB1RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
