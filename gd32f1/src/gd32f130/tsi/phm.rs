#[doc = "Register `PHM` reader"]
pub struct R(crate::R<PHM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHM` writer"]
pub struct W(crate::W<PHM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHM_SPEC>;
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
impl From<crate::W<PHM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `G5P3` reader - G5P3 Schmitt trigger hysteresis mode"]
pub type G5P3_R = crate::BitReader<bool>;
#[doc = "Field `G5P3` writer - G5P3 Schmitt trigger hysteresis mode"]
pub type G5P3_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 23>;
#[doc = "Field `G5P2` reader - G5P2 Schmitt trigger hysteresis mode"]
pub type G5P2_R = crate::BitReader<bool>;
#[doc = "Field `G5P2` writer - G5P2 Schmitt trigger hysteresis mode"]
pub type G5P2_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 22>;
#[doc = "Field `G5P1` reader - G5P1 Schmitt trigger hysteresis mode"]
pub type G5P1_R = crate::BitReader<bool>;
#[doc = "Field `G5P1` writer - G5P1 Schmitt trigger hysteresis mode"]
pub type G5P1_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 21>;
#[doc = "Field `G5P0` reader - G5P0 Schmitt trigger hysteresis mode"]
pub type G5P0_R = crate::BitReader<bool>;
#[doc = "Field `G5P0` writer - G5P0 Schmitt trigger hysteresis mode"]
pub type G5P0_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 20>;
#[doc = "Field `G4P3` reader - G4P3 Schmitt trigger hysteresis mode"]
pub type G4P3_R = crate::BitReader<bool>;
#[doc = "Field `G4P3` writer - G4P3 Schmitt trigger hysteresis mode"]
pub type G4P3_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 19>;
#[doc = "Field `G4P2` reader - G4P2 Schmitt trigger hysteresis mode"]
pub type G4P2_R = crate::BitReader<bool>;
#[doc = "Field `G4P2` writer - G4P2 Schmitt trigger hysteresis mode"]
pub type G4P2_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 18>;
#[doc = "Field `G4P1` reader - G4P1 Schmitt trigger hysteresis mode"]
pub type G4P1_R = crate::BitReader<bool>;
#[doc = "Field `G4P1` writer - G4P1 Schmitt trigger hysteresis mode"]
pub type G4P1_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 17>;
#[doc = "Field `G4P0` reader - G4P0 Schmitt trigger hysteresis mode"]
pub type G4P0_R = crate::BitReader<bool>;
#[doc = "Field `G4P0` writer - G4P0 Schmitt trigger hysteresis mode"]
pub type G4P0_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 16>;
#[doc = "Field `G3P3` reader - G3P3 Schmitt trigger hysteresis mode"]
pub type G3P3_R = crate::BitReader<bool>;
#[doc = "Field `G3P3` writer - G3P3 Schmitt trigger hysteresis mode"]
pub type G3P3_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 15>;
#[doc = "Field `G3P2` reader - G3P2 Schmitt trigger hysteresis mode"]
pub type G3P2_R = crate::BitReader<bool>;
#[doc = "Field `G3P2` writer - G3P2 Schmitt trigger hysteresis mode"]
pub type G3P2_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 14>;
#[doc = "Field `G3P1` reader - G3P1 Schmitt trigger hysteresis mode"]
pub type G3P1_R = crate::BitReader<bool>;
#[doc = "Field `G3P1` writer - G3P1 Schmitt trigger hysteresis mode"]
pub type G3P1_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 13>;
#[doc = "Field `G3P0` reader - G3P0 Schmitt trigger hysteresis mode"]
pub type G3P0_R = crate::BitReader<bool>;
#[doc = "Field `G3P0` writer - G3P0 Schmitt trigger hysteresis mode"]
pub type G3P0_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 12>;
#[doc = "Field `G2P3` reader - G2P3 Schmitt trigger hysteresis mode"]
pub type G2P3_R = crate::BitReader<bool>;
#[doc = "Field `G2P3` writer - G2P3 Schmitt trigger hysteresis mode"]
pub type G2P3_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 11>;
#[doc = "Field `G2P2` reader - G2P2 Schmitt trigger hysteresis mode"]
pub type G2P2_R = crate::BitReader<bool>;
#[doc = "Field `G2P2` writer - G2P2 Schmitt trigger hysteresis mode"]
pub type G2P2_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 10>;
#[doc = "Field `G2P1` reader - G2P1 Schmitt trigger hysteresis mode"]
pub type G2P1_R = crate::BitReader<bool>;
#[doc = "Field `G2P1` writer - G2P1 Schmitt trigger hysteresis mode"]
pub type G2P1_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 9>;
#[doc = "Field `G2P0` reader - G2P0 Schmitt trigger hysteresis mode"]
pub type G2P0_R = crate::BitReader<bool>;
#[doc = "Field `G2P0` writer - G2P0 Schmitt trigger hysteresis mode"]
pub type G2P0_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 8>;
#[doc = "Field `G1P3` reader - G1P3 Schmitt trigger hysteresis mode"]
pub type G1P3_R = crate::BitReader<bool>;
#[doc = "Field `G1P3` writer - G1P3 Schmitt trigger hysteresis mode"]
pub type G1P3_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 7>;
#[doc = "Field `G1P2` reader - G1P2 Schmitt trigger hysteresis mode"]
pub type G1P2_R = crate::BitReader<bool>;
#[doc = "Field `G1P2` writer - G1P2 Schmitt trigger hysteresis mode"]
pub type G1P2_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 6>;
#[doc = "Field `G1P1` reader - G1P1 Schmitt trigger hysteresis mode"]
pub type G1P1_R = crate::BitReader<bool>;
#[doc = "Field `G1P1` writer - G1P1 Schmitt trigger hysteresis mode"]
pub type G1P1_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 5>;
#[doc = "Field `G1P0` reader - G1P0 Schmitt trigger hysteresis mode"]
pub type G1P0_R = crate::BitReader<bool>;
#[doc = "Field `G1P0` writer - G1P0 Schmitt trigger hysteresis mode"]
pub type G1P0_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 4>;
#[doc = "Field `G0P3` reader - G0P3 Schmitt trigger hysteresis mode"]
pub type G0P3_R = crate::BitReader<bool>;
#[doc = "Field `G0P3` writer - G0P3 Schmitt trigger hysteresis mode"]
pub type G0P3_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 3>;
#[doc = "Field `G0P2` reader - G0P2 Schmitt trigger hysteresis mode"]
pub type G0P2_R = crate::BitReader<bool>;
#[doc = "Field `G0P2` writer - G0P2 Schmitt trigger hysteresis mode"]
pub type G0P2_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 2>;
#[doc = "Field `G0P1` reader - G0P1 Schmitt trigger hysteresis mode"]
pub type G0P1_R = crate::BitReader<bool>;
#[doc = "Field `G0P1` writer - G0P1 Schmitt trigger hysteresis mode"]
pub type G0P1_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 1>;
#[doc = "Field `G0P0` reader - G0P0 Schmitt trigger hysteresis mode"]
pub type G0P0_R = crate::BitReader<bool>;
#[doc = "Field `G0P0` writer - G0P0 Schmitt trigger hysteresis mode"]
pub type G0P0_W<'a> = crate::BitWriter<'a, u32, PHM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 23 - G5P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g5p3(&self) -> G5P3_R {
        G5P3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - G5P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g5p2(&self) -> G5P2_R {
        G5P2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - G5P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g5p1(&self) -> G5P1_R {
        G5P1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - G5P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g5p0(&self) -> G5P0_R {
        G5P0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - G4P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g4p3(&self) -> G4P3_R {
        G4P3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - G4P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g4p2(&self) -> G4P2_R {
        G4P2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - G4P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g4p1(&self) -> G4P1_R {
        G4P1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - G4P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g4p0(&self) -> G4P0_R {
        G4P0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - G3P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g3p3(&self) -> G3P3_R {
        G3P3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - G3P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g3p2(&self) -> G3P2_R {
        G3P2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - G3P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g3p1(&self) -> G3P1_R {
        G3P1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - G3P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g3p0(&self) -> G3P0_R {
        G3P0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - G2P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g2p3(&self) -> G2P3_R {
        G2P3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - G2P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g2p2(&self) -> G2P2_R {
        G2P2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - G2P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g2p1(&self) -> G2P1_R {
        G2P1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - G2P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g2p0(&self) -> G2P0_R {
        G2P0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - G1P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g1p3(&self) -> G1P3_R {
        G1P3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - G1P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g1p2(&self) -> G1P2_R {
        G1P2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - G1P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g1p1(&self) -> G1P1_R {
        G1P1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - G1P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g1p0(&self) -> G1P0_R {
        G1P0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - G0P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g0p3(&self) -> G0P3_R {
        G0P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - G0P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g0p2(&self) -> G0P2_R {
        G0P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - G0P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g0p1(&self) -> G0P1_R {
        G0P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - G0P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g0p0(&self) -> G0P0_R {
        G0P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - G5P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g5p3(&mut self) -> G5P3_W {
        G5P3_W::new(self)
    }
    #[doc = "Bit 22 - G5P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g5p2(&mut self) -> G5P2_W {
        G5P2_W::new(self)
    }
    #[doc = "Bit 21 - G5P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g5p1(&mut self) -> G5P1_W {
        G5P1_W::new(self)
    }
    #[doc = "Bit 20 - G5P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g5p0(&mut self) -> G5P0_W {
        G5P0_W::new(self)
    }
    #[doc = "Bit 19 - G4P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g4p3(&mut self) -> G4P3_W {
        G4P3_W::new(self)
    }
    #[doc = "Bit 18 - G4P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g4p2(&mut self) -> G4P2_W {
        G4P2_W::new(self)
    }
    #[doc = "Bit 17 - G4P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g4p1(&mut self) -> G4P1_W {
        G4P1_W::new(self)
    }
    #[doc = "Bit 16 - G4P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g4p0(&mut self) -> G4P0_W {
        G4P0_W::new(self)
    }
    #[doc = "Bit 15 - G3P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g3p3(&mut self) -> G3P3_W {
        G3P3_W::new(self)
    }
    #[doc = "Bit 14 - G3P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g3p2(&mut self) -> G3P2_W {
        G3P2_W::new(self)
    }
    #[doc = "Bit 13 - G3P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g3p1(&mut self) -> G3P1_W {
        G3P1_W::new(self)
    }
    #[doc = "Bit 12 - G3P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g3p0(&mut self) -> G3P0_W {
        G3P0_W::new(self)
    }
    #[doc = "Bit 11 - G2P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g2p3(&mut self) -> G2P3_W {
        G2P3_W::new(self)
    }
    #[doc = "Bit 10 - G2P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g2p2(&mut self) -> G2P2_W {
        G2P2_W::new(self)
    }
    #[doc = "Bit 9 - G2P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g2p1(&mut self) -> G2P1_W {
        G2P1_W::new(self)
    }
    #[doc = "Bit 8 - G2P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g2p0(&mut self) -> G2P0_W {
        G2P0_W::new(self)
    }
    #[doc = "Bit 7 - G1P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g1p3(&mut self) -> G1P3_W {
        G1P3_W::new(self)
    }
    #[doc = "Bit 6 - G1P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g1p2(&mut self) -> G1P2_W {
        G1P2_W::new(self)
    }
    #[doc = "Bit 5 - G1P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g1p1(&mut self) -> G1P1_W {
        G1P1_W::new(self)
    }
    #[doc = "Bit 4 - G1P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g1p0(&mut self) -> G1P0_W {
        G1P0_W::new(self)
    }
    #[doc = "Bit 3 - G0P3 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g0p3(&mut self) -> G0P3_W {
        G0P3_W::new(self)
    }
    #[doc = "Bit 2 - G0P2 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g0p2(&mut self) -> G0P2_W {
        G0P2_W::new(self)
    }
    #[doc = "Bit 1 - G0P1 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g0p1(&mut self) -> G0P1_W {
        G0P1_W::new(self)
    }
    #[doc = "Bit 0 - G0P0 Schmitt trigger hysteresis mode"]
    #[inline(always)]
    pub fn g0p0(&mut self) -> G0P0_W {
        G0P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin hysteresis mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phm](index.html) module"]
pub struct PHM_SPEC;
impl crate::RegisterSpec for PHM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phm::R](R) reader structure"]
impl crate::Readable for PHM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phm::W](W) writer structure"]
impl crate::Writable for PHM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHM to value 0"]
impl crate::Resettable for PHM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
