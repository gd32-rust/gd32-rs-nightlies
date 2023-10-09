#[doc = "Register `PWRCTL` reader"]
pub type R = crate::R<PWRCTL_SPEC>;
#[doc = "Register `PWRCTL` writer"]
pub type W = crate::W<PWRCTL_SPEC>;
#[doc = "Field `PWRCTL` reader - SDIO power control bits"]
pub type PWRCTL_R = crate::FieldReader;
#[doc = "Field `PWRCTL` writer - SDIO power control bits"]
pub type PWRCTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - SDIO power control bits"]
    #[inline(always)]
    pub fn pwrctl(&self) -> PWRCTL_R {
        PWRCTL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDIO power control bits"]
    #[inline(always)]
    #[must_use]
    pub fn pwrctl(&mut self) -> PWRCTL_W<PWRCTL_SPEC, 0> {
        PWRCTL_W::new(self)
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
#[doc = "Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCTL_SPEC;
impl crate::RegisterSpec for PWRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctl::R`](R) reader structure"]
impl crate::Readable for PWRCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrctl::W`](W) writer structure"]
impl crate::Writable for PWRCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCTL to value 0"]
impl crate::Resettable for PWRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
