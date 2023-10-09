#[doc = "Register `ST3FLTCTL` reader"]
pub type R = crate::R<ST3FLTCTL_SPEC>;
#[doc = "Register `ST3FLTCTL` writer"]
pub type W = crate::W<ST3FLTCTL_SPEC>;
#[doc = "Field `FLT0EN` reader - Fault 0 enable"]
pub type FLT0EN_R = crate::BitReader;
#[doc = "Field `FLT0EN` writer - Fault 0 enable"]
pub type FLT0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT1EN` reader - Fault 1 enable"]
pub type FLT1EN_R = crate::BitReader;
#[doc = "Field `FLT1EN` writer - Fault 1 enable"]
pub type FLT1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT2EN` reader - Fault 2 enable"]
pub type FLT2EN_R = crate::BitReader;
#[doc = "Field `FLT2EN` writer - Fault 2 enable"]
pub type FLT2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT3EN` reader - Fault 3 enable"]
pub type FLT3EN_R = crate::BitReader;
#[doc = "Field `FLT3EN` writer - Fault 3 enable"]
pub type FLT3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT4EN` reader - Fault 4 enable"]
pub type FLT4EN_R = crate::BitReader;
#[doc = "Field `FLT4EN` writer - Fault 4 enable"]
pub type FLT4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLTENPROT` reader - Protect fault enable"]
pub type FLTENPROT_R = crate::BitReader;
#[doc = "Field `FLTENPROT` writer - Protect fault enable"]
pub type FLTENPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Fault 0 enable"]
    #[inline(always)]
    pub fn flt0en(&self) -> FLT0EN_R {
        FLT0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Protect fault enable"]
    #[inline(always)]
    pub fn fltenprot(&self) -> FLTENPROT_R {
        FLTENPROT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt0en(&mut self) -> FLT0EN_W<ST3FLTCTL_SPEC, 0> {
        FLT0EN_W::new(self)
    }
    #[doc = "Bit 1 - Fault 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1en(&mut self) -> FLT1EN_W<ST3FLTCTL_SPEC, 1> {
        FLT1EN_W::new(self)
    }
    #[doc = "Bit 2 - Fault 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2en(&mut self) -> FLT2EN_W<ST3FLTCTL_SPEC, 2> {
        FLT2EN_W::new(self)
    }
    #[doc = "Bit 3 - Fault 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3en(&mut self) -> FLT3EN_W<ST3FLTCTL_SPEC, 3> {
        FLT3EN_W::new(self)
    }
    #[doc = "Bit 4 - Fault 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4en(&mut self) -> FLT4EN_W<ST3FLTCTL_SPEC, 4> {
        FLT4EN_W::new(self)
    }
    #[doc = "Bit 31 - Protect fault enable"]
    #[inline(always)]
    #[must_use]
    pub fn fltenprot(&mut self) -> FLTENPROT_W<ST3FLTCTL_SPEC, 31> {
        FLTENPROT_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx fault control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3fltctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3fltctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST3FLTCTL_SPEC;
impl crate::RegisterSpec for ST3FLTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st3fltctl::R`](R) reader structure"]
impl crate::Readable for ST3FLTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st3fltctl::W`](W) writer structure"]
impl crate::Writable for ST3FLTCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST3FLTCTL to value 0"]
impl crate::Resettable for ST3FLTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
