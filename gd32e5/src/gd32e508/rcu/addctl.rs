#[doc = "Register `ADDCTL` reader"]
pub type R = crate::R<ADDCTL_SPEC>;
#[doc = "Register `ADDCTL` writer"]
pub type W = crate::W<ADDCTL_SPEC>;
#[doc = "Field `CK48MSEL` reader - 48MHz clock selection"]
pub type CK48MSEL_R = crate::FieldReader;
#[doc = "Field `CK48MSEL` writer - 48MHz clock selection"]
pub type CK48MSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `USBHSSEL` reader - USBHS clock divider factor"]
pub type USBHSSEL_R = crate::BitReader;
#[doc = "Field `USBHSSEL` writer - USBHS clock divider factor"]
pub type USBHSSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHSDV` reader - USBHS clock selection"]
pub type USBHSDV_R = crate::FieldReader;
#[doc = "Field `USBHSDV` writer - USBHS clock selection"]
pub type USBHSDV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `USBSWEN` reader - USB clock source selection enable"]
pub type USBSWEN_R = crate::BitReader;
#[doc = "Field `USBSWEN` writer - USB clock source selection enable"]
pub type USBSWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLUSBEN` reader - PLLUSB enable"]
pub type PLLUSBEN_R = crate::BitReader;
#[doc = "Field `PLLUSBEN` writer - PLLUSB enable"]
pub type PLLUSBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLUSBSTB` reader - PLLUSB clock stabilization flag"]
pub type PLLUSBSTB_R = crate::BitReader;
#[doc = "Field `PLLUSBSTB` writer - PLLUSB clock stabilization flag"]
pub type PLLUSBSTB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRC48MEN` reader - Internal 48MHz RC oscillator enable"]
pub type IRC48MEN_R = crate::BitReader;
#[doc = "Field `IRC48MEN` writer - Internal 48MHz RC oscillator enable"]
pub type IRC48MEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRC48MSTB` reader - Internal 48MHz RC oscillator clock stabilization Flag"]
pub type IRC48MSTB_R = crate::BitReader;
#[doc = "Field `IRC48MCALIB` reader - Internal 48MHz RC oscillator calibration value register"]
pub type IRC48MCALIB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 48MHz clock selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - USBHS clock divider factor"]
    #[inline(always)]
    pub fn usbhssel(&self) -> USBHSSEL_R {
        USBHSSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - USBHS clock selection"]
    #[inline(always)]
    pub fn usbhsdv(&self) -> USBHSDV_R {
        USBHSDV_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - USB clock source selection enable"]
    #[inline(always)]
    pub fn usbswen(&self) -> USBSWEN_R {
        USBSWEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - PLLUSB enable"]
    #[inline(always)]
    pub fn pllusben(&self) -> PLLUSBEN_R {
        PLLUSBEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PLLUSB clock stabilization flag"]
    #[inline(always)]
    pub fn pllusbstb(&self) -> PLLUSBSTB_R {
        PLLUSBSTB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    pub fn irc48men(&self) -> IRC48MEN_R {
        IRC48MEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal 48MHz RC oscillator clock stabilization Flag"]
    #[inline(always)]
    pub fn irc48mstb(&self) -> IRC48MSTB_R {
        IRC48MSTB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Internal 48MHz RC oscillator calibration value register"]
    #[inline(always)]
    pub fn irc48mcalib(&self) -> IRC48MCALIB_R {
        IRC48MCALIB_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 48MHz clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn ck48msel(&mut self) -> CK48MSEL_W<ADDCTL_SPEC, 0> {
        CK48MSEL_W::new(self)
    }
    #[doc = "Bit 2 - USBHS clock divider factor"]
    #[inline(always)]
    #[must_use]
    pub fn usbhssel(&mut self) -> USBHSSEL_W<ADDCTL_SPEC, 2> {
        USBHSSEL_W::new(self)
    }
    #[doc = "Bits 3:5 - USBHS clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbhsdv(&mut self) -> USBHSDV_W<ADDCTL_SPEC, 3> {
        USBHSDV_W::new(self)
    }
    #[doc = "Bit 6 - USB clock source selection enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbswen(&mut self) -> USBSWEN_W<ADDCTL_SPEC, 6> {
        USBSWEN_W::new(self)
    }
    #[doc = "Bit 14 - PLLUSB enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllusben(&mut self) -> PLLUSBEN_W<ADDCTL_SPEC, 14> {
        PLLUSBEN_W::new(self)
    }
    #[doc = "Bit 15 - PLLUSB clock stabilization flag"]
    #[inline(always)]
    #[must_use]
    pub fn pllusbstb(&mut self) -> PLLUSBSTB_W<ADDCTL_SPEC, 15> {
        PLLUSBSTB_W::new(self)
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc48men(&mut self) -> IRC48MEN_W<ADDCTL_SPEC, 16> {
        IRC48MEN_W::new(self)
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
#[doc = "Additional clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDCTL_SPEC;
impl crate::RegisterSpec for ADDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addctl::R`](R) reader structure"]
impl crate::Readable for ADDCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addctl::W`](W) writer structure"]
impl crate::Writable for ADDCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDCTL to value 0x8000_0000"]
impl crate::Resettable for ADDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
