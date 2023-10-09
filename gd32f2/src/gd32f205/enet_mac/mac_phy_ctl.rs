#[doc = "Register `MAC_PHY_CTL` reader"]
pub type R = crate::R<MAC_PHY_CTL_SPEC>;
#[doc = "Register `MAC_PHY_CTL` writer"]
pub type W = crate::W<MAC_PHY_CTL_SPEC>;
#[doc = "Field `PB` reader - PHY busy"]
pub type PB_R = crate::BitReader;
#[doc = "Field `PB` writer - PHY busy"]
pub type PB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PW` reader - PHY write"]
pub type PW_R = crate::BitReader;
#[doc = "Field `PW` writer - PHY write"]
pub type PW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR` reader - Clock range"]
pub type CLR_R = crate::FieldReader;
#[doc = "Field `CLR` writer - Clock range"]
pub type CLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PR` reader - PHY register"]
pub type PR_R = crate::FieldReader;
#[doc = "Field `PR` writer - PHY register"]
pub type PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PA` reader - PHY address"]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - PHY address"]
pub type PA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bit 0 - PHY busy"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PHY write"]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:10 - PHY register"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PHY busy"]
    #[inline(always)]
    #[must_use]
    pub fn pb(&mut self) -> PB_W<MAC_PHY_CTL_SPEC, 0> {
        PB_W::new(self)
    }
    #[doc = "Bit 1 - PHY write"]
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PW_W<MAC_PHY_CTL_SPEC, 1> {
        PW_W::new(self)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<MAC_PHY_CTL_SPEC, 2> {
        CLR_W::new(self)
    }
    #[doc = "Bits 6:10 - PHY register"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<MAC_PHY_CTL_SPEC, 6> {
        PR_W::new(self)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<MAC_PHY_CTL_SPEC, 11> {
        PA_W::new(self)
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
#[doc = "Ethernet MAC PHY control register (MAC_PHY_CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_phy_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_phy_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_PHY_CTL_SPEC;
impl crate::RegisterSpec for MAC_PHY_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_phy_ctl::R`](R) reader structure"]
impl crate::Readable for MAC_PHY_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_phy_ctl::W`](W) writer structure"]
impl crate::Writable for MAC_PHY_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_PHY_CTL to value 0"]
impl crate::Resettable for MAC_PHY_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
