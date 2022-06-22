#[doc = "Register `ACFG` reader"]
pub struct R(crate::R<ACFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACFG` writer"]
pub struct W(crate::W<ACFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACFG_SPEC>;
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
impl From<crate::W<ACFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FD_CH5EN` reader - Enable bit for channel 5 Full_Data transfer mode"]
pub type FD_CH5EN_R = crate::BitReader<bool>;
#[doc = "Field `FD_CH5EN` writer - Enable bit for channel 5 Full_Data transfer mode"]
pub type FD_CH5EN_W<'a> = crate::BitWriter<'a, u32, ACFG_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 5 - Enable bit for channel 5 Full_Data transfer mode"]
    #[inline(always)]
    pub fn fd_ch5en(&self) -> FD_CH5EN_R {
        FD_CH5EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Enable bit for channel 5 Full_Data transfer mode"]
    #[inline(always)]
    pub fn fd_ch5en(&mut self) -> FD_CH5EN_W {
        FD_CH5EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA additional configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acfg](index.html) module"]
pub struct ACFG_SPEC;
impl crate::RegisterSpec for ACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acfg::R](R) reader structure"]
impl crate::Readable for ACFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acfg::W](W) writer structure"]
impl crate::Writable for ACFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACFG to value 0"]
impl crate::Resettable for ACFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
