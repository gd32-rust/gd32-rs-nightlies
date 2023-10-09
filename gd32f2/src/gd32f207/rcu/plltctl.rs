#[doc = "Register `PLLTCTL` reader"]
pub type R = crate::R<PLLTCTL_SPEC>;
#[doc = "Register `PLLTCTL` writer"]
pub type W = crate::W<PLLTCTL_SPEC>;
#[doc = "Field `PLLTEN` reader - PLLT enable"]
pub type PLLTEN_R = crate::BitReader;
#[doc = "Field `PLLTEN` writer - PLLT enable"]
pub type PLLTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLTSTB` reader - PLLTclock stabilization flag"]
pub type PLLTSTB_R = crate::BitReader;
impl R {
    #[doc = "Bit 28 - PLLT enable"]
    #[inline(always)]
    pub fn pllten(&self) -> PLLTEN_R {
        PLLTEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLLTclock stabilization flag"]
    #[inline(always)]
    pub fn plltstb(&self) -> PLLTSTB_R {
        PLLTSTB_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - PLLT enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllten(&mut self) -> PLLTEN_W<PLLTCTL_SPEC, 28> {
        PLLTEN_W::new(self)
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
#[doc = "PLLT control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plltctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plltctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLTCTL_SPEC;
impl crate::RegisterSpec for PLLTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plltctl::R`](R) reader structure"]
impl crate::Readable for PLLTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plltctl::W`](W) writer structure"]
impl crate::Writable for PLLTCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLTCTL to value 0"]
impl crate::Resettable for PLLTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
