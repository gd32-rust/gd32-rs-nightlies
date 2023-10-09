#[doc = "Register `RL` reader"]
pub type R = crate::R<RL_SPEC>;
#[doc = "Register `RL` writer"]
pub type W = crate::W<RL_SPEC>;
#[doc = "Field `RQR` reader - Request Reload"]
pub type RQR_R = crate::BitReader;
#[doc = "Field `RQR` writer - Request Reload"]
pub type RQR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBR` reader - Frame Blank Reload"]
pub type FBR_R = crate::BitReader;
#[doc = "Field `FBR` writer - Frame Blank Reload"]
pub type FBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Request Reload"]
    #[inline(always)]
    pub fn rqr(&self) -> RQR_R {
        RQR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Blank Reload"]
    #[inline(always)]
    pub fn fbr(&self) -> FBR_R {
        FBR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request Reload"]
    #[inline(always)]
    #[must_use]
    pub fn rqr(&mut self) -> RQR_W<RL_SPEC, 0> {
        RQR_W::new(self)
    }
    #[doc = "Bit 1 - Frame Blank Reload"]
    #[inline(always)]
    #[must_use]
    pub fn fbr(&mut self) -> FBR_W<RL_SPEC, 1> {
        FBR_W::new(self)
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
#[doc = "Reload layer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RL_SPEC;
impl crate::RegisterSpec for RL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rl::R`](R) reader structure"]
impl crate::Readable for RL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rl::W`](W) writer structure"]
impl crate::Writable for RL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RL to value 0"]
impl crate::Resettable for RL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
