#[doc = "Register `DATATO` reader"]
pub type R = crate::R<DATATO_SPEC>;
#[doc = "Register `DATATO` writer"]
pub type W = crate::W<DATATO_SPEC>;
#[doc = "Field `DATATO` reader - Data timeout period"]
pub type DATATO_R = crate::FieldReader<u32>;
#[doc = "Field `DATATO` writer - Data timeout period"]
pub type DATATO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn datato(&self) -> DATATO_R {
        DATATO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    #[must_use]
    pub fn datato(&mut self) -> DATATO_W<DATATO_SPEC, 0> {
        DATATO_W::new(self)
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
#[doc = "Data timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datato::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datato::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATATO_SPEC;
impl crate::RegisterSpec for DATATO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datato::R`](R) reader structure"]
impl crate::Readable for DATATO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datato::W`](W) writer structure"]
impl crate::Writable for DATATO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATATO to value 0"]
impl crate::Resettable for DATATO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
