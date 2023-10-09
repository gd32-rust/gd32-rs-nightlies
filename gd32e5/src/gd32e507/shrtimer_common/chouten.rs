#[doc = "Register `CHOUTEN` reader"]
pub type R = crate::R<CHOUTEN_SPEC>;
#[doc = "Register `CHOUTEN` writer"]
pub type W = crate::W<CHOUTEN_SPEC>;
#[doc = "Field `ST0CH0EN` reader - Slave_TIMER0 channel 0 output (ST0CH0_O) enable"]
pub type ST0CH0EN_R = crate::BitReader;
#[doc = "Field `ST0CH0EN` writer - Slave_TIMER0 channel 0 output (ST0CH0_O) enable"]
pub type ST0CH0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0CH1EN` reader - Slave_TIMER0 channel 1 output (ST0CH1_O) enable"]
pub type ST0CH1EN_R = crate::BitReader;
#[doc = "Field `ST0CH1EN` writer - Slave_TIMER0 channel 1 output (ST0CH1_O) enable"]
pub type ST0CH1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1CH0EN` reader - Slave_TIMER1 channel 0 output (ST1CH0_O) enable"]
pub type ST1CH0EN_R = crate::BitReader;
#[doc = "Field `ST1CH0EN` writer - Slave_TIMER1 channel 0 output (ST1CH0_O) enable"]
pub type ST1CH0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1CH1EN` reader - Slave_TIMER1 channel 1 output (ST1CH1_O) enable"]
pub type ST1CH1EN_R = crate::BitReader;
#[doc = "Field `ST1CH1EN` writer - Slave_TIMER1 channel 1 output (ST1CH1_O) enable"]
pub type ST1CH1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2CH0EN` reader - Slave_TIMER2 channel 0 output (ST2CH0_O) enable"]
pub type ST2CH0EN_R = crate::BitReader;
#[doc = "Field `ST2CH0EN` writer - Slave_TIMER2 channel 0 output (ST2CH0_O) enable"]
pub type ST2CH0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2CH1EN` reader - Slave_TIMER2 channel 1 output (ST2CH1_O) enable"]
pub type ST2CH1EN_R = crate::BitReader;
#[doc = "Field `ST2CH1EN` writer - Slave_TIMER2 channel 1 output (ST2CH1_O) enable"]
pub type ST2CH1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3CH0EN` reader - Slave_TIMER3 channel 0 output (ST3CH0_O) enable"]
pub type ST3CH0EN_R = crate::BitReader;
#[doc = "Field `ST3CH0EN` writer - Slave_TIMER3 channel 0 output (ST3CH0_O) enable"]
pub type ST3CH0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3CH1EN` reader - Slave_TIMER3 channel 1 output (ST3CH1_O) enable"]
pub type ST3CH1EN_R = crate::BitReader;
#[doc = "Field `ST3CH1EN` writer - Slave_TIMER3 channel 1 output (ST3CH1_O) enable"]
pub type ST3CH1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4CH0EN` reader - Slave_TIMER4 channel 0 output (ST4CH0_O) enable"]
pub type ST4CH0EN_R = crate::BitReader;
#[doc = "Field `ST4CH0EN` writer - Slave_TIMER4 channel 0 output (ST4CH0_O) enable"]
pub type ST4CH0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4CH1EN` reader - Slave_TIMER4 channel 1 output (ST4CH1_O) enable"]
pub type ST4CH1EN_R = crate::BitReader;
#[doc = "Field `ST4CH1EN` writer - Slave_TIMER4 channel 1 output (ST4CH1_O) enable"]
pub type ST4CH1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Slave_TIMER0 channel 0 output (ST0CH0_O) enable"]
    #[inline(always)]
    pub fn st0ch0en(&self) -> ST0CH0EN_R {
        ST0CH0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave_TIMER0 channel 1 output (ST0CH1_O) enable"]
    #[inline(always)]
    pub fn st0ch1en(&self) -> ST0CH1EN_R {
        ST0CH1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave_TIMER1 channel 0 output (ST1CH0_O) enable"]
    #[inline(always)]
    pub fn st1ch0en(&self) -> ST1CH0EN_R {
        ST1CH0EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave_TIMER1 channel 1 output (ST1CH1_O) enable"]
    #[inline(always)]
    pub fn st1ch1en(&self) -> ST1CH1EN_R {
        ST1CH1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave_TIMER2 channel 0 output (ST2CH0_O) enable"]
    #[inline(always)]
    pub fn st2ch0en(&self) -> ST2CH0EN_R {
        ST2CH0EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave_TIMER2 channel 1 output (ST2CH1_O) enable"]
    #[inline(always)]
    pub fn st2ch1en(&self) -> ST2CH1EN_R {
        ST2CH1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slave_TIMER3 channel 0 output (ST3CH0_O) enable"]
    #[inline(always)]
    pub fn st3ch0en(&self) -> ST3CH0EN_R {
        ST3CH0EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave_TIMER3 channel 1 output (ST3CH1_O) enable"]
    #[inline(always)]
    pub fn st3ch1en(&self) -> ST3CH1EN_R {
        ST3CH1EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave_TIMER4 channel 0 output (ST4CH0_O) enable"]
    #[inline(always)]
    pub fn st4ch0en(&self) -> ST4CH0EN_R {
        ST4CH0EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave_TIMER4 channel 1 output (ST4CH1_O) enable"]
    #[inline(always)]
    pub fn st4ch1en(&self) -> ST4CH1EN_R {
        ST4CH1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave_TIMER0 channel 0 output (ST0CH0_O) enable"]
    #[inline(always)]
    #[must_use]
    pub fn st0ch0en(&mut self) -> ST0CH0EN_W<CHOUTEN_SPEC, 0> {
        ST0CH0EN_W::new(self)
    }
    #[doc = "Bit 1 - Slave_TIMER0 channel 1 output (ST0CH1_O) enable"]
    #[inline(always)]
    #[must_use]
    pub fn st0ch1en(&mut self) -> ST0CH1EN_W<CHOUTEN_SPEC, 1> {
        ST0CH1EN_W::new(self)
    }
    #[doc = "Bit 2 - Slave_TIMER1 channel 0 output (ST1CH0_O) enable"]
    #[inline(always)]
    #[must_use]
    pub fn st1ch0en(&mut self) -> ST1CH0EN_W<CHOUTEN_SPEC, 2> {
        ST1CH0EN_W::new(self)
    }
    #[doc = "Bit 3 - Slave_TIMER1 channel 1 output (ST1CH1_O) enable"]
    #[inline(always)]
    #[must_use]
    pub fn st1ch1en(&mut self) -> ST1CH1EN_W<CHOUTEN_SPEC, 3> {
        ST1CH1EN_W::new(self)
    }
    #[doc = "Bit 4 - Slave_TIMER2 channel 0 output (ST2CH0_O) enable"]
    #[inline(always)]
    #[must_use]
    pub fn st2ch0en(&mut self) -> ST2CH0EN_W<CHOUTEN_SPEC, 4> {
        ST2CH0EN_W::new(self)
    }
    #[doc = "Bit 5 - Slave_TIMER2 channel 1 output (ST2CH1_O) enable"]
    #[inline(always)]
    #[must_use]
    pub fn st2ch1en(&mut self) -> ST2CH1EN_W<CHOUTEN_SPEC, 5> {
        ST2CH1EN_W::new(self)
    }
    #[doc = "Bit 6 - Slave_TIMER3 channel 0 output (ST3CH0_O) enable"]
    #[inline(always)]
    #[must_use]
    pub fn st3ch0en(&mut self) -> ST3CH0EN_W<CHOUTEN_SPEC, 6> {
        ST3CH0EN_W::new(self)
    }
    #[doc = "Bit 7 - Slave_TIMER3 channel 1 output (ST3CH1_O) enable"]
    #[inline(always)]
    #[must_use]
    pub fn st3ch1en(&mut self) -> ST3CH1EN_W<CHOUTEN_SPEC, 7> {
        ST3CH1EN_W::new(self)
    }
    #[doc = "Bit 8 - Slave_TIMER4 channel 0 output (ST4CH0_O) enable"]
    #[inline(always)]
    #[must_use]
    pub fn st4ch0en(&mut self) -> ST4CH0EN_W<CHOUTEN_SPEC, 8> {
        ST4CH0EN_W::new(self)
    }
    #[doc = "Bit 9 - Slave_TIMER4 channel 1 output (ST4CH1_O) enable"]
    #[inline(always)]
    #[must_use]
    pub fn st4ch1en(&mut self) -> ST4CH1EN_W<CHOUTEN_SPEC, 9> {
        ST4CH1EN_W::new(self)
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
#[doc = "SHRTIMER channel output enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chouten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chouten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHOUTEN_SPEC;
impl crate::RegisterSpec for CHOUTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chouten::R`](R) reader structure"]
impl crate::Readable for CHOUTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chouten::W`](W) writer structure"]
impl crate::Writable for CHOUTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHOUTEN to value 0"]
impl crate::Resettable for CHOUTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
