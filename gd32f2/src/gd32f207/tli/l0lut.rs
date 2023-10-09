#[doc = "Register `L0LUT` writer"]
pub type W = crate::W<L0LUT_SPEC>;
#[doc = "Field `TB` writer - Blue channel of a LUT entry"]
pub type TB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TG` writer - Green channel of a LUT entry"]
pub type TG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TR` writer - Red Channel of a LUT entry"]
pub type TR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TADD` writer - Look Up Table Write Address"]
pub type TADD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Blue channel of a LUT entry"]
    #[inline(always)]
    #[must_use]
    pub fn tb(&mut self) -> TB_W<L0LUT_SPEC, 0> {
        TB_W::new(self)
    }
    #[doc = "Bits 8:15 - Green channel of a LUT entry"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<L0LUT_SPEC, 8> {
        TG_W::new(self)
    }
    #[doc = "Bits 16:23 - Red Channel of a LUT entry"]
    #[inline(always)]
    #[must_use]
    pub fn tr(&mut self) -> TR_W<L0LUT_SPEC, 16> {
        TR_W::new(self)
    }
    #[doc = "Bits 24:31 - Look Up Table Write Address"]
    #[inline(always)]
    #[must_use]
    pub fn tadd(&mut self) -> TADD_W<L0LUT_SPEC, 24> {
        TADD_W::new(self)
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
#[doc = "Layer 0 look up table register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0lut::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0LUT_SPEC;
impl crate::RegisterSpec for L0LUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l0lut::W`](W) writer structure"]
impl crate::Writable for L0LUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L0LUT to value 0"]
impl crate::Resettable for L0LUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
