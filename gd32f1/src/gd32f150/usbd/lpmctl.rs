#[doc = "Register `LPMCTL` reader"]
pub struct R(crate::R<LPMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMCTL` writer"]
pub struct W(crate::W<LPMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMCTL_SPEC>;
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
impl From<crate::W<LPMCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMSTIE` reader - LPM token successful transfer interrupt enable"]
pub type LPMSTIE_R = crate::BitReader<bool>;
#[doc = "Field `LPMSTIE` writer - LPM token successful transfer interrupt enable"]
pub type LPMSTIE_W<'a> = crate::BitWriter<'a, u16, LPMCTL_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 15 - LPM token successful transfer interrupt enable"]
    #[inline(always)]
    pub fn lpmstie(&self) -> LPMSTIE_R {
        LPMSTIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - LPM token successful transfer interrupt enable"]
    #[inline(always)]
    pub fn lpmstie(&mut self) -> LPMSTIE_W {
        LPMSTIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB LPM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmctl](index.html) module"]
pub struct LPMCTL_SPEC;
impl crate::RegisterSpec for LPMCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lpmctl::R](R) reader structure"]
impl crate::Readable for LPMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmctl::W](W) writer structure"]
impl crate::Writable for LPMCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMCTL to value 0"]
impl crate::Resettable for LPMCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
