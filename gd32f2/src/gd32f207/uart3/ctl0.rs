#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEN` reader - USART enable"]
pub type UEN_R = crate::BitReader<bool>;
#[doc = "Field `UEN` writer - USART enable"]
pub type UEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 13>;
#[doc = "Field `WL` reader - Word length"]
pub type WL_R = crate::BitReader<bool>;
#[doc = "Field `WL` writer - Word length"]
pub type WL_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 12>;
#[doc = "Field `WM` reader - Wakeup method in mute mode"]
pub type WM_R = crate::BitReader<bool>;
#[doc = "Field `WM` writer - Wakeup method in mute mode"]
pub type WM_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 11>;
#[doc = "Field `PCEN` reader - Parity check function enable"]
pub type PCEN_R = crate::BitReader<bool>;
#[doc = "Field `PCEN` writer - Parity check function enable"]
pub type PCEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 10>;
#[doc = "Field `PM` reader - Parity mode"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - Parity mode"]
pub type PM_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 9>;
#[doc = "Field `PERRIE` reader - Parity error interrupt enable"]
pub type PERRIE_R = crate::BitReader<bool>;
#[doc = "Field `PERRIE` writer - Parity error interrupt enable"]
pub type PERRIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 8>;
#[doc = "Field `TBEIE` reader - Transmitter buffer empty interrupt enable"]
pub type TBEIE_R = crate::BitReader<bool>;
#[doc = "Field `TBEIE` writer - Transmitter buffer empty interrupt enable"]
pub type TBEIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 7>;
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TCIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 6>;
#[doc = "Field `RBNEIE` reader - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RBNEIE_R = crate::BitReader<bool>;
#[doc = "Field `RBNEIE` writer - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RBNEIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 5>;
#[doc = "Field `IDLEIE` reader - IDLE line detected interrupt enable"]
pub type IDLEIE_R = crate::BitReader<bool>;
#[doc = "Field `IDLEIE` writer - IDLE line detected interrupt enable"]
pub type IDLEIE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 4>;
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader<bool>;
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 3>;
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader<bool>;
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 2>;
#[doc = "Field `RWU` reader - Receiver wakeup from mute mode"]
pub type RWU_R = crate::BitReader<bool>;
#[doc = "Field `RWU` writer - Receiver wakeup from mute mode"]
pub type RWU_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 1>;
#[doc = "Field `SBKCMD` reader - Send break command"]
pub type SBKCMD_R = crate::BitReader<bool>;
#[doc = "Field `SBKCMD` writer - Send break command"]
pub type SBKCMD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UEN_R {
        UEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn wl(&self) -> WL_R {
        WL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity check function enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Send break command"]
    #[inline(always)]
    pub fn sbkcmd(&self) -> SBKCMD_R {
        SBKCMD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&mut self) -> UEN_W {
        UEN_W::new(self)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn wl(&mut self) -> WL_W {
        WL_W::new(self)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    pub fn wm(&mut self) -> WM_W {
        WM_W::new(self)
    }
    #[doc = "Bit 10 - Parity check function enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W::new(self)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W::new(self)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&mut self) -> PERRIE_W {
        PERRIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&mut self) -> TBEIE_W {
        TBEIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W::new(self)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&mut self) -> RBNEIE_W {
        RBNEIE_W::new(self)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W {
        IDLEIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W {
        REN_W::new(self)
    }
    #[doc = "Bit 1 - Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn rwu(&mut self) -> RWU_W {
        RWU_W::new(self)
    }
    #[doc = "Bit 0 - Send break command"]
    #[inline(always)]
    pub fn sbkcmd(&mut self) -> SBKCMD_W {
        SBKCMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
