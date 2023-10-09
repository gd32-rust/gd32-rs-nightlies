#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `SETRST` reader - USB Reset"]
pub type SETRST_R = crate::BitReader;
#[doc = "Field `SETRST` writer - USB Reset"]
pub type SETRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLOSE` reader - USB close"]
pub type CLOSE_R = crate::BitReader;
#[doc = "Field `CLOSE` writer - USB close"]
pub type CLOSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOWM` reader - Low-power mode"]
pub type LOWM_R = crate::BitReader;
#[doc = "Field `LOWM` writer - Low-power mode"]
pub type LOWM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETSPS` reader - Set suspend state"]
pub type SETSPS_R = crate::BitReader;
#[doc = "Field `SETSPS` writer - Set suspend state"]
pub type SETSPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSREQ` reader - Send resume request"]
pub type RSREQ_R = crate::BitReader;
#[doc = "Field `RSREQ` writer - Send resume request"]
pub type RSREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESOFIE` reader - Expected start of frame interrupt enable"]
pub type ESOFIE_R = crate::BitReader;
#[doc = "Field `ESOFIE` writer - Expected start of frame interrupt enable"]
pub type ESOFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFIE` reader - Start of frame interrupt enable"]
pub type SOFIE_R = crate::BitReader;
#[doc = "Field `SOFIE` writer - Start of frame interrupt enable"]
pub type SOFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIE` reader - USB reset interrupt enable"]
pub type RSTIE_R = crate::BitReader;
#[doc = "Field `RSTIE` writer - USB reset interrupt enable"]
pub type RSTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPSIE` reader - Suspend state interrupt enable"]
pub type SPSIE_R = crate::BitReader;
#[doc = "Field `SPSIE` writer - Suspend state interrupt enable"]
pub type SPSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKUPIE` reader - Wakeup interrupt mask"]
pub type WKUPIE_R = crate::BitReader;
#[doc = "Field `WKUPIE` writer - Wakeup interrupt mask"]
pub type WKUPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt mask"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt mask"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PMOUIE` reader - Packet memory overrun / underrun interrupt enable"]
pub type PMOUIE_R = crate::BitReader;
#[doc = "Field `PMOUIE` writer - Packet memory overrun / underrun interrupt enable"]
pub type PMOUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STIE` reader - Successful transfer interrupt enable"]
pub type STIE_R = crate::BitReader;
#[doc = "Field `STIE` writer - Successful transfer interrupt enable"]
pub type STIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB Reset"]
    #[inline(always)]
    pub fn setrst(&self) -> SETRST_R {
        SETRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB close"]
    #[inline(always)]
    pub fn close(&self) -> CLOSE_R {
        CLOSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lowm(&self) -> LOWM_R {
        LOWM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set suspend state"]
    #[inline(always)]
    pub fn setsps(&self) -> SETSPS_R {
        SETSPS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Send resume request"]
    #[inline(always)]
    pub fn rsreq(&self) -> RSREQ_R {
        RSREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt enable"]
    #[inline(always)]
    pub fn esofie(&self) -> ESOFIE_R {
        ESOFIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset interrupt enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend state interrupt enable"]
    #[inline(always)]
    pub fn spsie(&self) -> SPSIE_R {
        SPSIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory overrun / underrun interrupt enable"]
    #[inline(always)]
    pub fn pmouie(&self) -> PMOUIE_R {
        PMOUIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Successful transfer interrupt enable"]
    #[inline(always)]
    pub fn stie(&self) -> STIE_R {
        STIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn setrst(&mut self) -> SETRST_W<CTL_SPEC, 0> {
        SETRST_W::new(self)
    }
    #[doc = "Bit 1 - USB close"]
    #[inline(always)]
    #[must_use]
    pub fn close(&mut self) -> CLOSE_W<CTL_SPEC, 1> {
        CLOSE_W::new(self)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lowm(&mut self) -> LOWM_W<CTL_SPEC, 2> {
        LOWM_W::new(self)
    }
    #[doc = "Bit 3 - Set suspend state"]
    #[inline(always)]
    #[must_use]
    pub fn setsps(&mut self) -> SETSPS_W<CTL_SPEC, 3> {
        SETSPS_W::new(self)
    }
    #[doc = "Bit 4 - Send resume request"]
    #[inline(always)]
    #[must_use]
    pub fn rsreq(&mut self) -> RSREQ_W<CTL_SPEC, 4> {
        RSREQ_W::new(self)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn esofie(&mut self) -> ESOFIE_W<CTL_SPEC, 8> {
        ESOFIE_W::new(self)
    }
    #[doc = "Bit 9 - Start of frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<CTL_SPEC, 9> {
        SOFIE_W::new(self)
    }
    #[doc = "Bit 10 - USB reset interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstie(&mut self) -> RSTIE_W<CTL_SPEC, 10> {
        RSTIE_W::new(self)
    }
    #[doc = "Bit 11 - Suspend state interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn spsie(&mut self) -> SPSIE_W<CTL_SPEC, 11> {
        SPSIE_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wkupie(&mut self) -> WKUPIE_W<CTL_SPEC, 12> {
        WKUPIE_W::new(self)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL_SPEC, 13> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 14 - Packet memory overrun / underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmouie(&mut self) -> PMOUIE_W<CTL_SPEC, 14> {
        PMOUIE_W::new(self)
    }
    #[doc = "Bit 15 - Successful transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stie(&mut self) -> STIE_W<CTL_SPEC, 15> {
        STIE_W::new(self)
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
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x03"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
