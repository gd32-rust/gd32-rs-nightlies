#[doc = "Register `BGC` reader"]
pub type R = crate::R<BGC_SPEC>;
#[doc = "Register `BGC` writer"]
pub type W = crate::W<BGC_SPEC>;
#[doc = "Field `BVB` reader - Background value blue"]
pub type BVB_R = crate::FieldReader;
#[doc = "Field `BVB` writer - Background value blue"]
pub type BVB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BVG` reader - Background value green"]
pub type BVG_R = crate::FieldReader;
#[doc = "Field `BVG` writer - Background value green"]
pub type BVG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BVR` reader - Background value red"]
pub type BVR_R = crate::FieldReader;
#[doc = "Field `BVR` writer - Background value red"]
pub type BVR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Background value blue"]
    #[inline(always)]
    pub fn bvb(&self) -> BVB_R {
        BVB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Background value green"]
    #[inline(always)]
    pub fn bvg(&self) -> BVG_R {
        BVG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Background value red"]
    #[inline(always)]
    pub fn bvr(&self) -> BVR_R {
        BVR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Background value blue"]
    #[inline(always)]
    #[must_use]
    pub fn bvb(&mut self) -> BVB_W<BGC_SPEC, 0> {
        BVB_W::new(self)
    }
    #[doc = "Bits 8:15 - Background value green"]
    #[inline(always)]
    #[must_use]
    pub fn bvg(&mut self) -> BVG_W<BGC_SPEC, 8> {
        BVG_W::new(self)
    }
    #[doc = "Bits 16:23 - Background value red"]
    #[inline(always)]
    #[must_use]
    pub fn bvr(&mut self) -> BVR_W<BGC_SPEC, 16> {
        BVR_W::new(self)
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
#[doc = "Background color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BGC_SPEC;
impl crate::RegisterSpec for BGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgc::R`](R) reader structure"]
impl crate::Readable for BGC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bgc::W`](W) writer structure"]
impl crate::Writable for BGC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BGC to value 0"]
impl crate::Resettable for BGC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
