#[doc = "Register `DI` reader"]
pub type R = crate::R<DI_SPEC>;
#[doc = "Register `DI` writer"]
pub type W = crate::W<DI_SPEC>;
#[doc = "Field `DI` reader - Data input"]
pub type DI_R = crate::FieldReader<u32>;
#[doc = "Field `DI` writer - Data input"]
pub type DI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data input"]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data input"]
    #[inline(always)]
    #[must_use]
    pub fn di(&mut self) -> DI_W<DI_SPEC, 0> {
        DI_W::new(self)
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
#[doc = "CAU data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`di::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`di::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DI_SPEC;
impl crate::RegisterSpec for DI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`di::R`](R) reader structure"]
impl crate::Readable for DI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`di::W`](W) writer structure"]
impl crate::Writable for DI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DI to value 0"]
impl crate::Resettable for DI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
