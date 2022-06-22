#[doc = "Register `PCF1` reader"]
pub struct R(crate::R<PCF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCF1` writer"]
pub struct W(crate::W<PCF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PCF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXMC_NADV` reader - EXMC_NADV connect/disconnect"]
pub type EXMC_NADV_R = crate::BitReader<bool>;
#[doc = "Field `EXMC_NADV` writer - EXMC_NADV connect/disconnect"]
pub type EXMC_NADV_W<'a> = crate::BitWriter<'a, u32, PCF1_SPEC, bool, 10>;
#[doc = "Field `TIMER13_REMAP` reader - TIMER13 remapping"]
pub type TIMER13_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIMER13_REMAP` writer - TIMER13 remapping"]
pub type TIMER13_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF1_SPEC, bool, 9>;
#[doc = "Field `TIMER12_REMAP` reader - TIMER12 remapping"]
pub type TIMER12_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIMER12_REMAP` writer - TIMER12 remapping"]
pub type TIMER12_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF1_SPEC, bool, 8>;
#[doc = "Field `TIMER10_REMAP` reader - TIMER10 remapping"]
pub type TIMER10_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIMER10_REMAP` writer - TIMER10 remapping"]
pub type TIMER10_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF1_SPEC, bool, 7>;
#[doc = "Field `TIMER9_REMAP` reader - TIMER9 remapping"]
pub type TIMER9_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIMER9_REMAP` writer - TIMER9 remapping"]
pub type TIMER9_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF1_SPEC, bool, 6>;
#[doc = "Field `TIMER8_REMAP` reader - TIMER8 remapping"]
pub type TIMER8_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIMER8_REMAP` writer - TIMER8 remapping"]
pub type TIMER8_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF1_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    pub fn exmc_nadv(&self) -> EXMC_NADV_R {
        EXMC_NADV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - TIMER13 remapping"]
    #[inline(always)]
    pub fn timer13_remap(&self) -> TIMER13_REMAP_R {
        TIMER13_REMAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER12 remapping"]
    #[inline(always)]
    pub fn timer12_remap(&self) -> TIMER12_REMAP_R {
        TIMER12_REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - TIMER10 remapping"]
    #[inline(always)]
    pub fn timer10_remap(&self) -> TIMER10_REMAP_R {
        TIMER10_REMAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - TIMER9 remapping"]
    #[inline(always)]
    pub fn timer9_remap(&self) -> TIMER9_REMAP_R {
        TIMER9_REMAP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - TIMER8 remapping"]
    #[inline(always)]
    pub fn timer8_remap(&self) -> TIMER8_REMAP_R {
        TIMER8_REMAP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    pub fn exmc_nadv(&mut self) -> EXMC_NADV_W {
        EXMC_NADV_W::new(self)
    }
    #[doc = "Bit 9 - TIMER13 remapping"]
    #[inline(always)]
    pub fn timer13_remap(&mut self) -> TIMER13_REMAP_W {
        TIMER13_REMAP_W::new(self)
    }
    #[doc = "Bit 8 - TIMER12 remapping"]
    #[inline(always)]
    pub fn timer12_remap(&mut self) -> TIMER12_REMAP_W {
        TIMER12_REMAP_W::new(self)
    }
    #[doc = "Bit 7 - TIMER10 remapping"]
    #[inline(always)]
    pub fn timer10_remap(&mut self) -> TIMER10_REMAP_W {
        TIMER10_REMAP_W::new(self)
    }
    #[doc = "Bit 6 - TIMER9 remapping"]
    #[inline(always)]
    pub fn timer9_remap(&mut self) -> TIMER9_REMAP_W {
        TIMER9_REMAP_W::new(self)
    }
    #[doc = "Bit 5 - TIMER8 remapping"]
    #[inline(always)]
    pub fn timer8_remap(&mut self) -> TIMER8_REMAP_W {
        TIMER8_REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO port configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcf1](index.html) module"]
pub struct PCF1_SPEC;
impl crate::RegisterSpec for PCF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcf1::R](R) reader structure"]
impl crate::Readable for PCF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcf1::W](W) writer structure"]
impl crate::Writable for PCF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCF1 to value 0"]
impl crate::Resettable for PCF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
