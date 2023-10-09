#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<INTF_SPEC>;
#[doc = "Capture/compare 0 interrupt flag"]
pub use crate::gd32e230::timer0::intf::CH0IF_A;
#[doc = "Field `CH0IF` reader - Capture/compare 0 interrupt flag"]
pub use crate::gd32e230::timer0::intf::CH0IF_R;
#[doc = "Field `CH0IF` writer - Capture/compare 0 interrupt flag"]
pub use crate::gd32e230::timer0::intf::CH0IF_W;
#[doc = "Capture/Compare 0 overcapture flag"]
pub use crate::gd32e230::timer0::intf::CH0OF_A;
#[doc = "Field `CH0OF` reader - Capture/Compare 0 overcapture flag"]
pub use crate::gd32e230::timer0::intf::CH0OF_R;
#[doc = "Field `CH0OF` writer - Capture/Compare 0 overcapture flag"]
pub use crate::gd32e230::timer0::intf::CH0OF_W;
#[doc = "Update interrupt flag"]
pub use crate::gd32e230::timer0::intf::UPIF_A;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub use crate::gd32e230::timer0::intf::UPIF_R;
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub use crate::gd32e230::timer0::intf::UPIF_W;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UPIF_R {
        UPIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 0 interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> CH0IF_R {
        CH0IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 0 overcapture flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn upif(&mut self) -> UPIF_W<INTF_SPEC, 0> {
        UPIF_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 0 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0if(&mut self) -> CH0IF_W<INTF_SPEC, 1> {
        CH0IF_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 0 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> CH0OF_W<INTF_SPEC, 9> {
        CH0OF_W::new(self)
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
#[doc = "interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}