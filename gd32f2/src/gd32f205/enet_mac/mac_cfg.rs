#[doc = "Register `MAC_CFG` reader"]
pub type R = crate::R<MAC_CFG_SPEC>;
#[doc = "Register `MAC_CFG` writer"]
pub type W = crate::W<MAC_CFG_SPEC>;
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader;
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader;
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFC` reader - Deferral check"]
pub type DFC_R = crate::BitReader;
#[doc = "Field `DFC` writer - Deferral check"]
pub type DFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOL` reader - Back-off limit"]
pub type BOL_R = crate::FieldReader;
#[doc = "Field `BOL` writer - Back-off limit"]
pub type BOL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `APCD` reader - Automatic pad/CRC drop"]
pub type APCD_R = crate::BitReader;
#[doc = "Field `APCD` writer - Automatic pad/CRC drop"]
pub type APCD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTD` reader - Retry disable"]
pub type RTD_R = crate::BitReader;
#[doc = "Field `RTD` writer - Retry disable"]
pub type RTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IPFCO` reader - IP frame checksum offload"]
pub type IPFCO_R = crate::BitReader;
#[doc = "Field `IPFCO` writer - IP frame checksum offload"]
pub type IPFCO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPM` reader - Duplex mode"]
pub type DPM_R = crate::BitReader;
#[doc = "Field `DPM` writer - Duplex mode"]
pub type DPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LBM` reader - Loopback mode"]
pub type LBM_R = crate::BitReader;
#[doc = "Field `LBM` writer - Loopback mode"]
pub type LBM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROD` reader - Receive own disable"]
pub type ROD_R = crate::BitReader;
#[doc = "Field `ROD` writer - Receive own disable"]
pub type ROD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPD` reader - Fast Ethernet speed"]
pub type SPD_R = crate::BitReader;
#[doc = "Field `SPD` writer - Fast Ethernet speed"]
pub type SPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSD` reader - Carrier sense disable"]
pub type CSD_R = crate::BitReader;
#[doc = "Field `CSD` writer - Carrier sense disable"]
pub type CSD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IGBS` reader - Inter frame gap bit selection"]
pub type IGBS_R = crate::FieldReader;
#[doc = "Field `IGBS` writer - Inter frame gap bit selection"]
pub type IGBS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `JBD` reader - Jabber disable"]
pub type JBD_R = crate::BitReader;
#[doc = "Field `JBD` writer - Jabber disable"]
pub type JBD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDD` reader - Watchdog disable"]
pub type WDD_R = crate::BitReader;
#[doc = "Field `WDD` writer - Watchdog disable"]
pub type WDD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn ren(&mut self) -> REN_W<MAC_CFG_SPEC, 2> {
        REN_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<MAC_CFG_SPEC, 3> {
        TEN_W::new(self)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    #[must_use]
    pub fn dfc(&mut self) -> DFC_W<MAC_CFG_SPEC, 4> {
        DFC_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    #[must_use]
    pub fn bol(&mut self) -> BOL_W<MAC_CFG_SPEC, 5> {
        BOL_W::new(self)
    }
    #[doc = "Bit 7 - Automatic pad/CRC drop"]
    #[inline(always)]
    #[must_use]
    pub fn apcd(&mut self) -> APCD_W<MAC_CFG_SPEC, 7> {
        APCD_W::new(self)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    #[must_use]
    pub fn rtd(&mut self) -> RTD_W<MAC_CFG_SPEC, 9> {
        RTD_W::new(self)
    }
    #[doc = "Bit 10 - IP frame checksum offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipfco(&mut self) -> IPFCO_W<MAC_CFG_SPEC, 10> {
        IPFCO_W::new(self)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn dpm(&mut self) -> DPM_W<MAC_CFG_SPEC, 11> {
        DPM_W::new(self)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LBM_W<MAC_CFG_SPEC, 12> {
        LBM_W::new(self)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    #[must_use]
    pub fn rod(&mut self) -> ROD_W<MAC_CFG_SPEC, 13> {
        ROD_W::new(self)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SPD_W<MAC_CFG_SPEC, 14> {
        SPD_W::new(self)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    #[must_use]
    pub fn csd(&mut self) -> CSD_W<MAC_CFG_SPEC, 16> {
        CSD_W::new(self)
    }
    #[doc = "Bits 17:19 - Inter frame gap bit selection"]
    #[inline(always)]
    #[must_use]
    pub fn igbs(&mut self) -> IGBS_W<MAC_CFG_SPEC, 17> {
        IGBS_W::new(self)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    #[must_use]
    pub fn jbd(&mut self) -> JBD_W<MAC_CFG_SPEC, 22> {
        JBD_W::new(self)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    #[must_use]
    pub fn wdd(&mut self) -> WDD_W<MAC_CFG_SPEC, 23> {
        WDD_W::new(self)
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
#[doc = "Ethernet MAC configuration register (MAC_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_CFG_SPEC;
impl crate::RegisterSpec for MAC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_cfg::R`](R) reader structure"]
impl crate::Readable for MAC_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_cfg::W`](W) writer structure"]
impl crate::Writable for MAC_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_CFG to value 0x8000"]
impl crate::Resettable for MAC_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
