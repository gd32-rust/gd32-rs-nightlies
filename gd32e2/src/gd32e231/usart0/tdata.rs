#[doc = "Register `TDATA` reader"]
pub type R = crate::R<TDATA_SPEC>;
#[doc = "Register `TDATA` writer"]
pub type W = crate::W<TDATA_SPEC>;
#[doc = "Field `TDATA` reader - Transmit data value"]
pub type TDATA_R = crate::FieldReader<u16>;
#[doc = "Field `TDATA` writer - Transmit data value"]
pub type TDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - Transmit data value"]
    #[inline(always)]
    pub fn tdata(&self) -> TDATA_R {
        TDATA_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit data value"]
    #[inline(always)]
    #[must_use]
    pub fn tdata(&mut self) -> TDATA_W<TDATA_SPEC, 0> {
        TDATA_W::new(self)
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
#[doc = "Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDATA_SPEC;
impl crate::RegisterSpec for TDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdata::R`](R) reader structure"]
impl crate::Readable for TDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tdata::W`](W) writer structure"]
impl crate::Writable for TDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDATA to value 0"]
impl crate::Resettable for TDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
