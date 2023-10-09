#[doc = "Register `DATALEN` reader"]
pub type R = crate::R<DATALEN_SPEC>;
#[doc = "Register `DATALEN` writer"]
pub type W = crate::W<DATALEN_SPEC>;
#[doc = "Field `DATALEN` reader - Data transfer length"]
pub type DATALEN_R = crate::FieldReader<u32>;
#[doc = "Field `DATALEN` writer - Data transfer length"]
pub type DATALEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 25, O, u32>;
impl R {
    #[doc = "Bits 0:24 - Data transfer length"]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Data transfer length"]
    #[inline(always)]
    #[must_use]
    pub fn datalen(&mut self) -> DATALEN_W<DATALEN_SPEC, 0> {
        DATALEN_W::new(self)
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
#[doc = "Data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datalen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datalen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATALEN_SPEC;
impl crate::RegisterSpec for DATALEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datalen::R`](R) reader structure"]
impl crate::Readable for DATALEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datalen::W`](W) writer structure"]
impl crate::Writable for DATALEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATALEN to value 0"]
impl crate::Resettable for DATALEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
