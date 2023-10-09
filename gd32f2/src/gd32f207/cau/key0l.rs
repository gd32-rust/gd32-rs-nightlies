#[doc = "Register `KEY0L` writer"]
pub type W = crate::W<KEY0L_SPEC>;
#[doc = "Field `KEY0L` writer - Key for DES,TDES,AES"]
pub type KEY0L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Key for DES,TDES,AES"]
    #[inline(always)]
    #[must_use]
    pub fn key0l(&mut self) -> KEY0L_W<KEY0L_SPEC, 0> {
        KEY0L_W::new(self)
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
#[doc = "CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0l::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY0L_SPEC;
impl crate::RegisterSpec for KEY0L_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key0l::W`](W) writer structure"]
impl crate::Writable for KEY0L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY0L to value 0"]
impl crate::Resettable for KEY0L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
