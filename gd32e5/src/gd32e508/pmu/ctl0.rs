#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `LDOLP` reader - LDO Low Power Mode"]
pub type LDOLP_R = crate::BitReader;
#[doc = "Field `LDOLP` writer - LDO Low Power Mode"]
pub type LDOLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STBMOD` reader - Standby Mode"]
pub type STBMOD_R = crate::BitReader;
#[doc = "Field `STBMOD` writer - Standby Mode"]
pub type STBMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WURST` reader - Wakeup Flag Reset"]
pub type WURST_R = crate::BitReader;
#[doc = "Field `WURST` writer - Wakeup Flag Reset"]
pub type WURST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STBRST` reader - Standby Flag Reset"]
pub type STBRST_R = crate::BitReader;
#[doc = "Field `STBRST` writer - Standby Flag Reset"]
pub type STBRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LVDEN` reader - Low Voltage Detector Enable"]
pub type LVDEN_R = crate::BitReader;
#[doc = "Field `LVDEN` writer - Low Voltage Detector Enable"]
pub type LVDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LVDT` reader - Low Voltage Detector Threshold"]
pub type LVDT_R = crate::FieldReader;
#[doc = "Field `LVDT` writer - Low Voltage Detector Threshold"]
pub type LVDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BKPWEN` reader - Backup Domain Write Enable"]
pub type BKPWEN_R = crate::BitReader;
#[doc = "Field `BKPWEN` writer - Backup Domain Write Enable"]
pub type BKPWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LDLP` reader - Low-driver mode when use low power LDO."]
pub type LDLP_R = crate::BitReader;
#[doc = "Field `LDLP` writer - Low-driver mode when use low power LDO."]
pub type LDLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LDNP` reader - Low-driver mode when use normal power LDO"]
pub type LDNP_R = crate::BitReader;
#[doc = "Field `LDNP` writer - Low-driver mode when use normal power LDO"]
pub type LDNP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDEN` reader - High-driver mode enable"]
pub type HDEN_R = crate::BitReader;
#[doc = "Field `HDEN` writer - High-driver mode enable"]
pub type HDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDS` reader - High-driver mode switch"]
pub type HDS_R = crate::BitReader;
#[doc = "Field `HDS` writer - High-driver mode switch"]
pub type HDS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LDEN` reader - Low-driver mode enable in Deep-sleep mode"]
pub type LDEN_R = crate::FieldReader;
#[doc = "Field `LDEN` writer - Low-driver mode enable in Deep-sleep mode"]
pub type LDEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    pub fn ldolp(&self) -> LDOLP_R {
        LDOLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    pub fn stbmod(&self) -> STBMOD_R {
        STBMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    pub fn wurst(&self) -> WURST_R {
        WURST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    pub fn stbrst(&self) -> STBRST_R {
        STBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    pub fn lvden(&self) -> LVDEN_R {
        LVDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    pub fn lvdt(&self) -> LVDT_R {
        LVDT_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    pub fn bkpwen(&self) -> BKPWEN_R {
        BKPWEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Low-driver mode when use low power LDO."]
    #[inline(always)]
    pub fn ldlp(&self) -> LDLP_R {
        LDLP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Low-driver mode when use normal power LDO"]
    #[inline(always)]
    pub fn ldnp(&self) -> LDNP_R {
        LDNP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - High-driver mode enable"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High-driver mode switch"]
    #[inline(always)]
    pub fn hds(&self) -> HDS_R {
        HDS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Low-driver mode enable in Deep-sleep mode"]
    #[inline(always)]
    pub fn lden(&self) -> LDEN_R {
        LDEN_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ldolp(&mut self) -> LDOLP_W<CTL0_SPEC, 0> {
        LDOLP_W::new(self)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stbmod(&mut self) -> STBMOD_W<CTL0_SPEC, 1> {
        STBMOD_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wurst(&mut self) -> WURST_W<CTL0_SPEC, 2> {
        WURST_W::new(self)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    #[must_use]
    pub fn stbrst(&mut self) -> STBRST_W<CTL0_SPEC, 3> {
        STBRST_W::new(self)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvden(&mut self) -> LVDEN_W<CTL0_SPEC, 4> {
        LVDEN_W::new(self)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lvdt(&mut self) -> LVDT_W<CTL0_SPEC, 5> {
        LVDT_W::new(self)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpwen(&mut self) -> BKPWEN_W<CTL0_SPEC, 8> {
        BKPWEN_W::new(self)
    }
    #[doc = "Bit 10 - Low-driver mode when use low power LDO."]
    #[inline(always)]
    #[must_use]
    pub fn ldlp(&mut self) -> LDLP_W<CTL0_SPEC, 10> {
        LDLP_W::new(self)
    }
    #[doc = "Bit 11 - Low-driver mode when use normal power LDO"]
    #[inline(always)]
    #[must_use]
    pub fn ldnp(&mut self) -> LDNP_W<CTL0_SPEC, 11> {
        LDNP_W::new(self)
    }
    #[doc = "Bit 16 - High-driver mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn hden(&mut self) -> HDEN_W<CTL0_SPEC, 16> {
        HDEN_W::new(self)
    }
    #[doc = "Bit 17 - High-driver mode switch"]
    #[inline(always)]
    #[must_use]
    pub fn hds(&mut self) -> HDS_W<CTL0_SPEC, 17> {
        HDS_W::new(self)
    }
    #[doc = "Bits 18:19 - Low-driver mode enable in Deep-sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lden(&mut self) -> LDEN_W<CTL0_SPEC, 18> {
        LDEN_W::new(self)
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
#[doc = "power control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0xc000"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000;
}
