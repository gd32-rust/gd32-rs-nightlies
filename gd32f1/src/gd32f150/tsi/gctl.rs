#[doc = "Register `GCTL` reader"]
pub type R = crate::R<GCTL_SPEC>;
#[doc = "Register `GCTL` writer"]
pub type W = crate::W<GCTL_SPEC>;
#[doc = "Field `GE0` reader - Analog I/O group 0 enable"]
pub type GE0_R = crate::BitReader;
#[doc = "Field `GE0` writer - Analog I/O group 0 enable"]
pub type GE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GE1` reader - Analog I/O group 1 enable"]
pub type GE1_R = crate::BitReader;
#[doc = "Field `GE1` writer - Analog I/O group 1 enable"]
pub type GE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GE2` reader - Analog I/O group 2 enable"]
pub type GE2_R = crate::BitReader;
#[doc = "Field `GE2` writer - Analog I/O group 2 enable"]
pub type GE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GE3` reader - Analog I/O group 3 enable"]
pub type GE3_R = crate::BitReader;
#[doc = "Field `GE3` writer - Analog I/O group 3 enable"]
pub type GE3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GE4` reader - Analog I/O group 4 enable"]
pub type GE4_R = crate::BitReader;
#[doc = "Field `GE4` writer - Analog I/O group 4 enable"]
pub type GE4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GE5` reader - Analog I/O group 5 enable"]
pub type GE5_R = crate::BitReader;
#[doc = "Field `GE5` writer - Analog I/O group 5 enable"]
pub type GE5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GC0` reader - Analog I/O group 0 status"]
pub type GC0_R = crate::BitReader;
#[doc = "Field `GC1` reader - Analog I/O group 1 status"]
pub type GC1_R = crate::BitReader;
#[doc = "Field `GC2` reader - Analog I/O group 2 status"]
pub type GC2_R = crate::BitReader;
#[doc = "Field `GC3` reader - Analog I/O group 3 status"]
pub type GC3_R = crate::BitReader;
#[doc = "Field `GC4` reader - Analog I/O group 4 status"]
pub type GC4_R = crate::BitReader;
#[doc = "Field `GC5` reader - Analog I/O group 5 status"]
pub type GC5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog I/O group 0 enable"]
    #[inline(always)]
    pub fn ge0(&self) -> GE0_R {
        GE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog I/O group 1 enable"]
    #[inline(always)]
    pub fn ge1(&self) -> GE1_R {
        GE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog I/O group 2 enable"]
    #[inline(always)]
    pub fn ge2(&self) -> GE2_R {
        GE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog I/O group 3 enable"]
    #[inline(always)]
    pub fn ge3(&self) -> GE3_R {
        GE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog I/O group 4 enable"]
    #[inline(always)]
    pub fn ge4(&self) -> GE4_R {
        GE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog I/O group 5 enable"]
    #[inline(always)]
    pub fn ge5(&self) -> GE5_R {
        GE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog I/O group 0 status"]
    #[inline(always)]
    pub fn gc0(&self) -> GC0_R {
        GC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog I/O group 1 status"]
    #[inline(always)]
    pub fn gc1(&self) -> GC1_R {
        GC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog I/O group 2 status"]
    #[inline(always)]
    pub fn gc2(&self) -> GC2_R {
        GC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog I/O group 3 status"]
    #[inline(always)]
    pub fn gc3(&self) -> GC3_R {
        GC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog I/O group 4 status"]
    #[inline(always)]
    pub fn gc4(&self) -> GC4_R {
        GC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog I/O group 5 status"]
    #[inline(always)]
    pub fn gc5(&self) -> GC5_R {
        GC5_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog I/O group 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge0(&mut self) -> GE0_W<GCTL_SPEC, 0> {
        GE0_W::new(self)
    }
    #[doc = "Bit 1 - Analog I/O group 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge1(&mut self) -> GE1_W<GCTL_SPEC, 1> {
        GE1_W::new(self)
    }
    #[doc = "Bit 2 - Analog I/O group 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge2(&mut self) -> GE2_W<GCTL_SPEC, 2> {
        GE2_W::new(self)
    }
    #[doc = "Bit 3 - Analog I/O group 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge3(&mut self) -> GE3_W<GCTL_SPEC, 3> {
        GE3_W::new(self)
    }
    #[doc = "Bit 4 - Analog I/O group 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge4(&mut self) -> GE4_W<GCTL_SPEC, 4> {
        GE4_W::new(self)
    }
    #[doc = "Bit 5 - Analog I/O group 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge5(&mut self) -> GE5_W<GCTL_SPEC, 5> {
        GE5_W::new(self)
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
#[doc = "I/O group control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCTL_SPEC;
impl crate::RegisterSpec for GCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gctl::R`](R) reader structure"]
impl crate::Readable for GCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gctl::W`](W) writer structure"]
impl crate::Writable for GCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCTL to value 0"]
impl crate::Resettable for GCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
