#[doc = "Register `MAC_HLH` reader"]
pub type R = crate::R<MAC_HLH_SPEC>;
#[doc = "Register `MAC_HLH` writer"]
pub type W = crate::W<MAC_HLH_SPEC>;
#[doc = "Field `HLH` reader - Hash list high"]
pub type HLH_R = crate::FieldReader<u32>;
#[doc = "Field `HLH` writer - Hash list high"]
pub type HLH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash list high"]
    #[inline(always)]
    pub fn hlh(&self) -> HLH_R {
        HLH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash list high"]
    #[inline(always)]
    #[must_use]
    pub fn hlh(&mut self) -> HLH_W<MAC_HLH_SPEC, 0> {
        HLH_W::new(self)
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
#[doc = "Ethernet MAC hash list high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_hlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_hlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_HLH_SPEC;
impl crate::RegisterSpec for MAC_HLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_hlh::R`](R) reader structure"]
impl crate::Readable for MAC_HLH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_hlh::W`](W) writer structure"]
impl crate::Writable for MAC_HLH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_HLH to value 0"]
impl crate::Resettable for MAC_HLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
