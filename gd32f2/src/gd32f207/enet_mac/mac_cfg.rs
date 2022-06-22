#[doc = "Register `MAC_CFG` reader"]
pub struct R(crate::R<MAC_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_CFG` writer"]
pub struct W(crate::W<MAC_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_CFG_SPEC>;
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
impl From<crate::W<MAC_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader<bool>;
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 2>;
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader<bool>;
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 3>;
#[doc = "Field `DFC` reader - Deferral check"]
pub type DFC_R = crate::BitReader<bool>;
#[doc = "Field `DFC` writer - Deferral check"]
pub type DFC_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 4>;
#[doc = "Field `BOL` reader - Back-off limit"]
pub type BOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOL` writer - Back-off limit"]
pub type BOL_W<'a> = crate::FieldWriter<'a, u32, MAC_CFG_SPEC, u8, u8, 2, 5>;
#[doc = "Field `APCD` reader - Automatic pad/CRC drop"]
pub type APCD_R = crate::BitReader<bool>;
#[doc = "Field `APCD` writer - Automatic pad/CRC drop"]
pub type APCD_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 7>;
#[doc = "Field `RTD` reader - Retry disable"]
pub type RTD_R = crate::BitReader<bool>;
#[doc = "Field `RTD` writer - Retry disable"]
pub type RTD_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 9>;
#[doc = "Field `IPFCO` reader - IP frame checksum offload"]
pub type IPFCO_R = crate::BitReader<bool>;
#[doc = "Field `IPFCO` writer - IP frame checksum offload"]
pub type IPFCO_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 10>;
#[doc = "Field `DPM` reader - Duplex mode"]
pub type DPM_R = crate::BitReader<bool>;
#[doc = "Field `DPM` writer - Duplex mode"]
pub type DPM_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 11>;
#[doc = "Field `LBM` reader - Loopback mode"]
pub type LBM_R = crate::BitReader<bool>;
#[doc = "Field `LBM` writer - Loopback mode"]
pub type LBM_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 12>;
#[doc = "Field `ROD` reader - Receive own disable"]
pub type ROD_R = crate::BitReader<bool>;
#[doc = "Field `ROD` writer - Receive own disable"]
pub type ROD_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 13>;
#[doc = "Field `SPD` reader - Fast Ethernet speed"]
pub type SPD_R = crate::BitReader<bool>;
#[doc = "Field `SPD` writer - Fast Ethernet speed"]
pub type SPD_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 14>;
#[doc = "Field `CSD` reader - Carrier sense disable"]
pub type CSD_R = crate::BitReader<bool>;
#[doc = "Field `CSD` writer - Carrier sense disable"]
pub type CSD_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 16>;
#[doc = "Field `IGBS` reader - Inter frame gap bit selection"]
pub type IGBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IGBS` writer - Inter frame gap bit selection"]
pub type IGBS_W<'a> = crate::FieldWriter<'a, u32, MAC_CFG_SPEC, u8, u8, 3, 17>;
#[doc = "Field `JBD` reader - Jabber disable"]
pub type JBD_R = crate::BitReader<bool>;
#[doc = "Field `JBD` writer - Jabber disable"]
pub type JBD_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 22>;
#[doc = "Field `WDD` reader - Watchdog disable"]
pub type WDD_R = crate::BitReader<bool>;
#[doc = "Field `WDD` writer - Watchdog disable"]
pub type WDD_W<'a> = crate::BitWriter<'a, u32, MAC_CFG_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dfc(&self) -> DFC_R {
        DFC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bol(&self) -> BOL_R {
        BOL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC drop"]
    #[inline(always)]
    pub fn apcd(&self) -> APCD_R {
        APCD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rtd(&self) -> RTD_R {
        RTD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IP frame checksum offload"]
    #[inline(always)]
    pub fn ipfco(&self) -> IPFCO_R {
        IPFCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dpm(&self) -> DPM_R {
        DPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    pub fn spd(&self) -> SPD_R {
        SPD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Inter frame gap bit selection"]
    #[inline(always)]
    pub fn igbs(&self) -> IGBS_R {
        IGBS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jbd(&self) -> JBD_R {
        JBD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wdd(&self) -> WDD_R {
        WDD_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W {
        REN_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W::new(self)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dfc(&mut self) -> DFC_W {
        DFC_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bol(&mut self) -> BOL_W {
        BOL_W::new(self)
    }
    #[doc = "Bit 7 - Automatic pad/CRC drop"]
    #[inline(always)]
    pub fn apcd(&mut self) -> APCD_W {
        APCD_W::new(self)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rtd(&mut self) -> RTD_W {
        RTD_W::new(self)
    }
    #[doc = "Bit 10 - IP frame checksum offload"]
    #[inline(always)]
    pub fn ipfco(&mut self) -> IPFCO_W {
        IPFCO_W::new(self)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dpm(&mut self) -> DPM_W {
        DPM_W::new(self)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LBM_W {
        LBM_W::new(self)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    pub fn rod(&mut self) -> ROD_W {
        ROD_W::new(self)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    pub fn spd(&mut self) -> SPD_W {
        SPD_W::new(self)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn csd(&mut self) -> CSD_W {
        CSD_W::new(self)
    }
    #[doc = "Bits 17:19 - Inter frame gap bit selection"]
    #[inline(always)]
    pub fn igbs(&mut self) -> IGBS_W {
        IGBS_W::new(self)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jbd(&mut self) -> JBD_W {
        JBD_W::new(self)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wdd(&mut self) -> WDD_W {
        WDD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC configuration register (MAC_CFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_cfg](index.html) module"]
pub struct MAC_CFG_SPEC;
impl crate::RegisterSpec for MAC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_cfg::R](R) reader structure"]
impl crate::Readable for MAC_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_cfg::W](W) writer structure"]
impl crate::Writable for MAC_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_CFG to value 0x8000"]
impl crate::Resettable for MAC_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
