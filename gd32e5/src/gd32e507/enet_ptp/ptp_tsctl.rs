#[doc = "Register `PTP_TSCTL` reader"]
pub type R = crate::R<PTP_TSCTL_SPEC>;
#[doc = "Register `PTP_TSCTL` writer"]
pub type W = crate::W<PTP_TSCTL_SPEC>;
#[doc = "Field `TMSEN` reader - Time stamp enable"]
pub type TMSEN_R = crate::BitReader;
#[doc = "Field `TMSEN` writer - Time stamp enable"]
pub type TMSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMSFCU` reader - Time stamp fine or coarse update"]
pub type TMSFCU_R = crate::BitReader;
#[doc = "Field `TMSFCU` writer - Time stamp fine or coarse update"]
pub type TMSFCU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMSSTI` reader - Time stamp system time initialize"]
pub type TMSSTI_R = crate::BitReader;
#[doc = "Field `TMSSTI` writer - Time stamp system time initialize"]
pub type TMSSTI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMSSTU` reader - Time stamp system time update"]
pub type TMSSTU_R = crate::BitReader;
#[doc = "Field `TMSSTU` writer - Time stamp system time update"]
pub type TMSSTU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMSITEN` reader - Time stamp interrupt trigger enable"]
pub type TMSITEN_R = crate::BitReader;
#[doc = "Field `TMSITEN` writer - Time stamp interrupt trigger enable"]
pub type TMSITEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMSARU` reader - Time stamp addend register update"]
pub type TMSARU_R = crate::BitReader;
#[doc = "Field `TMSARU` writer - Time stamp addend register update"]
pub type TMSARU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARFSEN` reader - All received frames snapshot enable"]
pub type ARFSEN_R = crate::BitReader;
#[doc = "Field `ARFSEN` writer - All received frames snapshot enable"]
pub type ARFSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCROM` reader - Subsecond counter rollover mode"]
pub type SCROM_R = crate::BitReader;
#[doc = "Field `SCROM` writer - Subsecond counter rollover mode"]
pub type SCROM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFSV` reader - PTP frame snooping version"]
pub type PFSV_R = crate::BitReader;
#[doc = "Field `PFSV` writer - PTP frame snooping version"]
pub type PFSV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESEN` reader - Received Ethernet snapshot enable"]
pub type ESEN_R = crate::BitReader;
#[doc = "Field `ESEN` writer - Received Ethernet snapshot enable"]
pub type ESEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IP6SEN` reader - Received IPv6 snapshot enable"]
pub type IP6SEN_R = crate::BitReader;
#[doc = "Field `IP6SEN` writer - Received IPv6 snapshot enable"]
pub type IP6SEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IP4SEN` reader - Received IPv4 snapshot enable"]
pub type IP4SEN_R = crate::BitReader;
#[doc = "Field `IP4SEN` writer - Received IPv4 snapshot enable"]
pub type IP4SEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETMSEN` reader - Received event type message snapshot enable"]
pub type ETMSEN_R = crate::BitReader;
#[doc = "Field `ETMSEN` writer - Received event type message snapshot enable"]
pub type ETMSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MNMSEN` reader - Received master node message snapshot enable"]
pub type MNMSEN_R = crate::BitReader;
#[doc = "Field `MNMSEN` writer - Received master node message snapshot enable"]
pub type MNMSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKNT` reader - Clock node type for time stamp"]
pub type CKNT_R = crate::FieldReader;
#[doc = "Field `CKNT` writer - Clock node type for time stamp"]
pub type CKNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MAFEN` reader - MAC address filter enable for PTP frame"]
pub type MAFEN_R = crate::BitReader;
#[doc = "Field `MAFEN` writer - MAC address filter enable for PTP frame"]
pub type MAFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn tmsen(&mut self) -> TMSEN_W<PTP_TSCTL_SPEC, 0> {
        TMSEN_W::new(self)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    #[must_use]
    pub fn tmsfcu(&mut self) -> TMSFCU_W<PTP_TSCTL_SPEC, 1> {
        TMSFCU_W::new(self)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    #[must_use]
    pub fn tmssti(&mut self) -> TMSSTI_W<PTP_TSCTL_SPEC, 2> {
        TMSSTI_W::new(self)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    #[must_use]
    pub fn tmsstu(&mut self) -> TMSSTU_W<PTP_TSCTL_SPEC, 3> {
        TMSSTU_W::new(self)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmsiten(&mut self) -> TMSITEN_W<PTP_TSCTL_SPEC, 4> {
        TMSITEN_W::new(self)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    #[must_use]
    pub fn tmsaru(&mut self) -> TMSARU_W<PTP_TSCTL_SPEC, 5> {
        TMSARU_W::new(self)
    }
    #[doc = "Bit 8 - All received frames snapshot enable"]
    #[inline(always)]
    #[must_use]
    pub fn arfsen(&mut self) -> ARFSEN_W<PTP_TSCTL_SPEC, 8> {
        ARFSEN_W::new(self)
    }
    #[doc = "Bit 9 - Subsecond counter rollover mode"]
    #[inline(always)]
    #[must_use]
    pub fn scrom(&mut self) -> SCROM_W<PTP_TSCTL_SPEC, 9> {
        SCROM_W::new(self)
    }
    #[doc = "Bit 10 - PTP frame snooping version"]
    #[inline(always)]
    #[must_use]
    pub fn pfsv(&mut self) -> PFSV_W<PTP_TSCTL_SPEC, 10> {
        PFSV_W::new(self)
    }
    #[doc = "Bit 11 - Received Ethernet snapshot enable"]
    #[inline(always)]
    #[must_use]
    pub fn esen(&mut self) -> ESEN_W<PTP_TSCTL_SPEC, 11> {
        ESEN_W::new(self)
    }
    #[doc = "Bit 12 - Received IPv6 snapshot enable"]
    #[inline(always)]
    #[must_use]
    pub fn ip6sen(&mut self) -> IP6SEN_W<PTP_TSCTL_SPEC, 12> {
        IP6SEN_W::new(self)
    }
    #[doc = "Bit 13 - Received IPv4 snapshot enable"]
    #[inline(always)]
    #[must_use]
    pub fn ip4sen(&mut self) -> IP4SEN_W<PTP_TSCTL_SPEC, 13> {
        IP4SEN_W::new(self)
    }
    #[doc = "Bit 14 - Received event type message snapshot enable"]
    #[inline(always)]
    #[must_use]
    pub fn etmsen(&mut self) -> ETMSEN_W<PTP_TSCTL_SPEC, 14> {
        ETMSEN_W::new(self)
    }
    #[doc = "Bit 15 - Received master node message snapshot enable"]
    #[inline(always)]
    #[must_use]
    pub fn mnmsen(&mut self) -> MNMSEN_W<PTP_TSCTL_SPEC, 15> {
        MNMSEN_W::new(self)
    }
    #[doc = "Bits 16:17 - Clock node type for time stamp"]
    #[inline(always)]
    #[must_use]
    pub fn cknt(&mut self) -> CKNT_W<PTP_TSCTL_SPEC, 16> {
        CKNT_W::new(self)
    }
    #[doc = "Bit 18 - MAC address filter enable for PTP frame"]
    #[inline(always)]
    #[must_use]
    pub fn mafen(&mut self) -> MAFEN_W<PTP_TSCTL_SPEC, 18> {
        MAFEN_W::new(self)
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
#[doc = "Ethernet PTP time stamp control register (PTP_TSCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_tsctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTP_TSCTL_SPEC;
impl crate::RegisterSpec for PTP_TSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_tsctl::R`](R) reader structure"]
impl crate::Readable for PTP_TSCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptp_tsctl::W`](W) writer structure"]
impl crate::Writable for PTP_TSCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTP_TSCTL to value 0"]
impl crate::Resettable for PTP_TSCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
