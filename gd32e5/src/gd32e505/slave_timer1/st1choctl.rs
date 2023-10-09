#[doc = "Register `ST1CHOCTL` reader"]
pub type R = crate::R<ST1CHOCTL_SPEC>;
#[doc = "Register `ST1CHOCTL` writer"]
pub type W = crate::W<ST1CHOCTL_SPEC>;
#[doc = "Field `CH0P` reader - Channel 0 output polarity"]
pub type CH0P_R = crate::BitReader;
#[doc = "Field `CH0P` writer - Channel 0 output polarity"]
pub type CH0P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMCH0IEN` reader - Channel 0 IDLE state enable in bunch mode"]
pub type BMCH0IEN_R = crate::BitReader;
#[doc = "Field `BMCH0IEN` writer - Channel 0 IDLE state enable in bunch mode"]
pub type BMCH0IEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISO0` reader - Channel 0 output idle state"]
pub type ISO0_R = crate::BitReader;
#[doc = "Field `ISO0` writer - Channel 0 output idle state"]
pub type ISO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0FLTOS` reader - Channel 0 Fault output state"]
pub type CH0FLTOS_R = crate::FieldReader;
#[doc = "Field `CH0FLTOS` writer - Channel 0 Fault output state"]
pub type CH0FLTOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CH0CSEN` reader - Channel 0 carrier-signal mode enable"]
pub type CH0CSEN_R = crate::BitReader;
#[doc = "Field `CH0CSEN` writer - Channel 0 carrier-signal mode enable"]
pub type CH0CSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMCH0DTI` reader - Channel 0 dead-time insert in bunch mode"]
pub type BMCH0DTI_R = crate::BitReader;
#[doc = "Field `BMCH0DTI` writer - Channel 0 dead-time insert in bunch mode"]
pub type BMCH0DTI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTEN` reader - Dead time enable"]
pub type DTEN_R = crate::BitReader;
#[doc = "Field `DTEN` writer - Dead time enable"]
pub type DTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLYISMEN` reader - Delayed IDLE state mode enable"]
pub type DLYISMEN_R = crate::BitReader;
#[doc = "Field `DLYISMEN` writer - Delayed IDLE state mode enable"]
pub type DLYISMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLYISCH` reader - Delayed IDLE source and channel"]
pub type DLYISCH_R = crate::FieldReader;
#[doc = "Field `DLYISCH` writer - Delayed IDLE source and channel"]
pub type DLYISCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CH1P` reader - Channel 1 output polarity"]
pub type CH1P_R = crate::BitReader;
#[doc = "Field `CH1P` writer - Channel 1 output polarity"]
pub type CH1P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMCH1IEN` reader - Channel 1 IDLE state enable in bunch mode"]
pub type BMCH1IEN_R = crate::BitReader;
#[doc = "Field `BMCH1IEN` writer - Channel 1 IDLE state enable in bunch mode"]
pub type BMCH1IEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISO1` reader - channel 1 output idle state"]
pub type ISO1_R = crate::BitReader;
#[doc = "Field `ISO1` writer - channel 1 output idle state"]
pub type ISO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1FLTOS` reader - Channel 1 Fault output state"]
pub type CH1FLTOS_R = crate::FieldReader;
#[doc = "Field `CH1FLTOS` writer - Channel 1 Fault output state"]
pub type CH1FLTOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CH1CSEN` reader - Channel 1 carrier-signal mode enable"]
pub type CH1CSEN_R = crate::BitReader;
#[doc = "Field `CH1CSEN` writer - Channel 1 carrier-signal mode enable"]
pub type CH1CSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMCH1DTI` reader - Channel 1 dead-time insert in bunch mode"]
pub type BMCH1DTI_R = crate::BitReader;
#[doc = "Field `BMCH1DTI` writer - Channel 1 dead-time insert in bunch mode"]
pub type BMCH1DTI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Channel 0 output polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> CH0P_R {
        CH0P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 IDLE state enable in bunch mode"]
    #[inline(always)]
    pub fn bmch0ien(&self) -> BMCH0IEN_R {
        BMCH0IEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 output idle state"]
    #[inline(always)]
    pub fn iso0(&self) -> ISO0_R {
        ISO0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Channel 0 Fault output state"]
    #[inline(always)]
    pub fn ch0fltos(&self) -> CH0FLTOS_R {
        CH0FLTOS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Channel 0 carrier-signal mode enable"]
    #[inline(always)]
    pub fn ch0csen(&self) -> CH0CSEN_R {
        CH0CSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 0 dead-time insert in bunch mode"]
    #[inline(always)]
    pub fn bmch0dti(&self) -> BMCH0DTI_R {
        BMCH0DTI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Dead time enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Delayed IDLE state mode enable"]
    #[inline(always)]
    pub fn dlyismen(&self) -> DLYISMEN_R {
        DLYISMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Delayed IDLE source and channel"]
    #[inline(always)]
    pub fn dlyisch(&self) -> DLYISCH_R {
        DLYISCH_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 17 - Channel 1 output polarity"]
    #[inline(always)]
    pub fn ch1p(&self) -> CH1P_R {
        CH1P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 1 IDLE state enable in bunch mode"]
    #[inline(always)]
    pub fn bmch1ien(&self) -> BMCH1IEN_R {
        BMCH1IEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - channel 1 output idle state"]
    #[inline(always)]
    pub fn iso1(&self) -> ISO1_R {
        ISO1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Channel 1 Fault output state"]
    #[inline(always)]
    pub fn ch1fltos(&self) -> CH1FLTOS_R {
        CH1FLTOS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Channel 1 carrier-signal mode enable"]
    #[inline(always)]
    pub fn ch1csen(&self) -> CH1CSEN_R {
        CH1CSEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 1 dead-time insert in bunch mode"]
    #[inline(always)]
    pub fn bmch1dti(&self) -> BMCH1DTI_R {
        BMCH1DTI_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Channel 0 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0p(&mut self) -> CH0P_W<ST1CHOCTL_SPEC, 1> {
        CH0P_W::new(self)
    }
    #[doc = "Bit 2 - Channel 0 IDLE state enable in bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmch0ien(&mut self) -> BMCH0IEN_W<ST1CHOCTL_SPEC, 2> {
        BMCH0IEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 0 output idle state"]
    #[inline(always)]
    #[must_use]
    pub fn iso0(&mut self) -> ISO0_W<ST1CHOCTL_SPEC, 3> {
        ISO0_W::new(self)
    }
    #[doc = "Bits 4:5 - Channel 0 Fault output state"]
    #[inline(always)]
    #[must_use]
    pub fn ch0fltos(&mut self) -> CH0FLTOS_W<ST1CHOCTL_SPEC, 4> {
        CH0FLTOS_W::new(self)
    }
    #[doc = "Bit 6 - Channel 0 carrier-signal mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0csen(&mut self) -> CH0CSEN_W<ST1CHOCTL_SPEC, 6> {
        CH0CSEN_W::new(self)
    }
    #[doc = "Bit 7 - Channel 0 dead-time insert in bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmch0dti(&mut self) -> BMCH0DTI_W<ST1CHOCTL_SPEC, 7> {
        BMCH0DTI_W::new(self)
    }
    #[doc = "Bit 8 - Dead time enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<ST1CHOCTL_SPEC, 8> {
        DTEN_W::new(self)
    }
    #[doc = "Bit 9 - Delayed IDLE state mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlyismen(&mut self) -> DLYISMEN_W<ST1CHOCTL_SPEC, 9> {
        DLYISMEN_W::new(self)
    }
    #[doc = "Bits 10:12 - Delayed IDLE source and channel"]
    #[inline(always)]
    #[must_use]
    pub fn dlyisch(&mut self) -> DLYISCH_W<ST1CHOCTL_SPEC, 10> {
        DLYISCH_W::new(self)
    }
    #[doc = "Bit 17 - Channel 1 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch1p(&mut self) -> CH1P_W<ST1CHOCTL_SPEC, 17> {
        CH1P_W::new(self)
    }
    #[doc = "Bit 18 - Channel 1 IDLE state enable in bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmch1ien(&mut self) -> BMCH1IEN_W<ST1CHOCTL_SPEC, 18> {
        BMCH1IEN_W::new(self)
    }
    #[doc = "Bit 19 - channel 1 output idle state"]
    #[inline(always)]
    #[must_use]
    pub fn iso1(&mut self) -> ISO1_W<ST1CHOCTL_SPEC, 19> {
        ISO1_W::new(self)
    }
    #[doc = "Bits 20:21 - Channel 1 Fault output state"]
    #[inline(always)]
    #[must_use]
    pub fn ch1fltos(&mut self) -> CH1FLTOS_W<ST1CHOCTL_SPEC, 20> {
        CH1FLTOS_W::new(self)
    }
    #[doc = "Bit 22 - Channel 1 carrier-signal mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1csen(&mut self) -> CH1CSEN_W<ST1CHOCTL_SPEC, 22> {
        CH1CSEN_W::new(self)
    }
    #[doc = "Bit 23 - Channel 1 dead-time insert in bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmch1dti(&mut self) -> BMCH1DTI_W<ST1CHOCTL_SPEC, 23> {
        BMCH1DTI_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx channel output control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1choctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1choctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1CHOCTL_SPEC;
impl crate::RegisterSpec for ST1CHOCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1choctl::R`](R) reader structure"]
impl crate::Readable for ST1CHOCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1choctl::W`](W) writer structure"]
impl crate::Writable for ST1CHOCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1CHOCTL to value 0"]
impl crate::Resettable for ST1CHOCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
