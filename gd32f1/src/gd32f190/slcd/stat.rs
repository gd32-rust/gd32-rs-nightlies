#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `ONF` reader - SLCD controller on flag"]
pub type ONF_R = crate::BitReader;
#[doc = "Field `SOF` reader - Start of frame flag"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `UPRF` reader - Update SLCD data request flag"]
pub type UPRF_R = crate::BitReader;
#[doc = "Field `UPRF` writer - Update SLCD data request flag"]
pub type UPRF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPDF` reader - Update SLCD data done flag"]
pub type UPDF_R = crate::BitReader;
#[doc = "Field `VRDYF` reader - SLCD voltage ready flag"]
pub type VRDYF_R = crate::BitReader;
#[doc = "Field `SYNF` reader - SLCD_CFG register synchronization flag"]
pub type SYNF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SLCD controller on flag"]
    #[inline(always)]
    pub fn onf(&self) -> ONF_R {
        ONF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame flag"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update SLCD data request flag"]
    #[inline(always)]
    pub fn uprf(&self) -> UPRF_R {
        UPRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Update SLCD data done flag"]
    #[inline(always)]
    pub fn updf(&self) -> UPDF_R {
        UPDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLCD voltage ready flag"]
    #[inline(always)]
    pub fn vrdyf(&self) -> VRDYF_R {
        VRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SLCD_CFG register synchronization flag"]
    #[inline(always)]
    pub fn synf(&self) -> SYNF_R {
        SYNF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Update SLCD data request flag"]
    #[inline(always)]
    #[must_use]
    pub fn uprf(&mut self) -> UPRF_W<STAT_SPEC, 2> {
        UPRF_W::new(self)
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
#[doc = "SLCD status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0x20"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
