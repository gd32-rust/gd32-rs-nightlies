#[doc = "Register `CH3CNT` reader"]
pub struct R(crate::R<CH3CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3CNT` writer"]
pub struct W(crate::W<CH3CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3CNT_SPEC>;
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
impl From<crate::W<CH3CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Transfer counter"]
pub type CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT` writer - Transfer counter"]
pub type CNT_W<'a> = crate::FieldWriterSafe<'a, u32, CH3CNT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Transfer counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transfer counter"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel 3 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3cnt](index.html) module"]
pub struct CH3CNT_SPEC;
impl crate::RegisterSpec for CH3CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3cnt::R](R) reader structure"]
impl crate::Readable for CH3CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3cnt::W](W) writer structure"]
impl crate::Writable for CH3CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH3CNT to value 0"]
impl crate::Resettable for CH3CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
