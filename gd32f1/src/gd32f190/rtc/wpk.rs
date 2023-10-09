#[doc = "Register `WPK` writer"]
pub type W = crate::W<WPK_SPEC>;
#[doc = "Field `WPK` writer - Write protection key"]
pub type WPK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Write protection key"]
    #[inline(always)]
    #[must_use]
    pub fn wpk(&mut self) -> WPK_W<WPK_SPEC, 0> {
        WPK_W::new(self)
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
#[doc = "Write protection key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPK_SPEC;
impl crate::RegisterSpec for WPK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpk::W`](W) writer structure"]
impl crate::Writable for WPK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPK to value 0"]
impl crate::Resettable for WPK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
