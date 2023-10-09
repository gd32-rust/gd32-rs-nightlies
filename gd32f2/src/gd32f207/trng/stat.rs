#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `DRDY` reader - Random data ready status bit"]
pub type DRDY_R = crate::BitReader;
#[doc = "Field `CECS` reader - Clock error current status"]
pub type CECS_R = crate::BitReader;
#[doc = "Field `SECS` reader - Seed error current status"]
pub type SECS_R = crate::BitReader;
#[doc = "Field `CEIF` reader - Clock error interrupt flag"]
pub type CEIF_R = crate::BitReader;
#[doc = "Field `CEIF` writer - Clock error interrupt flag"]
pub type CEIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEIF` reader - Seed error interrupt flag"]
pub type SEIF_R = crate::BitReader;
#[doc = "Field `SEIF` writer - Seed error interrupt flag"]
pub type SEIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Random data ready status bit"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock error current status"]
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed error current status"]
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt flag"]
    #[inline(always)]
    pub fn ceif(&self) -> CEIF_R {
        CEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed error interrupt flag"]
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ceif(&mut self) -> CEIF_W<STAT_SPEC, 5> {
        CEIF_W::new(self)
    }
    #[doc = "Bit 6 - Seed error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn seif(&mut self) -> SEIF_W<STAT_SPEC, 6> {
        SEIF_W::new(self)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
