#[doc = "Register `SMCFG` reader"]
pub type R = crate::R<SMCFG_SPEC>;
#[doc = "Register `SMCFG` writer"]
pub type W = crate::W<SMCFG_SPEC>;
#[doc = "Field `SMC` reader - Slave mode control"]
pub type SMC_R = crate::FieldReader;
#[doc = "Field `SMC` writer - Slave mode control"]
pub type SMC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TRGS` reader - Trigger selection"]
pub type TRGS_R = crate::FieldReader;
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TRGS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MSM` reader - Master-slave mode"]
pub type MSM_R = crate::BitReader;
#[doc = "Field `MSM` writer - Master-slave mode"]
pub type MSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Slave mode control"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&self) -> TRGS_R {
        TRGS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master-slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode control"]
    #[inline(always)]
    #[must_use]
    pub fn smc(&mut self) -> SMC_W<SMCFG_SPEC, 0> {
        SMC_W::new(self)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgs(&mut self) -> TRGS_W<SMCFG_SPEC, 4> {
        TRGS_W::new(self)
    }
    #[doc = "Bit 7 - Master-slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<SMCFG_SPEC, 7> {
        MSM_W::new(self)
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
#[doc = "slave mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCFG_SPEC;
impl crate::RegisterSpec for SMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcfg::R`](R) reader structure"]
impl crate::Readable for SMCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smcfg::W`](W) writer structure"]
impl crate::Writable for SMCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCFG to value 0"]
impl crate::Resettable for SMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
