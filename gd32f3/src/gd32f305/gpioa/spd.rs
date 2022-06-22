#[doc = "Register `SPD` reader"]
pub struct R(crate::R<SPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPD` writer"]
pub struct W(crate::W<SPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPD_SPEC>;
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
impl From<crate::W<SPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPD15` reader - Port 15 output max speed bits"]
pub type SPD15_R = crate::BitReader<bool>;
#[doc = "Field `SPD15` writer - Port 15 output max speed bits"]
pub type SPD15_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 15>;
#[doc = "Field `SPD14` reader - Port 14 output max speed bits"]
pub type SPD14_R = crate::BitReader<bool>;
#[doc = "Field `SPD14` writer - Port 14 output max speed bits"]
pub type SPD14_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 14>;
#[doc = "Field `SPD13` reader - Port 13 output max speed bits"]
pub type SPD13_R = crate::BitReader<bool>;
#[doc = "Field `SPD13` writer - Port 13 output max speed bits"]
pub type SPD13_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 13>;
#[doc = "Field `SPD12` reader - Port 12 output max speed bits"]
pub type SPD12_R = crate::BitReader<bool>;
#[doc = "Field `SPD12` writer - Port 12 output max speed bits"]
pub type SPD12_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 12>;
#[doc = "Field `SPD11` reader - Port 11 output max speed bits"]
pub type SPD11_R = crate::BitReader<bool>;
#[doc = "Field `SPD11` writer - Port 11 output max speed bits"]
pub type SPD11_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 11>;
#[doc = "Field `SPD10` reader - Port 10 output max speed bits"]
pub type SPD10_R = crate::BitReader<bool>;
#[doc = "Field `SPD10` writer - Port 10 output max speed bits"]
pub type SPD10_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 10>;
#[doc = "Field `SPD9` reader - Port 9 output max speed bits"]
pub type SPD9_R = crate::BitReader<bool>;
#[doc = "Field `SPD9` writer - Port 9 output max speed bits"]
pub type SPD9_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 9>;
#[doc = "Field `SPD8` reader - Port 8 output max speed bits"]
pub type SPD8_R = crate::BitReader<bool>;
#[doc = "Field `SPD8` writer - Port 8 output max speed bits"]
pub type SPD8_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 8>;
#[doc = "Field `SPD7` reader - Port 7 output max speed bits"]
pub type SPD7_R = crate::BitReader<bool>;
#[doc = "Field `SPD7` writer - Port 7 output max speed bits"]
pub type SPD7_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 7>;
#[doc = "Field `SPD6` reader - Port 6 output max speed bits"]
pub type SPD6_R = crate::BitReader<bool>;
#[doc = "Field `SPD6` writer - Port 6 output max speed bits"]
pub type SPD6_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 6>;
#[doc = "Field `SPD5` reader - Port 5 output max speed bits"]
pub type SPD5_R = crate::BitReader<bool>;
#[doc = "Field `SPD5` writer - Port 5 output max speed bits"]
pub type SPD5_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 5>;
#[doc = "Field `SPD4` reader - Port 4 output max speed bits"]
pub type SPD4_R = crate::BitReader<bool>;
#[doc = "Field `SPD4` writer - Port 4 output max speed bits"]
pub type SPD4_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 4>;
#[doc = "Field `SPD3` reader - Port 3 output max speed bits"]
pub type SPD3_R = crate::BitReader<bool>;
#[doc = "Field `SPD3` writer - Port 3 output max speed bits"]
pub type SPD3_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 3>;
#[doc = "Field `SPD2` reader - Port 2 output max speed bits"]
pub type SPD2_R = crate::BitReader<bool>;
#[doc = "Field `SPD2` writer - Port 2 output max speed bits"]
pub type SPD2_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 2>;
#[doc = "Field `SPD1` reader - Port 1 output max speed bits"]
pub type SPD1_R = crate::BitReader<bool>;
#[doc = "Field `SPD1` writer - Port 1 output max speed bits"]
pub type SPD1_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 1>;
#[doc = "Field `SPD0` reader - Port 0 output max speed bits"]
pub type SPD0_R = crate::BitReader<bool>;
#[doc = "Field `SPD0` writer - Port 0 output max speed bits"]
pub type SPD0_W<'a> = crate::BitWriter<'a, u32, SPD_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 15 - Port 15 output max speed bits"]
    #[inline(always)]
    pub fn spd15(&self) -> SPD15_R {
        SPD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Port 14 output max speed bits"]
    #[inline(always)]
    pub fn spd14(&self) -> SPD14_R {
        SPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Port 13 output max speed bits"]
    #[inline(always)]
    pub fn spd13(&self) -> SPD13_R {
        SPD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Port 12 output max speed bits"]
    #[inline(always)]
    pub fn spd12(&self) -> SPD12_R {
        SPD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Port 11 output max speed bits"]
    #[inline(always)]
    pub fn spd11(&self) -> SPD11_R {
        SPD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 10 output max speed bits"]
    #[inline(always)]
    pub fn spd10(&self) -> SPD10_R {
        SPD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 9 output max speed bits"]
    #[inline(always)]
    pub fn spd9(&self) -> SPD9_R {
        SPD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 8 output max speed bits"]
    #[inline(always)]
    pub fn spd8(&self) -> SPD8_R {
        SPD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Port 7 output max speed bits"]
    #[inline(always)]
    pub fn spd7(&self) -> SPD7_R {
        SPD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Port 6 output max speed bits"]
    #[inline(always)]
    pub fn spd6(&self) -> SPD6_R {
        SPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Port 5 output max speed bits"]
    #[inline(always)]
    pub fn spd5(&self) -> SPD5_R {
        SPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 4 output max speed bits"]
    #[inline(always)]
    pub fn spd4(&self) -> SPD4_R {
        SPD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 3 output max speed bits"]
    #[inline(always)]
    pub fn spd3(&self) -> SPD3_R {
        SPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 2 output max speed bits"]
    #[inline(always)]
    pub fn spd2(&self) -> SPD2_R {
        SPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Port 1 output max speed bits"]
    #[inline(always)]
    pub fn spd1(&self) -> SPD1_R {
        SPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port 0 output max speed bits"]
    #[inline(always)]
    pub fn spd0(&self) -> SPD0_R {
        SPD0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port 15 output max speed bits"]
    #[inline(always)]
    pub fn spd15(&mut self) -> SPD15_W {
        SPD15_W::new(self)
    }
    #[doc = "Bit 14 - Port 14 output max speed bits"]
    #[inline(always)]
    pub fn spd14(&mut self) -> SPD14_W {
        SPD14_W::new(self)
    }
    #[doc = "Bit 13 - Port 13 output max speed bits"]
    #[inline(always)]
    pub fn spd13(&mut self) -> SPD13_W {
        SPD13_W::new(self)
    }
    #[doc = "Bit 12 - Port 12 output max speed bits"]
    #[inline(always)]
    pub fn spd12(&mut self) -> SPD12_W {
        SPD12_W::new(self)
    }
    #[doc = "Bit 11 - Port 11 output max speed bits"]
    #[inline(always)]
    pub fn spd11(&mut self) -> SPD11_W {
        SPD11_W::new(self)
    }
    #[doc = "Bit 10 - Port 10 output max speed bits"]
    #[inline(always)]
    pub fn spd10(&mut self) -> SPD10_W {
        SPD10_W::new(self)
    }
    #[doc = "Bit 9 - Port 9 output max speed bits"]
    #[inline(always)]
    pub fn spd9(&mut self) -> SPD9_W {
        SPD9_W::new(self)
    }
    #[doc = "Bit 8 - Port 8 output max speed bits"]
    #[inline(always)]
    pub fn spd8(&mut self) -> SPD8_W {
        SPD8_W::new(self)
    }
    #[doc = "Bit 7 - Port 7 output max speed bits"]
    #[inline(always)]
    pub fn spd7(&mut self) -> SPD7_W {
        SPD7_W::new(self)
    }
    #[doc = "Bit 6 - Port 6 output max speed bits"]
    #[inline(always)]
    pub fn spd6(&mut self) -> SPD6_W {
        SPD6_W::new(self)
    }
    #[doc = "Bit 5 - Port 5 output max speed bits"]
    #[inline(always)]
    pub fn spd5(&mut self) -> SPD5_W {
        SPD5_W::new(self)
    }
    #[doc = "Bit 4 - Port 4 output max speed bits"]
    #[inline(always)]
    pub fn spd4(&mut self) -> SPD4_W {
        SPD4_W::new(self)
    }
    #[doc = "Bit 3 - Port 3 output max speed bits"]
    #[inline(always)]
    pub fn spd3(&mut self) -> SPD3_W {
        SPD3_W::new(self)
    }
    #[doc = "Bit 2 - Port 2 output max speed bits"]
    #[inline(always)]
    pub fn spd2(&mut self) -> SPD2_W {
        SPD2_W::new(self)
    }
    #[doc = "Bit 1 - Port 1 output max speed bits"]
    #[inline(always)]
    pub fn spd1(&mut self) -> SPD1_W {
        SPD1_W::new(self)
    }
    #[doc = "Bit 0 - Port 0 output max speed bits"]
    #[inline(always)]
    pub fn spd0(&mut self) -> SPD0_W {
        SPD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port bit speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spd](index.html) module"]
pub struct SPD_SPEC;
impl crate::RegisterSpec for SPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spd::R](R) reader structure"]
impl crate::Readable for SPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spd::W](W) writer structure"]
impl crate::Writable for SPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPD to value 0"]
impl crate::Resettable for SPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
