#[doc = "Register `MAC_ADDR1L` reader"]
pub type R = crate::R<MAC_ADDR1L_SPEC>;
#[doc = "Register `MAC_ADDR1L` writer"]
pub type W = crate::W<MAC_ADDR1L_SPEC>;
#[doc = "Field `ADDR1L` reader - MAC address1 low"]
pub type ADDR1L_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR1L` writer - MAC address1 low"]
pub type ADDR1L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address1 low"]
    #[inline(always)]
    pub fn addr1l(&self) -> ADDR1L_R {
        ADDR1L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address1 low"]
    #[inline(always)]
    #[must_use]
    pub fn addr1l(&mut self) -> ADDR1L_W<MAC_ADDR1L_SPEC, 0> {
        ADDR1L_W::new(self)
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
#[doc = "Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr1l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr1l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_ADDR1L_SPEC;
impl crate::RegisterSpec for MAC_ADDR1L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr1l::R`](R) reader structure"]
impl crate::Readable for MAC_ADDR1L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_addr1l::W`](W) writer structure"]
impl crate::Writable for MAC_ADDR1L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_ADDR1L to value 0xffff_ffff"]
impl crate::Resettable for MAC_ADDR1L_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
