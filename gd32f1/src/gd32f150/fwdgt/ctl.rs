#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Key value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CMD` writer - Key value"]
pub type CMD_W<'a> = crate::FieldWriter<'a, u16, CTL_SPEC, u16, CMD_AW, 16, 0>;
impl<'a> CMD_W<'a> {
    #[doc = "Enable access to PR, RLR and WINR registers (0x5555)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CMD_AW::ENABLE)
    }
    #[doc = "Reset the watchdog value (0xAAAA)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CMD_AW::RESET)
    }
    #[doc = "Start the watchdog (0xCCCC)"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CMD_AW::START)
    }
}
impl W {
    #[doc = "Bits 0:15 - Key value"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
