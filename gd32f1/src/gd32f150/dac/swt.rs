#[doc = "Register `SWT` writer"]
pub struct W(crate::W<SWT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWT_SPEC>;
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
impl From<crate::W<SWT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC0 software trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTR0_AW {
    #[doc = "0: DAC channel X software trigger disabled"]
    DISABLED = 0,
    #[doc = "1: DAC channel X software trigger enabled"]
    ENABLED = 1,
}
impl From<SWTR0_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTR0` writer - DAC0 software trigger"]
pub type SWTR0_W<'a> = crate::BitWriter<'a, u32, SWT_SPEC, SWTR0_AW, 0>;
impl<'a> SWTR0_W<'a> {
    #[doc = "DAC channel X software trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWTR0_AW::DISABLED)
    }
    #[doc = "DAC channel X software trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWTR0_AW::ENABLED)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 software trigger"]
    #[inline(always)]
    pub fn swtr0(&mut self) -> SWTR0_W {
        SWTR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "software trigger register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swt](index.html) module"]
pub struct SWT_SPEC;
impl crate::RegisterSpec for SWT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swt::W](W) writer structure"]
impl crate::Writable for SWT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWT to value 0"]
impl crate::Resettable for SWT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
