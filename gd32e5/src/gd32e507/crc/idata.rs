#[doc = "Register `IDATA` reader"]
pub type R = crate::R<IDATA_SPEC>;
#[doc = "Register `IDATA` writer"]
pub type W = crate::W<IDATA_SPEC>;
#[doc = "Field `IDATA` reader - Configurable initial CRC data value"]
pub type IDATA_R = crate::FieldReader<u32>;
#[doc = "Field `IDATA` writer - Configurable initial CRC data value"]
pub type IDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Configurable initial CRC data value"]
    #[inline(always)]
    pub fn idata(&self) -> IDATA_R {
        IDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configurable initial CRC data value"]
    #[inline(always)]
    #[must_use]
    pub fn idata(&mut self) -> IDATA_W<IDATA_SPEC, 0> {
        IDATA_W::new(self)
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
#[doc = "Initialization data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDATA_SPEC;
impl crate::RegisterSpec for IDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idata::R`](R) reader structure"]
impl crate::Readable for IDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idata::W`](W) writer structure"]
impl crate::Writable for IDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDATA to value 0xffff_ffff"]
impl crate::Resettable for IDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
