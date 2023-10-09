#[doc = "Register `WDT1` reader"]
pub type R = crate::R<WDT1_SPEC>;
#[doc = "Register `WDT1` writer"]
pub type W = crate::W<WDT1_SPEC>;
#[doc = "Field `WDLT1` reader - Analog watchdog 1 low threshold"]
pub type WDLT1_R = crate::FieldReader;
#[doc = "Field `WDLT1` writer - Analog watchdog 1 low threshold"]
pub type WDLT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WDHT1` reader - Analog watchdog 1 high threshold"]
pub type WDHT1_R = crate::FieldReader;
#[doc = "Field `WDHT1` writer - Analog watchdog 1 high threshold"]
pub type WDHT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Analog watchdog 1 low threshold"]
    #[inline(always)]
    pub fn wdlt1(&self) -> WDLT1_R {
        WDLT1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Analog watchdog 1 high threshold"]
    #[inline(always)]
    pub fn wdht1(&self) -> WDHT1_R {
        WDHT1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Analog watchdog 1 low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdlt1(&mut self) -> WDLT1_W<WDT1_SPEC, 0> {
        WDLT1_W::new(self)
    }
    #[doc = "Bits 16:23 - Analog watchdog 1 high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdht1(&mut self) -> WDHT1_W<WDT1_SPEC, 16> {
        WDHT1_W::new(self)
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
#[doc = "Watchdog threshold register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDT1_SPEC;
impl crate::RegisterSpec for WDT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt1::R`](R) reader structure"]
impl crate::Readable for WDT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdt1::W`](W) writer structure"]
impl crate::Writable for WDT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT1 to value 0x00ff_0000"]
impl crate::Resettable for WDT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_0000;
}
