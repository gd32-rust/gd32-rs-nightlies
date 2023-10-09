#[doc = "Register `CHOUTDIS` writer"]
pub type W = crate::W<CHOUTDIS_SPEC>;
#[doc = "Field `ST0CH0DIS` writer - Slave_TIMER0 channel 0 output (ST0CH0_O) disable"]
pub type ST0CH0DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0CH1DIS` writer - Slave_TIMER0 channel 1 output (ST4CH0_O) disable"]
pub type ST0CH1DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1CH0DIS` writer - Slave_TIMER1 channel 0 output (ST1CH0_O) disable"]
pub type ST1CH0DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1CH1DIS` writer - Slave_TIMER1 channel 1 output (ST1CH1_O) disable"]
pub type ST1CH1DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2CH0DIS` writer - Slave_TIMER2 channel 0 output (ST2CH0_O) disable"]
pub type ST2CH0DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2CH1DIS` writer - Slave_TIMER2 channel 1 output (ST2CH1_O) disable"]
pub type ST2CH1DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3CH0DIS` writer - Slave_TIMER3 channel 0 output (ST3CH0_O) disable"]
pub type ST3CH0DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3CH1DIS` writer - Slave_TIMER3 channel 1 output (ST3CH1_O) disable"]
pub type ST3CH1DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4CH0DIS` writer - Slave_TIMER4 channel 0 output (ST4CH0_O) disable"]
pub type ST4CH0DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4CH1DIS` writer - Slave_TIMER4 channel 1 output (ST4CH1_O) disable"]
pub type ST4CH1DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Slave_TIMER0 channel 0 output (ST0CH0_O) disable"]
    #[inline(always)]
    #[must_use]
    pub fn st0ch0dis(&mut self) -> ST0CH0DIS_W<CHOUTDIS_SPEC, 0> {
        ST0CH0DIS_W::new(self)
    }
    #[doc = "Bit 1 - Slave_TIMER0 channel 1 output (ST4CH0_O) disable"]
    #[inline(always)]
    #[must_use]
    pub fn st0ch1dis(&mut self) -> ST0CH1DIS_W<CHOUTDIS_SPEC, 1> {
        ST0CH1DIS_W::new(self)
    }
    #[doc = "Bit 2 - Slave_TIMER1 channel 0 output (ST1CH0_O) disable"]
    #[inline(always)]
    #[must_use]
    pub fn st1ch0dis(&mut self) -> ST1CH0DIS_W<CHOUTDIS_SPEC, 2> {
        ST1CH0DIS_W::new(self)
    }
    #[doc = "Bit 3 - Slave_TIMER1 channel 1 output (ST1CH1_O) disable"]
    #[inline(always)]
    #[must_use]
    pub fn st1ch1dis(&mut self) -> ST1CH1DIS_W<CHOUTDIS_SPEC, 3> {
        ST1CH1DIS_W::new(self)
    }
    #[doc = "Bit 4 - Slave_TIMER2 channel 0 output (ST2CH0_O) disable"]
    #[inline(always)]
    #[must_use]
    pub fn st2ch0dis(&mut self) -> ST2CH0DIS_W<CHOUTDIS_SPEC, 4> {
        ST2CH0DIS_W::new(self)
    }
    #[doc = "Bit 5 - Slave_TIMER2 channel 1 output (ST2CH1_O) disable"]
    #[inline(always)]
    #[must_use]
    pub fn st2ch1dis(&mut self) -> ST2CH1DIS_W<CHOUTDIS_SPEC, 5> {
        ST2CH1DIS_W::new(self)
    }
    #[doc = "Bit 6 - Slave_TIMER3 channel 0 output (ST3CH0_O) disable"]
    #[inline(always)]
    #[must_use]
    pub fn st3ch0dis(&mut self) -> ST3CH0DIS_W<CHOUTDIS_SPEC, 6> {
        ST3CH0DIS_W::new(self)
    }
    #[doc = "Bit 7 - Slave_TIMER3 channel 1 output (ST3CH1_O) disable"]
    #[inline(always)]
    #[must_use]
    pub fn st3ch1dis(&mut self) -> ST3CH1DIS_W<CHOUTDIS_SPEC, 7> {
        ST3CH1DIS_W::new(self)
    }
    #[doc = "Bit 8 - Slave_TIMER4 channel 0 output (ST4CH0_O) disable"]
    #[inline(always)]
    #[must_use]
    pub fn st4ch0dis(&mut self) -> ST4CH0DIS_W<CHOUTDIS_SPEC, 8> {
        ST4CH0DIS_W::new(self)
    }
    #[doc = "Bit 9 - Slave_TIMER4 channel 1 output (ST4CH1_O) disable"]
    #[inline(always)]
    #[must_use]
    pub fn st4ch1dis(&mut self) -> ST4CH1DIS_W<CHOUTDIS_SPEC, 9> {
        ST4CH1DIS_W::new(self)
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
#[doc = "SHRTIMER channel output disable register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`choutdis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHOUTDIS_SPEC;
impl crate::RegisterSpec for CHOUTDIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`choutdis::W`](W) writer structure"]
impl crate::Writable for CHOUTDIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHOUTDIS to value 0"]
impl crate::Resettable for CHOUTDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
