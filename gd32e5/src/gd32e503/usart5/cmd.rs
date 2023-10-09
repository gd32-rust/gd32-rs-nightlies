#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `ABDCMD` writer - Auto baudrate detection command"]
pub type ABDCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SBKCMD` writer - Send break command"]
pub type SBKCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MMCMD` writer - Mute mode command"]
pub type MMCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFCMD` writer - Receive data flush command"]
pub type RXFCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFCMD` writer - Transmit data flush request"]
pub type TXFCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Auto baudrate detection command"]
    #[inline(always)]
    #[must_use]
    pub fn abdcmd(&mut self) -> ABDCMD_W<CMD_SPEC, 0> {
        ABDCMD_W::new(self)
    }
    #[doc = "Bit 1 - Send break command"]
    #[inline(always)]
    #[must_use]
    pub fn sbkcmd(&mut self) -> SBKCMD_W<CMD_SPEC, 1> {
        SBKCMD_W::new(self)
    }
    #[doc = "Bit 2 - Mute mode command"]
    #[inline(always)]
    #[must_use]
    pub fn mmcmd(&mut self) -> MMCMD_W<CMD_SPEC, 2> {
        MMCMD_W::new(self)
    }
    #[doc = "Bit 3 - Receive data flush command"]
    #[inline(always)]
    #[must_use]
    pub fn rxfcmd(&mut self) -> RXFCMD_W<CMD_SPEC, 3> {
        RXFCMD_W::new(self)
    }
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    #[must_use]
    pub fn txfcmd(&mut self) -> TXFCMD_W<CMD_SPEC, 4> {
        TXFCMD_W::new(self)
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
#[doc = "Command register (USART_CMD)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
