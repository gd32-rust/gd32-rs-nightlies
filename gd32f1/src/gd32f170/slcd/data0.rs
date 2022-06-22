#[doc = "Register `DATA0` reader"]
pub struct R(crate::R<DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA0` writer"]
pub struct W(crate::W<DATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA0_SPEC>;
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
impl From<crate::W<DATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEG_DATA0` reader - Each bit corresponds to one segment to display"]
pub type SEG_DATA0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEG_DATA0` writer - Each bit corresponds to one segment to display"]
pub type SEG_DATA0_W<'a> = crate::FieldWriter<'a, u32, DATA0_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data0(&self) -> SEG_DATA0_R {
        SEG_DATA0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data0(&mut self) -> SEG_DATA0_W {
        SEG_DATA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLCD display data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0](index.html) module"]
pub struct DATA0_SPEC;
impl crate::RegisterSpec for DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0::R](R) reader structure"]
impl crate::Readable for DATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data0::W](W) writer structure"]
impl crate::Writable for DATA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for DATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
