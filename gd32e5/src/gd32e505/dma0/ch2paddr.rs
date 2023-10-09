#[doc = "Register `CH2PADDR` reader"]
pub type R = crate::R<CH2PADDR_SPEC>;
#[doc = "Register `CH2PADDR` writer"]
pub type W = crate::W<CH2PADDR_SPEC>;
#[doc = "Field `PADDR` reader - Peripheral base address"]
pub type PADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PADDR` writer - Peripheral base address"]
pub type PADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral base address"]
    #[inline(always)]
    pub fn paddr(&self) -> PADDR_R {
        PADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral base address"]
    #[inline(always)]
    #[must_use]
    pub fn paddr(&mut self) -> PADDR_W<CH2PADDR_SPEC, 0> {
        PADDR_W::new(self)
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
#[doc = "Channel 2 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2paddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2paddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2PADDR_SPEC;
impl crate::RegisterSpec for CH2PADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2paddr::R`](R) reader structure"]
impl crate::Readable for CH2PADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2paddr::W`](W) writer structure"]
impl crate::Writable for CH2PADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2PADDR to value 0"]
impl crate::Resettable for CH2PADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}