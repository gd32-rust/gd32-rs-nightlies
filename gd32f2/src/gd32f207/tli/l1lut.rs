#[doc = "Register `L1LUT` writer"]
pub struct W(crate::W<L1LUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1LUT_SPEC>;
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
impl From<crate::W<L1LUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1LUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TADD` writer - Look Up Table Write Address"]
pub type TADD_W<'a> = crate::FieldWriter<'a, u32, L1LUT_SPEC, u8, u8, 8, 24>;
#[doc = "Field `TR` writer - Red channel of a LUT entry"]
pub type TR_W<'a> = crate::FieldWriter<'a, u32, L1LUT_SPEC, u8, u8, 8, 16>;
#[doc = "Field `TG` writer - Green channel of a LUT entry"]
pub type TG_W<'a> = crate::FieldWriter<'a, u32, L1LUT_SPEC, u8, u8, 8, 8>;
#[doc = "Field `TB` writer - Blue channel of a LUT entry"]
pub type TB_W<'a> = crate::FieldWriter<'a, u32, L1LUT_SPEC, u8, u8, 8, 0>;
impl W {
    #[doc = "Bits 24:31 - Look Up Table Write Address"]
    #[inline(always)]
    pub fn tadd(&mut self) -> TADD_W {
        TADD_W::new(self)
    }
    #[doc = "Bits 16:23 - Red channel of a LUT entry"]
    #[inline(always)]
    pub fn tr(&mut self) -> TR_W {
        TR_W::new(self)
    }
    #[doc = "Bits 8:15 - Green channel of a LUT entry"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W {
        TG_W::new(self)
    }
    #[doc = "Bits 0:7 - Blue channel of a LUT entry"]
    #[inline(always)]
    pub fn tb(&mut self) -> TB_W {
        TB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 1 look up table register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1lut](index.html) module"]
pub struct L1LUT_SPEC;
impl crate::RegisterSpec for L1LUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [l1lut::W](W) writer structure"]
impl crate::Writable for L1LUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L1LUT to value 0"]
impl crate::Resettable for L1LUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
