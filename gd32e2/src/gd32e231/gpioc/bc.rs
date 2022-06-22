#[doc = "Register `BC` writer"]
pub struct W(crate::W<BC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BC_SPEC>;
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
impl From<crate::W<BC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port cleat bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CR0_AW {
    #[doc = "1: Resets the corresponding OCTLx bit"]
    RESET = 1,
}
impl From<CR0_AW> for bool {
    #[inline(always)]
    fn from(variant: CR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CR0` writer - Port cleat bit"]
pub type CR0_W<'a> = crate::BitWriter<'a, u32, BC_SPEC, CR0_AW, 0>;
impl<'a> CR0_W<'a> {
    #[doc = "Resets the corresponding OCTLx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CR0_AW::RESET)
    }
}
#[doc = "Port cleat bit"]
pub use CR0_AW as CR1_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR2_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR3_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR4_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR5_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR6_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR7_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR8_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR9_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR10_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR11_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR12_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR13_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR14_AW;
#[doc = "Port cleat bit"]
pub use CR0_AW as CR15_AW;
#[doc = "Field `CR1` writer - Port cleat bit"]
pub use CR0_W as CR1_W;
#[doc = "Field `CR2` writer - Port cleat bit"]
pub use CR0_W as CR2_W;
#[doc = "Field `CR3` writer - Port cleat bit"]
pub use CR0_W as CR3_W;
#[doc = "Field `CR4` writer - Port cleat bit"]
pub use CR0_W as CR4_W;
#[doc = "Field `CR5` writer - Port cleat bit"]
pub use CR0_W as CR5_W;
#[doc = "Field `CR6` writer - Port cleat bit"]
pub use CR0_W as CR6_W;
#[doc = "Field `CR7` writer - Port cleat bit"]
pub use CR0_W as CR7_W;
#[doc = "Field `CR8` writer - Port cleat bit"]
pub use CR0_W as CR8_W;
#[doc = "Field `CR9` writer - Port cleat bit"]
pub use CR0_W as CR9_W;
#[doc = "Field `CR10` writer - Port cleat bit"]
pub use CR0_W as CR10_W;
#[doc = "Field `CR11` writer - Port cleat bit"]
pub use CR0_W as CR11_W;
#[doc = "Field `CR12` writer - Port cleat bit"]
pub use CR0_W as CR12_W;
#[doc = "Field `CR13` writer - Port cleat bit"]
pub use CR0_W as CR13_W;
#[doc = "Field `CR14` writer - Port cleat bit"]
pub use CR0_W as CR14_W;
#[doc = "Field `CR15` writer - Port cleat bit"]
pub use CR0_W as CR15_W;
impl W {
    #[doc = "Bit 0 - Port cleat bit"]
    #[inline(always)]
    pub fn cr0(&mut self) -> CR0_W {
        CR0_W::new(self)
    }
    #[doc = "Bit 1 - Port cleat bit"]
    #[inline(always)]
    pub fn cr1(&mut self) -> CR1_W {
        CR1_W::new(self)
    }
    #[doc = "Bit 2 - Port cleat bit"]
    #[inline(always)]
    pub fn cr2(&mut self) -> CR2_W {
        CR2_W::new(self)
    }
    #[doc = "Bit 3 - Port cleat bit"]
    #[inline(always)]
    pub fn cr3(&mut self) -> CR3_W {
        CR3_W::new(self)
    }
    #[doc = "Bit 4 - Port cleat bit"]
    #[inline(always)]
    pub fn cr4(&mut self) -> CR4_W {
        CR4_W::new(self)
    }
    #[doc = "Bit 5 - Port cleat bit"]
    #[inline(always)]
    pub fn cr5(&mut self) -> CR5_W {
        CR5_W::new(self)
    }
    #[doc = "Bit 6 - Port cleat bit"]
    #[inline(always)]
    pub fn cr6(&mut self) -> CR6_W {
        CR6_W::new(self)
    }
    #[doc = "Bit 7 - Port cleat bit"]
    #[inline(always)]
    pub fn cr7(&mut self) -> CR7_W {
        CR7_W::new(self)
    }
    #[doc = "Bit 8 - Port cleat bit"]
    #[inline(always)]
    pub fn cr8(&mut self) -> CR8_W {
        CR8_W::new(self)
    }
    #[doc = "Bit 9 - Port cleat bit"]
    #[inline(always)]
    pub fn cr9(&mut self) -> CR9_W {
        CR9_W::new(self)
    }
    #[doc = "Bit 10 - Port cleat bit"]
    #[inline(always)]
    pub fn cr10(&mut self) -> CR10_W {
        CR10_W::new(self)
    }
    #[doc = "Bit 11 - Port cleat bit"]
    #[inline(always)]
    pub fn cr11(&mut self) -> CR11_W {
        CR11_W::new(self)
    }
    #[doc = "Bit 12 - Port cleat bit"]
    #[inline(always)]
    pub fn cr12(&mut self) -> CR12_W {
        CR12_W::new(self)
    }
    #[doc = "Bit 13 - Port cleat bit"]
    #[inline(always)]
    pub fn cr13(&mut self) -> CR13_W {
        CR13_W::new(self)
    }
    #[doc = "Bit 14 - Port cleat bit"]
    #[inline(always)]
    pub fn cr14(&mut self) -> CR14_W {
        CR14_W::new(self)
    }
    #[doc = "Bit 15 - Port cleat bit"]
    #[inline(always)]
    pub fn cr15(&mut self) -> CR15_W {
        CR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port bit reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bc](index.html) module"]
pub struct BC_SPEC;
impl crate::RegisterSpec for BC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bc::W](W) writer structure"]
impl crate::Writable for BC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BC to value 0"]
impl crate::Resettable for BC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
