#[doc = "Register `CH6MADDR` reader"]
pub type R = crate::R<CH6MADDR_SPEC>;
#[doc = "Register `CH6MADDR` writer"]
pub type W = crate::W<CH6MADDR_SPEC>;
#[doc = "Field `MADDR` reader - Memory base address"]
pub type MADDR_R = crate::FieldReader<u32>;
#[doc = "Field `MADDR` writer - Memory base address"]
pub type MADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory base address"]
    #[inline(always)]
    pub fn maddr(&self) -> MADDR_R {
        MADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory base address"]
    #[inline(always)]
    #[must_use]
    pub fn maddr(&mut self) -> MADDR_W<CH6MADDR_SPEC, 0> {
        MADDR_W::new(self)
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
#[doc = "Channel 6 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6maddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6maddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH6MADDR_SPEC;
impl crate::RegisterSpec for CH6MADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6maddr::R`](R) reader structure"]
impl crate::Readable for CH6MADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch6maddr::W`](W) writer structure"]
impl crate::Writable for CH6MADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH6MADDR to value 0"]
impl crate::Resettable for CH6MADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}