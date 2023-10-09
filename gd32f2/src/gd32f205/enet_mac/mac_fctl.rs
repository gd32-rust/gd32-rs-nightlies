#[doc = "Register `MAC_FCTL` reader"]
pub type R = crate::R<MAC_FCTL_SPEC>;
#[doc = "Register `MAC_FCTL` writer"]
pub type W = crate::W<MAC_FCTL_SPEC>;
#[doc = "Field `FLCB_BKPA` reader - Flow control busy/back pressure activate"]
pub type FLCB_BKPA_R = crate::BitReader;
#[doc = "Field `FLCB_BKPA` writer - Flow control busy/back pressure activate"]
pub type FLCB_BKPA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFCEN` reader - Transmit flow control enable"]
pub type TFCEN_R = crate::BitReader;
#[doc = "Field `TFCEN` writer - Transmit flow control enable"]
pub type TFCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFCEN` reader - Receive flow control enable"]
pub type RFCEN_R = crate::BitReader;
#[doc = "Field `RFCEN` writer - Receive flow control enable"]
pub type RFCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPFDT` reader - Unicast pause frame detect"]
pub type UPFDT_R = crate::BitReader;
#[doc = "Field `UPFDT` writer - Unicast pause frame detect"]
pub type UPFDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLTS` reader - Pause low threshold"]
pub type PLTS_R = crate::FieldReader;
#[doc = "Field `PLTS` writer - Pause low threshold"]
pub type PLTS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DZQP` reader - Disable Zero-quanta pause"]
pub type DZQP_R = crate::BitReader;
#[doc = "Field `DZQP` writer - Disable Zero-quanta pause"]
pub type DZQP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTM` reader - Pause time"]
pub type PTM_R = crate::FieldReader<u16>;
#[doc = "Field `PTM` writer - Pause time"]
pub type PTM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn flcb_bkpa(&self) -> FLCB_BKPA_R {
        FLCB_BKPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    pub fn tfcen(&self) -> TFCEN_R {
        TFCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    pub fn rfcen(&self) -> RFCEN_R {
        RFCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    pub fn upfdt(&self) -> UPFDT_R {
        UPFDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plts(&self) -> PLTS_R {
        PLTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-quanta pause"]
    #[inline(always)]
    pub fn dzqp(&self) -> DZQP_R {
        DZQP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    pub fn ptm(&self) -> PTM_R {
        PTM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    #[must_use]
    pub fn flcb_bkpa(&mut self) -> FLCB_BKPA_W<MAC_FCTL_SPEC, 0> {
        FLCB_BKPA_W::new(self)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfcen(&mut self) -> TFCEN_W<MAC_FCTL_SPEC, 1> {
        TFCEN_W::new(self)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfcen(&mut self) -> RFCEN_W<MAC_FCTL_SPEC, 2> {
        RFCEN_W::new(self)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    #[must_use]
    pub fn upfdt(&mut self) -> UPFDT_W<MAC_FCTL_SPEC, 3> {
        UPFDT_W::new(self)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plts(&mut self) -> PLTS_W<MAC_FCTL_SPEC, 4> {
        PLTS_W::new(self)
    }
    #[doc = "Bit 7 - Disable Zero-quanta pause"]
    #[inline(always)]
    #[must_use]
    pub fn dzqp(&mut self) -> DZQP_W<MAC_FCTL_SPEC, 7> {
        DZQP_W::new(self)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    #[must_use]
    pub fn ptm(&mut self) -> PTM_W<MAC_FCTL_SPEC, 16> {
        PTM_W::new(self)
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
#[doc = "Ethernet MAC flow control register (MAC_FCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_fctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_fctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_FCTL_SPEC;
impl crate::RegisterSpec for MAC_FCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_fctl::R`](R) reader structure"]
impl crate::Readable for MAC_FCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_fctl::W`](W) writer structure"]
impl crate::Writable for MAC_FCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_FCTL to value 0"]
impl crate::Resettable for MAC_FCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
