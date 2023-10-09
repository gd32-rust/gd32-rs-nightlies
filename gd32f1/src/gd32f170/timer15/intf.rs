#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<INTF_SPEC>;
#[doc = "Break interrupt flag"]
pub use crate::gd32f170::timer0::intf::BRKIF_A;
#[doc = "Field `BRKIF` reader - Break interrupt flag"]
pub use crate::gd32f170::timer0::intf::BRKIF_R;
#[doc = "Field `BRKIF` writer - Break interrupt flag"]
pub use crate::gd32f170::timer0::intf::BRKIF_W;
#[doc = "Channel 0 interrupt flag"]
pub use crate::gd32f170::timer0::intf::CH0IF_A;
#[doc = "Field `CH0IF` reader - Channel 0 interrupt flag"]
pub use crate::gd32f170::timer0::intf::CH0IF_R;
#[doc = "Field `CH0IF` writer - Channel 0 interrupt flag"]
pub use crate::gd32f170::timer0::intf::CH0IF_W;
#[doc = "Channel 0 Capture overflow flag"]
pub use crate::gd32f170::timer0::intf::CH0OF_A;
#[doc = "Field `CH0OF` reader - Channel 0 Capture overflow flag"]
pub use crate::gd32f170::timer0::intf::CH0OF_R;
#[doc = "Field `CH0OF` writer - Channel 0 Capture overflow flag"]
pub use crate::gd32f170::timer0::intf::CH0OF_W;
#[doc = "Channel commutation interrupt flag"]
pub use crate::gd32f170::timer0::intf::CMTIF_A;
#[doc = "Field `CMTIF` reader - Channel commutation interrupt flag"]
pub use crate::gd32f170::timer0::intf::CMTIF_R;
#[doc = "Field `CMTIF` writer - Channel commutation interrupt flag"]
pub use crate::gd32f170::timer0::intf::CMTIF_W;
#[doc = "Update interrupt flag"]
pub use crate::gd32f170::timer0::intf::UPIF_A;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub use crate::gd32f170::timer0::intf::UPIF_R;
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub use crate::gd32f170::timer0::intf::UPIF_W;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UPIF_R {
        UPIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> CH0IF_R {
        CH0IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&self) -> CMTIF_R {
        CMTIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkif(&self) -> BRKIF_R {
        BRKIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
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
    #[doc = "Bit 1 - Channel 0 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0if(&mut self) -> CH0IF_W<INTF_SPEC, 1> {
        CH0IF_W::new(self)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmtif(&mut self) -> CMTIF_W<INTF_SPEC, 5> {
        CMTIF_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn brkif(&mut self) -> BRKIF_W<INTF_SPEC, 7> {
        BRKIF_W::new(self)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u16;
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
