#[doc = "Register `KEY` writer"]
pub type W = crate::W<KEY_SPEC>;
#[doc = "Field `KEY` writer - FMC_CTL unlock register"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - FMC_CTL unlock register"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEY_SPEC, 0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Unlock key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_SPEC;
impl crate::RegisterSpec for KEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY to value 0"]
impl crate::Resettable for KEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
