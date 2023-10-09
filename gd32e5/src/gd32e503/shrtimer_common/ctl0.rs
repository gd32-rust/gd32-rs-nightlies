#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `MTUPDIS` reader - Master_TIMER update disable"]
pub type MTUPDIS_R = crate::BitReader;
#[doc = "Field `MTUPDIS` writer - Master_TIMER update disable"]
pub type MTUPDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0UPDIS` reader - Slave_TIMER0 update disable"]
pub type ST0UPDIS_R = crate::BitReader;
#[doc = "Field `ST0UPDIS` writer - Slave_TIMER0 update disable"]
pub type ST0UPDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2UPDIS` reader - Slave_TIMER2 update disable"]
pub type ST2UPDIS_R = crate::BitReader;
#[doc = "Field `ST2UPDIS` writer - Slave_TIMER2 update disable"]
pub type ST2UPDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3UPDIS` reader - Slave_TIMER3 update disable"]
pub type ST3UPDIS_R = crate::BitReader;
#[doc = "Field `ST3UPDIS` writer - Slave_TIMER3 update disable"]
pub type ST3UPDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4UPDIS` reader - Slave_TIMER4 update disable"]
pub type ST4UPDIS_R = crate::BitReader;
#[doc = "Field `ST4UPDIS` writer - Slave_TIMER4 update disable"]
pub type ST4UPDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADTG0USRC` reader - SHRTIMER_ADCTRIG0 update source"]
pub type ADTG0USRC_R = crate::FieldReader;
#[doc = "Field `ADTG0USRC` writer - SHRTIMER_ADCTRIG0 update source"]
pub type ADTG0USRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ADTG1USRC` reader - SHRTIMER_ADCTRIG1 update source"]
pub type ADTG1USRC_R = crate::FieldReader;
#[doc = "Field `ADTG1USRC` writer - SHRTIMER_ADCTRIG1 update source"]
pub type ADTG1USRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ADTG2USRC` reader - SHRTIMER_ADCTRIG2 update source"]
pub type ADTG2USRC_R = crate::FieldReader;
#[doc = "Field `ADTG2USRC` writer - SHRTIMER_ADCTRIG2 update source"]
pub type ADTG2USRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ADTG3USRC` reader - SHRTIMER_ADCTRIG3 update source"]
pub type ADTG3USRC_R = crate::FieldReader;
#[doc = "Field `ADTG3USRC` writer - SHRTIMER_ADCTRIG3 update source"]
pub type ADTG3USRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - Master_TIMER update disable"]
    #[inline(always)]
    pub fn mtupdis(&self) -> MTUPDIS_R {
        MTUPDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave_TIMER0 update disable"]
    #[inline(always)]
    pub fn st0updis(&self) -> ST0UPDIS_R {
        ST0UPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave_TIMER2 update disable"]
    #[inline(always)]
    pub fn st2updis(&self) -> ST2UPDIS_R {
        ST2UPDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave_TIMER3 update disable"]
    #[inline(always)]
    pub fn st3updis(&self) -> ST3UPDIS_R {
        ST3UPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave_TIMER4 update disable"]
    #[inline(always)]
    pub fn st4updis(&self) -> ST4UPDIS_R {
        ST4UPDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SHRTIMER_ADCTRIG0 update source"]
    #[inline(always)]
    pub fn adtg0usrc(&self) -> ADTG0USRC_R {
        ADTG0USRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - SHRTIMER_ADCTRIG1 update source"]
    #[inline(always)]
    pub fn adtg1usrc(&self) -> ADTG1USRC_R {
        ADTG1USRC_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - SHRTIMER_ADCTRIG2 update source"]
    #[inline(always)]
    pub fn adtg2usrc(&self) -> ADTG2USRC_R {
        ADTG2USRC_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - SHRTIMER_ADCTRIG3 update source"]
    #[inline(always)]
    pub fn adtg3usrc(&self) -> ADTG3USRC_R {
        ADTG3USRC_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master_TIMER update disable"]
    #[inline(always)]
    #[must_use]
    pub fn mtupdis(&mut self) -> MTUPDIS_W<CTL0_SPEC, 0> {
        MTUPDIS_W::new(self)
    }
    #[doc = "Bit 1 - Slave_TIMER0 update disable"]
    #[inline(always)]
    #[must_use]
    pub fn st0updis(&mut self) -> ST0UPDIS_W<CTL0_SPEC, 1> {
        ST0UPDIS_W::new(self)
    }
    #[doc = "Bit 3 - Slave_TIMER2 update disable"]
    #[inline(always)]
    #[must_use]
    pub fn st2updis(&mut self) -> ST2UPDIS_W<CTL0_SPEC, 3> {
        ST2UPDIS_W::new(self)
    }
    #[doc = "Bit 4 - Slave_TIMER3 update disable"]
    #[inline(always)]
    #[must_use]
    pub fn st3updis(&mut self) -> ST3UPDIS_W<CTL0_SPEC, 4> {
        ST3UPDIS_W::new(self)
    }
    #[doc = "Bit 5 - Slave_TIMER4 update disable"]
    #[inline(always)]
    #[must_use]
    pub fn st4updis(&mut self) -> ST4UPDIS_W<CTL0_SPEC, 5> {
        ST4UPDIS_W::new(self)
    }
    #[doc = "Bits 16:18 - SHRTIMER_ADCTRIG0 update source"]
    #[inline(always)]
    #[must_use]
    pub fn adtg0usrc(&mut self) -> ADTG0USRC_W<CTL0_SPEC, 16> {
        ADTG0USRC_W::new(self)
    }
    #[doc = "Bits 19:21 - SHRTIMER_ADCTRIG1 update source"]
    #[inline(always)]
    #[must_use]
    pub fn adtg1usrc(&mut self) -> ADTG1USRC_W<CTL0_SPEC, 19> {
        ADTG1USRC_W::new(self)
    }
    #[doc = "Bits 22:24 - SHRTIMER_ADCTRIG2 update source"]
    #[inline(always)]
    #[must_use]
    pub fn adtg2usrc(&mut self) -> ADTG2USRC_W<CTL0_SPEC, 22> {
        ADTG2USRC_W::new(self)
    }
    #[doc = "Bits 25:27 - SHRTIMER_ADCTRIG3 update source"]
    #[inline(always)]
    #[must_use]
    pub fn adtg3usrc(&mut self) -> ADTG3USRC_W<CTL0_SPEC, 25> {
        ADTG3USRC_W::new(self)
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
#[doc = "SHRTIMER control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
