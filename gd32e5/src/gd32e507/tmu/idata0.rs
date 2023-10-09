#[doc = "Register `IDATA0` reader"]
pub type R = crate::R<IDATA0_SPEC>;
#[doc = "Register `IDATA0` writer"]
pub type W = crate::W<IDATA0_SPEC>;
#[doc = "Field `IDATA0` reader - IDATA0"]
pub type IDATA0_R = crate::FieldReader<u32>;
#[doc = "Field `IDATA0` writer - IDATA0"]
pub type IDATA0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - IDATA0"]
    #[inline(always)]
    pub fn idata0(&self) -> IDATA0_R {
        IDATA0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IDATA0"]
    #[inline(always)]
    #[must_use]
    pub fn idata0(&mut self) -> IDATA0_W<IDATA0_SPEC, 0> {
        IDATA0_W::new(self)
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
#[doc = "Input data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idata0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDATA0_SPEC;
impl crate::RegisterSpec for IDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idata0::R`](R) reader structure"]
impl crate::Readable for IDATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idata0::W`](W) writer structure"]
impl crate::Writable for IDATA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDATA0 to value 0x3f80_0000"]
impl crate::Resettable for IDATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f80_0000;
}
