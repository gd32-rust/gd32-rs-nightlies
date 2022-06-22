#[doc = "Register `CAR` reader"]
pub struct R(crate::R<CAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAR` writer"]
pub struct W(crate::W<CAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAR_SPEC>;
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
impl From<crate::W<CAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAR` reader - Low Auto-reload value"]
pub type CAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAR` writer - Low Auto-reload value"]
pub type CAR_W<'a> = crate::FieldWriter<'a, u32, CAR_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    pub fn car(&self) -> CAR_R {
        CAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    pub fn car(&mut self) -> CAR_W {
        CAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [car](index.html) module"]
pub struct CAR_SPEC;
impl crate::RegisterSpec for CAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [car::R](R) reader structure"]
impl crate::Readable for CAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [car::W](W) writer structure"]
impl crate::Writable for CAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAR to value 0"]
impl crate::Resettable for CAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
