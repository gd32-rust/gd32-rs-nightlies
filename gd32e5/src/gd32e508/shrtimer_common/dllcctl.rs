#[doc = "Register `DLLCCTL` reader"]
pub type R = crate::R<DLLCCTL_SPEC>;
#[doc = "Register `DLLCCTL` writer"]
pub type W = crate::W<DLLCCTL_SPEC>;
#[doc = "Field `CLBSTRT` reader - DLL calibration start once"]
pub type CLBSTRT_R = crate::BitReader;
#[doc = "Field `CLBSTRT` writer - DLL calibration start once"]
pub type CLBSTRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLBPEREN` reader - DLL periodic calibration enable"]
pub type CLBPEREN_R = crate::BitReader;
#[doc = "Field `CLBPEREN` writer - DLL periodic calibration enable"]
pub type CLBPEREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLBPER` reader - DLL calibration period"]
pub type CLBPER_R = crate::FieldReader;
#[doc = "Field `CLBPER` writer - DLL calibration period"]
pub type CLBPER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - DLL calibration start once"]
    #[inline(always)]
    pub fn clbstrt(&self) -> CLBSTRT_R {
        CLBSTRT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLL periodic calibration enable"]
    #[inline(always)]
    pub fn clbperen(&self) -> CLBPEREN_R {
        CLBPEREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DLL calibration period"]
    #[inline(always)]
    pub fn clbper(&self) -> CLBPER_R {
        CLBPER_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DLL calibration start once"]
    #[inline(always)]
    #[must_use]
    pub fn clbstrt(&mut self) -> CLBSTRT_W<DLLCCTL_SPEC, 0> {
        CLBSTRT_W::new(self)
    }
    #[doc = "Bit 1 - DLL periodic calibration enable"]
    #[inline(always)]
    #[must_use]
    pub fn clbperen(&mut self) -> CLBPEREN_W<DLLCCTL_SPEC, 1> {
        CLBPEREN_W::new(self)
    }
    #[doc = "Bits 2:3 - DLL calibration period"]
    #[inline(always)]
    #[must_use]
    pub fn clbper(&mut self) -> CLBPER_W<DLLCCTL_SPEC, 2> {
        CLBPER_W::new(self)
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
#[doc = "SHRTIMER DLL calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dllcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dllcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLLCCTL_SPEC;
impl crate::RegisterSpec for DLLCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dllcctl::R`](R) reader structure"]
impl crate::Readable for DLLCCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dllcctl::W`](W) writer structure"]
impl crate::Writable for DLLCCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLLCCTL to value 0"]
impl crate::Resettable for DLLCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
