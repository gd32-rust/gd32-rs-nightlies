#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type TXFCMD_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, TXFCMD_AW, 4>;
impl<'a> TXFCMD_W<'a> {
    #[doc = "Set the TBE flag. This allows to discard the transmit data"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(TXFCMD_AW::DISCARD)
    }
}
#[doc = "Receive data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type RXFCMD_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, RXFCMD_AW, 3>;
impl<'a> RXFCMD_W<'a> {
    #[doc = "Clears the RBNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(RXFCMD_AW::DISCARD)
    }
}
#[doc = "Mute mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type MMCMD_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, MMCMD_AW, 2>;
impl<'a> MMCMD_W<'a> {
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(MMCMD_AW::MUTE)
    }
}
#[doc = "Send break request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type SBKCMD_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, SBKCMD_AW, 1>;
impl<'a> SBKCMD_W<'a> {
    #[doc = "Sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBKCMD_AW::BREAK)
    }
}
#[doc = "Auto baud rate request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type ABDCMD_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, ABDCMD_AW, 0>;
impl<'a> ABDCMD_W<'a> {
    #[doc = "Resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(ABDCMD_AW::REQUEST)
    }
}
impl W {
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfcmd(&mut self) -> TXFCMD_W {
        TXFCMD_W::new(self)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfcmd(&mut self) -> RXFCMD_W {
        RXFCMD_W::new(self)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmcmd(&mut self) -> MMCMD_W {
        MMCMD_W::new(self)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkcmd(&mut self) -> SBKCMD_W {
        SBKCMD_W::new(self)
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abdcmd(&mut self) -> ABDCMD_W {
        ABDCMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Request register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
