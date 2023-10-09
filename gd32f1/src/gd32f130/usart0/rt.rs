#[doc = "Register `RT` reader"]
pub type R = crate::R<RT_SPEC>;
#[doc = "Register `RT` writer"]
pub type W = crate::W<RT_SPEC>;
#[doc = "Field `RT` reader - Receiver timeout value"]
pub type RT_R = crate::FieldReader<u32>;
#[doc = "Field `RT` writer - Receiver timeout value"]
pub type RT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 24, O, u32>;
#[doc = "Field `BL` reader - Block Length"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - Block Length"]
pub type BL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    pub fn rt(&self) -> RT_R {
        RT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn rt(&mut self) -> RT_W<RT_SPEC, 0> {
        RT_W::new(self)
    }
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<RT_SPEC, 24> {
        BL_W::new(self)
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
#[doc = "Receiver timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RT_SPEC;
impl crate::RegisterSpec for RT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rt::R`](R) reader structure"]
impl crate::Readable for RT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rt::W`](W) writer structure"]
impl crate::Writable for RT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RT to value 0"]
impl crate::Resettable for RT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
