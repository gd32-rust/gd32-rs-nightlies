#[doc = "Register `INTF` reader"]
pub struct R(crate::R<INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTF` writer"]
pub struct W(crate::W<INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAERR` reader - Tx ACK Error flag"]
pub type TAERR_R = crate::BitReader<bool>;
#[doc = "Field `TAERR` writer - Tx ACK Error flag"]
pub type TAERR_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 12>;
#[doc = "Field `TERR` reader - Tx-Error"]
pub type TERR_R = crate::BitReader<bool>;
#[doc = "Field `TERR` writer - Tx-Error"]
pub type TERR_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 11>;
#[doc = "Field `TU` reader - Tx data buffer underrun"]
pub type TU_R = crate::BitReader<bool>;
#[doc = "Field `TU` writer - Tx data buffer underrun"]
pub type TU_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 10>;
#[doc = "Field `TEND` reader - Transmission successfully end"]
pub type TEND_R = crate::BitReader<bool>;
#[doc = "Field `TEND` writer - Transmission successfully end"]
pub type TEND_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 9>;
#[doc = "Field `TBR` reader - Tx-Byte data request"]
pub type TBR_R = crate::BitReader<bool>;
#[doc = "Field `TBR` writer - Tx-Byte data request"]
pub type TBR_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 8>;
#[doc = "Field `LSTARB` reader - Arbitration lost"]
pub type LSTARB_R = crate::BitReader<bool>;
#[doc = "Field `LSTARB` writer - Arbitration lost"]
pub type LSTARB_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 7>;
#[doc = "Field `RAE` reader - Rx ACK Error"]
pub type RAE_R = crate::BitReader<bool>;
#[doc = "Field `RAE` writer - Rx ACK Error"]
pub type RAE_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 6>;
#[doc = "Field `RLBPE` reader - Long Bit Period Error"]
pub type RLBPE_R = crate::BitReader<bool>;
#[doc = "Field `RLBPE` writer - Long Bit Period Error"]
pub type RLBPE_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 5>;
#[doc = "Field `RSBPE` reader - Short Bit Period Error"]
pub type RSBPE_R = crate::BitReader<bool>;
#[doc = "Field `RSBPE` writer - Short Bit Period Error"]
pub type RSBPE_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 4>;
#[doc = "Field `RBRE` reader - Bit Rising Error"]
pub type RBRE_R = crate::BitReader<bool>;
#[doc = "Field `RBRE` writer - Bit Rising Error"]
pub type RBRE_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 3>;
#[doc = "Field `RO` reader - RX Overrun"]
pub type RO_R = crate::BitReader<bool>;
#[doc = "Field `RO` writer - RX Overrun"]
pub type RO_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 2>;
#[doc = "Field `REND` reader - End of Reception"]
pub type REND_R = crate::BitReader<bool>;
#[doc = "Field `REND` writer - End of Reception"]
pub type REND_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 1>;
#[doc = "Field `RBR` reader - Rx-Byte data received"]
pub type RBR_R = crate::BitReader<bool>;
#[doc = "Field `RBR` writer - Rx-Byte data received"]
pub type RBR_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 12 - Tx ACK Error flag"]
    #[inline(always)]
    pub fn taerr(&self) -> TAERR_R {
        TAERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx-Error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx data buffer underrun"]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission successfully end"]
    #[inline(always)]
    pub fn tend(&self) -> TEND_R {
        TEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx-Byte data request"]
    #[inline(always)]
    pub fn tbr(&self) -> TBR_R {
        TBR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration lost"]
    #[inline(always)]
    pub fn lstarb(&self) -> LSTARB_R {
        LSTARB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx ACK Error"]
    #[inline(always)]
    pub fn rae(&self) -> RAE_R {
        RAE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Long Bit Period Error"]
    #[inline(always)]
    pub fn rlbpe(&self) -> RLBPE_R {
        RLBPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Short Bit Period Error"]
    #[inline(always)]
    pub fn rsbpe(&self) -> RSBPE_R {
        RSBPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit Rising Error"]
    #[inline(always)]
    pub fn rbre(&self) -> RBRE_R {
        RBRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Overrun"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - End of Reception"]
    #[inline(always)]
    pub fn rend(&self) -> REND_R {
        REND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Rx-Byte data received"]
    #[inline(always)]
    pub fn rbr(&self) -> RBR_R {
        RBR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Tx ACK Error flag"]
    #[inline(always)]
    pub fn taerr(&mut self) -> TAERR_W {
        TAERR_W::new(self)
    }
    #[doc = "Bit 11 - Tx-Error"]
    #[inline(always)]
    pub fn terr(&mut self) -> TERR_W {
        TERR_W::new(self)
    }
    #[doc = "Bit 10 - Tx data buffer underrun"]
    #[inline(always)]
    pub fn tu(&mut self) -> TU_W {
        TU_W::new(self)
    }
    #[doc = "Bit 9 - Transmission successfully end"]
    #[inline(always)]
    pub fn tend(&mut self) -> TEND_W {
        TEND_W::new(self)
    }
    #[doc = "Bit 8 - Tx-Byte data request"]
    #[inline(always)]
    pub fn tbr(&mut self) -> TBR_W {
        TBR_W::new(self)
    }
    #[doc = "Bit 7 - Arbitration lost"]
    #[inline(always)]
    pub fn lstarb(&mut self) -> LSTARB_W {
        LSTARB_W::new(self)
    }
    #[doc = "Bit 6 - Rx ACK Error"]
    #[inline(always)]
    pub fn rae(&mut self) -> RAE_W {
        RAE_W::new(self)
    }
    #[doc = "Bit 5 - Long Bit Period Error"]
    #[inline(always)]
    pub fn rlbpe(&mut self) -> RLBPE_W {
        RLBPE_W::new(self)
    }
    #[doc = "Bit 4 - Short Bit Period Error"]
    #[inline(always)]
    pub fn rsbpe(&mut self) -> RSBPE_W {
        RSBPE_W::new(self)
    }
    #[doc = "Bit 3 - Bit Rising Error"]
    #[inline(always)]
    pub fn rbre(&mut self) -> RBRE_W {
        RBRE_W::new(self)
    }
    #[doc = "Bit 2 - RX Overrun"]
    #[inline(always)]
    pub fn ro(&mut self) -> RO_W {
        RO_W::new(self)
    }
    #[doc = "Bit 1 - End of Reception"]
    #[inline(always)]
    pub fn rend(&mut self) -> REND_W {
        REND_W::new(self)
    }
    #[doc = "Bit 0 - Rx-Byte data received"]
    #[inline(always)]
    pub fn rbr(&mut self) -> RBR_W {
        RBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](index.html) module"]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intf::R](R) reader structure"]
impl crate::Readable for INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intf::W](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
