#[doc = "Register `ADDR1` writer"]
pub struct W(crate::W<ADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR1_SPEC>;
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
impl From<crate::W<ADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` writer - Flash erase/program command address bits"]
pub type ADDR_W<'a> = crate::FieldWriterSafe<'a, u32, ADDR1_SPEC, u32, u32, 32, 0>;
impl W {
    #[doc = "Bits 0:31 - Flash erase/program command address bits"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Address register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr1](index.html) module"]
pub struct ADDR1_SPEC;
impl crate::RegisterSpec for ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [addr1::W](W) writer structure"]
impl crate::Writable for ADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR1 to value 0"]
impl crate::Resettable for ADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
