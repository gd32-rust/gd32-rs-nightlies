#[doc = "Register `HCH3INTEN` reader"]
pub struct R(crate::R<HCH3INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCH3INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCH3INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCH3INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCH3INTEN` writer"]
pub struct W(crate::W<HCH3INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCH3INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HCH3INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCH3INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFIE` reader - Transfer completed interrupt enable"]
pub type TFIE_R = crate::BitReader<bool>;
#[doc = "Field `TFIE` writer - Transfer completed interrupt enable"]
pub type TFIE_W<'a> = crate::BitWriter<'a, u32, HCH3INTEN_SPEC, bool, 0>;
#[doc = "Field `CHIE` reader - Channel halted interrupt enable"]
pub type CHIE_R = crate::BitReader<bool>;
#[doc = "Field `CHIE` writer - Channel halted interrupt enable"]
pub type CHIE_W<'a> = crate::BitWriter<'a, u32, HCH3INTEN_SPEC, bool, 1>;
#[doc = "Field `STALLIE` reader - STALL interrupt enable"]
pub type STALLIE_R = crate::BitReader<bool>;
#[doc = "Field `STALLIE` writer - STALL interrupt enable"]
pub type STALLIE_W<'a> = crate::BitWriter<'a, u32, HCH3INTEN_SPEC, bool, 3>;
#[doc = "Field `NAKIE` reader - NAK interrupt enable"]
pub type NAKIE_R = crate::BitReader<bool>;
#[doc = "Field `NAKIE` writer - NAK interrupt enable"]
pub type NAKIE_W<'a> = crate::BitWriter<'a, u32, HCH3INTEN_SPEC, bool, 4>;
#[doc = "Field `ACKIE` reader - ACK interrupt enable"]
pub type ACKIE_R = crate::BitReader<bool>;
#[doc = "Field `ACKIE` writer - ACK interrupt enable"]
pub type ACKIE_W<'a> = crate::BitWriter<'a, u32, HCH3INTEN_SPEC, bool, 5>;
#[doc = "Field `USBERIE` reader - USB bus error interrupt enable"]
pub type USBERIE_R = crate::BitReader<bool>;
#[doc = "Field `USBERIE` writer - USB bus error interrupt enable"]
pub type USBERIE_W<'a> = crate::BitWriter<'a, u32, HCH3INTEN_SPEC, bool, 7>;
#[doc = "Field `BBERIE` reader - Babble error interrupt enable"]
pub type BBERIE_R = crate::BitReader<bool>;
#[doc = "Field `BBERIE` writer - Babble error interrupt enable"]
pub type BBERIE_W<'a> = crate::BitWriter<'a, u32, HCH3INTEN_SPEC, bool, 8>;
#[doc = "Field `REQOVRIE` reader - request queue overrun interrupt enable"]
pub type REQOVRIE_R = crate::BitReader<bool>;
#[doc = "Field `REQOVRIE` writer - request queue overrun interrupt enable"]
pub type REQOVRIE_W<'a> = crate::BitWriter<'a, u32, HCH3INTEN_SPEC, bool, 9>;
#[doc = "Field `DTERIE` reader - Data toggle error interrupt enable"]
pub type DTERIE_R = crate::BitReader<bool>;
#[doc = "Field `DTERIE` writer - Data toggle error interrupt enable"]
pub type DTERIE_W<'a> = crate::BitWriter<'a, u32, HCH3INTEN_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt enable"]
    #[inline(always)]
    pub fn tfie(&self) -> TFIE_R {
        TFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted interrupt enable"]
    #[inline(always)]
    pub fn chie(&self) -> CHIE_R {
        CHIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL interrupt enable"]
    #[inline(always)]
    pub fn stallie(&self) -> STALLIE_R {
        STALLIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK interrupt enable"]
    #[inline(always)]
    pub fn nakie(&self) -> NAKIE_R {
        NAKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK interrupt enable"]
    #[inline(always)]
    pub fn ackie(&self) -> ACKIE_R {
        ACKIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USB bus error interrupt enable"]
    #[inline(always)]
    pub fn usberie(&self) -> USBERIE_R {
        USBERIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error interrupt enable"]
    #[inline(always)]
    pub fn bberie(&self) -> BBERIE_R {
        BBERIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - request queue overrun interrupt enable"]
    #[inline(always)]
    pub fn reqovrie(&self) -> REQOVRIE_R {
        REQOVRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error interrupt enable"]
    #[inline(always)]
    pub fn dterie(&self) -> DTERIE_R {
        DTERIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt enable"]
    #[inline(always)]
    pub fn tfie(&mut self) -> TFIE_W {
        TFIE_W::new(self)
    }
    #[doc = "Bit 1 - Channel halted interrupt enable"]
    #[inline(always)]
    pub fn chie(&mut self) -> CHIE_W {
        CHIE_W::new(self)
    }
    #[doc = "Bit 3 - STALL interrupt enable"]
    #[inline(always)]
    pub fn stallie(&mut self) -> STALLIE_W {
        STALLIE_W::new(self)
    }
    #[doc = "Bit 4 - NAK interrupt enable"]
    #[inline(always)]
    pub fn nakie(&mut self) -> NAKIE_W {
        NAKIE_W::new(self)
    }
    #[doc = "Bit 5 - ACK interrupt enable"]
    #[inline(always)]
    pub fn ackie(&mut self) -> ACKIE_W {
        ACKIE_W::new(self)
    }
    #[doc = "Bit 7 - USB bus error interrupt enable"]
    #[inline(always)]
    pub fn usberie(&mut self) -> USBERIE_W {
        USBERIE_W::new(self)
    }
    #[doc = "Bit 8 - Babble error interrupt enable"]
    #[inline(always)]
    pub fn bberie(&mut self) -> BBERIE_W {
        BBERIE_W::new(self)
    }
    #[doc = "Bit 9 - request queue overrun interrupt enable"]
    #[inline(always)]
    pub fn reqovrie(&mut self) -> REQOVRIE_W {
        REQOVRIE_W::new(self)
    }
    #[doc = "Bit 10 - Data toggle error interrupt enable"]
    #[inline(always)]
    pub fn dterie(&mut self) -> DTERIE_W {
        DTERIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "host channel-3 interrupt enable register (HCH3INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch3inten](index.html) module"]
pub struct HCH3INTEN_SPEC;
impl crate::RegisterSpec for HCH3INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hch3inten::R](R) reader structure"]
impl crate::Readable for HCH3INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hch3inten::W](W) writer structure"]
impl crate::Writable for HCH3INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCH3INTEN to value 0"]
impl crate::Resettable for HCH3INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
