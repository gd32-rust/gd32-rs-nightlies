#[doc = "Register `EP6CS` reader"]
pub type R = crate::R<EP6CS_SPEC>;
#[doc = "Register `EP6CS` writer"]
pub type W = crate::W<EP6CS_SPEC>;
#[doc = "Field `EP_AR` reader - Endpoint address"]
pub type EP_AR_R = crate::FieldReader;
#[doc = "Field `EP_AR` writer - Endpoint address"]
pub type EP_AR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TX_STA` reader - Status bits, for transmission transfers"]
pub type TX_STA_R = crate::FieldReader;
#[doc = "Field `TX_STA` writer - Status bits, for transmission transfers"]
pub type TX_STA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TX_DTG` reader - Transmission Data PID Toggle"]
pub type TX_DTG_R = crate::BitReader;
#[doc = "Field `TX_DTG` writer - Transmission Data PID Toggle"]
pub type TX_DTG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_ST` reader - Transmission Successful Transfer"]
pub type TX_ST_R = crate::BitReader;
#[doc = "Field `TX_ST` writer - Transmission Successful Transfer"]
pub type TX_ST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_KCTL` reader - Endpoint kind control"]
pub type EP_KCTL_R = crate::BitReader;
#[doc = "Field `EP_KCTL` writer - Endpoint kind control"]
pub type EP_KCTL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_CTL` reader - Endpoint type control"]
pub type EP_CTL_R = crate::FieldReader;
#[doc = "Field `EP_CTL` writer - Endpoint type control"]
pub type EP_CTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SETUP` reader - Setup transaction completed"]
pub type SETUP_R = crate::BitReader;
#[doc = "Field `SETUP` writer - Setup transaction completed"]
pub type SETUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_STA` reader - Reception status bits"]
pub type RX_STA_R = crate::FieldReader;
#[doc = "Field `RX_STA` writer - Reception status bits"]
pub type RX_STA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RX_DTG` reader - Reception Data PID Toggle"]
pub type RX_DTG_R = crate::BitReader;
#[doc = "Field `RX_DTG` writer - Reception Data PID Toggle"]
pub type RX_DTG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_ST` reader - Reception Successful Transferred"]
pub type RX_ST_R = crate::BitReader;
#[doc = "Field `RX_ST` writer - Reception Successful Transferred"]
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
    #[must_use]
    pub fn ep_ar(&mut self) -> EP_AR_W<EP6CS_SPEC, 0> {
        EP_AR_W::new(self)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn tx_sta(&mut self) -> TX_STA_W<EP6CS_SPEC, 4> {
        TX_STA_W::new(self)
    }
    #[doc = "Bit 6 - Transmission Data PID Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dtg(&mut self) -> TX_DTG_W<EP6CS_SPEC, 6> {
        TX_DTG_W::new(self)
    }
    #[doc = "Bit 7 - Transmission Successful Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn tx_st(&mut self) -> TX_ST_W<EP6CS_SPEC, 7> {
        TX_ST_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint kind control"]
    #[inline(always)]
    #[must_use]
    pub fn ep_kctl(&mut self) -> EP_KCTL_W<EP6CS_SPEC, 8> {
        EP_KCTL_W::new(self)
    }
    #[doc = "Bits 9:10 - Endpoint type control"]
    #[inline(always)]
    #[must_use]
    pub fn ep_ctl(&mut self) -> EP_CTL_W<EP6CS_SPEC, 9> {
        EP_CTL_W::new(self)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<EP6CS_SPEC, 11> {
        SETUP_W::new(self)
    }
    #[doc = "Bits 12:13 - Reception status bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sta(&mut self) -> RX_STA_W<EP6CS_SPEC, 12> {
        RX_STA_W::new(self)
    }
    #[doc = "Bit 14 - Reception Data PID Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dtg(&mut self) -> RX_DTG_W<EP6CS_SPEC, 14> {
        RX_DTG_W::new(self)
    }
    #[doc = "Bit 15 - Reception Successful Transferred"]
    #[inline(always)]
    #[must_use]
    pub fn rx_st(&mut self) -> RX_ST_W<EP6CS_SPEC, 15> {
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
#[doc = "endpoint 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP6CS_SPEC;
impl crate::RegisterSpec for EP6CS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep6cs::R`](R) reader structure"]
impl crate::Readable for EP6CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep6cs::W`](W) writer structure"]
impl crate::Writable for EP6CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP6CS to value 0"]
impl crate::Resettable for EP6CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
