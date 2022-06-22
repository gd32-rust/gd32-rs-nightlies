#[doc = "Register `MAC_PHY_CTL` reader"]
pub struct R(crate::R<MAC_PHY_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_PHY_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_PHY_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_PHY_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_PHY_CTL` writer"]
pub struct W(crate::W<MAC_PHY_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_PHY_CTL_SPEC>;
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
impl From<crate::W<MAC_PHY_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_PHY_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB` reader - PHY busy"]
pub type PB_R = crate::BitReader<bool>;
#[doc = "Field `PB` writer - PHY busy"]
pub type PB_W<'a> = crate::BitWriter<'a, u32, MAC_PHY_CTL_SPEC, bool, 0>;
#[doc = "Field `PW` reader - PHY write"]
pub type PW_R = crate::BitReader<bool>;
#[doc = "Field `PW` writer - PHY write"]
pub type PW_W<'a> = crate::BitWriter<'a, u32, MAC_PHY_CTL_SPEC, bool, 1>;
#[doc = "Field `CLR` reader - Clock range"]
pub type CLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLR` writer - Clock range"]
pub type CLR_W<'a> = crate::FieldWriter<'a, u32, MAC_PHY_CTL_SPEC, u8, u8, 3, 2>;
#[doc = "Field `PR` reader - PHY register"]
pub type PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PR` writer - PHY register"]
pub type PR_W<'a> = crate::FieldWriter<'a, u32, MAC_PHY_CTL_SPEC, u8, u8, 5, 6>;
#[doc = "Field `PA` reader - PHY address"]
pub type PA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA` writer - PHY address"]
pub type PA_W<'a> = crate::FieldWriter<'a, u32, MAC_PHY_CTL_SPEC, u8, u8, 5, 11>;
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
    pub fn pb(&mut self) -> PB_W {
        PB_W::new(self)
    }
    #[doc = "Bit 1 - PHY write"]
    #[inline(always)]
    pub fn pw(&mut self) -> PW_W {
        PW_W::new(self)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W::new(self)
    }
    #[doc = "Bits 6:10 - PHY register"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W::new(self)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC PHY control register (MAC_PHY_CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_phy_ctl](index.html) module"]
pub struct MAC_PHY_CTL_SPEC;
impl crate::RegisterSpec for MAC_PHY_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_phy_ctl::R](R) reader structure"]
impl crate::Readable for MAC_PHY_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_phy_ctl::W](W) writer structure"]
impl crate::Writable for MAC_PHY_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_PHY_CTL to value 0"]
impl crate::Resettable for MAC_PHY_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
