#[doc = "Register `DMATB` writer"]
pub type W = crate::W<DMATB_SPEC>;
#[doc = "Field `DMATB` writer - DMA transfer buffer"]
pub type DMATB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - DMA transfer buffer"]
    #[inline(always)]
    #[must_use]
    pub fn dmatb(&mut self) -> DMATB_W<DMATB_SPEC, 0> {
        DMATB_W::new(self)
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
#[doc = "SHRTIMER DMA transfer buffer register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATB_SPEC;
impl crate::RegisterSpec for DMATB_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmatb::W`](W) writer structure"]
impl crate::Writable for DMATB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATB to value 0"]
impl crate::Resettable for DMATB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
