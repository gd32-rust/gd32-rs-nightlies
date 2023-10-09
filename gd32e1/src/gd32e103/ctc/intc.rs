#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Field `CKOKIC` writer - CKOKIF interrupt clear bit"]
pub type CKOKIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKWARNIC` writer - CKWARNIF interrupt clear bit"]
pub type CKWARNIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIC` writer - ERRIF interrupt clear bit"]
pub type ERRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EREFIC` writer - EREFIF interrupt clear bit"]
pub type EREFIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - CKOKIF interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ckokic(&mut self) -> CKOKIC_W<INTC_SPEC, 0> {
        CKOKIC_W::new(self)
    }
    #[doc = "Bit 1 - CKWARNIF interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ckwarnic(&mut self) -> CKWARNIC_W<INTC_SPEC, 1> {
        CKWARNIC_W::new(self)
    }
    #[doc = "Bit 2 - ERRIF interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn erric(&mut self) -> ERRIC_W<INTC_SPEC, 2> {
        ERRIC_W::new(self)
    }
    #[doc = "Bit 3 - EREFIF interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn erefic(&mut self) -> EREFIC_W<INTC_SPEC, 3> {
        EREFIC_W::new(self)
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
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
