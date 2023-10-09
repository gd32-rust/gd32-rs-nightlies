#[doc = "Register `KEY1H` writer"]
pub type W = crate::W<KEY1H_SPEC>;
#[doc = "Field `KEY1H` writer - Key for DES,TDES,AES"]
pub type KEY1H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn key1h(&mut self) -> KEY1H_W<KEY1H_SPEC, 0> {
        KEY1H_W::new(self)
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
#[doc = "CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1h::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY1H_SPEC;
impl crate::RegisterSpec for KEY1H_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key1h::W`](W) writer structure"]
impl crate::Writable for KEY1H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY1H to value 0"]
impl crate::Resettable for KEY1H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
