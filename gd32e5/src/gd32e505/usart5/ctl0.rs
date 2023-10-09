#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `UEN` reader - USART enable"]
pub type UEN_R = crate::BitReader;
#[doc = "Field `UEN` writer - USART enable"]
pub type UEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UESM` reader - USART enable in Deep-sleep mode"]
pub type UESM_R = crate::BitReader;
#[doc = "Field `UESM` writer - USART enable in Deep-sleep mode"]
pub type UESM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader;
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader;
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDLEIE` reader - IDLE line detected interrupt enable"]
pub type IDLEIE_R = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLE line detected interrupt enable"]
pub type IDLEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBNEIE` reader - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RBNEIE_R = crate::BitReader;
#[doc = "Field `RBNEIE` writer - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RBNEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TBEIE` reader - Transmitter register empty interrupt enable"]
pub type TBEIE_R = crate::BitReader;
#[doc = "Field `TBEIE` writer - Transmitter register empty interrupt enable"]
pub type TBEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERRIE` reader - Parity error interrupt enable"]
pub type PERRIE_R = crate::BitReader;
#[doc = "Field `PERRIE` writer - Parity error interrupt enable"]
pub type PERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PM` reader - Parity mode"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - Parity mode"]
pub type PM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCEN` reader - Parity control enable"]
pub type PCEN_R = crate::BitReader;
#[doc = "Field `PCEN` writer - Parity control enable"]
pub type PCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WM` reader - Wakeup method in mute mode"]
pub type WM_R = crate::BitReader;
#[doc = "Field `WM` writer - Wakeup method in mute mode"]
pub type WM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WL` reader - Word length"]
pub type WL_R = crate::BitReader;
#[doc = "Field `WL` writer - Word length"]
pub type WL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MEN` reader - Mute mode enable"]
pub type MEN_R = crate::BitReader;
#[doc = "Field `MEN` writer - Mute mode enable"]
pub type MEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMIE` reader - ADDR match interrupt enable"]
pub type AMIE_R = crate::BitReader;
#[doc = "Field `AMIE` writer - ADDR match interrupt enable"]
pub type AMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVSMOD` reader - Oversample mode"]
pub type OVSMOD_R = crate::BitReader;
#[doc = "Field `OVSMOD` writer - Oversample mode"]
pub type OVSMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTIE` reader - Receiver timeout interrupt enable"]
pub type RTIE_R = crate::BitReader;
#[doc = "Field `RTIE` writer - Receiver timeout interrupt enable"]
pub type RTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EBIE` reader - End of Block interrupt enable"]
pub type EBIE_R = crate::BitReader;
#[doc = "Field `EBIE` writer - End of Block interrupt enable"]
pub type EBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UEN_R {
        UEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART enable in Deep-sleep mode"]
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter register empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn wl(&self) -> WL_R {
        WL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn men(&self) -> MEN_R {
        MEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADDR match interrupt enable"]
    #[inline(always)]
    pub fn amie(&self) -> AMIE_R {
        AMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Oversample mode"]
    #[inline(always)]
    pub fn ovsmod(&self) -> OVSMOD_R {
        OVSMOD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    pub fn ebie(&self) -> EBIE_R {
        EBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn uen(&mut self) -> UEN_W<CTL0_SPEC, 0> {
        UEN_W::new(self)
    }
    #[doc = "Bit 1 - USART enable in Deep-sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UESM_W<CTL0_SPEC, 1> {
        UESM_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<CTL0_SPEC, 2> {
        REN_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<CTL0_SPEC, 3> {
        TEN_W::new(self)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<CTL0_SPEC, 4> {
        IDLEIE_W::new(self)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbneie(&mut self) -> RBNEIE_W<CTL0_SPEC, 5> {
        RBNEIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CTL0_SPEC, 6> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter register empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbeie(&mut self) -> TBEIE_W<CTL0_SPEC, 7> {
        TBEIE_W::new(self)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrie(&mut self) -> PERRIE_W<CTL0_SPEC, 8> {
        PERRIE_W::new(self)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<CTL0_SPEC, 9> {
        PM_W::new(self)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<CTL0_SPEC, 10> {
        PCEN_W::new(self)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    #[must_use]
    pub fn wm(&mut self) -> WM_W<CTL0_SPEC, 11> {
        WM_W::new(self)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    #[must_use]
    pub fn wl(&mut self) -> WL_W<CTL0_SPEC, 12> {
        WL_W::new(self)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn men(&mut self) -> MEN_W<CTL0_SPEC, 13> {
        MEN_W::new(self)
    }
    #[doc = "Bit 14 - ADDR match interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn amie(&mut self) -> AMIE_W<CTL0_SPEC, 14> {
        AMIE_W::new(self)
    }
    #[doc = "Bit 15 - Oversample mode"]
    #[inline(always)]
    #[must_use]
    pub fn ovsmod(&mut self) -> OVSMOD_W<CTL0_SPEC, 15> {
        OVSMOD_W::new(self)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtie(&mut self) -> RTIE_W<CTL0_SPEC, 26> {
        RTIE_W::new(self)
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ebie(&mut self) -> EBIE_W<CTL0_SPEC, 27> {
        EBIE_W::new(self)
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
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
