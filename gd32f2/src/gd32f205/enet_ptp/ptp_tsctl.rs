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
