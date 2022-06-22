#[doc = "Register `BADDR` reader"]
pub struct R(crate::R<BADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BADDR` writer"]
pub struct W(crate::W<BADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BADDR_SPEC>;
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
impl From<crate::W<BADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAR` reader - Buffer address"]
pub type BAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BAR` writer - Buffer address"]
pub type BAR_W<'a> = crate::FieldWriter<'a, u16, BADDR_SPEC, u16, u16, 13, 3>;
impl R {
    #[doc = "Bits 3:15 - Buffer address"]
    #[inline(always)]
    pub fn bar(&self) -> BAR_R {
        BAR_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Buffer address"]
    #[inline(always)]
    pub fn bar(&mut self) -> BAR_W {
        BAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baddr](index.html) module"]
pub struct BADDR_SPEC;
impl crate::RegisterSpec for BADDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [baddr::R](R) reader structure"]
impl crate::Readable for BADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baddr::W](W) writer structure"]
impl crate::Writable for BADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BADDR to value 0"]
impl crate::Resettable for BADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
