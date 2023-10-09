#[doc = "Register `MAC_ADDR0H` reader"]
pub type R = crate::R<MAC_ADDR0H_SPEC>;
#[doc = "Register `MAC_ADDR0H` writer"]
pub type W = crate::W<MAC_ADDR0H_SPEC>;
#[doc = "Field `ADDR0H` reader - MAC address0 high"]
pub type ADDR0H_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR0H` writer - MAC address0 high"]
pub type ADDR0H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `MO` reader - Always 1"]
pub type MO_R = crate::BitReader;
#[doc = "Field `MO` writer - Always 1"]
pub type MO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn addr0h(&self) -> ADDR0H_R {
        ADDR0H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Always 1"]
    #[inline(always)]
    pub fn mo(&self) -> MO_R {
        MO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    #[must_use]
    pub fn addr0h(&mut self) -> ADDR0H_W<MAC_ADDR0H_SPEC, 0> {
        ADDR0H_W::new(self)
    }
    #[doc = "Bit 31 - Always 1"]
    #[inline(always)]
    #[must_use]
    pub fn mo(&mut self) -> MO_W<MAC_ADDR0H_SPEC, 31> {
        MO_W::new(self)
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
#[doc = "Ethernet MAC address 0 high register (MAC_ADDR0H)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr0h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr0h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_ADDR0H_SPEC;
impl crate::RegisterSpec for MAC_ADDR0H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr0h::R`](R) reader structure"]
impl crate::Readable for MAC_ADDR0H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_addr0h::W`](W) writer structure"]
impl crate::Writable for MAC_ADDR0H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_ADDR0H to value 0x8000_ffff"]
impl crate::Resettable for MAC_ADDR0H_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_ffff;
}
