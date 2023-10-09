#[doc = "Register `SHIFTCTL` writer"]
pub type W = crate::W<SHIFTCTL_SPEC>;
#[doc = "Field `SFS` writer - Subtract a fraction of a second"]
pub type SFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `A1S` writer - One second add"]
pub type A1S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:14 - Subtract a fraction of a second"]
    #[inline(always)]
    #[must_use]
    pub fn sfs(&mut self) -> SFS_W<SHIFTCTL_SPEC, 0> {
        SFS_W::new(self)
    }
    #[doc = "Bit 31 - One second add"]
    #[inline(always)]
    #[must_use]
    pub fn a1s(&mut self) -> A1S_W<SHIFTCTL_SPEC, 31> {
        A1S_W::new(self)
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
#[doc = "Shift function control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHIFTCTL_SPEC;
impl crate::RegisterSpec for SHIFTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`shiftctl::W`](W) writer structure"]
impl crate::Writable for SHIFTCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTCTL to value 0"]
impl crate::Resettable for SHIFTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
