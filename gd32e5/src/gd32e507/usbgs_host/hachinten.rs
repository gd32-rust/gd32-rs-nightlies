#[doc = "Register `HACHINTEN` reader"]
pub type R = crate::R<HACHINTEN_SPEC>;
#[doc = "Register `HACHINTEN` writer"]
pub type W = crate::W<HACHINTEN_SPEC>;
#[doc = "Field `CINTEN` reader - Channel interrupt enable"]
pub type CINTEN_R = crate::FieldReader<u16>;
#[doc = "Field `CINTEN` writer - Channel interrupt enable"]
pub type CINTEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Channel interrupt enable"]
    #[inline(always)]
    pub fn cinten(&self) -> CINTEN_R {
        CINTEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cinten(&mut self) -> CINTEN_W<HACHINTEN_SPEC, 0> {
        CINTEN_W::new(self)
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
#[doc = "host all channels interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hachinten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hachinten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HACHINTEN_SPEC;
impl crate::RegisterSpec for HACHINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hachinten::R`](R) reader structure"]
impl crate::Readable for HACHINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hachinten::W`](W) writer structure"]
impl crate::Writable for HACHINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HACHINTEN to value 0"]
impl crate::Resettable for HACHINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
