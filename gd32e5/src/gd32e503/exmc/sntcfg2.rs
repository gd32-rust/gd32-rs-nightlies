#[doc = "Register `SNTCFG2` reader"]
pub type R = crate::R<SNTCFG2_SPEC>;
#[doc = "Register `SNTCFG2` writer"]
pub type W = crate::W<SNTCFG2_SPEC>;
#[doc = "Field `ASET` reader - Address setup time"]
pub type ASET_R = crate::FieldReader;
#[doc = "Field `ASET` writer - Address setup time"]
pub type ASET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AHLD` reader - Address hold time"]
pub type AHLD_R = crate::FieldReader;
#[doc = "Field `AHLD` writer - Address hold time"]
pub type AHLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DSET` reader - Data setup time"]
pub type DSET_R = crate::FieldReader;
#[doc = "Field `DSET` writer - Data setup time"]
pub type DSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BUSLAT` reader - Bus latency"]
pub type BUSLAT_R = crate::FieldReader;
#[doc = "Field `BUSLAT` writer - Bus latency"]
pub type BUSLAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CKDIV` reader - Synchronous clock divide ratio"]
pub type CKDIV_R = crate::FieldReader;
#[doc = "Field `CKDIV` writer - Synchronous clock divide ratio"]
pub type CKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DLAT` reader - Data latency for NOR Flash"]
pub type DLAT_R = crate::FieldReader;
#[doc = "Field `DLAT` writer - Data latency for NOR Flash"]
pub type DLAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ASYNCMOD` reader - Asynchronous access mode"]
pub type ASYNCMOD_R = crate::FieldReader;
#[doc = "Field `ASYNCMOD` writer - Asynchronous access mode"]
pub type ASYNCMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn aset(&self) -> ASET_R {
        ASET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn ahld(&self) -> AHLD_R {
        AHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn dset(&self) -> DSET_R {
        DSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&self) -> BUSLAT_R {
        BUSLAT_R::new(((self.bits >> 16) & 0x0f) as u8)
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
    pub fn asyncmod(&self) -> ASYNCMOD_R {
        ASYNCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn aset(&mut self) -> ASET_W<SNTCFG2_SPEC, 0> {
        ASET_W::new(self)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    #[must_use]
    pub fn ahld(&mut self) -> AHLD_W<SNTCFG2_SPEC, 4> {
        AHLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn dset(&mut self) -> DSET_W<SNTCFG2_SPEC, 8> {
        DSET_W::new(self)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    #[must_use]
    pub fn buslat(&mut self) -> BUSLAT_W<SNTCFG2_SPEC, 16> {
        BUSLAT_W::new(self)
    }
    #[doc = "Bits 20:23 - Synchronous clock divide ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv(&mut self) -> CKDIV_W<SNTCFG2_SPEC, 20> {
        CKDIV_W::new(self)
    }
    #[doc = "Bits 24:27 - Data latency for NOR Flash"]
    #[inline(always)]
    #[must_use]
    pub fn dlat(&mut self) -> DLAT_W<SNTCFG2_SPEC, 24> {
        DLAT_W::new(self)
    }
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    #[must_use]
    pub fn asyncmod(&mut self) -> ASYNCMOD_W<SNTCFG2_SPEC, 28> {
        ASYNCMOD_W::new(self)
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
#[doc = "SRAM/NOR flash timing configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sntcfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sntcfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SNTCFG2_SPEC;
impl crate::RegisterSpec for SNTCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sntcfg2::R`](R) reader structure"]
impl crate::Readable for SNTCFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sntcfg2::W`](W) writer structure"]
impl crate::Writable for SNTCFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNTCFG2 to value 0x0fff_ffff"]
impl crate::Resettable for SNTCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
