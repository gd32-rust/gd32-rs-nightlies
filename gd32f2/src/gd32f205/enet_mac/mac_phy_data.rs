#[doc = "Register `MAC_PHY_DATA` reader"]
pub type R = crate::R<MAC_PHY_DATA_SPEC>;
#[doc = "Register `MAC_PHY_DATA` writer"]
pub type W = crate::W<MAC_PHY_DATA_SPEC>;
#[doc = "Field `PD` reader - PHY data"]
pub type PD_R = crate::FieldReader<u16>;
#[doc = "Field `PD` writer - PHY data"]
pub type PD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PHY data"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY data"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<MAC_PHY_DATA_SPEC, 0> {
        PD_W::new(self)
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
#[doc = "Ethernet MAC MII data register (MAC_PHY_DATA)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_phy_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_phy_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_PHY_DATA_SPEC;
impl crate::RegisterSpec for MAC_PHY_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_phy_data::R`](R) reader structure"]
impl crate::Readable for MAC_PHY_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_phy_data::W`](W) writer structure"]
impl crate::Writable for MAC_PHY_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_PHY_DATA to value 0"]
impl crate::Resettable for MAC_PHY_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
