#[doc = "Register `BMCAR` reader"]
pub type R = crate::R<BMCAR_SPEC>;
#[doc = "Register `BMCAR` writer"]
pub type W = crate::W<BMCAR_SPEC>;
#[doc = "Field `BMCARL` reader - Bunch mode counter auto reload value"]
pub type BMCARL_R = crate::FieldReader<u16>;
#[doc = "Field `BMCARL` writer - Bunch mode counter auto reload value"]
pub type BMCARL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Bunch mode counter auto reload value"]
    #[inline(always)]
    pub fn bmcarl(&self) -> BMCARL_R {
        BMCARL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bunch mode counter auto reload value"]
    #[inline(always)]
    #[must_use]
    pub fn bmcarl(&mut self) -> BMCARL_W<BMCAR_SPEC, 0> {
        BMCARL_W::new(self)
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
#[doc = "SHRTIMER bunch mode counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMCAR_SPEC;
impl crate::RegisterSpec for BMCAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmcar::R`](R) reader structure"]
impl crate::Readable for BMCAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmcar::W`](W) writer structure"]
impl crate::Writable for BMCAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMCAR to value 0"]
impl crate::Resettable for BMCAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
