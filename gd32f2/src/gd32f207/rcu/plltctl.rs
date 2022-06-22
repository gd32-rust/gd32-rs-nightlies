#[doc = "Register `PLLTCTL` reader"]
pub struct R(crate::R<PLLTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLTCTL` writer"]
pub struct W(crate::W<PLLTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLTCTL_SPEC>;
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
impl From<crate::W<PLLTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLTEN` reader - PLLT enable"]
pub type PLLTEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLTEN` writer - PLLT enable"]
pub type PLLTEN_W<'a> = crate::BitWriter<'a, u32, PLLTCTL_SPEC, bool, 28>;
#[doc = "Field `PLLTSTB` reader - PLLTclock stabilization flag"]
pub type PLLTSTB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 28 - PLLT enable"]
    #[inline(always)]
    pub fn pllten(&self) -> PLLTEN_R {
        PLLTEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLLTclock stabilization flag"]
    #[inline(always)]
    pub fn plltstb(&self) -> PLLTSTB_R {
        PLLTSTB_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - PLLT enable"]
    #[inline(always)]
    pub fn pllten(&mut self) -> PLLTEN_W {
        PLLTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLT control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plltctl](index.html) module"]
pub struct PLLTCTL_SPEC;
impl crate::RegisterSpec for PLLTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plltctl::R](R) reader structure"]
impl crate::Readable for PLLTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plltctl::W](W) writer structure"]
impl crate::Writable for PLLTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLTCTL to value 0"]
impl crate::Resettable for PLLTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
