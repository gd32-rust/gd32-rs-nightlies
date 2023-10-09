#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<INTF_SPEC>;
#[doc = "Field `RBR` reader - Rx-Byte data received"]
pub type RBR_R = crate::BitReader;
#[doc = "Field `RBR` writer - Rx-Byte data received"]
pub type RBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REND` reader - End of Reception"]
pub type REND_R = crate::BitReader;
#[doc = "Field `REND` writer - End of Reception"]
pub type REND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RO` reader - RX Overrun"]
pub type RO_R = crate::BitReader;
#[doc = "Field `RO` writer - RX Overrun"]
pub type RO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBRE` reader - Bit Rising Error"]
pub type RBRE_R = crate::BitReader;
#[doc = "Field `RBRE` writer - Bit Rising Error"]
pub type RBRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSBPE` reader - Short Bit Period Error"]
pub type RSBPE_R = crate::BitReader;
#[doc = "Field `RSBPE` writer - Short Bit Period Error"]
pub type RSBPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RLBPE` reader - Long Bit Period Error"]
pub type RLBPE_R = crate::BitReader;
#[doc = "Field `RLBPE` writer - Long Bit Period Error"]
pub type RLBPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAE` reader - Rx ACK Error"]
pub type RAE_R = crate::BitReader;
#[doc = "Field `RAE` writer - Rx ACK Error"]
pub type RAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSTARB` reader - Arbitration lost"]
pub type LSTARB_R = crate::BitReader;
#[doc = "Field `LSTARB` writer - Arbitration lost"]
pub type LSTARB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TBR` reader - Tx-Byte data request"]
pub type TBR_R = crate::BitReader;
#[doc = "Field `TBR` writer - Tx-Byte data request"]
pub type TBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEND` reader - Transmission successfully end"]
pub type TEND_R = crate::BitReader;
#[doc = "Field `TEND` writer - Transmission successfully end"]
pub type TEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TU` reader - Tx data buffer underrun"]
pub type TU_R = crate::BitReader;
#[doc = "Field `TU` writer - Tx data buffer underrun"]
pub type TU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TERR` reader - Tx-Error"]
pub type TERR_R = crate::BitReader;
#[doc = "Field `TERR` writer - Tx-Error"]
pub type TERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAERR` reader - Tx ACK Error flag"]
pub type TAERR_R = crate::BitReader;
#[doc = "Field `TAERR` writer - Tx ACK Error flag"]
pub type TAERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Rx-Byte data received"]
    #[inline(always)]
    pub fn rbr(&self) -> RBR_R {
        RBR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Reception"]
    #[inline(always)]
    pub fn rend(&self) -> REND_R {
        REND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Overrun"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit Rising Error"]
    #[inline(always)]
    pub fn rbre(&self) -> RBRE_R {
        RBRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Short Bit Period Error"]
    #[inline(always)]
    pub fn rsbpe(&self) -> RSBPE_R {
        RSBPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Long Bit Period Error"]
    #[inline(always)]
    pub fn rlbpe(&self) -> RLBPE_R {
        RLBPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx ACK Error"]
    #[inline(always)]
    pub fn rae(&self) -> RAE_R {
        RAE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration lost"]
    #[inline(always)]
    pub fn lstarb(&self) -> LSTARB_R {
        LSTARB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx-Byte data request"]
    #[inline(always)]
    pub fn tbr(&self) -> TBR_R {
        TBR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission successfully end"]
    #[inline(always)]
    pub fn tend(&self) -> TEND_R {
        TEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx data buffer underrun"]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx-Error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx ACK Error flag"]
    #[inline(always)]
    pub fn taerr(&self) -> TAERR_R {
        TAERR_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx-Byte data received"]
    #[inline(always)]
    #[must_use]
    pub fn rbr(&mut self) -> RBR_W<INTF_SPEC, 0> {
        RBR_W::new(self)
    }
    #[doc = "Bit 1 - End of Reception"]
    #[inline(always)]
    #[must_use]
    pub fn rend(&mut self) -> REND_W<INTF_SPEC, 1> {
        REND_W::new(self)
    }
    #[doc = "Bit 2 - RX Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RO_W<INTF_SPEC, 2> {
        RO_W::new(self)
    }
    #[doc = "Bit 3 - Bit Rising Error"]
    #[inline(always)]
    #[must_use]
    pub fn rbre(&mut self) -> RBRE_W<INTF_SPEC, 3> {
        RBRE_W::new(self)
    }
    #[doc = "Bit 4 - Short Bit Period Error"]
    #[inline(always)]
    #[must_use]
    pub fn rsbpe(&mut self) -> RSBPE_W<INTF_SPEC, 4> {
        RSBPE_W::new(self)
    }
    #[doc = "Bit 5 - Long Bit Period Error"]
    #[inline(always)]
    #[must_use]
    pub fn rlbpe(&mut self) -> RLBPE_W<INTF_SPEC, 5> {
        RLBPE_W::new(self)
    }
    #[doc = "Bit 6 - Rx ACK Error"]
    #[inline(always)]
    #[must_use]
    pub fn rae(&mut self) -> RAE_W<INTF_SPEC, 6> {
        RAE_W::new(self)
    }
    #[doc = "Bit 7 - Arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn lstarb(&mut self) -> LSTARB_W<INTF_SPEC, 7> {
        LSTARB_W::new(self)
    }
    #[doc = "Bit 8 - Tx-Byte data request"]
    #[inline(always)]
    #[must_use]
    pub fn tbr(&mut self) -> TBR_W<INTF_SPEC, 8> {
        TBR_W::new(self)
    }
    #[doc = "Bit 9 - Transmission successfully end"]
    #[inline(always)]
    #[must_use]
    pub fn tend(&mut self) -> TEND_W<INTF_SPEC, 9> {
        TEND_W::new(self)
    }
    #[doc = "Bit 10 - Tx data buffer underrun"]
    #[inline(always)]
    #[must_use]
    pub fn tu(&mut self) -> TU_W<INTF_SPEC, 10> {
        TU_W::new(self)
    }
    #[doc = "Bit 11 - Tx-Error"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TERR_W<INTF_SPEC, 11> {
        TERR_W::new(self)
    }
    #[doc = "Bit 12 - Tx ACK Error flag"]
    #[inline(always)]
    #[must_use]
    pub fn taerr(&mut self) -> TAERR_W<INTF_SPEC, 12> {
        TAERR_W::new(self)
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
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
