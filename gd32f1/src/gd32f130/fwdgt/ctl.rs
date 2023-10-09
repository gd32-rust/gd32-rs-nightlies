#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Key value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CMD_AW {
    #[doc = "21845: Enable access to PR, RLR and WINR registers (0x5555)"]
    ENABLE = 21845,
    #[doc = "43690: Reset the watchdog value (0xAAAA)"]
    RESET = 43690,
    #[doc = "52428: Start the watchdog (0xCCCC)"]
    START = 52428,
}
impl From<CMD_AW> for u16 {
    #[inline(always)]
    fn from(variant: CMD_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMD_AW {
    type Ux = u16;
}
#[doc = "Field `CMD` writer - Key value"]
pub type CMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, CMD_AW>;
impl<'a, REG, const O: u8> CMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Enable access to PR, RLR and WINR registers (0x5555)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_AW::ENABLE)
    }
    #[doc = "Reset the watchdog value (0xAAAA)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_AW::RESET)
    }
    #[doc = "Start the watchdog (0xCCCC)"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_AW::START)
    }
}
impl W {
    #[doc = "Bits 0:15 - Key value"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<CTL_SPEC, 0> {
        CMD_W::new(self)
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
#[doc = "Control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
