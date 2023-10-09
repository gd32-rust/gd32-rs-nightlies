#[doc = "Register `BOP` writer"]
pub type W = crate::W<BOP_SPEC>;
#[doc = "Port 0 Set bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOP0W_AW {
    #[doc = "1: Sets the corresponding OCTLx bit"]
    SET = 1,
}
impl From<BOP0W_AW> for bool {
    #[inline(always)]
    fn from(variant: BOP0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOP0` writer - Port 0 Set bit"]
pub type BOP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BOP0W_AW>;
impl<'a, REG, const O: u8> BOP0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sets the corresponding OCTLx bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(BOP0W_AW::SET)
    }
}
#[doc = "Field `BOP1` writer - Port 1 Set bit"]
pub use BOP0_W as BOP1_W;
#[doc = "Field `BOP2` writer - Port 2 Set bit"]
pub use BOP0_W as BOP2_W;
#[doc = "Field `BOP3` writer - Port 3 Set bit"]
pub use BOP0_W as BOP3_W;
#[doc = "Field `BOP4` writer - Port 4 Set bit"]
pub use BOP0_W as BOP4_W;
#[doc = "Field `BOP5` writer - Port 5 Set bit"]
pub use BOP0_W as BOP5_W;
#[doc = "Field `BOP6` writer - Port 6 Set bit"]
pub use BOP0_W as BOP6_W;
#[doc = "Field `BOP7` writer - Port 7 Set bit"]
pub use BOP0_W as BOP7_W;
#[doc = "Field `BOP8` writer - Port 8 Set bit"]
pub use BOP0_W as BOP8_W;
#[doc = "Field `BOP9` writer - Port 9 Set bit"]
pub use BOP0_W as BOP9_W;
#[doc = "Field `BOP10` writer - Port 10 Set bit"]
pub use BOP0_W as BOP10_W;
#[doc = "Field `BOP11` writer - Port 11 Set bit"]
pub use BOP0_W as BOP11_W;
#[doc = "Field `BOP12` writer - Port 12 Set bit"]
pub use BOP0_W as BOP12_W;
#[doc = "Field `BOP13` writer - Port 13 Set bit"]
pub use BOP0_W as BOP13_W;
#[doc = "Field `BOP14` writer - Port 14 Set bit"]
pub use BOP0_W as BOP14_W;
#[doc = "Field `BOP15` writer - Port 15 Set bit"]
pub use BOP0_W as BOP15_W;
#[doc = "Port 0 Clear bit\n\nValue on reset: 0"]
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
#[doc = "Field `CR0` writer - Port 0 Clear bit"]
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
#[doc = "Field `CR1` writer - Port 1 Clear bit"]
pub use CR0_W as CR1_W;
#[doc = "Field `CR2` writer - Port 2 Clear bit"]
pub use CR0_W as CR2_W;
#[doc = "Field `CR3` writer - Port 3 Clear bit"]
pub use CR0_W as CR3_W;
#[doc = "Field `CR4` writer - Port 4 Clear bit"]
pub use CR0_W as CR4_W;
#[doc = "Field `CR5` writer - Port 5 Clear bit"]
pub use CR0_W as CR5_W;
#[doc = "Field `CR6` writer - Port 6 Clear bit"]
pub use CR0_W as CR6_W;
#[doc = "Field `CR7` writer - Port 7 Clear bit"]
pub use CR0_W as CR7_W;
#[doc = "Field `CR8` writer - Port 8 Clear bit"]
pub use CR0_W as CR8_W;
#[doc = "Field `CR9` writer - Port 9 Clear bit"]
pub use CR0_W as CR9_W;
#[doc = "Field `CR10` writer - Port 10 Clear bit"]
pub use CR0_W as CR10_W;
#[doc = "Field `CR11` writer - Port 11 Clear bit"]
pub use CR0_W as CR11_W;
#[doc = "Field `CR12` writer - Port 12 Clear bit"]
pub use CR0_W as CR12_W;
#[doc = "Field `CR13` writer - Port 13 Clear bit"]
pub use CR0_W as CR13_W;
#[doc = "Field `CR14` writer - Port 14 Clear bit"]
pub use CR0_W as CR14_W;
#[doc = "Field `CR15` writer - Port 15 Clear bit"]
pub use CR0_W as CR15_W;
impl W {
    #[doc = "Bit 0 - Port 0 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop0(&mut self) -> BOP0_W<BOP_SPEC, 0> {
        BOP0_W::new(self)
    }
    #[doc = "Bit 1 - Port 1 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop1(&mut self) -> BOP1_W<BOP_SPEC, 1> {
        BOP1_W::new(self)
    }
    #[doc = "Bit 2 - Port 2 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop2(&mut self) -> BOP2_W<BOP_SPEC, 2> {
        BOP2_W::new(self)
    }
    #[doc = "Bit 3 - Port 3 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop3(&mut self) -> BOP3_W<BOP_SPEC, 3> {
        BOP3_W::new(self)
    }
    #[doc = "Bit 4 - Port 4 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop4(&mut self) -> BOP4_W<BOP_SPEC, 4> {
        BOP4_W::new(self)
    }
    #[doc = "Bit 5 - Port 5 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop5(&mut self) -> BOP5_W<BOP_SPEC, 5> {
        BOP5_W::new(self)
    }
    #[doc = "Bit 6 - Port 6 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop6(&mut self) -> BOP6_W<BOP_SPEC, 6> {
        BOP6_W::new(self)
    }
    #[doc = "Bit 7 - Port 7 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop7(&mut self) -> BOP7_W<BOP_SPEC, 7> {
        BOP7_W::new(self)
    }
    #[doc = "Bit 8 - Port 8 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop8(&mut self) -> BOP8_W<BOP_SPEC, 8> {
        BOP8_W::new(self)
    }
    #[doc = "Bit 9 - Port 9 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop9(&mut self) -> BOP9_W<BOP_SPEC, 9> {
        BOP9_W::new(self)
    }
    #[doc = "Bit 10 - Port 10 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop10(&mut self) -> BOP10_W<BOP_SPEC, 10> {
        BOP10_W::new(self)
    }
    #[doc = "Bit 11 - Port 11 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop11(&mut self) -> BOP11_W<BOP_SPEC, 11> {
        BOP11_W::new(self)
    }
    #[doc = "Bit 12 - Port 12 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop12(&mut self) -> BOP12_W<BOP_SPEC, 12> {
        BOP12_W::new(self)
    }
    #[doc = "Bit 13 - Port 13 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop13(&mut self) -> BOP13_W<BOP_SPEC, 13> {
        BOP13_W::new(self)
    }
    #[doc = "Bit 14 - Port 14 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop14(&mut self) -> BOP14_W<BOP_SPEC, 14> {
        BOP14_W::new(self)
    }
    #[doc = "Bit 15 - Port 15 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop15(&mut self) -> BOP15_W<BOP_SPEC, 15> {
        BOP15_W::new(self)
    }
    #[doc = "Bit 16 - Port 0 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr0(&mut self) -> CR0_W<BOP_SPEC, 16> {
        CR0_W::new(self)
    }
    #[doc = "Bit 17 - Port 1 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr1(&mut self) -> CR1_W<BOP_SPEC, 17> {
        CR1_W::new(self)
    }
    #[doc = "Bit 18 - Port 2 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr2(&mut self) -> CR2_W<BOP_SPEC, 18> {
        CR2_W::new(self)
    }
    #[doc = "Bit 19 - Port 3 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr3(&mut self) -> CR3_W<BOP_SPEC, 19> {
        CR3_W::new(self)
    }
    #[doc = "Bit 20 - Port 4 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr4(&mut self) -> CR4_W<BOP_SPEC, 20> {
        CR4_W::new(self)
    }
    #[doc = "Bit 21 - Port 5 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr5(&mut self) -> CR5_W<BOP_SPEC, 21> {
        CR5_W::new(self)
    }
    #[doc = "Bit 22 - Port 6 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr6(&mut self) -> CR6_W<BOP_SPEC, 22> {
        CR6_W::new(self)
    }
    #[doc = "Bit 23 - Port 7 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr7(&mut self) -> CR7_W<BOP_SPEC, 23> {
        CR7_W::new(self)
    }
    #[doc = "Bit 24 - Port 8 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr8(&mut self) -> CR8_W<BOP_SPEC, 24> {
        CR8_W::new(self)
    }
    #[doc = "Bit 25 - Port 9 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr9(&mut self) -> CR9_W<BOP_SPEC, 25> {
        CR9_W::new(self)
    }
    #[doc = "Bit 26 - Port 10 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr10(&mut self) -> CR10_W<BOP_SPEC, 26> {
        CR10_W::new(self)
    }
    #[doc = "Bit 27 - Port 11 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr11(&mut self) -> CR11_W<BOP_SPEC, 27> {
        CR11_W::new(self)
    }
    #[doc = "Bit 28 - Port 12 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr12(&mut self) -> CR12_W<BOP_SPEC, 28> {
        CR12_W::new(self)
    }
    #[doc = "Bit 29 - Port 13 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr13(&mut self) -> CR13_W<BOP_SPEC, 29> {
        CR13_W::new(self)
    }
    #[doc = "Bit 30 - Port 14 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr14(&mut self) -> CR14_W<BOP_SPEC, 30> {
        CR14_W::new(self)
    }
    #[doc = "Bit 31 - Port 15 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr15(&mut self) -> CR15_W<BOP_SPEC, 31> {
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
#[doc = "Port bit operate register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOP_SPEC;
impl crate::RegisterSpec for BOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bop::W`](W) writer structure"]
impl crate::Writable for BOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOP to value 0"]
impl crate::Resettable for BOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
