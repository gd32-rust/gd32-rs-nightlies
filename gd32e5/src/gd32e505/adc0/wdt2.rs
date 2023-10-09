#[doc = "Register `WDT2` reader"]
pub type R = crate::R<WDT2_SPEC>;
#[doc = "Register `WDT2` writer"]
pub type W = crate::W<WDT2_SPEC>;
#[doc = "Field `WDLT2` reader - Analog watchdog 2 low threshold"]
pub type WDLT2_R = crate::FieldReader;
#[doc = "Field `WDLT2` writer - Analog watchdog 2 low threshold"]
pub type WDLT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WDHT2` reader - Analog watchdog 2 high threshold"]
pub type WDHT2_R = crate::FieldReader;
#[doc = "Field `WDHT2` writer - Analog watchdog 2 high threshold"]
pub type WDHT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Analog watchdog 2 low threshold"]
    #[inline(always)]
    pub fn wdlt2(&self) -> WDLT2_R {
        WDLT2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Analog watchdog 2 high threshold"]
    #[inline(always)]
    pub fn wdht2(&self) -> WDHT2_R {
        WDHT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Analog watchdog 2 low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdlt2(&mut self) -> WDLT2_W<WDT2_SPEC, 0> {
        WDLT2_W::new(self)
    }
    #[doc = "Bits 16:23 - Analog watchdog 2 high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdht2(&mut self) -> WDHT2_W<WDT2_SPEC, 16> {
        WDHT2_W::new(self)
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
#[doc = "Watchdog threshold register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDT2_SPEC;
impl crate::RegisterSpec for WDT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt2::R`](R) reader structure"]
impl crate::Readable for WDT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdt2::W`](W) writer structure"]
impl crate::Writable for WDT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT2 to value 0x00ff_0000"]
impl crate::Resettable for WDT2_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_0000;
}
