#[doc = "Register `BMCMPV` reader"]
pub type R = crate::R<BMCMPV_SPEC>;
#[doc = "Register `BMCMPV` writer"]
pub type W = crate::W<BMCMPV_SPEC>;
#[doc = "Field `BMCMPVAL` reader - Bunch mode compare value"]
pub type BMCMPVAL_R = crate::FieldReader<u16>;
#[doc = "Field `BMCMPVAL` writer - Bunch mode compare value"]
pub type BMCMPVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Bunch mode compare value"]
    #[inline(always)]
    pub fn bmcmpval(&self) -> BMCMPVAL_R {
        BMCMPVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bunch mode compare value"]
    #[inline(always)]
    #[must_use]
    pub fn bmcmpval(&mut self) -> BMCMPVAL_W<BMCMPV_SPEC, 0> {
        BMCMPVAL_W::new(self)
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
#[doc = "SHRTIMER bunch mode compare value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcmpv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcmpv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMCMPV_SPEC;
impl crate::RegisterSpec for BMCMPV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmcmpv::R`](R) reader structure"]
impl crate::Readable for BMCMPV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmcmpv::W`](W) writer structure"]
impl crate::Writable for BMCMPV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMCMPV to value 0"]
impl crate::Resettable for BMCMPV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
