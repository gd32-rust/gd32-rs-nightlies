#[doc = "Register `MTCAR` reader"]
pub type R = crate::R<MTCAR_SPEC>;
#[doc = "Register `MTCAR` writer"]
pub type W = crate::W<MTCAR_SPEC>;
#[doc = "Field `CARL` reader - Counter auto reload value"]
pub type CARL_R = crate::FieldReader<u16>;
#[doc = "Field `CARL` writer - Counter auto reload value"]
pub type CARL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter auto reload value"]
    #[inline(always)]
    pub fn carl(&self) -> CARL_R {
        CARL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter auto reload value"]
    #[inline(always)]
    #[must_use]
    pub fn carl(&mut self) -> CARL_W<MTCAR_SPEC, 0> {
        CARL_W::new(self)
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
#[doc = "SHRTIMER Master_TIMER counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTCAR_SPEC;
impl crate::RegisterSpec for MTCAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtcar::R`](R) reader structure"]
impl crate::Readable for MTCAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtcar::W`](W) writer structure"]
impl crate::Writable for MTCAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTCAR to value 0"]
impl crate::Resettable for MTCAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
