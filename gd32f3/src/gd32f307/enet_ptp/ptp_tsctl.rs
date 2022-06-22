#[doc = "Register `PTP_TSCTL` reader"]
pub struct R(crate::R<PTP_TSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_TSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_TSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_TSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTP_TSCTL` writer"]
pub struct W(crate::W<PTP_TSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTP_TSCTL_SPEC>;
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
impl From<crate::W<PTP_TSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTP_TSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMSEN` reader - Time stamp enable"]
pub type TMSEN_R = crate::BitReader<bool>;
#[doc = "Field `TMSEN` writer - Time stamp enable"]
pub type TMSEN_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 0>;
#[doc = "Field `TMSFCU` reader - Time stamp fine or coarse update"]
pub type TMSFCU_R = crate::BitReader<bool>;
#[doc = "Field `TMSFCU` writer - Time stamp fine or coarse update"]
pub type TMSFCU_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 1>;
#[doc = "Field `TMSSTI` reader - Time stamp system time initialize"]
pub type TMSSTI_R = crate::BitReader<bool>;
#[doc = "Field `TMSSTI` writer - Time stamp system time initialize"]
pub type TMSSTI_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 2>;
#[doc = "Field `TMSSTU` reader - Time stamp system time update"]
pub type TMSSTU_R = crate::BitReader<bool>;
#[doc = "Field `TMSSTU` writer - Time stamp system time update"]
pub type TMSSTU_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 3>;
#[doc = "Field `TMSITEN` reader - Time stamp interrupt trigger enable"]
pub type TMSITEN_R = crate::BitReader<bool>;
#[doc = "Field `TMSITEN` writer - Time stamp interrupt trigger enable"]
pub type TMSITEN_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 4>;
#[doc = "Field `TMSARU` reader - Time stamp addend register update"]
pub type TMSARU_R = crate::BitReader<bool>;
#[doc = "Field `TMSARU` writer - Time stamp addend register update"]
pub type TMSARU_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 5>;
#[doc = "Field `ARFSEN` reader - All received frames snapshot enable"]
pub type ARFSEN_R = crate::BitReader<bool>;
#[doc = "Field `ARFSEN` writer - All received frames snapshot enable"]
pub type ARFSEN_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 8>;
#[doc = "Field `SCROM` reader - Subsecond counter rollover mode"]
pub type SCROM_R = crate::BitReader<bool>;
#[doc = "Field `SCROM` writer - Subsecond counter rollover mode"]
pub type SCROM_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 9>;
#[doc = "Field `PFSV` reader - PTP frame snooping version"]
pub type PFSV_R = crate::BitReader<bool>;
#[doc = "Field `PFSV` writer - PTP frame snooping version"]
pub type PFSV_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 10>;
#[doc = "Field `ESEN` reader - Received Ethernet snapshot enable"]
pub type ESEN_R = crate::BitReader<bool>;
#[doc = "Field `ESEN` writer - Received Ethernet snapshot enable"]
pub type ESEN_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 11>;
#[doc = "Field `IP6SEN` reader - Received IPv6 snapshot enable"]
pub type IP6SEN_R = crate::BitReader<bool>;
#[doc = "Field `IP6SEN` writer - Received IPv6 snapshot enable"]
pub type IP6SEN_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 12>;
#[doc = "Field `IP4SEN` reader - Received IPv4 snapshot enable"]
pub type IP4SEN_R = crate::BitReader<bool>;
#[doc = "Field `IP4SEN` writer - Received IPv4 snapshot enable"]
pub type IP4SEN_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 13>;
#[doc = "Field `ETMSEN` reader - Received event type message snapshot enable"]
pub type ETMSEN_R = crate::BitReader<bool>;
#[doc = "Field `ETMSEN` writer - Received event type message snapshot enable"]
pub type ETMSEN_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 14>;
#[doc = "Field `MNMSEN` reader - Received master node message snapshot enable"]
pub type MNMSEN_R = crate::BitReader<bool>;
#[doc = "Field `MNMSEN` writer - Received master node message snapshot enable"]
pub type MNMSEN_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 15>;
#[doc = "Field `CKNT` reader - Clock node type for time stamp"]
pub type CKNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKNT` writer - Clock node type for time stamp"]
pub type CKNT_W<'a> = crate::FieldWriter<'a, u32, PTP_TSCTL_SPEC, u8, u8, 2, 16>;
#[doc = "Field `MAFEN` reader - MAC address filter enable for PTP frame"]
pub type MAFEN_R = crate::BitReader<bool>;
#[doc = "Field `MAFEN` writer - MAC address filter enable for PTP frame"]
pub type MAFEN_W<'a> = crate::BitWriter<'a, u32, PTP_TSCTL_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0 - Time stamp enable"]
    #[inline(always)]
    pub fn tmsen(&self) -> TMSEN_R {
        TMSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    pub fn tmsfcu(&self) -> TMSFCU_R {
        TMSFCU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    pub fn tmssti(&self) -> TMSSTI_R {
        TMSSTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    pub fn tmsstu(&self) -> TMSSTU_R {
        TMSSTU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    pub fn tmsiten(&self) -> TMSITEN_R {
        TMSITEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    pub fn tmsaru(&self) -> TMSARU_R {
        TMSARU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - All received frames snapshot enable"]
    #[inline(always)]
    pub fn arfsen(&self) -> ARFSEN_R {
        ARFSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Subsecond counter rollover mode"]
    #[inline(always)]
    pub fn scrom(&self) -> SCROM_R {
        SCROM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PTP frame snooping version"]
    #[inline(always)]
    pub fn pfsv(&self) -> PFSV_R {
        PFSV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Received Ethernet snapshot enable"]
    #[inline(always)]
    pub fn esen(&self) -> ESEN_R {
        ESEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Received IPv6 snapshot enable"]
    #[inline(always)]
    pub fn ip6sen(&self) -> IP6SEN_R {
        IP6SEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received IPv4 snapshot enable"]
    #[inline(always)]
    pub fn ip4sen(&self) -> IP4SEN_R {
        IP4SEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Received event type message snapshot enable"]
    #[inline(always)]
    pub fn etmsen(&self) -> ETMSEN_R {
        ETMSEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Received master node message snapshot enable"]
    #[inline(always)]
    pub fn mnmsen(&self) -> MNMSEN_R {
        MNMSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Clock node type for time stamp"]
    #[inline(always)]
    pub fn cknt(&self) -> CKNT_R {
        CKNT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - MAC address filter enable for PTP frame"]
    #[inline(always)]
    pub fn mafen(&self) -> MAFEN_R {
        MAFEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time stamp enable"]
    #[inline(always)]
    pub fn tmsen(&mut self) -> TMSEN_W {
        TMSEN_W::new(self)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    pub fn tmsfcu(&mut self) -> TMSFCU_W {
        TMSFCU_W::new(self)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    pub fn tmssti(&mut self) -> TMSSTI_W {
        TMSSTI_W::new(self)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    pub fn tmsstu(&mut self) -> TMSSTU_W {
        TMSSTU_W::new(self)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    pub fn tmsiten(&mut self) -> TMSITEN_W {
        TMSITEN_W::new(self)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    pub fn tmsaru(&mut self) -> TMSARU_W {
        TMSARU_W::new(self)
    }
    #[doc = "Bit 8 - All received frames snapshot enable"]
    #[inline(always)]
    pub fn arfsen(&mut self) -> ARFSEN_W {
        ARFSEN_W::new(self)
    }
    #[doc = "Bit 9 - Subsecond counter rollover mode"]
    #[inline(always)]
    pub fn scrom(&mut self) -> SCROM_W {
        SCROM_W::new(self)
    }
    #[doc = "Bit 10 - PTP frame snooping version"]
    #[inline(always)]
    pub fn pfsv(&mut self) -> PFSV_W {
        PFSV_W::new(self)
    }
    #[doc = "Bit 11 - Received Ethernet snapshot enable"]
    #[inline(always)]
    pub fn esen(&mut self) -> ESEN_W {
        ESEN_W::new(self)
    }
    #[doc = "Bit 12 - Received IPv6 snapshot enable"]
    #[inline(always)]
    pub fn ip6sen(&mut self) -> IP6SEN_W {
        IP6SEN_W::new(self)
    }
    #[doc = "Bit 13 - Received IPv4 snapshot enable"]
    #[inline(always)]
    pub fn ip4sen(&mut self) -> IP4SEN_W {
        IP4SEN_W::new(self)
    }
    #[doc = "Bit 14 - Received event type message snapshot enable"]
    #[inline(always)]
    pub fn etmsen(&mut self) -> ETMSEN_W {
        ETMSEN_W::new(self)
    }
    #[doc = "Bit 15 - Received master node message snapshot enable"]
    #[inline(always)]
    pub fn mnmsen(&mut self) -> MNMSEN_W {
        MNMSEN_W::new(self)
    }
    #[doc = "Bits 16:17 - Clock node type for time stamp"]
    #[inline(always)]
    pub fn cknt(&mut self) -> CKNT_W {
        CKNT_W::new(self)
    }
    #[doc = "Bit 18 - MAC address filter enable for PTP frame"]
    #[inline(always)]
    pub fn mafen(&mut self) -> MAFEN_W {
        MAFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP time stamp control register (PTP_TSCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_tsctl](index.html) module"]
pub struct PTP_TSCTL_SPEC;
impl crate::RegisterSpec for PTP_TSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_tsctl::R](R) reader structure"]
impl crate::Readable for PTP_TSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptp_tsctl::W](W) writer structure"]
impl crate::Writable for PTP_TSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTP_TSCTL to value 0"]
impl crate::Resettable for PTP_TSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
