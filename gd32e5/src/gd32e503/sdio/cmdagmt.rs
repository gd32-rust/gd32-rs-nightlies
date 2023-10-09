#[doc = "Register `CMDAGMT` reader"]
pub type R = crate::R<CMDAGMT_SPEC>;
#[doc = "Register `CMDAGMT` writer"]
pub type W = crate::W<CMDAGMT_SPEC>;
#[doc = "Field `CMDAGMT` reader - SDIO card command argument"]
pub type CMDAGMT_R = crate::FieldReader<u32>;
#[doc = "Field `CMDAGMT` writer - SDIO card command argument"]
pub type CMDAGMT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - SDIO card command argument"]
    #[inline(always)]
    pub fn cmdagmt(&self) -> CMDAGMT_R {
        CMDAGMT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SDIO card command argument"]
    #[inline(always)]
    #[must_use]
    pub fn cmdagmt(&mut self) -> CMDAGMT_W<CMDAGMT_SPEC, 0> {
        CMDAGMT_W::new(self)
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
#[doc = "Command argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdagmt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdagmt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDAGMT_SPEC;
impl crate::RegisterSpec for CMDAGMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdagmt::R`](R) reader structure"]
impl crate::Readable for CMDAGMT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmdagmt::W`](W) writer structure"]
impl crate::Writable for CMDAGMT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDAGMT to value 0"]
impl crate::Resettable for CMDAGMT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
