#[doc = "Register `LM` reader"]
pub type R = crate::R<LM_SPEC>;
#[doc = "Register `LM` writer"]
pub type W = crate::W<LM_SPEC>;
#[doc = "Field `LM` reader - Line Mark value"]
pub type LM_R = crate::FieldReader<u16>;
#[doc = "Field `LM` writer - Line Mark value"]
pub type LM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - Line Mark value"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Line Mark value"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<LM_SPEC, 0> {
        LM_W::new(self)
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
#[doc = "Line mark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LM_SPEC;
impl crate::RegisterSpec for LM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lm::R`](R) reader structure"]
impl crate::Readable for LM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lm::W`](W) writer structure"]
impl crate::Writable for LM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LM to value 0"]
impl crate::Resettable for LM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
