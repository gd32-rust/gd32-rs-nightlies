#[doc = "Register `DADDR` reader"]
pub type R = crate::R<DADDR_SPEC>;
#[doc = "Register `DADDR` writer"]
pub type W = crate::W<DADDR_SPEC>;
#[doc = "Field `USBADDR` reader - USB device address"]
pub type USBADDR_R = crate::FieldReader;
#[doc = "Field `USBADDR` writer - USB device address"]
pub type USBADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `USBEN` reader - USB device enable"]
pub type USBEN_R = crate::BitReader;
#[doc = "Field `USBEN` writer - USB device enable"]
pub type USBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - USB device address"]
    #[inline(always)]
    pub fn usbaddr(&self) -> USBADDR_R {
        USBADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB device address"]
    #[inline(always)]
    #[must_use]
    pub fn usbaddr(&mut self) -> USBADDR_W<DADDR_SPEC, 0> {
        USBADDR_W::new(self)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<DADDR_SPEC, 7> {
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
#[doc = "device address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DADDR_SPEC;
impl crate::RegisterSpec for DADDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`daddr::R`](R) reader structure"]
impl crate::Readable for DADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`daddr::W`](W) writer structure"]
impl crate::Writable for DADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
