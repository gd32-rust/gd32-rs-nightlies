#[doc = "Register `ST0CTL0` reader"]
pub type R = crate::R<ST0CTL0_SPEC>;
#[doc = "Register `ST0CTL0` writer"]
pub type W = crate::W<ST0CTL0_SPEC>;
#[doc = "Field `CNTCKDIV` reader - Counter clock division"]
pub type CNTCKDIV_R = crate::FieldReader;
#[doc = "Field `CNTCKDIV` writer - Counter clock division"]
pub type CNTCKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CTNM` reader - Continuous mode"]
pub type CTNM_R = crate::BitReader;
#[doc = "Field `CTNM` writer - Continuous mode"]
pub type CTNM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTRSTM` reader - Counter reset mode"]
pub type CNTRSTM_R = crate::BitReader;
#[doc = "Field `CNTRSTM` writer - Counter reset mode"]
pub type CNTRSTM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HALFM` reader - Half mode"]
pub type HALFM_R = crate::BitReader;
#[doc = "Field `HALFM` writer - Half mode"]
pub type HALFM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLNMEN` reader - Balanced mode enable"]
pub type BLNMEN_R = crate::BitReader;
#[doc = "Field `BLNMEN` writer - Balanced mode enable"]
pub type BLNMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNIRST` reader - Synchronization input reset counter"]
pub type SYNIRST_R = crate::BitReader;
#[doc = "Field `SYNIRST` writer - Synchronization input reset counter"]
pub type SYNIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNISTRT` reader - Synchronization input start counter"]
pub type SYNISTRT_R = crate::BitReader;
#[doc = "Field `SYNISTRT` writer - Synchronization input start counter"]
pub type SYNISTRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DELCMP1M` reader - Compare 1 delayed mode"]
pub type DELCMP1M_R = crate::FieldReader;
#[doc = "Field `DELCMP1M` writer - Compare 1 delayed mode"]
pub type DELCMP1M_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DELCMP3M` reader - Compare 3 delayed mode"]
pub type DELCMP3M_R = crate::FieldReader;
#[doc = "Field `DELCMP3M` writer - Compare 3 delayed mode"]
pub type DELCMP3M_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `UPREP` reader - Update event generated by repetition event"]
pub type UPREP_R = crate::BitReader;
#[doc = "Field `UPREP` writer - Update event generated by repetition event"]
pub type UPREP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPRST` reader - Update event generated by reset event"]
pub type UPRST_R = crate::BitReader;
#[doc = "Field `UPRST` writer - Update event generated by reset event"]
pub type UPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPBST1` reader - Update by Slave_TIMER1 update event"]
pub type UPBST1_R = crate::BitReader;
#[doc = "Field `UPBST1` writer - Update by Slave_TIMER1 update event"]
pub type UPBST1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPBST2` reader - Update by Slave_TIMER2 update event"]
pub type UPBST2_R = crate::BitReader;
#[doc = "Field `UPBST2` writer - Update by Slave_TIMER2 update event"]
pub type UPBST2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPBST3` reader - Update by Slave_TIMER3 update event"]
pub type UPBST3_R = crate::BitReader;
#[doc = "Field `UPBST3` writer - Update by Slave_TIMER3 update event"]
pub type UPBST3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPBST4` reader - Update by Slave_TIMER4 update event"]
pub type UPBST4_R = crate::BitReader;
#[doc = "Field `UPBST4` writer - Update by Slave_TIMER4 update event"]
pub type UPBST4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPBMT` reader - Update by Master_TIMER update event"]
pub type UPBMT_R = crate::BitReader;
#[doc = "Field `UPBMT` writer - Update by Master_TIMER update event"]
pub type UPBMT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DACTRGS` reader - Trigger source to DAC"]
pub type DACTRGS_R = crate::FieldReader;
#[doc = "Field `DACTRGS` writer - Trigger source to DAC"]
pub type DACTRGS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SHWEN` reader - Shadow registers enable"]
pub type SHWEN_R = crate::BitReader;
#[doc = "Field `SHWEN` writer - Shadow registers enable"]
pub type SHWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPSEL` reader - Update event selection"]
pub type UPSEL_R = crate::FieldReader;
#[doc = "Field `UPSEL` writer - Update event selection"]
pub type UPSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:2 - Counter clock division"]
    #[inline(always)]
    pub fn cntckdiv(&self) -> CNTCKDIV_R {
        CNTCKDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn ctnm(&self) -> CTNM_R {
        CTNM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter reset mode"]
    #[inline(always)]
    pub fn cntrstm(&self) -> CNTRSTM_R {
        CNTRSTM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half mode"]
    #[inline(always)]
    pub fn halfm(&self) -> HALFM_R {
        HALFM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Balanced mode enable"]
    #[inline(always)]
    pub fn blnmen(&self) -> BLNMEN_R {
        BLNMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization input reset counter"]
    #[inline(always)]
    pub fn synirst(&self) -> SYNIRST_R {
        SYNIRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization input start counter"]
    #[inline(always)]
    pub fn synistrt(&self) -> SYNISTRT_R {
        SYNISTRT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Compare 1 delayed mode"]
    #[inline(always)]
    pub fn delcmp1m(&self) -> DELCMP1M_R {
        DELCMP1M_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Compare 3 delayed mode"]
    #[inline(always)]
    pub fn delcmp3m(&self) -> DELCMP3M_R {
        DELCMP3M_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Update event generated by repetition event"]
    #[inline(always)]
    pub fn uprep(&self) -> UPREP_R {
        UPREP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Update event generated by reset event"]
    #[inline(always)]
    pub fn uprst(&self) -> UPRST_R {
        UPRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Update by Slave_TIMER1 update event"]
    #[inline(always)]
    pub fn upbst1(&self) -> UPBST1_R {
        UPBST1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Update by Slave_TIMER2 update event"]
    #[inline(always)]
    pub fn upbst2(&self) -> UPBST2_R {
        UPBST2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Update by Slave_TIMER3 update event"]
    #[inline(always)]
    pub fn upbst3(&self) -> UPBST3_R {
        UPBST3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Update by Slave_TIMER4 update event"]
    #[inline(always)]
    pub fn upbst4(&self) -> UPBST4_R {
        UPBST4_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Update by Master_TIMER update event"]
    #[inline(always)]
    pub fn upbmt(&self) -> UPBMT_R {
        UPBMT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Trigger source to DAC"]
    #[inline(always)]
    pub fn dactrgs(&self) -> DACTRGS_R {
        DACTRGS_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Shadow registers enable"]
    #[inline(always)]
    pub fn shwen(&self) -> SHWEN_R {
        SHWEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Update event selection"]
    #[inline(always)]
    pub fn upsel(&self) -> UPSEL_R {
        UPSEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Counter clock division"]
    #[inline(always)]
    #[must_use]
    pub fn cntckdiv(&mut self) -> CNTCKDIV_W<ST0CTL0_SPEC, 0> {
        CNTCKDIV_W::new(self)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctnm(&mut self) -> CTNM_W<ST0CTL0_SPEC, 3> {
        CTNM_W::new(self)
    }
    #[doc = "Bit 4 - Counter reset mode"]
    #[inline(always)]
    #[must_use]
    pub fn cntrstm(&mut self) -> CNTRSTM_W<ST0CTL0_SPEC, 4> {
        CNTRSTM_W::new(self)
    }
    #[doc = "Bit 5 - Half mode"]
    #[inline(always)]
    #[must_use]
    pub fn halfm(&mut self) -> HALFM_W<ST0CTL0_SPEC, 5> {
        HALFM_W::new(self)
    }
    #[doc = "Bit 6 - Balanced mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn blnmen(&mut self) -> BLNMEN_W<ST0CTL0_SPEC, 6> {
        BLNMEN_W::new(self)
    }
    #[doc = "Bit 10 - Synchronization input reset counter"]
    #[inline(always)]
    #[must_use]
    pub fn synirst(&mut self) -> SYNIRST_W<ST0CTL0_SPEC, 10> {
        SYNIRST_W::new(self)
    }
    #[doc = "Bit 11 - Synchronization input start counter"]
    #[inline(always)]
    #[must_use]
    pub fn synistrt(&mut self) -> SYNISTRT_W<ST0CTL0_SPEC, 11> {
        SYNISTRT_W::new(self)
    }
    #[doc = "Bits 12:13 - Compare 1 delayed mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp1m(&mut self) -> DELCMP1M_W<ST0CTL0_SPEC, 12> {
        DELCMP1M_W::new(self)
    }
    #[doc = "Bits 14:15 - Compare 3 delayed mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp3m(&mut self) -> DELCMP3M_W<ST0CTL0_SPEC, 14> {
        DELCMP3M_W::new(self)
    }
    #[doc = "Bit 17 - Update event generated by repetition event"]
    #[inline(always)]
    #[must_use]
    pub fn uprep(&mut self) -> UPREP_W<ST0CTL0_SPEC, 17> {
        UPREP_W::new(self)
    }
    #[doc = "Bit 18 - Update event generated by reset event"]
    #[inline(always)]
    #[must_use]
    pub fn uprst(&mut self) -> UPRST_W<ST0CTL0_SPEC, 18> {
        UPRST_W::new(self)
    }
    #[doc = "Bit 20 - Update by Slave_TIMER1 update event"]
    #[inline(always)]
    #[must_use]
    pub fn upbst1(&mut self) -> UPBST1_W<ST0CTL0_SPEC, 20> {
        UPBST1_W::new(self)
    }
    #[doc = "Bit 21 - Update by Slave_TIMER2 update event"]
    #[inline(always)]
    #[must_use]
    pub fn upbst2(&mut self) -> UPBST2_W<ST0CTL0_SPEC, 21> {
        UPBST2_W::new(self)
    }
    #[doc = "Bit 22 - Update by Slave_TIMER3 update event"]
    #[inline(always)]
    #[must_use]
    pub fn upbst3(&mut self) -> UPBST3_W<ST0CTL0_SPEC, 22> {
        UPBST3_W::new(self)
    }
    #[doc = "Bit 23 - Update by Slave_TIMER4 update event"]
    #[inline(always)]
    #[must_use]
    pub fn upbst4(&mut self) -> UPBST4_W<ST0CTL0_SPEC, 23> {
        UPBST4_W::new(self)
    }
    #[doc = "Bit 24 - Update by Master_TIMER update event"]
    #[inline(always)]
    #[must_use]
    pub fn upbmt(&mut self) -> UPBMT_W<ST0CTL0_SPEC, 24> {
        UPBMT_W::new(self)
    }
    #[doc = "Bits 25:26 - Trigger source to DAC"]
    #[inline(always)]
    #[must_use]
    pub fn dactrgs(&mut self) -> DACTRGS_W<ST0CTL0_SPEC, 25> {
        DACTRGS_W::new(self)
    }
    #[doc = "Bit 27 - Shadow registers enable"]
    #[inline(always)]
    #[must_use]
    pub fn shwen(&mut self) -> SHWEN_W<ST0CTL0_SPEC, 27> {
        SHWEN_W::new(self)
    }
    #[doc = "Bits 28:31 - Update event selection"]
    #[inline(always)]
    #[must_use]
    pub fn upsel(&mut self) -> UPSEL_W<ST0CTL0_SPEC, 28> {
        UPSEL_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMER0 control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST0CTL0_SPEC;
impl crate::RegisterSpec for ST0CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0ctl0::R`](R) reader structure"]
impl crate::Readable for ST0CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st0ctl0::W`](W) writer structure"]
impl crate::Writable for ST0CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST0CTL0 to value 0"]
impl crate::Resettable for ST0CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
