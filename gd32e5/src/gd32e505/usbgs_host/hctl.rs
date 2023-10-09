#[doc = "Register `HCTL` reader"]
pub type R = crate::R<HCTL_SPEC>;
#[doc = "Register `HCTL` writer"]
pub type W = crate::W<HCTL_SPEC>;
#[doc = "Field `SPDFSLS` reader - Speed limited to FS and LS"]
pub type SPDFSLS_R = crate::BitReader;
#[doc = "Field `SPDFSLS` writer - Speed limited to FS and LS"]
pub type SPDFSLS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Speed limited to FS and LS"]
    #[inline(always)]
    pub fn spdfsls(&self) -> SPDFSLS_R {
        SPDFSLS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Speed limited to FS and LS"]
    #[inline(always)]
    #[must_use]
    pub fn spdfsls(&mut self) -> SPDFSLS_W<HCTL_SPEC, 2> {
        SPDFSLS_W::new(self)
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
#[doc = "host configuration register (HCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTL_SPEC;
impl crate::RegisterSpec for HCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctl::R`](R) reader structure"]
impl crate::Readable for HCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctl::W`](W) writer structure"]
impl crate::Writable for HCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCTL to value 0"]
impl crate::Resettable for HCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
