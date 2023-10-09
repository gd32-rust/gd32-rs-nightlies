#[doc = "Register `WDHT` reader"]
pub type R = crate::R<WDHT_SPEC>;
#[doc = "Register `WDHT` writer"]
pub type W = crate::W<WDHT_SPEC>;
#[doc = "Field `WDHT0` reader - Analog watchdog 0 higher threshold"]
pub type WDHT0_R = crate::FieldReader<u16>;
#[doc = "Field `WDHT0` writer - Analog watchdog 0 higher threshold"]
pub type WDHT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 0 higher threshold"]
    #[inline(always)]
    pub fn wdht0(&self) -> WDHT0_R {
        WDHT0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 0 higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdht0(&mut self) -> WDHT0_W<WDHT_SPEC, 0> {
        WDHT0_W::new(self)
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
#[doc = "watchdog higher threshold register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdht::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdht::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDHT_SPEC;
impl crate::RegisterSpec for WDHT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdht::R`](R) reader structure"]
impl crate::Readable for WDHT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdht::W`](W) writer structure"]
impl crate::Writable for WDHT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDHT to value 0x0fff"]
impl crate::Resettable for WDHT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
