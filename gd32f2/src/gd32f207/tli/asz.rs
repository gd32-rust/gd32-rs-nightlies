#[doc = "Register `ASZ` reader"]
pub type R = crate::R<ASZ_SPEC>;
#[doc = "Register `ASZ` writer"]
pub type W = crate::W<ASZ_SPEC>;
#[doc = "Field `VASZ` reader - Size of the vertical active area width plus back porch and synchronous pulse"]
pub type VASZ_R = crate::FieldReader<u16>;
#[doc = "Field `VASZ` writer - Size of the vertical active area width plus back porch and synchronous pulse"]
pub type VASZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `HASZ` reader - Size of the horizontal active area width plus back porch and synchronous pulse"]
pub type HASZ_R = crate::FieldReader<u16>;
#[doc = "Field `HASZ` writer - Size of the horizontal active area width plus back porch and synchronous pulse"]
pub type HASZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Size of the vertical active area width plus back porch and synchronous pulse"]
    #[inline(always)]
    pub fn vasz(&self) -> VASZ_R {
        VASZ_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Size of the horizontal active area width plus back porch and synchronous pulse"]
    #[inline(always)]
    pub fn hasz(&self) -> HASZ_R {
        HASZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Size of the vertical active area width plus back porch and synchronous pulse"]
    #[inline(always)]
    #[must_use]
    pub fn vasz(&mut self) -> VASZ_W<ASZ_SPEC, 0> {
        VASZ_W::new(self)
    }
    #[doc = "Bits 16:27 - Size of the horizontal active area width plus back porch and synchronous pulse"]
    #[inline(always)]
    #[must_use]
    pub fn hasz(&mut self) -> HASZ_W<ASZ_SPEC, 16> {
        HASZ_W::new(self)
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
#[doc = "Active size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASZ_SPEC;
impl crate::RegisterSpec for ASZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asz::R`](R) reader structure"]
impl crate::Readable for ASZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asz::W`](W) writer structure"]
impl crate::Writable for ASZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASZ to value 0"]
impl crate::Resettable for ASZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
