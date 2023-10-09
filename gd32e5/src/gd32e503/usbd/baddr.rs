#[doc = "Register `BADDR` reader"]
pub type R = crate::R<BADDR_SPEC>;
#[doc = "Register `BADDR` writer"]
pub type W = crate::W<BADDR_SPEC>;
#[doc = "Field `BAR` reader - Buffer address"]
pub type BAR_R = crate::FieldReader<u16>;
#[doc = "Field `BAR` writer - Buffer address"]
pub type BAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 3:15 - Buffer address"]
    #[inline(always)]
    pub fn bar(&self) -> BAR_R {
        BAR_R::new((self.bits >> 3) & 0x1fff)
    }
}
impl W {
    #[doc = "Bits 3:15 - Buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn bar(&mut self) -> BAR_W<BADDR_SPEC, 3> {
        BAR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BADDR_SPEC;
impl crate::RegisterSpec for BADDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`baddr::R`](R) reader structure"]
impl crate::Readable for BADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baddr::W`](W) writer structure"]
impl crate::Writable for BADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BADDR to value 0"]
impl crate::Resettable for BADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
