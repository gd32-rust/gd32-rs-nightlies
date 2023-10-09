#[doc = "Register `MAC_ADDR2L` reader"]
pub type R = crate::R<MAC_ADDR2L_SPEC>;
#[doc = "Register `MAC_ADDR2L` writer"]
pub type W = crate::W<MAC_ADDR2L_SPEC>;
#[doc = "Field `ADDR2L` reader - MAC address2 low"]
pub type ADDR2L_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR2L` writer - MAC address2 low"]
pub type ADDR2L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address2 low"]
    #[inline(always)]
    pub fn addr2l(&self) -> ADDR2L_R {
        ADDR2L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address2 low"]
    #[inline(always)]
    #[must_use]
    pub fn addr2l(&mut self) -> ADDR2L_W<MAC_ADDR2L_SPEC, 0> {
        ADDR2L_W::new(self)
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
#[doc = "Ethernet MAC address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr2l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr2l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_ADDR2L_SPEC;
impl crate::RegisterSpec for MAC_ADDR2L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr2l::R`](R) reader structure"]
impl crate::Readable for MAC_ADDR2L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_addr2l::W`](W) writer structure"]
impl crate::Writable for MAC_ADDR2L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_ADDR2L to value 0xffff_ffff"]
impl crate::Resettable for MAC_ADDR2L_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
