#[doc = "Register `SHIFTCTL` writer"]
pub struct W(crate::W<SHIFTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SHIFTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A1S` writer - One second add"]
pub type A1S_W<'a> = crate::BitWriter<'a, u32, SHIFTCTL_SPEC, bool, 31>;
#[doc = "Field `SFS` writer - Subtract a fraction of a second"]
pub type SFS_W<'a> = crate::FieldWriter<'a, u32, SHIFTCTL_SPEC, u16, u16, 15, 0>;
impl W {
    #[doc = "Bit 31 - One second add"]
    #[inline(always)]
    pub fn a1s(&mut self) -> A1S_W {
        A1S_W::new(self)
    }
    #[doc = "Bits 0:14 - Subtract a fraction of a second"]
    #[inline(always)]
    pub fn sfs(&mut self) -> SFS_W {
        SFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "shift control register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftctl](index.html) module"]
pub struct SHIFTCTL_SPEC;
impl crate::RegisterSpec for SHIFTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [shiftctl::W](W) writer structure"]
impl crate::Writable for SHIFTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTCTL to value 0"]
impl crate::Resettable for SHIFTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
