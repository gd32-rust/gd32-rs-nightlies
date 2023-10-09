#[doc = "Register `SNWTCFG3` reader"]
pub type R = crate::R<SNWTCFG3_SPEC>;
#[doc = "Register `SNWTCFG3` writer"]
pub type W = crate::W<SNWTCFG3_SPEC>;
#[doc = "Field `WASET` reader - Address setup time"]
pub type WASET_R = crate::FieldReader;
#[doc = "Field `WASET` writer - Address setup time"]
pub type WASET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `WAHLD` reader - Address hold time"]
pub type WAHLD_R = crate::FieldReader;
#[doc = "Field `WAHLD` writer - Address hold time"]
pub type WAHLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `WDSET` reader - Data setup time"]
pub type WDSET_R = crate::FieldReader;
#[doc = "Field `WDSET` writer - Data setup time"]
pub type WDSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CKDIV` reader - Synchronous clock divide ratio"]
pub type CKDIV_R = crate::FieldReader;
#[doc = "Field `CKDIV` writer - Synchronous clock divide ratio"]
pub type CKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DLAT` reader - Data latency for NOR Flash"]
pub type DLAT_R = crate::FieldReader;
#[doc = "Field `DLAT` writer - Data latency for NOR Flash"]
pub type DLAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `WASYNCMOD` reader - Asynchronous access mode"]
pub type WASYNCMOD_R = crate::FieldReader;
#[doc = "Field `WASYNCMOD` writer - Asynchronous access mode"]
pub type WASYNCMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn waset(&self) -> WASET_R {
        WASET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn wahld(&self) -> WAHLD_R {
        WAHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn wdset(&self) -> WDSET_R {
        WDSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - Synchronous clock divide ratio"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Data latency for NOR Flash"]
    #[inline(always)]
    pub fn dlat(&self) -> DLAT_R {
        DLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn wasyncmod(&self) -> WASYNCMOD_R {
        WASYNCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn waset(&mut self) -> WASET_W<SNWTCFG3_SPEC, 0> {
        WASET_W::new(self)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    #[must_use]
    pub fn wahld(&mut self) -> WAHLD_W<SNWTCFG3_SPEC, 4> {
        WAHLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn wdset(&mut self) -> WDSET_W<SNWTCFG3_SPEC, 8> {
        WDSET_W::new(self)
    }
    #[doc = "Bits 20:23 - Synchronous clock divide ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv(&mut self) -> CKDIV_W<SNWTCFG3_SPEC, 20> {
        CKDIV_W::new(self)
    }
    #[doc = "Bits 24:27 - Data latency for NOR Flash"]
    #[inline(always)]
    #[must_use]
    pub fn dlat(&mut self) -> DLAT_W<SNWTCFG3_SPEC, 24> {
        DLAT_W::new(self)
    }
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    #[must_use]
    pub fn wasyncmod(&mut self) -> WASYNCMOD_W<SNWTCFG3_SPEC, 28> {
        WASYNCMOD_W::new(self)
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
#[doc = "SRAM/NOR flash write timing configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snwtcfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snwtcfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SNWTCFG3_SPEC;
impl crate::RegisterSpec for SNWTCFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snwtcfg3::R`](R) reader structure"]
impl crate::Readable for SNWTCFG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`snwtcfg3::W`](W) writer structure"]
impl crate::Writable for SNWTCFG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNWTCFG3 to value 0x0fff_ffff"]
impl crate::Resettable for SNWTCFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
