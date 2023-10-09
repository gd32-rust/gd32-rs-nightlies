#[doc = "Register `ST0ACTL` reader"]
pub type R = crate::R<ST0ACTL_SPEC>;
#[doc = "Register `ST0ACTL` writer"]
pub type W = crate::W<ST0ACTL_SPEC>;
#[doc = "Field `CNTCKDIV_3` reader - Counter clock division"]
pub type CNTCKDIV_3_R = crate::BitReader;
#[doc = "Field `CNTCKDIV_3` writer - Counter clock division"]
pub type CNTCKDIV_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTRCFG_15_9` reader - Rising edge dead-time value configure"]
pub type DTRCFG_15_9_R = crate::FieldReader;
#[doc = "Field `DTRCFG_15_9` writer - Rising edge dead-time value configure"]
pub type DTRCFG_15_9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DTFCFG_15_9` reader - Falling edge dead-time value configure"]
pub type DTFCFG_15_9_R = crate::FieldReader;
#[doc = "Field `DTFCFG_15_9` writer - Falling edge dead-time value configure"]
pub type DTFCFG_15_9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bit 3 - Counter clock division"]
    #[inline(always)]
    pub fn cntckdiv_3(&self) -> CNTCKDIV_3_R {
        CNTCKDIV_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Rising edge dead-time value configure"]
    #[inline(always)]
    pub fn dtrcfg_15_9(&self) -> DTRCFG_15_9_R {
        DTRCFG_15_9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - Falling edge dead-time value configure"]
    #[inline(always)]
    pub fn dtfcfg_15_9(&self) -> DTFCFG_15_9_R {
        DTFCFG_15_9_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Counter clock division"]
    #[inline(always)]
    #[must_use]
    pub fn cntckdiv_3(&mut self) -> CNTCKDIV_3_W<ST0ACTL_SPEC, 3> {
        CNTCKDIV_3_W::new(self)
    }
    #[doc = "Bits 9:15 - Rising edge dead-time value configure"]
    #[inline(always)]
    #[must_use]
    pub fn dtrcfg_15_9(&mut self) -> DTRCFG_15_9_W<ST0ACTL_SPEC, 9> {
        DTRCFG_15_9_W::new(self)
    }
    #[doc = "Bits 25:31 - Falling edge dead-time value configure"]
    #[inline(always)]
    #[must_use]
    pub fn dtfcfg_15_9(&mut self) -> DTFCFG_15_9_W<ST0ACTL_SPEC, 25> {
        DTFCFG_15_9_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx additional control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0actl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0actl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST0ACTL_SPEC;
impl crate::RegisterSpec for ST0ACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0actl::R`](R) reader structure"]
impl crate::Readable for ST0ACTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st0actl::W`](W) writer structure"]
impl crate::Writable for ST0ACTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST0ACTL to value 0"]
impl crate::Resettable for ST0ACTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
