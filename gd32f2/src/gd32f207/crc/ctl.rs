#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_AW {
    #[doc = "1: Resets the DATA register to IDATA, with no effect on FDATA"]
    RESET = 1,
}
impl From<RST_AW> for bool {
    #[inline(always)]
    fn from(variant: RST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` writer - Reset bit"]
pub type RST_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, RST_AW, 0>;
impl<'a> RST_W<'a> {
    #[doc = "Resets the DATA register to IDATA, with no effect on FDATA"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RST_AW::RESET)
    }
}
impl W {
    #[doc = "Bit 0 - Reset bit"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
