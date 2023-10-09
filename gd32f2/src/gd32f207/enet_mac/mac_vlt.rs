#[doc = "Register `MAC_VLT` reader"]
pub type R = crate::R<MAC_VLT_SPEC>;
#[doc = "Register `MAC_VLT` writer"]
pub type W = crate::W<MAC_VLT_SPEC>;
#[doc = "Field `VLTI` reader - VLAN tag identifier (for receive frames)"]
pub type VLTI_R = crate::FieldReader<u16>;
#[doc = "Field `VLTI` writer - VLAN tag identifier (for receive frames)"]
pub type VLTI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `VLTC` reader - 12-bit VLAN tag comparison"]
pub type VLTC_R = crate::BitReader;
#[doc = "Field `VLTC` writer - 12-bit VLAN tag comparison"]
pub type VLTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vltc(&self) -> VLTC_R {
        VLTC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    #[must_use]
    pub fn vlti(&mut self) -> VLTI_W<MAC_VLT_SPEC, 0> {
        VLTI_W::new(self)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    #[must_use]
    pub fn vltc(&mut self) -> VLTC_W<MAC_VLT_SPEC, 16> {
        VLTC_W::new(self)
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
#[doc = "Ethernet MAC VLAN tag register (MAC_VLT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_vlt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_vlt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_VLT_SPEC;
impl crate::RegisterSpec for MAC_VLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_vlt::R`](R) reader structure"]
impl crate::Readable for MAC_VLT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_vlt::W`](W) writer structure"]
impl crate::Writable for MAC_VLT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_VLT to value 0"]
impl crate::Resettable for MAC_VLT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
