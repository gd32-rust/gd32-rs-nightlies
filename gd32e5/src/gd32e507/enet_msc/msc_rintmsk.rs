#[doc = "Register `MSC_RINTMSK` reader"]
pub type R = crate::R<MSC_RINTMSK_SPEC>;
#[doc = "Register `MSC_RINTMSK` writer"]
pub type W = crate::W<MSC_RINTMSK_SPEC>;
#[doc = "Field `RFCEIM` reader - Received frame CRC error interrupt mask"]
pub type RFCEIM_R = crate::BitReader;
#[doc = "Field `RFCEIM` writer - Received frame CRC error interrupt mask"]
pub type RFCEIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFAEIM` reader - Received frames alignment error interrupt mask"]
pub type RFAEIM_R = crate::BitReader;
#[doc = "Field `RFAEIM` writer - Received frames alignment error interrupt mask"]
pub type RFAEIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RGUFIM` reader - Received good unicast frames interrupt mask"]
pub type RGUFIM_R = crate::BitReader;
#[doc = "Field `RGUFIM` writer - Received good unicast frames interrupt mask"]
pub type RGUFIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 5 - Received frame CRC error interrupt mask"]
    #[inline(always)]
    pub fn rfceim(&self) -> RFCEIM_R {
        RFCEIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error interrupt mask"]
    #[inline(always)]
    pub fn rfaeim(&self) -> RFAEIM_R {
        RFAEIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good unicast frames interrupt mask"]
    #[inline(always)]
    pub fn rgufim(&self) -> RGUFIM_R {
        RGUFIM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frame CRC error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rfceim(&mut self) -> RFCEIM_W<MSC_RINTMSK_SPEC, 5> {
        RFCEIM_W::new(self)
    }
    #[doc = "Bit 6 - Received frames alignment error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rfaeim(&mut self) -> RFAEIM_W<MSC_RINTMSK_SPEC, 6> {
        RFAEIM_W::new(self)
    }
    #[doc = "Bit 17 - Received good unicast frames interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rgufim(&mut self) -> RGUFIM_W<MSC_RINTMSK_SPEC, 17> {
        RGUFIM_W::new(self)
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
#[doc = "Ethernet MSC receive interrupt mask register (MSC_RINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_rintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msc_rintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_RINTMSK_SPEC;
impl crate::RegisterSpec for MSC_RINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_rintmsk::R`](R) reader structure"]
impl crate::Readable for MSC_RINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msc_rintmsk::W`](W) writer structure"]
impl crate::Writable for MSC_RINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSC_RINTMSK to value 0"]
impl crate::Resettable for MSC_RINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
