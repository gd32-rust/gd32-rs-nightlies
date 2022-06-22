#[doc = "Register `CH2CV` reader"]
pub struct R(crate::R<CH2CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CV` writer"]
pub struct W(crate::W<CH2CV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CV_SPEC>;
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
impl From<crate::W<CH2CV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH2VAL` reader - Capture/Compare 2 value"]
pub type CH2VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH2VAL` writer - Capture/Compare 2 value"]
pub type CH2VAL_W<'a> = crate::FieldWriter<'a, u32, CH2CV_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 2 value"]
    #[inline(always)]
    pub fn ch2val(&self) -> CH2VAL_R {
        CH2VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 2 value"]
    #[inline(always)]
    pub fn ch2val(&mut self) -> CH2VAL_W {
        CH2VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cv](index.html) module"]
pub struct CH2CV_SPEC;
impl crate::RegisterSpec for CH2CV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2cv::R](R) reader structure"]
impl crate::Readable for CH2CV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2cv::W](W) writer structure"]
impl crate::Writable for CH2CV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2CV to value 0"]
impl crate::Resettable for CH2CV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
