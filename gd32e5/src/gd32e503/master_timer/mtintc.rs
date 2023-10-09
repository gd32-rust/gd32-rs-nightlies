#[doc = "Register `MTINTC` writer"]
pub type W = crate::W<MTINTC_SPEC>;
#[doc = "Field `CMP0IFC` writer - Clear compare 0 interrupt flag"]
pub type CMP0IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1IFC` writer - Clear compare 1 interrupt flag"]
pub type CMP1IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP2IFC` writer - Clear compare 2 interrupt flag"]
pub type CMP2IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP3IFC` writer - Clear compare 3 interrupt flag"]
pub type CMP3IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REPIFC` writer - Clear repetition interrupt flag"]
pub type REPIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNIIFC` writer - Clear synchronization input interrupt flag"]
pub type SYNIIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPIFC` writer - Clear update interrupt flag"]
pub type UPIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Clear compare 0 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0ifc(&mut self) -> CMP0IFC_W<MTINTC_SPEC, 0> {
        CMP0IFC_W::new(self)
    }
    #[doc = "Bit 1 - Clear compare 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ifc(&mut self) -> CMP1IFC_W<MTINTC_SPEC, 1> {
        CMP1IFC_W::new(self)
    }
    #[doc = "Bit 2 - Clear compare 2 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ifc(&mut self) -> CMP2IFC_W<MTINTC_SPEC, 2> {
        CMP2IFC_W::new(self)
    }
    #[doc = "Bit 3 - Clear compare 3 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3ifc(&mut self) -> CMP3IFC_W<MTINTC_SPEC, 3> {
        CMP3IFC_W::new(self)
    }
    #[doc = "Bit 4 - Clear repetition interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn repifc(&mut self) -> REPIFC_W<MTINTC_SPEC, 4> {
        REPIFC_W::new(self)
    }
    #[doc = "Bit 5 - Clear synchronization input interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syniifc(&mut self) -> SYNIIFC_W<MTINTC_SPEC, 5> {
        SYNIIFC_W::new(self)
    }
    #[doc = "Bit 6 - Clear update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn upifc(&mut self) -> UPIFC_W<MTINTC_SPEC, 6> {
        UPIFC_W::new(self)
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
#[doc = "SHRTIMER Master_TIMER interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtintc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTINTC_SPEC;
impl crate::RegisterSpec for MTINTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mtintc::W`](W) writer structure"]
impl crate::Writable for MTINTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTINTC to value 0"]
impl crate::Resettable for MTINTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
