#[doc = "Register `MAC_ADDR1H` reader"]
pub type R = crate::R<MAC_ADDR1H_SPEC>;
#[doc = "Register `MAC_ADDR1H` writer"]
pub type W = crate::W<MAC_ADDR1H_SPEC>;
#[doc = "Field `ADDR1H` reader - MAC address1 high"]
pub type ADDR1H_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR1H` writer - MAC address1 high"]
pub type ADDR1H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `MB` reader - Mask byte"]
pub type MB_R = crate::FieldReader;
#[doc = "Field `MB` writer - Mask byte"]
pub type MB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SAF` reader - Source address filter"]
pub type SAF_R = crate::BitReader;
#[doc = "Field `SAF` writer - Source address filter"]
pub type SAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AFE` reader - Address filter enable"]
pub type AFE_R = crate::BitReader;
#[doc = "Field `AFE` writer - Address filter enable"]
pub type AFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - MAC address1 high"]
    #[inline(always)]
    pub fn addr1h(&self) -> ADDR1H_R {
        ADDR1H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask byte"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address filter enable"]
    #[inline(always)]
    pub fn afe(&self) -> AFE_R {
        AFE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address1 high"]
    #[inline(always)]
    #[must_use]
    pub fn addr1h(&mut self) -> ADDR1H_W<MAC_ADDR1H_SPEC, 0> {
        ADDR1H_W::new(self)
    }
    #[doc = "Bits 24:29 - Mask byte"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<MAC_ADDR1H_SPEC, 24> {
        MB_W::new(self)
    }
    #[doc = "Bit 30 - Source address filter"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<MAC_ADDR1H_SPEC, 30> {
        SAF_W::new(self)
    }
    #[doc = "Bit 31 - Address filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn afe(&mut self) -> AFE_W<MAC_ADDR1H_SPEC, 31> {
        AFE_W::new(self)
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
#[doc = "Ethernet MAC address 1 high register (MAC_ADDR1H)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr1h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr1h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_ADDR1H_SPEC;
impl crate::RegisterSpec for MAC_ADDR1H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr1h::R`](R) reader structure"]
impl crate::Readable for MAC_ADDR1H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_addr1h::W`](W) writer structure"]
impl crate::Writable for MAC_ADDR1H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_ADDR1H to value 0xffff"]
impl crate::Resettable for MAC_ADDR1H_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
