#[doc = "Register `BC` writer"]
pub type W = crate::W<BC_SPEC>;
#[doc = "Port cleat bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CR0W_AW {
    #[doc = "1: Resets the corresponding OCTLx bit"]
    RESET = 1,
}
impl From<CR0W_AW> for bool {
    #[inline(always)]
    fn from(variant: CR0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CR0` writer - Port cleat bit"]
pub type CR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CR0W_AW>;
impl<'a, REG, const O: u8> CR0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the corresponding OCTLx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CR0W_AW::RESET)
    }
}
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
    #[must_use]
    pub fn cr0(&mut self) -> CR0_W<BC_SPEC, 0> {
        CR0_W::new(self)
    }
    #[doc = "Bit 1 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr1(&mut self) -> CR1_W<BC_SPEC, 1> {
        CR1_W::new(self)
    }
    #[doc = "Bit 2 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr2(&mut self) -> CR2_W<BC_SPEC, 2> {
        CR2_W::new(self)
    }
    #[doc = "Bit 3 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr3(&mut self) -> CR3_W<BC_SPEC, 3> {
        CR3_W::new(self)
    }
    #[doc = "Bit 4 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr4(&mut self) -> CR4_W<BC_SPEC, 4> {
        CR4_W::new(self)
    }
    #[doc = "Bit 5 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr5(&mut self) -> CR5_W<BC_SPEC, 5> {
        CR5_W::new(self)
    }
    #[doc = "Bit 6 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr6(&mut self) -> CR6_W<BC_SPEC, 6> {
        CR6_W::new(self)
    }
    #[doc = "Bit 7 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr7(&mut self) -> CR7_W<BC_SPEC, 7> {
        CR7_W::new(self)
    }
    #[doc = "Bit 8 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr8(&mut self) -> CR8_W<BC_SPEC, 8> {
        CR8_W::new(self)
    }
    #[doc = "Bit 9 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr9(&mut self) -> CR9_W<BC_SPEC, 9> {
        CR9_W::new(self)
    }
    #[doc = "Bit 10 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr10(&mut self) -> CR10_W<BC_SPEC, 10> {
        CR10_W::new(self)
    }
    #[doc = "Bit 11 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr11(&mut self) -> CR11_W<BC_SPEC, 11> {
        CR11_W::new(self)
    }
    #[doc = "Bit 12 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr12(&mut self) -> CR12_W<BC_SPEC, 12> {
        CR12_W::new(self)
    }
    #[doc = "Bit 13 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr13(&mut self) -> CR13_W<BC_SPEC, 13> {
        CR13_W::new(self)
    }
    #[doc = "Bit 14 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr14(&mut self) -> CR14_W<BC_SPEC, 14> {
        CR14_W::new(self)
    }
    #[doc = "Bit 15 - Port cleat bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr15(&mut self) -> CR15_W<BC_SPEC, 15> {
        CR15_W::new(self)
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
#[doc = "Port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BC_SPEC;
impl crate::RegisterSpec for BC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bc::W`](W) writer structure"]
impl crate::Writable for BC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BC to value 0"]
impl crate::Resettable for BC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
