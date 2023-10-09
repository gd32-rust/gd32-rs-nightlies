#[doc = "Register `ST1CREP` reader"]
pub type R = crate::R<ST1CREP_SPEC>;
#[doc = "Register `ST1CREP` writer"]
pub type W = crate::W<ST1CREP_SPEC>;
#[doc = "Field `CREP` reader - Counter repetition value"]
pub type CREP_R = crate::FieldReader;
#[doc = "Field `CREP` writer - Counter repetition value"]
pub type CREP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Counter repetition value"]
    #[inline(always)]
    pub fn crep(&self) -> CREP_R {
        CREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter repetition value"]
    #[inline(always)]
    #[must_use]
    pub fn crep(&mut self) -> CREP_W<ST1CREP_SPEC, 0> {
        CREP_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMER1 counter repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1crep::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1crep::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1CREP_SPEC;
impl crate::RegisterSpec for ST1CREP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1crep::R`](R) reader structure"]
impl crate::Readable for ST1CREP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1crep::W`](W) writer structure"]
impl crate::Writable for ST1CREP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1CREP to value 0"]
impl crate::Resettable for ST1CREP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
