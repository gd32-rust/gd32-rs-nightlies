#[doc = "Register `DOEP3INTF` reader"]
pub type R = crate::R<Doep3intfSpec>;
#[doc = "Register `DOEP3INTF` writer"]
pub type W = crate::W<Doep3intfSpec>;
#[doc = "Field `TF` reader - Transfer finished"]
pub type TfR = crate::BitReader;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint disabled"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint disabled"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPF` reader - Setup phase finished"]
pub type StpfR = crate::BitReader;
#[doc = "Field `STPF` writer - Setup phase finished"]
pub type StpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRXFOVR` reader - Endpoint Rx FIFO overrun"]
pub type EprxfovrR = crate::BitReader;
#[doc = "Field `EPRXFOVR` writer - Endpoint Rx FIFO overrun"]
pub type EprxfovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTBSTP` reader - Back-to-back SETUP packets"]
pub type BtbstpR = crate::BitReader;
#[doc = "Field `BTBSTP` writer - Back-to-back SETUP packets"]
pub type BtbstpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TfR {
        TfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Setup phase finished"]
    #[inline(always)]
    pub fn stpf(&self) -> StpfR {
        StpfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun"]
    #[inline(always)]
    pub fn eprxfovr(&self) -> EprxfovrR {
        EprxfovrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets"]
    #[inline(always)]
    pub fn btbstp(&self) -> BtbstpR {
        BtbstpR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn tf(&mut self) -> TfW<Doep3intfSpec> {
        TfW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EpdisW<Doep3intfSpec> {
        EpdisW::new(self, 1)
    }
    #[doc = "Bit 3 - Setup phase finished"]
    #[inline(always)]
    #[must_use]
    pub fn stpf(&mut self) -> StpfW<Doep3intfSpec> {
        StpfW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun"]
    #[inline(always)]
    #[must_use]
    pub fn eprxfovr(&mut self) -> EprxfovrW<Doep3intfSpec> {
        EprxfovrW::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets"]
    #[inline(always)]
    #[must_use]
    pub fn btbstp(&mut self) -> BtbstpW<Doep3intfSpec> {
        BtbstpW::new(self, 6)
    }
}
#[doc = "device out endpoint-3 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep3intfSpec;
impl crate::RegisterSpec for Doep3intfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep3intf::R`](R) reader structure"]
impl crate::Readable for Doep3intfSpec {}
#[doc = "`write(|w| ..)` method takes [`doep3intf::W`](W) writer structure"]
impl crate::Writable for Doep3intfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEP3INTF to value 0"]
impl crate::Resettable for Doep3intfSpec {
    const RESET_VALUE: u32 = 0;
}
