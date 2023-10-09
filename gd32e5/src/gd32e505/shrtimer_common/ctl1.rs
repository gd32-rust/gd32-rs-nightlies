#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `MTSUP` reader - Master_TIMER software update"]
pub type MTSUP_R = crate::BitReader;
#[doc = "Field `MTSUP` writer - Master_TIMER software update"]
pub type MTSUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0SUP` reader - Slave_TIMER0 software update"]
pub type ST0SUP_R = crate::BitReader;
#[doc = "Field `ST0SUP` writer - Slave_TIMER0 software update"]
pub type ST0SUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1SUP` reader - Slave_TIMER1 software update"]
pub type ST1SUP_R = crate::BitReader;
#[doc = "Field `ST1SUP` writer - Slave_TIMER1 software update"]
pub type ST1SUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2SUP` reader - Slave_TIMER2 software update"]
pub type ST2SUP_R = crate::BitReader;
#[doc = "Field `ST2SUP` writer - Slave_TIMER2 software update"]
pub type ST2SUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3SUP` reader - Slave_TIMER3 software update"]
pub type ST3SUP_R = crate::BitReader;
#[doc = "Field `ST3SUP` writer - Slave_TIMER3 software update"]
pub type ST3SUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4SUP` reader - Slave_TIMER4 software update"]
pub type ST4SUP_R = crate::BitReader;
#[doc = "Field `ST4SUP` writer - Slave_TIMER4 software update"]
pub type ST4SUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTSRST` reader - Master_TIMER software reset"]
pub type MTSRST_R = crate::BitReader;
#[doc = "Field `MTSRST` writer - Master_TIMER software reset"]
pub type MTSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0SRST` reader - Slave_TIMER0 software reset"]
pub type ST0SRST_R = crate::BitReader;
#[doc = "Field `ST0SRST` writer - Slave_TIMER0 software reset"]
pub type ST0SRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1SRST` reader - Slave_TIMER1 software reset"]
pub type ST1SRST_R = crate::BitReader;
#[doc = "Field `ST1SRST` writer - Slave_TIMER1 software reset"]
pub type ST1SRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2SRST` reader - Slave_TIMER2 software reset"]
pub type ST2SRST_R = crate::BitReader;
#[doc = "Field `ST2SRST` writer - Slave_TIMER2 software reset"]
pub type ST2SRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3SRST` reader - Slave_TIMER3 software reset"]
pub type ST3SRST_R = crate::BitReader;
#[doc = "Field `ST3SRST` writer - Slave_TIMER3 software reset"]
pub type ST3SRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4SRST` reader - Slave_TIMER4 software reset"]
pub type ST4SRST_R = crate::BitReader;
#[doc = "Field `ST4SRST` writer - Slave_TIMER4 software reset"]
pub type ST4SRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Master_TIMER software update"]
    #[inline(always)]
    pub fn mtsup(&self) -> MTSUP_R {
        MTSUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave_TIMER0 software update"]
    #[inline(always)]
    pub fn st0sup(&self) -> ST0SUP_R {
        ST0SUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave_TIMER1 software update"]
    #[inline(always)]
    pub fn st1sup(&self) -> ST1SUP_R {
        ST1SUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave_TIMER2 software update"]
    #[inline(always)]
    pub fn st2sup(&self) -> ST2SUP_R {
        ST2SUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave_TIMER3 software update"]
    #[inline(always)]
    pub fn st3sup(&self) -> ST3SUP_R {
        ST3SUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave_TIMER4 software update"]
    #[inline(always)]
    pub fn st4sup(&self) -> ST4SUP_R {
        ST4SUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Master_TIMER software reset"]
    #[inline(always)]
    pub fn mtsrst(&self) -> MTSRST_R {
        MTSRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave_TIMER0 software reset"]
    #[inline(always)]
    pub fn st0srst(&self) -> ST0SRST_R {
        ST0SRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Slave_TIMER1 software reset"]
    #[inline(always)]
    pub fn st1srst(&self) -> ST1SRST_R {
        ST1SRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave_TIMER2 software reset"]
    #[inline(always)]
    pub fn st2srst(&self) -> ST2SRST_R {
        ST2SRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slave_TIMER3 software reset"]
    #[inline(always)]
    pub fn st3srst(&self) -> ST3SRST_R {
        ST3SRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Slave_TIMER4 software reset"]
    #[inline(always)]
    pub fn st4srst(&self) -> ST4SRST_R {
        ST4SRST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master_TIMER software update"]
    #[inline(always)]
    #[must_use]
    pub fn mtsup(&mut self) -> MTSUP_W<CTL1_SPEC, 0> {
        MTSUP_W::new(self)
    }
    #[doc = "Bit 1 - Slave_TIMER0 software update"]
    #[inline(always)]
    #[must_use]
    pub fn st0sup(&mut self) -> ST0SUP_W<CTL1_SPEC, 1> {
        ST0SUP_W::new(self)
    }
    #[doc = "Bit 2 - Slave_TIMER1 software update"]
    #[inline(always)]
    #[must_use]
    pub fn st1sup(&mut self) -> ST1SUP_W<CTL1_SPEC, 2> {
        ST1SUP_W::new(self)
    }
    #[doc = "Bit 3 - Slave_TIMER2 software update"]
    #[inline(always)]
    #[must_use]
    pub fn st2sup(&mut self) -> ST2SUP_W<CTL1_SPEC, 3> {
        ST2SUP_W::new(self)
    }
    #[doc = "Bit 4 - Slave_TIMER3 software update"]
    #[inline(always)]
    #[must_use]
    pub fn st3sup(&mut self) -> ST3SUP_W<CTL1_SPEC, 4> {
        ST3SUP_W::new(self)
    }
    #[doc = "Bit 5 - Slave_TIMER4 software update"]
    #[inline(always)]
    #[must_use]
    pub fn st4sup(&mut self) -> ST4SUP_W<CTL1_SPEC, 5> {
        ST4SUP_W::new(self)
    }
    #[doc = "Bit 8 - Master_TIMER software reset"]
    #[inline(always)]
    #[must_use]
    pub fn mtsrst(&mut self) -> MTSRST_W<CTL1_SPEC, 8> {
        MTSRST_W::new(self)
    }
    #[doc = "Bit 9 - Slave_TIMER0 software reset"]
    #[inline(always)]
    #[must_use]
    pub fn st0srst(&mut self) -> ST0SRST_W<CTL1_SPEC, 9> {
        ST0SRST_W::new(self)
    }
    #[doc = "Bit 10 - Slave_TIMER1 software reset"]
    #[inline(always)]
    #[must_use]
    pub fn st1srst(&mut self) -> ST1SRST_W<CTL1_SPEC, 10> {
        ST1SRST_W::new(self)
    }
    #[doc = "Bit 11 - Slave_TIMER2 software reset"]
    #[inline(always)]
    #[must_use]
    pub fn st2srst(&mut self) -> ST2SRST_W<CTL1_SPEC, 11> {
        ST2SRST_W::new(self)
    }
    #[doc = "Bit 12 - Slave_TIMER3 software reset"]
    #[inline(always)]
    #[must_use]
    pub fn st3srst(&mut self) -> ST3SRST_W<CTL1_SPEC, 12> {
        ST3SRST_W::new(self)
    }
    #[doc = "Bit 13 - Slave_TIMER4 software reset"]
    #[inline(always)]
    #[must_use]
    pub fn st4srst(&mut self) -> ST4SRST_W<CTL1_SPEC, 13> {
        ST4SRST_W::new(self)
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
#[doc = "SHRTIMER control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
