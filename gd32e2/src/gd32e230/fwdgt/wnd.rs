#[doc = "Register `WND` reader"]
pub type R = crate::R<WND_SPEC>;
#[doc = "Register `WND` writer"]
pub type W = crate::W<WND_SPEC>;
#[doc = "Field `WND` reader - Watchdog counter window value"]
pub type WND_R = crate::FieldReader<u16>;
#[doc = "Field `WND` writer - Watchdog counter window value"]
pub type WND_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&self) -> WND_R {
        WND_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    #[must_use]
    pub fn wnd(&mut self) -> WND_W<WND_SPEC, 0> {
        WND_W::new(self)
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
#[doc = "Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wnd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wnd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WND_SPEC;
impl crate::RegisterSpec for WND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wnd::R`](R) reader structure"]
impl crate::Readable for WND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wnd::W`](W) writer structure"]
impl crate::Writable for WND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WND to value 0x0fff"]
impl crate::Resettable for WND_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
