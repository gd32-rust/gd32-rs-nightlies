#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `VDE` reader - Current VDE status"]
pub type VDE_R = crate::BitReader;
#[doc = "Field `HDE` reader - Current HDE status"]
pub type HDE_R = crate::BitReader;
#[doc = "Field `VS` reader - Current VS staus of the TLI"]
pub type VS_R = crate::BitReader;
#[doc = "Field `HS` reader - Current HS staus of the TLI"]
pub type HS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Current VDE status"]
    #[inline(always)]
    pub fn vde(&self) -> VDE_R {
        VDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current HDE status"]
    #[inline(always)]
    pub fn hde(&self) -> HDE_R {
        HDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Current VS staus of the TLI"]
    #[inline(always)]
    pub fn vs(&self) -> VS_R {
        VS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Current HS staus of the TLI"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0x0f"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
