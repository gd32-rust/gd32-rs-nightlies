#[doc = "Register `WDLT0` reader"]
pub type R = crate::R<WDLT0_SPEC>;
#[doc = "Register `WDLT0` writer"]
pub type W = crate::W<WDLT0_SPEC>;
#[doc = "Field `WDLT0` reader - Analog watchdog 0 lower threshold"]
pub type WDLT0_R = crate::FieldReader<u16>;
#[doc = "Field `WDLT0` writer - Analog watchdog 0 lower threshold"]
pub type WDLT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 0 lower threshold"]
    #[inline(always)]
    pub fn wdlt0(&self) -> WDLT0_R {
        WDLT0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 0 lower threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdlt0(&mut self) -> WDLT0_W<WDLT0_SPEC, 0> {
        WDLT0_W::new(self)
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
#[doc = "watchdog lower threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdlt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdlt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDLT0_SPEC;
impl crate::RegisterSpec for WDLT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdlt0::R`](R) reader structure"]
impl crate::Readable for WDLT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdlt0::W`](W) writer structure"]
impl crate::Writable for WDLT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDLT0 to value 0"]
impl crate::Resettable for WDLT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
