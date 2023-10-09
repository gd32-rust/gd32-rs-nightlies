#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Auto baud rate request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABDCMD_AW {
    #[doc = "1: Resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    REQUEST = 1,
}
impl From<ABDCMD_AW> for bool {
    #[inline(always)]
    fn from(variant: ABDCMD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABDCMD` writer - Auto baud rate request"]
pub type ABDCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABDCMD_AW>;
impl<'a, REG, const O: u8> ABDCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    #[inline(always)]
    pub fn request(self) -> &'a mut crate::W<REG> {
        self.variant(ABDCMD_AW::REQUEST)
    }
}
#[doc = "Send break request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKCMD_AW {
    #[doc = "1: Sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    BREAK = 1,
}
impl From<SBKCMD_AW> for bool {
    #[inline(always)]
    fn from(variant: SBKCMD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBKCMD` writer - Send break request"]
pub type SBKCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SBKCMD_AW>;
impl<'a, REG, const O: u8> SBKCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(SBKCMD_AW::BREAK)
    }
}
#[doc = "Mute mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCMD_AW {
    #[doc = "1: Puts the USART in mute mode and sets the RWU flag"]
    MUTE = 1,
}
impl From<MMCMD_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCMD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCMD` writer - Mute mode request"]
pub type MMCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MMCMD_AW>;
impl<'a, REG, const O: u8> MMCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut crate::W<REG> {
        self.variant(MMCMD_AW::MUTE)
    }
}
#[doc = "Receive data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFCMD_AW {
    #[doc = "1: Clears the RBNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    DISCARD = 1,
}
impl From<RXFCMD_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFCMD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFCMD` writer - Receive data flush request"]
pub type RXFCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXFCMD_AW>;
impl<'a, REG, const O: u8> RXFCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the RBNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(RXFCMD_AW::DISCARD)
    }
}
#[doc = "Transmit data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFCMD_AW {
    #[doc = "1: Set the TBE flag. This allows to discard the transmit data"]
    DISCARD = 1,
}
impl From<TXFCMD_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFCMD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFCMD` writer - Transmit data flush request"]
pub type TXFCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXFCMD_AW>;
impl<'a, REG, const O: u8> TXFCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set the TBE flag. This allows to discard the transmit data"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(TXFCMD_AW::DISCARD)
    }
}
impl W {
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    #[must_use]
    pub fn abdcmd(&mut self) -> ABDCMD_W<CMD_SPEC, 0> {
        ABDCMD_W::new(self)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    #[must_use]
    pub fn sbkcmd(&mut self) -> SBKCMD_W<CMD_SPEC, 1> {
        SBKCMD_W::new(self)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    #[must_use]
    pub fn mmcmd(&mut self) -> MMCMD_W<CMD_SPEC, 2> {
        MMCMD_W::new(self)
    }
    #[doc = "Bit 3 - Receive data flush request"]
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
#[doc = "Request register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
