#[doc = "Register `PCF1` reader"]
pub type R = crate::R<PCF1_SPEC>;
#[doc = "Register `PCF1` writer"]
pub type W = crate::W<PCF1_SPEC>;
#[doc = "Field `TIMER8_REMAP` reader - TIMER8 remapping"]
pub type TIMER8_REMAP_R = crate::BitReader;
#[doc = "Field `TIMER8_REMAP` writer - TIMER8 remapping"]
pub type TIMER8_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER9_REMAP` reader - TIMER9 remapping"]
pub type TIMER9_REMAP_R = crate::BitReader;
#[doc = "Field `TIMER9_REMAP` writer - TIMER9 remapping"]
pub type TIMER9_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER10_REMAP` reader - TIMER10 remapping"]
pub type TIMER10_REMAP_R = crate::BitReader;
#[doc = "Field `TIMER10_REMAP` writer - TIMER10 remapping"]
pub type TIMER10_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER12_REMAP` reader - TIMER12 remapping"]
pub type TIMER12_REMAP_R = crate::BitReader;
#[doc = "Field `TIMER12_REMAP` writer - TIMER12 remapping"]
pub type TIMER12_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER13_REMAP` reader - TIMER13 remapping"]
pub type TIMER13_REMAP_R = crate::BitReader;
#[doc = "Field `TIMER13_REMAP` writer - TIMER13 remapping"]
pub type TIMER13_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXMC_NADV` reader - EXMC_NADV connect/disconnect"]
pub type EXMC_NADV_R = crate::BitReader;
#[doc = "Field `EXMC_NADV` writer - EXMC_NADV connect/disconnect"]
pub type EXMC_NADV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 5 - TIMER8 remapping"]
    #[inline(always)]
    pub fn timer8_remap(&self) -> TIMER8_REMAP_R {
        TIMER8_REMAP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIMER9 remapping"]
    #[inline(always)]
    pub fn timer9_remap(&self) -> TIMER9_REMAP_R {
        TIMER9_REMAP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIMER10 remapping"]
    #[inline(always)]
    pub fn timer10_remap(&self) -> TIMER10_REMAP_R {
        TIMER10_REMAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER12 remapping"]
    #[inline(always)]
    pub fn timer12_remap(&self) -> TIMER12_REMAP_R {
        TIMER12_REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIMER13 remapping"]
    #[inline(always)]
    pub fn timer13_remap(&self) -> TIMER13_REMAP_R {
        TIMER13_REMAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    pub fn exmc_nadv(&self) -> EXMC_NADV_R {
        EXMC_NADV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - TIMER8 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer8_remap(&mut self) -> TIMER8_REMAP_W<PCF1_SPEC, 5> {
        TIMER8_REMAP_W::new(self)
    }
    #[doc = "Bit 6 - TIMER9 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer9_remap(&mut self) -> TIMER9_REMAP_W<PCF1_SPEC, 6> {
        TIMER9_REMAP_W::new(self)
    }
    #[doc = "Bit 7 - TIMER10 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer10_remap(&mut self) -> TIMER10_REMAP_W<PCF1_SPEC, 7> {
        TIMER10_REMAP_W::new(self)
    }
    #[doc = "Bit 8 - TIMER12 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer12_remap(&mut self) -> TIMER12_REMAP_W<PCF1_SPEC, 8> {
        TIMER12_REMAP_W::new(self)
    }
    #[doc = "Bit 9 - TIMER13 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer13_remap(&mut self) -> TIMER13_REMAP_W<PCF1_SPEC, 9> {
        TIMER13_REMAP_W::new(self)
    }
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_nadv(&mut self) -> EXMC_NADV_W<PCF1_SPEC, 10> {
        EXMC_NADV_W::new(self)
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
#[doc = "AFIO port configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCF1_SPEC;
impl crate::RegisterSpec for PCF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcf1::R`](R) reader structure"]
impl crate::Readable for PCF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcf1::W`](W) writer structure"]
impl crate::Writable for PCF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCF1 to value 0"]
impl crate::Resettable for PCF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
