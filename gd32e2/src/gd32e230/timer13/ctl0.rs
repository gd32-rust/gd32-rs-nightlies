#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Auto-reload preload enable"]
pub use crate::gd32e230::timer0::ctl0::ARSE_A;
#[doc = "Field `ARSE` reader - Auto-reload preload enable"]
pub use crate::gd32e230::timer0::ctl0::ARSE_R;
#[doc = "Field `ARSE` writer - Auto-reload preload enable"]
pub use crate::gd32e230::timer0::ctl0::ARSE_W;
#[doc = "Counter enable"]
pub use crate::gd32e230::timer0::ctl0::CEN_A;
#[doc = "Field `CEN` reader - Counter enable"]
pub use crate::gd32e230::timer0::ctl0::CEN_R;
#[doc = "Field `CEN` writer - Counter enable"]
pub use crate::gd32e230::timer0::ctl0::CEN_W;
#[doc = "Clock division"]
pub use crate::gd32e230::timer0::ctl0::CKDIV_A;
#[doc = "Field `CKDIV` reader - Clock division"]
pub use crate::gd32e230::timer0::ctl0::CKDIV_R;
#[doc = "Field `CKDIV` writer - Clock division"]
pub use crate::gd32e230::timer0::ctl0::CKDIV_W;
#[doc = "Update disable"]
pub use crate::gd32e230::timer0::ctl0::UPDIS_A;
#[doc = "Field `UPDIS` reader - Update disable"]
pub use crate::gd32e230::timer0::ctl0::UPDIS_R;
#[doc = "Field `UPDIS` writer - Update disable"]
pub use crate::gd32e230::timer0::ctl0::UPDIS_W;
#[doc = "Update request source"]
pub use crate::gd32e230::timer0::ctl0::UPS_A;
#[doc = "Field `UPS` reader - Update request source"]
pub use crate::gd32e230::timer0::ctl0::UPS_R;
#[doc = "Field `UPS` writer - Update request source"]
pub use crate::gd32e230::timer0::ctl0::UPS_W;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&self) -> UPDIS_R {
        UPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn ups(&self) -> UPS_R {
        UPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arse(&self) -> ARSE_R {
        ARSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CTL0_SPEC, 0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    #[must_use]
    pub fn updis(&mut self) -> UPDIS_W<CTL0_SPEC, 1> {
        UPDIS_W::new(self)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    #[must_use]
    pub fn ups(&mut self) -> UPS_W<CTL0_SPEC, 2> {
        UPS_W::new(self)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn arse(&mut self) -> ARSE_W<CTL0_SPEC, 7> {
        ARSE_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv(&mut self) -> CKDIV_W<CTL0_SPEC, 8> {
        CKDIV_W::new(self)
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
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
