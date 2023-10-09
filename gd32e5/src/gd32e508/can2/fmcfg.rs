#[doc = "Register `FMCFG` reader"]
pub type R = crate::R<FMCFG_SPEC>;
#[doc = "Register `FMCFG` writer"]
pub type W = crate::W<FMCFG_SPEC>;
#[doc = "Field `FMOD0` reader - Filter mode"]
pub type FMOD0_R = crate::BitReader;
#[doc = "Field `FMOD0` writer - Filter mode"]
pub type FMOD0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD1` reader - Filter mode"]
pub type FMOD1_R = crate::BitReader;
#[doc = "Field `FMOD1` writer - Filter mode"]
pub type FMOD1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD2` reader - Filter mode"]
pub type FMOD2_R = crate::BitReader;
#[doc = "Field `FMOD2` writer - Filter mode"]
pub type FMOD2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD3` reader - Filter mode"]
pub type FMOD3_R = crate::BitReader;
#[doc = "Field `FMOD3` writer - Filter mode"]
pub type FMOD3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD4` reader - Filter mode"]
pub type FMOD4_R = crate::BitReader;
#[doc = "Field `FMOD4` writer - Filter mode"]
pub type FMOD4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD5` reader - Filter mode"]
pub type FMOD5_R = crate::BitReader;
#[doc = "Field `FMOD5` writer - Filter mode"]
pub type FMOD5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD6` reader - Filter mode"]
pub type FMOD6_R = crate::BitReader;
#[doc = "Field `FMOD6` writer - Filter mode"]
pub type FMOD6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD7` reader - Filter mode"]
pub type FMOD7_R = crate::BitReader;
#[doc = "Field `FMOD7` writer - Filter mode"]
pub type FMOD7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD8` reader - Filter mode"]
pub type FMOD8_R = crate::BitReader;
#[doc = "Field `FMOD8` writer - Filter mode"]
pub type FMOD8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD9` reader - Filter mode"]
pub type FMOD9_R = crate::BitReader;
#[doc = "Field `FMOD9` writer - Filter mode"]
pub type FMOD9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD10` reader - Filter mode"]
pub type FMOD10_R = crate::BitReader;
#[doc = "Field `FMOD10` writer - Filter mode"]
pub type FMOD10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD11` reader - Filter mode"]
pub type FMOD11_R = crate::BitReader;
#[doc = "Field `FMOD11` writer - Filter mode"]
pub type FMOD11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD12` reader - Filter mode"]
pub type FMOD12_R = crate::BitReader;
#[doc = "Field `FMOD12` writer - Filter mode"]
pub type FMOD12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMOD13` reader - Filter mode"]
pub type FMOD13_R = crate::BitReader;
#[doc = "Field `FMOD13` writer - Filter mode"]
pub type FMOD13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fmod0(&self) -> FMOD0_R {
        FMOD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fmod1(&self) -> FMOD1_R {
        FMOD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fmod2(&self) -> FMOD2_R {
        FMOD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fmod3(&self) -> FMOD3_R {
        FMOD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fmod4(&self) -> FMOD4_R {
        FMOD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fmod5(&self) -> FMOD5_R {
        FMOD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fmod6(&self) -> FMOD6_R {
        FMOD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fmod7(&self) -> FMOD7_R {
        FMOD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fmod8(&self) -> FMOD8_R {
        FMOD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fmod9(&self) -> FMOD9_R {
        FMOD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fmod10(&self) -> FMOD10_R {
        FMOD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fmod11(&self) -> FMOD11_R {
        FMOD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fmod12(&self) -> FMOD12_R {
        FMOD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fmod13(&self) -> FMOD13_R {
        FMOD13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod0(&mut self) -> FMOD0_W<FMCFG_SPEC, 0> {
        FMOD0_W::new(self)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod1(&mut self) -> FMOD1_W<FMCFG_SPEC, 1> {
        FMOD1_W::new(self)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod2(&mut self) -> FMOD2_W<FMCFG_SPEC, 2> {
        FMOD2_W::new(self)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod3(&mut self) -> FMOD3_W<FMCFG_SPEC, 3> {
        FMOD3_W::new(self)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod4(&mut self) -> FMOD4_W<FMCFG_SPEC, 4> {
        FMOD4_W::new(self)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod5(&mut self) -> FMOD5_W<FMCFG_SPEC, 5> {
        FMOD5_W::new(self)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod6(&mut self) -> FMOD6_W<FMCFG_SPEC, 6> {
        FMOD6_W::new(self)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod7(&mut self) -> FMOD7_W<FMCFG_SPEC, 7> {
        FMOD7_W::new(self)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod8(&mut self) -> FMOD8_W<FMCFG_SPEC, 8> {
        FMOD8_W::new(self)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod9(&mut self) -> FMOD9_W<FMCFG_SPEC, 9> {
        FMOD9_W::new(self)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod10(&mut self) -> FMOD10_W<FMCFG_SPEC, 10> {
        FMOD10_W::new(self)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod11(&mut self) -> FMOD11_W<FMCFG_SPEC, 11> {
        FMOD11_W::new(self)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod12(&mut self) -> FMOD12_W<FMCFG_SPEC, 12> {
        FMOD12_W::new(self)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod13(&mut self) -> FMOD13_W<FMCFG_SPEC, 13> {
        FMOD13_W::new(self)
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
#[doc = "Filter mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMCFG_SPEC;
impl crate::RegisterSpec for FMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmcfg::R`](R) reader structure"]
impl crate::Readable for FMCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmcfg::W`](W) writer structure"]
impl crate::Writable for FMCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMCFG to value 0"]
impl crate::Resettable for FMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
