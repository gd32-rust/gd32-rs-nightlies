#[doc = "Register `DOEP1INTEN` reader"]
pub type R = crate::R<DOEP1INTEN_SPEC>;
#[doc = "Register `DOEP1INTEN` writer"]
pub type W = crate::W<DOEP1INTEN_SPEC>;
#[doc = "Field `TFEN` reader - Transfer finished interrupt enable bit"]
pub type TFEN_R = crate::BitReader;
#[doc = "Field `TFEN` writer - Transfer finished interrupt enable bit"]
pub type TFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDISEN` reader - Endpoint disabled interrupt enable bit"]
pub type EPDISEN_R = crate::BitReader;
#[doc = "Field `EPDISEN` writer - Endpoint disabled interrupt enable bit"]
pub type EPDISEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STPFEN` reader - SETUP phase finished interrupt enable bit"]
pub type STPFEN_R = crate::BitReader;
#[doc = "Field `STPFEN` writer - SETUP phase finished interrupt enable bit"]
pub type STPFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPRXFOVREN` reader - Endpoint Rx FIFO over run interrupt enable bit"]
pub type EPRXFOVREN_R = crate::BitReader;
#[doc = "Field `EPRXFOVREN` writer - Endpoint Rx FIFO over run interrupt enable bit"]
pub type EPRXFOVREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BTBSTPEN` reader - Back-to-back SETUP packets interrupt enable bit"]
pub type BTBSTPEN_R = crate::BitReader;
#[doc = "Field `BTBSTPEN` writer - Back-to-back SETUP packets interrupt enable bit"]
pub type BTBSTPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NYETEN` reader - Send NYET handshake interrupt enable bit"]
pub type NYETEN_R = crate::BitReader;
#[doc = "Field `NYETEN` writer - Send NYET handshake interrupt enable bit"]
pub type NYETEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transfer finished interrupt enable bit"]
    #[inline(always)]
    pub fn tfen(&self) -> TFEN_R {
        TFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable bit"]
    #[inline(always)]
    pub fn epdisen(&self) -> EPDISEN_R {
        EPDISEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable bit"]
    #[inline(always)]
    pub fn stpfen(&self) -> STPFEN_R {
        STPFEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO over run interrupt enable bit"]
    #[inline(always)]
    pub fn eprxfovren(&self) -> EPRXFOVREN_R {
        EPRXFOVREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable bit"]
    #[inline(always)]
    pub fn btbstpen(&self) -> BTBSTPEN_R {
        BTBSTPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - Send NYET handshake interrupt enable bit"]
    #[inline(always)]
    pub fn nyeten(&self) -> NYETEN_R {
        NYETEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tfen(&mut self) -> TFEN_W<DOEP1INTEN_SPEC, 0> {
        TFEN_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn epdisen(&mut self) -> EPDISEN_W<DOEP1INTEN_SPEC, 1> {
        EPDISEN_W::new(self)
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn stpfen(&mut self) -> STPFEN_W<DOEP1INTEN_SPEC, 3> {
        STPFEN_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO over run interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn eprxfovren(&mut self) -> EPRXFOVREN_W<DOEP1INTEN_SPEC, 4> {
        EPRXFOVREN_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn btbstpen(&mut self) -> BTBSTPEN_W<DOEP1INTEN_SPEC, 6> {
        BTBSTPEN_W::new(self)
    }
    #[doc = "Bit 13 - Send NYET handshake interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn nyeten(&mut self) -> NYETEN_W<DOEP1INTEN_SPEC, 13> {
        NYETEN_W::new(self)
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
#[doc = "Device OUT endpoint 1 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEP1INTEN_SPEC;
impl crate::RegisterSpec for DOEP1INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep1inten::R`](R) reader structure"]
impl crate::Readable for DOEP1INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doep1inten::W`](W) writer structure"]
impl crate::Writable for DOEP1INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEP1INTEN to value 0"]
impl crate::Resettable for DOEP1INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
