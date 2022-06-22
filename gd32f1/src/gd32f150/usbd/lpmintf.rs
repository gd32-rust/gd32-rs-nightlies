#[doc = "Register `LPMINTF` reader"]
pub struct R(crate::R<LPMINTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMINTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMINTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMINTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMINTF` writer"]
pub struct W(crate::W<LPMINTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMINTF_SPEC>;
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
impl From<crate::W<LPMINTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMINTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMSTIF` reader - LPM token Correct transfer interrupt flag"]
pub type LPMSTIF_R = crate::BitReader<bool>;
#[doc = "Field `LPMSTIF` writer - LPM token Correct transfer interrupt flag"]
pub type LPMSTIF_W<'a> = crate::BitWriter<'a, u16, LPMINTF_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 15 - LPM token Correct transfer interrupt flag"]
    #[inline(always)]
    pub fn lpmstif(&self) -> LPMSTIF_R {
        LPMSTIF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - LPM token Correct transfer interrupt flag"]
    #[inline(always)]
    pub fn lpmstif(&mut self) -> LPMSTIF_W {
        LPMSTIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB LPM interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmintf](index.html) module"]
pub struct LPMINTF_SPEC;
impl crate::RegisterSpec for LPMINTF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lpmintf::R](R) reader structure"]
impl crate::Readable for LPMINTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmintf::W](W) writer structure"]
impl crate::Writable for LPMINTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMINTF to value 0"]
impl crate::Resettable for LPMINTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
