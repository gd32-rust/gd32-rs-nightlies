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
