#[doc = "Register `DIFCTL` reader"]
pub type R = crate::R<DIFCTL_SPEC>;
#[doc = "Register `DIFCTL` writer"]
pub type W = crate::W<DIFCTL_SPEC>;
#[doc = "Field `DIFCTL_14_0` reader - Differential mode for channel 14 to 0"]
pub type DIFCTL_14_0_R = crate::FieldReader<u16>;
#[doc = "Field `DIFCTL_14_0` writer - Differential mode for channel 14 to 0"]
pub type DIFCTL_14_0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `DIFCTL_17_15` reader - Differential mode for channel 17 to 15"]
pub type DIFCTL_17_15_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:14 - Differential mode for channel 14 to 0"]
    #[inline(always)]
    pub fn difctl_14_0(&self) -> DIFCTL_14_0_R {
        DIFCTL_14_0_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:17 - Differential mode for channel 17 to 15"]
    #[inline(always)]
    pub fn difctl_17_15(&self) -> DIFCTL_17_15_R {
        DIFCTL_17_15_R::new(((self.bits >> 15) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Differential mode for channel 14 to 0"]
    #[inline(always)]
    #[must_use]
    pub fn difctl_14_0(&mut self) -> DIFCTL_14_0_W<DIFCTL_SPEC, 0> {
        DIFCTL_14_0_W::new(self)
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
#[doc = "Differential mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`difctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`difctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIFCTL_SPEC;
impl crate::RegisterSpec for DIFCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`difctl::R`](R) reader structure"]
impl crate::Readable for DIFCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`difctl::W`](W) writer structure"]
impl crate::Writable for DIFCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIFCTL to value 0"]
impl crate::Resettable for DIFCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
