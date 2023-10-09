#[doc = "Register `ST4CAR` reader"]
pub type R = crate::R<ST4CAR_SPEC>;
#[doc = "Register `ST4CAR` writer"]
pub type W = crate::W<ST4CAR_SPEC>;
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
    pub fn carl(&mut self) -> CARL_W<ST4CAR_SPEC, 0> {
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
#[doc = "SHRTIMER Slave_TIMER4 counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4car::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4car::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST4CAR_SPEC;
impl crate::RegisterSpec for ST4CAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st4car::R`](R) reader structure"]
impl crate::Readable for ST4CAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st4car::W`](W) writer structure"]
impl crate::Writable for ST4CAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST4CAR to value 0"]
impl crate::Resettable for ST4CAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
