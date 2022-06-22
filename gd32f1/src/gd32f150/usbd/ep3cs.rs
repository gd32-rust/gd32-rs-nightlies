#[doc = "Register `EP3CS` reader"]
pub struct R(crate::R<EP3CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP3CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP3CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP3CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP3CS` writer"]
pub struct W(crate::W<EP3CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP3CS_SPEC>;
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
impl From<crate::W<EP3CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP3CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_AR` reader - Endpoint address"]
pub type EP_AR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP_AR` writer - Endpoint address"]
pub type EP_AR_W<'a> = crate::FieldWriter<'a, u16, EP3CS_SPEC, u8, u8, 4, 0>;
#[doc = "Field `TX_STA` reader - Status bits, for transmission transfers"]
pub type TX_STA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_STA` writer - Status bits, for transmission transfers"]
pub type TX_STA_W<'a> = crate::FieldWriter<'a, u16, EP3CS_SPEC, u8, u8, 2, 4>;
#[doc = "Field `TX_DTG` reader - Transmission Data PID Toggle"]
pub type TX_DTG_R = crate::BitReader<bool>;
#[doc = "Field `TX_DTG` writer - Transmission Data PID Toggle"]
pub type TX_DTG_W<'a> = crate::BitWriter<'a, u16, EP3CS_SPEC, bool, 6>;
#[doc = "Field `TX_ST` reader - Transmission Successful Transfer"]
pub type TX_ST_R = crate::BitReader<bool>;
#[doc = "Field `TX_ST` writer - Transmission Successful Transfer"]
pub type TX_ST_W<'a> = crate::BitWriter<'a, u16, EP3CS_SPEC, bool, 7>;
#[doc = "Field `EP_KCTL` reader - Endpoint kind control"]
pub type EP_KCTL_R = crate::BitReader<bool>;
#[doc = "Field `EP_KCTL` writer - Endpoint kind control"]
pub type EP_KCTL_W<'a> = crate::BitWriter<'a, u16, EP3CS_SPEC, bool, 8>;
#[doc = "Field `EP_CTL` reader - Endpoint type control"]
pub type EP_CTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP_CTL` writer - Endpoint type control"]
pub type EP_CTL_W<'a> = crate::FieldWriter<'a, u16, EP3CS_SPEC, u8, u8, 2, 9>;
#[doc = "Field `SETUP` reader - Setup transaction completed"]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SETUP` writer - Setup transaction completed"]
pub type SETUP_W<'a> = crate::BitWriter<'a, u16, EP3CS_SPEC, bool, 11>;
#[doc = "Field `RX_STA` reader - Reception status bits"]
pub type RX_STA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_STA` writer - Reception status bits"]
pub type RX_STA_W<'a> = crate::FieldWriter<'a, u16, EP3CS_SPEC, u8, u8, 2, 12>;
#[doc = "Field `RX_DTG` reader - Reception Data PID Toggle"]
pub type RX_DTG_R = crate::BitReader<bool>;
#[doc = "Field `RX_DTG` writer - Reception Data PID Toggle"]
pub type RX_DTG_W<'a> = crate::BitWriter<'a, u16, EP3CS_SPEC, bool, 14>;
#[doc = "Field `RX_ST` reader - Reception Successful Transferred"]
pub type RX_ST_R = crate::BitReader<bool>;
#[doc = "Field `RX_ST` writer - Reception Successful Transferred"]
pub type RX_ST_W<'a> = crate::BitWriter<'a, u16, EP3CS_SPEC, bool, 15>;
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
    #[doc = "Bit 6 - Transmission Data PID Toggle"]
    #[inline(always)]
    pub fn tx_dtg(&self) -> TX_DTG_R {
        TX_DTG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission Successful Transfer"]
    #[inline(always)]
    pub fn tx_st(&self) -> TX_ST_R {
        TX_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint kind control"]
    #[inline(always)]
    pub fn ep_kctl(&self) -> EP_KCTL_R {
        EP_KCTL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Endpoint type control"]
    #[inline(always)]
    pub fn ep_ctl(&self) -> EP_CTL_R {
        EP_CTL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Reception status bits"]
    #[inline(always)]
    pub fn rx_sta(&self) -> RX_STA_R {
        RX_STA_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Reception Data PID Toggle"]
    #[inline(always)]
    pub fn rx_dtg(&self) -> RX_DTG_R {
        RX_DTG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reception Successful Transferred"]
    #[inline(always)]
    pub fn rx_st(&self) -> RX_ST_R {
        RX_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ep_ar(&mut self) -> EP_AR_W {
        EP_AR_W::new(self)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn tx_sta(&mut self) -> TX_STA_W {
        TX_STA_W::new(self)
    }
    #[doc = "Bit 6 - Transmission Data PID Toggle"]
    #[inline(always)]
    pub fn tx_dtg(&mut self) -> TX_DTG_W {
        TX_DTG_W::new(self)
    }
    #[doc = "Bit 7 - Transmission Successful Transfer"]
    #[inline(always)]
    pub fn tx_st(&mut self) -> TX_ST_W {
        TX_ST_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint kind control"]
    #[inline(always)]
    pub fn ep_kctl(&mut self) -> EP_KCTL_W {
        EP_KCTL_W::new(self)
    }
    #[doc = "Bits 9:10 - Endpoint type control"]
    #[inline(always)]
    pub fn ep_ctl(&mut self) -> EP_CTL_W {
        EP_CTL_W::new(self)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W::new(self)
    }
    #[doc = "Bits 12:13 - Reception status bits"]
    #[inline(always)]
    pub fn rx_sta(&mut self) -> RX_STA_W {
        RX_STA_W::new(self)
    }
    #[doc = "Bit 14 - Reception Data PID Toggle"]
    #[inline(always)]
    pub fn rx_dtg(&mut self) -> RX_DTG_W {
        RX_DTG_W::new(self)
    }
    #[doc = "Bit 15 - Reception Successful Transferred"]
    #[inline(always)]
    pub fn rx_st(&mut self) -> RX_ST_W {
        RX_ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3cs](index.html) module"]
pub struct EP3CS_SPEC;
impl crate::RegisterSpec for EP3CS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ep3cs::R](R) reader structure"]
impl crate::Readable for EP3CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep3cs::W](W) writer structure"]
impl crate::Writable for EP3CS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP3CS to value 0"]
impl crate::Resettable for EP3CS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
