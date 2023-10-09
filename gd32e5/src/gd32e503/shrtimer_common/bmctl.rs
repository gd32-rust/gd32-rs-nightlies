#[doc = "Register `BMCTL` reader"]
pub type R = crate::R<BMCTL_SPEC>;
#[doc = "Register `BMCTL` writer"]
pub type W = crate::W<BMCTL_SPEC>;
#[doc = "Field `BMEN` reader - Bunch mode enable"]
pub type BMEN_R = crate::BitReader;
#[doc = "Field `BMEN` writer - Bunch mode enable"]
pub type BMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMCTN` reader - Continuous mode in bunch mode"]
pub type BMCTN_R = crate::BitReader;
#[doc = "Field `BMCTN` writer - Continuous mode in bunch mode"]
pub type BMCTN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMCLKS` reader - Bunch mode clock source"]
pub type BMCLKS_R = crate::FieldReader;
#[doc = "Field `BMCLKS` writer - Bunch mode clock source"]
pub type BMCLKS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `BMPSC` reader - Bunch mode clock division"]
pub type BMPSC_R = crate::FieldReader;
#[doc = "Field `BMPSC` writer - Bunch mode clock division"]
pub type BMPSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `BMSE` reader - Bunch mode shadow enable"]
pub type BMSE_R = crate::BitReader;
#[doc = "Field `BMSE` writer - Bunch mode shadow enable"]
pub type BMSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMMT` reader - Master_TIMER bunch mode"]
pub type BMMT_R = crate::BitReader;
#[doc = "Field `BMMT` writer - Master_TIMER bunch mode"]
pub type BMMT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMST0` reader - Slave_TIMER0 bunch mode"]
pub type BMST0_R = crate::BitReader;
#[doc = "Field `BMST0` writer - Slave_TIMER0 bunch mode"]
pub type BMST0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMST1` reader - Slave_TIMER1 bunch mode"]
pub type BMST1_R = crate::BitReader;
#[doc = "Field `BMST1` writer - Slave_TIMER1 bunch mode"]
pub type BMST1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMST2` reader - Slave_TIMER2 bunch mode"]
pub type BMST2_R = crate::BitReader;
#[doc = "Field `BMST2` writer - Slave_TIMER2 bunch mode"]
pub type BMST2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMST3` reader - Slave_TIMER3 bunch mode"]
pub type BMST3_R = crate::BitReader;
#[doc = "Field `BMST3` writer - Slave_TIMER3 bunch mode"]
pub type BMST3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMST4` reader - Slave_TIMER4 bunch mode"]
pub type BMST4_R = crate::BitReader;
#[doc = "Field `BMST4` writer - Slave_TIMER4 bunch mode"]
pub type BMST4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMOPTF` reader - Bunch mode operating flag"]
pub type BMOPTF_R = crate::BitReader;
#[doc = "Field `BMOPTF` writer - Bunch mode operating flag"]
pub type BMOPTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Bunch mode enable"]
    #[inline(always)]
    pub fn bmen(&self) -> BMEN_R {
        BMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous mode in bunch mode"]
    #[inline(always)]
    pub fn bmctn(&self) -> BMCTN_R {
        BMCTN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Bunch mode clock source"]
    #[inline(always)]
    pub fn bmclks(&self) -> BMCLKS_R {
        BMCLKS_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Bunch mode clock division"]
    #[inline(always)]
    pub fn bmpsc(&self) -> BMPSC_R {
        BMPSC_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Bunch mode shadow enable"]
    #[inline(always)]
    pub fn bmse(&self) -> BMSE_R {
        BMSE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Master_TIMER bunch mode"]
    #[inline(always)]
    pub fn bmmt(&self) -> BMMT_R {
        BMMT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave_TIMER0 bunch mode"]
    #[inline(always)]
    pub fn bmst0(&self) -> BMST0_R {
        BMST0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave_TIMER1 bunch mode"]
    #[inline(always)]
    pub fn bmst1(&self) -> BMST1_R {
        BMST1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave_TIMER2 bunch mode"]
    #[inline(always)]
    pub fn bmst2(&self) -> BMST2_R {
        BMST2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Slave_TIMER3 bunch mode"]
    #[inline(always)]
    pub fn bmst3(&self) -> BMST3_R {
        BMST3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Slave_TIMER4 bunch mode"]
    #[inline(always)]
    pub fn bmst4(&self) -> BMST4_R {
        BMST4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Bunch mode operating flag"]
    #[inline(always)]
    pub fn bmoptf(&self) -> BMOPTF_R {
        BMOPTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bunch mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmen(&mut self) -> BMEN_W<BMCTL_SPEC, 0> {
        BMEN_W::new(self)
    }
    #[doc = "Bit 1 - Continuous mode in bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmctn(&mut self) -> BMCTN_W<BMCTL_SPEC, 1> {
        BMCTN_W::new(self)
    }
    #[doc = "Bits 2:5 - Bunch mode clock source"]
    #[inline(always)]
    #[must_use]
    pub fn bmclks(&mut self) -> BMCLKS_W<BMCTL_SPEC, 2> {
        BMCLKS_W::new(self)
    }
    #[doc = "Bits 6:9 - Bunch mode clock division"]
    #[inline(always)]
    #[must_use]
    pub fn bmpsc(&mut self) -> BMPSC_W<BMCTL_SPEC, 6> {
        BMPSC_W::new(self)
    }
    #[doc = "Bit 10 - Bunch mode shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmse(&mut self) -> BMSE_W<BMCTL_SPEC, 10> {
        BMSE_W::new(self)
    }
    #[doc = "Bit 16 - Master_TIMER bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmmt(&mut self) -> BMMT_W<BMCTL_SPEC, 16> {
        BMMT_W::new(self)
    }
    #[doc = "Bit 17 - Slave_TIMER0 bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmst0(&mut self) -> BMST0_W<BMCTL_SPEC, 17> {
        BMST0_W::new(self)
    }
    #[doc = "Bit 18 - Slave_TIMER1 bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmst1(&mut self) -> BMST1_W<BMCTL_SPEC, 18> {
        BMST1_W::new(self)
    }
    #[doc = "Bit 19 - Slave_TIMER2 bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmst2(&mut self) -> BMST2_W<BMCTL_SPEC, 19> {
        BMST2_W::new(self)
    }
    #[doc = "Bit 20 - Slave_TIMER3 bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmst3(&mut self) -> BMST3_W<BMCTL_SPEC, 20> {
        BMST3_W::new(self)
    }
    #[doc = "Bit 21 - Slave_TIMER4 bunch mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmst4(&mut self) -> BMST4_W<BMCTL_SPEC, 21> {
        BMST4_W::new(self)
    }
    #[doc = "Bit 31 - Bunch mode operating flag"]
    #[inline(always)]
    #[must_use]
    pub fn bmoptf(&mut self) -> BMOPTF_W<BMCTL_SPEC, 31> {
        BMOPTF_W::new(self)
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
#[doc = "SHRTIMER bunch mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMCTL_SPEC;
impl crate::RegisterSpec for BMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmctl::R`](R) reader structure"]
impl crate::Readable for BMCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmctl::W`](W) writer structure"]
impl crate::Writable for BMCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMCTL to value 0"]
impl crate::Resettable for BMCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
