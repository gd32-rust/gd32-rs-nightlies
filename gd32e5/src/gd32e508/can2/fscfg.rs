#[doc = "Register `FSCFG` reader"]
pub type R = crate::R<FSCFG_SPEC>;
#[doc = "Register `FSCFG` writer"]
pub type W = crate::W<FSCFG_SPEC>;
#[doc = "Field `FS0` reader - Filter scale configuration"]
pub type FS0_R = crate::BitReader;
#[doc = "Field `FS0` writer - Filter scale configuration"]
pub type FS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS1` reader - Filter scale configuration"]
pub type FS1_R = crate::BitReader;
#[doc = "Field `FS1` writer - Filter scale configuration"]
pub type FS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS2` reader - Filter scale configuration"]
pub type FS2_R = crate::BitReader;
#[doc = "Field `FS2` writer - Filter scale configuration"]
pub type FS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS3` reader - Filter scale configuration"]
pub type FS3_R = crate::BitReader;
#[doc = "Field `FS3` writer - Filter scale configuration"]
pub type FS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS4` reader - Filter scale configuration"]
pub type FS4_R = crate::BitReader;
#[doc = "Field `FS4` writer - Filter scale configuration"]
pub type FS4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS5` reader - Filter scale configuration"]
pub type FS5_R = crate::BitReader;
#[doc = "Field `FS5` writer - Filter scale configuration"]
pub type FS5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS6` reader - Filter scale configuration"]
pub type FS6_R = crate::BitReader;
#[doc = "Field `FS6` writer - Filter scale configuration"]
pub type FS6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS7` reader - Filter scale configuration"]
pub type FS7_R = crate::BitReader;
#[doc = "Field `FS7` writer - Filter scale configuration"]
pub type FS7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS8` reader - Filter scale configuration"]
pub type FS8_R = crate::BitReader;
#[doc = "Field `FS8` writer - Filter scale configuration"]
pub type FS8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS9` reader - Filter scale configuration"]
pub type FS9_R = crate::BitReader;
#[doc = "Field `FS9` writer - Filter scale configuration"]
pub type FS9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS10` reader - Filter scale configuration"]
pub type FS10_R = crate::BitReader;
#[doc = "Field `FS10` writer - Filter scale configuration"]
pub type FS10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS11` reader - Filter scale configuration"]
pub type FS11_R = crate::BitReader;
#[doc = "Field `FS11` writer - Filter scale configuration"]
pub type FS11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS12` reader - Filter scale configuration"]
pub type FS12_R = crate::BitReader;
#[doc = "Field `FS12` writer - Filter scale configuration"]
pub type FS12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FS13` reader - Filter scale configuration"]
pub type FS13_R = crate::BitReader;
#[doc = "Field `FS13` writer - Filter scale configuration"]
pub type FS13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs0(&self) -> FS0_R {
        FS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs1(&self) -> FS1_R {
        FS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs2(&self) -> FS2_R {
        FS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs3(&self) -> FS3_R {
        FS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs4(&self) -> FS4_R {
        FS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs5(&self) -> FS5_R {
        FS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs6(&self) -> FS6_R {
        FS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs7(&self) -> FS7_R {
        FS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs8(&self) -> FS8_R {
        FS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs9(&self) -> FS9_R {
        FS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs10(&self) -> FS10_R {
        FS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs11(&self) -> FS11_R {
        FS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs12(&self) -> FS12_R {
        FS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs13(&self) -> FS13_R {
        FS13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs0(&mut self) -> FS0_W<FSCFG_SPEC, 0> {
        FS0_W::new(self)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs1(&mut self) -> FS1_W<FSCFG_SPEC, 1> {
        FS1_W::new(self)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs2(&mut self) -> FS2_W<FSCFG_SPEC, 2> {
        FS2_W::new(self)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs3(&mut self) -> FS3_W<FSCFG_SPEC, 3> {
        FS3_W::new(self)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs4(&mut self) -> FS4_W<FSCFG_SPEC, 4> {
        FS4_W::new(self)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs5(&mut self) -> FS5_W<FSCFG_SPEC, 5> {
        FS5_W::new(self)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs6(&mut self) -> FS6_W<FSCFG_SPEC, 6> {
        FS6_W::new(self)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs7(&mut self) -> FS7_W<FSCFG_SPEC, 7> {
        FS7_W::new(self)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs8(&mut self) -> FS8_W<FSCFG_SPEC, 8> {
        FS8_W::new(self)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs9(&mut self) -> FS9_W<FSCFG_SPEC, 9> {
        FS9_W::new(self)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs10(&mut self) -> FS10_W<FSCFG_SPEC, 10> {
        FS10_W::new(self)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs11(&mut self) -> FS11_W<FSCFG_SPEC, 11> {
        FS11_W::new(self)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs12(&mut self) -> FS12_W<FSCFG_SPEC, 12> {
        FS12_W::new(self)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs13(&mut self) -> FS13_W<FSCFG_SPEC, 13> {
        FS13_W::new(self)
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
#[doc = "Filter scale configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSCFG_SPEC;
impl crate::RegisterSpec for FSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscfg::R`](R) reader structure"]
impl crate::Readable for FSCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fscfg::W`](W) writer structure"]
impl crate::Writable for FSCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSCFG to value 0"]
impl crate::Resettable for FSCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
