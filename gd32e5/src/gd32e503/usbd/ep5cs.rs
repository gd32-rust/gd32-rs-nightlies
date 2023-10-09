#[doc = "Register `EP5CS` reader"]
pub type R = crate::R<EP5CS_SPEC>;
#[doc = "Register `EP5CS` writer"]
pub type W = crate::W<EP5CS_SPEC>;
#[doc = "Field `EP_AR` reader - Endpoint address"]
pub type EP_AR_R = crate::FieldReader;
#[doc = "Field `EP_AR` writer - Endpoint address"]
pub type EP_AR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TX_STA` reader - Status bits, for transmission transfers"]
pub type TX_STA_R = crate::FieldReader;
#[doc = "Field `TX_STA` writer - Status bits, for transmission transfers"]
pub type TX_STA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TX_DTG` reader - Data Toggle, for transmission transfers"]
pub type TX_DTG_R = crate::BitReader;
#[doc = "Field `TX_DTG` writer - Data Toggle, for transmission transfers"]
pub type TX_DTG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_ST` reader - Correct Transfer for transmission"]
pub type TX_ST_R = crate::BitReader;
#[doc = "Field `TX_ST` writer - Correct Transfer for transmission"]
pub type TX_ST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_KCTL` reader - Endpoint kind"]
pub type EP_KCTL_R = crate::BitReader;
#[doc = "Field `EP_KCTL` writer - Endpoint kind"]
pub type EP_KCTL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_CTL` reader - Endpoint type"]
pub type EP_CTL_R = crate::FieldReader;
#[doc = "Field `EP_CTL` writer - Endpoint type"]
pub type EP_CTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SETUP` reader - Setup transaction completed"]
pub type SETUP_R = crate::BitReader;
#[doc = "Field `SETUP` writer - Setup transaction completed"]
pub type SETUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_STA` reader - Status bits, for reception transfers"]
pub type RX_STA_R = crate::FieldReader;
#[doc = "Field `RX_STA` writer - Status bits, for reception transfers"]
pub type RX_STA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RX_DTG` reader - Data Toggle, for reception transfers"]
pub type RX_DTG_R = crate::BitReader;
#[doc = "Field `RX_DTG` writer - Data Toggle, for reception transfers"]
pub type RX_DTG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_ST` reader - Correct transfer for reception"]
pub type RX_ST_R = crate::BitReader;
#[doc = "Field `RX_ST` writer - Correct transfer for reception"]
pub type RX_ST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ep_ar(&self) -> EP_AR_R {
        EP_AR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn tx_sta(&self) -> TX_STA_R {
        TX_STA_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn tx_dtg(&self) -> TX_DTG_R {
        TX_DTG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn tx_st(&self) -> TX_ST_R {
        TX_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kctl(&self) -> EP_KCTL_R {
        EP_KCTL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_ctl(&self) -> EP_CTL_R {
        EP_CTL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn rx_sta(&self) -> RX_STA_R {
        RX_STA_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn rx_dtg(&self) -> RX_DTG_R {
        RX_DTG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn rx_st(&self) -> RX_ST_R {
        RX_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    #[must_use]
    pub fn ep_ar(&mut self) -> EP_AR_W<EP5CS_SPEC, 0> {
        EP_AR_W::new(self)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn tx_sta(&mut self) -> TX_STA_W<EP5CS_SPEC, 4> {
        TX_STA_W::new(self)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dtg(&mut self) -> TX_DTG_W<EP5CS_SPEC, 6> {
        TX_DTG_W::new(self)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tx_st(&mut self) -> TX_ST_W<EP5CS_SPEC, 7> {
        TX_ST_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    #[must_use]
    pub fn ep_kctl(&mut self) -> EP_KCTL_W<EP5CS_SPEC, 8> {
        EP_KCTL_W::new(self)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn ep_ctl(&mut self) -> EP_CTL_W<EP5CS_SPEC, 9> {
        EP_CTL_W::new(self)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<EP5CS_SPEC, 11> {
        SETUP_W::new(self)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sta(&mut self) -> RX_STA_W<EP5CS_SPEC, 12> {
        RX_STA_W::new(self)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dtg(&mut self) -> RX_DTG_W<EP5CS_SPEC, 14> {
        RX_DTG_W::new(self)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    #[must_use]
    pub fn rx_st(&mut self) -> RX_ST_W<EP5CS_SPEC, 15> {
        RX_ST_W::new(self)
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
#[doc = "endpoint 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP5CS_SPEC;
impl crate::RegisterSpec for EP5CS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep5cs::R`](R) reader structure"]
impl crate::Readable for EP5CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep5cs::W`](W) writer structure"]
impl crate::Writable for EP5CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP5CS to value 0"]
impl crate::Resettable for EP5CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
