#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Field `FLT0IFC` writer - Fault 0 interrupt flag"]
pub type FLT0IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT1IF` writer - Fault 1 interrupt flag"]
pub type FLT1IF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT2IFC` writer - Clear fault 2 interrupt flag"]
pub type FLT2IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT3IFC` writer - Clear fault 3 interrupt flag"]
pub type FLT3IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT4IFC` writer - Clear fault 4 interrupt flag"]
pub type FLT4IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSFLTIFC` writer - Clear system fault interrupt flag"]
pub type SYSFLTIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLLCALIF` writer - Clear DLL calibration completed interrupt flag"]
pub type DLLCALIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMPERIFC` writer - Clear bunch mode period interrupt flag"]
pub type BMPERIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Fault 0 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt0ifc(&mut self) -> FLT0IFC_W<INTC_SPEC, 0> {
        FLT0IFC_W::new(self)
    }
    #[doc = "Bit 1 - Fault 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt1if(&mut self) -> FLT1IF_W<INTC_SPEC, 1> {
        FLT1IF_W::new(self)
    }
    #[doc = "Bit 2 - Clear fault 2 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt2ifc(&mut self) -> FLT2IFC_W<INTC_SPEC, 2> {
        FLT2IFC_W::new(self)
    }
    #[doc = "Bit 3 - Clear fault 3 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt3ifc(&mut self) -> FLT3IFC_W<INTC_SPEC, 3> {
        FLT3IFC_W::new(self)
    }
    #[doc = "Bit 4 - Clear fault 4 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt4ifc(&mut self) -> FLT4IFC_W<INTC_SPEC, 4> {
        FLT4IFC_W::new(self)
    }
    #[doc = "Bit 5 - Clear system fault interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn sysfltifc(&mut self) -> SYSFLTIFC_W<INTC_SPEC, 5> {
        SYSFLTIFC_W::new(self)
    }
    #[doc = "Bit 16 - Clear DLL calibration completed interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dllcalif(&mut self) -> DLLCALIF_W<INTC_SPEC, 16> {
        DLLCALIF_W::new(self)
    }
    #[doc = "Bit 17 - Clear bunch mode period interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn bmperifc(&mut self) -> BMPERIFC_W<INTC_SPEC, 17> {
        BMPERIFC_W::new(self)
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
#[doc = "SHRTIMER interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
