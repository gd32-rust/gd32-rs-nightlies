#[doc = "Register `ADDR1` writer"]
pub type W = crate::W<ADDR1_SPEC>;
#[doc = "Field `ADDR` writer - Flash erase/program command address bits"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash erase/program command address bits"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<ADDR1_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Address register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR1_SPEC;
impl crate::RegisterSpec for ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`addr1::W`](W) writer structure"]
impl crate::Writable for ADDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR1 to value 0"]
impl crate::Resettable for ADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
