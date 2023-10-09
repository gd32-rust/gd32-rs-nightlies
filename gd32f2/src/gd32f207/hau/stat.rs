#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `DINT` reader - Data input interrupt status flag"]
pub type DINT_R = crate::BitReader;
#[doc = "Field `DINT` writer - Data input interrupt status flag"]
pub type DINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CINT` reader - Digest calculation completion interrupt flag"]
pub type CINT_R = crate::BitReader;
#[doc = "Field `CINT` writer - Digest calculation completion interrupt flag"]
pub type CINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAS` reader - DMA status flag"]
pub type DMAS_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag bit"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data input interrupt status flag"]
    #[inline(always)]
    pub fn dint(&self) -> DINT_R {
        DINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt flag"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA status flag"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy flag bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt status flag"]
    #[inline(always)]
    #[must_use]
    pub fn dint(&mut self) -> DINT_W<STAT_SPEC, 0> {
        DINT_W::new(self)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cint(&mut self) -> CINT_W<STAT_SPEC, 1> {
        CINT_W::new(self)
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
#[doc = "HAU status and interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets STAT to value 0x01"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
