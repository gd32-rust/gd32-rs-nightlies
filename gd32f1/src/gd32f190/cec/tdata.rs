#[doc = "Register `TDATA` writer"]
pub type W = crate::W<TDATA_SPEC>;
#[doc = "Field `TXDATA` writer - Tx Data register"]
pub type TXDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Tx Data register"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<TDATA_SPEC, 0> {
        TXDATA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDATA_SPEC;
impl crate::RegisterSpec for TDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdata::W`](W) writer structure"]
impl crate::Writable for TDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDATA to value 0"]
impl crate::Resettable for TDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
