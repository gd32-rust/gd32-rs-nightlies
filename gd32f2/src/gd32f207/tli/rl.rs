#[doc = "Register `RL` reader"]
pub struct R(crate::R<RL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RL` writer"]
pub struct W(crate::W<RL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RL_SPEC>;
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
impl From<crate::W<RL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBR` reader - Frame Blank Reload"]
pub type FBR_R = crate::BitReader<bool>;
#[doc = "Field `FBR` writer - Frame Blank Reload"]
pub type FBR_W<'a> = crate::BitWriter<'a, u32, RL_SPEC, bool, 1>;
#[doc = "Field `RQR` reader - Request Reload"]
pub type RQR_R = crate::BitReader<bool>;
#[doc = "Field `RQR` writer - Request Reload"]
pub type RQR_W<'a> = crate::BitWriter<'a, u32, RL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 1 - Frame Blank Reload"]
    #[inline(always)]
    pub fn fbr(&self) -> FBR_R {
        FBR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Request Reload"]
    #[inline(always)]
    pub fn rqr(&self) -> RQR_R {
        RQR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Frame Blank Reload"]
    #[inline(always)]
    pub fn fbr(&mut self) -> FBR_W {
        FBR_W::new(self)
    }
    #[doc = "Bit 0 - Request Reload"]
    #[inline(always)]
    pub fn rqr(&mut self) -> RQR_W {
        RQR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reload layer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rl](index.html) module"]
pub struct RL_SPEC;
impl crate::RegisterSpec for RL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rl::R](R) reader structure"]
impl crate::Readable for RL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rl::W](W) writer structure"]
impl crate::Writable for RL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RL to value 0"]
impl crate::Resettable for RL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
