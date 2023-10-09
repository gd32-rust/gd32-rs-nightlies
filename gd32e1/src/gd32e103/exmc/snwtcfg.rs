#[doc = "Register `SNWTCFG` reader"]
pub type R = crate::R<SNWTCFG_SPEC>;
#[doc = "Register `SNWTCFG` writer"]
pub type W = crate::W<SNWTCFG_SPEC>;
#[doc = "Field `WASET` reader - address setup time"]
pub type WASET_R = crate::FieldReader;
#[doc = "Field `WASET` writer - address setup time"]
pub type WASET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `WAHLD` reader - address hold time"]
pub type WAHLD_R = crate::FieldReader;
#[doc = "Field `WAHLD` writer - address hold time"]
pub type WAHLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `WDSET` reader - data setup time"]
pub type WDSET_R = crate::FieldReader;
#[doc = "Field `WDSET` writer - data setup time"]
pub type WDSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WBUSLAT` reader - bus latency"]
pub type WBUSLAT_R = crate::FieldReader;
#[doc = "Field `WBUSLAT` writer - bus latency"]
pub type WBUSLAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `WASYNCMOD` reader - asynchronous access mode"]
pub type WASYNCMOD_R = crate::FieldReader;
#[doc = "Field `WASYNCMOD` writer - asynchronous access mode"]
pub type WASYNCMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:3 - address setup time"]
    #[inline(always)]
    pub fn waset(&self) -> WASET_R {
        WASET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - address hold time"]
    #[inline(always)]
    pub fn wahld(&self) -> WAHLD_R {
        WAHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - data setup time"]
    #[inline(always)]
    pub fn wdset(&self) -> WDSET_R {
        WDSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - bus latency"]
    #[inline(always)]
    pub fn wbuslat(&self) -> WBUSLAT_R {
        WBUSLAT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - asynchronous access mode"]
    #[inline(always)]
    pub fn wasyncmod(&self) -> WASYNCMOD_R {
        WASYNCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn waset(&mut self) -> WASET_W<SNWTCFG_SPEC, 0> {
        WASET_W::new(self)
    }
    #[doc = "Bits 4:7 - address hold time"]
    #[inline(always)]
    #[must_use]
    pub fn wahld(&mut self) -> WAHLD_W<SNWTCFG_SPEC, 4> {
        WAHLD_W::new(self)
    }
    #[doc = "Bits 8:15 - data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn wdset(&mut self) -> WDSET_W<SNWTCFG_SPEC, 8> {
        WDSET_W::new(self)
    }
    #[doc = "Bits 16:19 - bus latency"]
    #[inline(always)]
    #[must_use]
    pub fn wbuslat(&mut self) -> WBUSLAT_W<SNWTCFG_SPEC, 16> {
        WBUSLAT_W::new(self)
    }
    #[doc = "Bits 28:29 - asynchronous access mode"]
    #[inline(always)]
    #[must_use]
    pub fn wasyncmod(&mut self) -> WASYNCMOD_W<SNWTCFG_SPEC, 28> {
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
#[doc = "SRAM/NOR write timing configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snwtcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snwtcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SNWTCFG_SPEC;
impl crate::RegisterSpec for SNWTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snwtcfg::R`](R) reader structure"]
impl crate::Readable for SNWTCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`snwtcfg::W`](W) writer structure"]
impl crate::Writable for SNWTCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNWTCFG to value 0x0fff_ffff"]
impl crate::Resettable for SNWTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
