#[doc = "Register `CTL2` reader"]
pub type R = crate::R<CTL2_SPEC>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<CTL2_SPEC>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IREN_R = crate::BitReader;
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IRLP_R = crate::BitReader;
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IRLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDEN` reader - Half-duplex enable"]
pub type HDEN_R = crate::BitReader;
#[doc = "Field `HDEN` writer - Half-duplex enable"]
pub type HDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NKEN` reader - NACK enable in Smartcard mode"]
pub type NKEN_R = crate::BitReader;
#[doc = "Field `NKEN` writer - NACK enable in Smartcard mode"]
pub type NKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type SCEN_R = crate::BitReader;
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type SCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DENR` reader - DMA enable for reception"]
pub type DENR_R = crate::BitReader;
#[doc = "Field `DENR` writer - DMA enable for reception"]
pub type DENR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DENT` reader - DMA enable for transmission"]
pub type DENT_R = crate::BitReader;
#[doc = "Field `DENT` writer - DMA enable for transmission"]
pub type DENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSB` reader - One sample bit method"]
pub type OSB_R = crate::BitReader;
#[doc = "Field `OSB` writer - One sample bit method"]
pub type OSB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRD` reader - Overrun disable"]
pub type OVRD_R = crate::BitReader;
#[doc = "Field `OVRD` writer - Overrun disable"]
pub type OVRD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDRE` reader - Disable DMA on reception error"]
pub type DDRE_R = crate::BitReader;
#[doc = "Field `DDRE` writer - Disable DMA on reception error"]
pub type DDRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCRTNUM` reader - Smartcard auto-retry number"]
pub type SCRTNUM_R = crate::FieldReader;
#[doc = "Field `SCRTNUM` writer - Smartcard auto-retry number"]
pub type SCRTNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `WUM` reader - Wakeup mode from Deep-sleep mode"]
pub type WUM_R = crate::FieldReader;
#[doc = "Field `WUM` writer - Wakeup mode from Deep-sleep mode"]
pub type WUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `WUIE` reader - Wakeup from Deep-sleep mode interrupt enable"]
pub type WUIE_R = crate::BitReader;
#[doc = "Field `WUIE` writer - Wakeup from Deep-sleep mode interrupt enable"]
pub type WUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex enable"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK enable in Smartcard mode"]
    #[inline(always)]
    pub fn nken(&self) -> NKEN_R {
        NKEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable for reception"]
    #[inline(always)]
    pub fn denr(&self) -> DENR_R {
        DENR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable for transmission"]
    #[inline(always)]
    pub fn dent(&self) -> DENT_R {
        DENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method"]
    #[inline(always)]
    pub fn osb(&self) -> OSB_R {
        OSB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun disable"]
    #[inline(always)]
    pub fn ovrd(&self) -> OVRD_R {
        OVRD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable DMA on reception error"]
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry number"]
    #[inline(always)]
    pub fn scrtnum(&self) -> SCRTNUM_R {
        SCRTNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Wakeup mode from Deep-sleep mode"]
    #[inline(always)]
    pub fn wum(&self) -> WUM_R {
        WUM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wakeup from Deep-sleep mode interrupt enable"]
    #[inline(always)]
    pub fn wuie(&self) -> WUIE_R {
        WUIE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL2_SPEC, 0> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<CTL2_SPEC, 1> {
        IREN_W::new(self)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IRLP_W<CTL2_SPEC, 2> {
        IRLP_W::new(self)
    }
    #[doc = "Bit 3 - Half-duplex enable"]
    #[inline(always)]
    #[must_use]
    pub fn hden(&mut self) -> HDEN_W<CTL2_SPEC, 3> {
        HDEN_W::new(self)
    }
    #[doc = "Bit 4 - NACK enable in Smartcard mode"]
    #[inline(always)]
    #[must_use]
    pub fn nken(&mut self) -> NKEN_W<CTL2_SPEC, 4> {
        NKEN_W::new(self)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<CTL2_SPEC, 5> {
        SCEN_W::new(self)
    }
    #[doc = "Bit 6 - DMA enable for reception"]
    #[inline(always)]
    #[must_use]
    pub fn denr(&mut self) -> DENR_W<CTL2_SPEC, 6> {
        DENR_W::new(self)
    }
    #[doc = "Bit 7 - DMA enable for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn dent(&mut self) -> DENT_W<CTL2_SPEC, 7> {
        DENT_W::new(self)
    }
    #[doc = "Bit 11 - One sample bit method"]
    #[inline(always)]
    #[must_use]
    pub fn osb(&mut self) -> OSB_W<CTL2_SPEC, 11> {
        OSB_W::new(self)
    }
    #[doc = "Bit 12 - Overrun disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrd(&mut self) -> OVRD_W<CTL2_SPEC, 12> {
        OVRD_W::new(self)
    }
    #[doc = "Bit 13 - Disable DMA on reception error"]
    #[inline(always)]
    #[must_use]
    pub fn ddre(&mut self) -> DDRE_W<CTL2_SPEC, 13> {
        DDRE_W::new(self)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry number"]
    #[inline(always)]
    #[must_use]
    pub fn scrtnum(&mut self) -> SCRTNUM_W<CTL2_SPEC, 17> {
        SCRTNUM_W::new(self)
    }
    #[doc = "Bits 20:21 - Wakeup mode from Deep-sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wum(&mut self) -> WUM_W<CTL2_SPEC, 20> {
        WUM_W::new(self)
    }
    #[doc = "Bit 22 - Wakeup from Deep-sleep mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wuie(&mut self) -> WUIE_W<CTL2_SPEC, 22> {
        WUIE_W::new(self)
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
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL2_SPEC;
impl crate::RegisterSpec for CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for CTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for CTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
