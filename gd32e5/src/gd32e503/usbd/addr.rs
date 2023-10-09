#[doc = "Register `ADDR` reader"]
pub type R = crate::R<ADDR_SPEC>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<ADDR_SPEC>;
#[doc = "Field `USBDAR` reader - Device address"]
pub type USBDAR_R = crate::FieldReader;
#[doc = "Field `USBDAR` writer - Device address"]
pub type USBDAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `USBEN` reader - USB device enable"]
pub type USBEN_R = crate::BitReader;
#[doc = "Field `USBEN` writer - USB device enable"]
pub type USBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    pub fn usbdar(&self) -> USBDAR_R {
        USBDAR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn usbdar(&mut self) -> USBDAR_W<ADDR_SPEC, 0> {
        USBDAR_W::new(self)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<ADDR_SPEC, 7> {
        USBEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "device address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
