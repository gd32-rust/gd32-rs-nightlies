#[doc = "Register `MAC_FCTL` reader"]
pub struct R(crate::R<MAC_FCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_FCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_FCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_FCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_FCTL` writer"]
pub struct W(crate::W<MAC_FCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_FCTL_SPEC>;
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
impl From<crate::W<MAC_FCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_FCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLCB_BKPA` reader - Flow control busy/back pressure activate"]
pub type FLCB_BKPA_R = crate::BitReader<bool>;
#[doc = "Field `FLCB_BKPA` writer - Flow control busy/back pressure activate"]
pub type FLCB_BKPA_W<'a> = crate::BitWriter<'a, u32, MAC_FCTL_SPEC, bool, 0>;
#[doc = "Field `TFCEN` reader - Transmit flow control enable"]
pub type TFCEN_R = crate::BitReader<bool>;
#[doc = "Field `TFCEN` writer - Transmit flow control enable"]
pub type TFCEN_W<'a> = crate::BitWriter<'a, u32, MAC_FCTL_SPEC, bool, 1>;
#[doc = "Field `RFCEN` reader - Receive flow control enable"]
pub type RFCEN_R = crate::BitReader<bool>;
#[doc = "Field `RFCEN` writer - Receive flow control enable"]
pub type RFCEN_W<'a> = crate::BitWriter<'a, u32, MAC_FCTL_SPEC, bool, 2>;
#[doc = "Field `UPFDT` reader - Unicast pause frame detect"]
pub type UPFDT_R = crate::BitReader<bool>;
#[doc = "Field `UPFDT` writer - Unicast pause frame detect"]
pub type UPFDT_W<'a> = crate::BitWriter<'a, u32, MAC_FCTL_SPEC, bool, 3>;
#[doc = "Field `PLTS` reader - Pause low threshold"]
pub type PLTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLTS` writer - Pause low threshold"]
pub type PLTS_W<'a> = crate::FieldWriter<'a, u32, MAC_FCTL_SPEC, u8, u8, 2, 4>;
#[doc = "Field `DZQP` reader - Disable Zero-quanta pause"]
pub type DZQP_R = crate::BitReader<bool>;
#[doc = "Field `DZQP` writer - Disable Zero-quanta pause"]
pub type DZQP_W<'a> = crate::BitWriter<'a, u32, MAC_FCTL_SPEC, bool, 7>;
#[doc = "Field `PTM` reader - Pause time"]
pub type PTM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTM` writer - Pause time"]
pub type PTM_W<'a> = crate::FieldWriter<'a, u32, MAC_FCTL_SPEC, u16, u16, 16, 16>;
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
    pub fn flcb_bkpa(&mut self) -> FLCB_BKPA_W {
        FLCB_BKPA_W::new(self)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    pub fn tfcen(&mut self) -> TFCEN_W {
        TFCEN_W::new(self)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    pub fn rfcen(&mut self) -> RFCEN_W {
        RFCEN_W::new(self)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    pub fn upfdt(&mut self) -> UPFDT_W {
        UPFDT_W::new(self)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plts(&mut self) -> PLTS_W {
        PLTS_W::new(self)
    }
    #[doc = "Bit 7 - Disable Zero-quanta pause"]
    #[inline(always)]
    pub fn dzqp(&mut self) -> DZQP_W {
        DZQP_W::new(self)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    pub fn ptm(&mut self) -> PTM_W {
        PTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC flow control register (MAC_FCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_fctl](index.html) module"]
pub struct MAC_FCTL_SPEC;
impl crate::RegisterSpec for MAC_FCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_fctl::R](R) reader structure"]
impl crate::Readable for MAC_FCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_fctl::W](W) writer structure"]
impl crate::Writable for MAC_FCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_FCTL to value 0"]
impl crate::Resettable for MAC_FCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
