#[doc = "Register `L0FBADDR` reader"]
pub type R = crate::R<L0FBADDR_SPEC>;
#[doc = "Register `L0FBADDR` writer"]
pub type W = crate::W<L0FBADDR_SPEC>;
#[doc = "Field `FBADD` reader - Frame Buffer base Address"]
pub type FBADD_R = crate::FieldReader<u32>;
#[doc = "Field `FBADD` writer - Frame Buffer base Address"]
pub type FBADD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Frame Buffer base Address"]
    #[inline(always)]
    pub fn fbadd(&self) -> FBADD_R {
        FBADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frame Buffer base Address"]
    #[inline(always)]
    #[must_use]
    pub fn fbadd(&mut self) -> FBADD_W<L0FBADDR_SPEC, 0> {
        FBADD_W::new(self)
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
#[doc = "Layer 0 frame base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0fbaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0fbaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0FBADDR_SPEC;
impl crate::RegisterSpec for L0FBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0fbaddr::R`](R) reader structure"]
impl crate::Readable for L0FBADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l0fbaddr::W`](W) writer structure"]
impl crate::Writable for L0FBADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L0FBADDR to value 0"]
impl crate::Resettable for L0FBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
