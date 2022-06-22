#[doc = "Register `BOP` writer"]
pub struct W(crate::W<BOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOP_SPEC>;
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
impl From<crate::W<BOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port reset bit 15"]
pub use CR0_AW as CR15_AW;
#[doc = "Port reset bit 14"]
pub use CR0_AW as CR14_AW;
#[doc = "Port reset bit 13"]
pub use CR0_AW as CR13_AW;
#[doc = "Port reset bit 12"]
pub use CR0_AW as CR12_AW;
#[doc = "Port reset bit 11"]
pub use CR0_AW as CR11_AW;
#[doc = "Port reset bit 10"]
pub use CR0_AW as CR10_AW;
#[doc = "Port reset bit 9"]
pub use CR0_AW as CR9_AW;
#[doc = "Port reset bit 8"]
pub use CR0_AW as CR8_AW;
#[doc = "Port reset bit 7"]
pub use CR0_AW as CR7_AW;
#[doc = "Port reset bit 6"]
pub use CR0_AW as CR6_AW;
#[doc = "Port reset bit 5"]
pub use CR0_AW as CR5_AW;
#[doc = "Port reset bit 4"]
pub use CR0_AW as CR4_AW;
#[doc = "Port reset bit 3"]
pub use CR0_AW as CR3_AW;
#[doc = "Port reset bit 2"]
pub use CR0_AW as CR2_AW;
#[doc = "Port reset bit 1"]
pub use CR0_AW as CR1_AW;
#[doc = "Field `CR15` writer - Port reset bit 15"]
pub use CR0_W as CR15_W;
#[doc = "Field `CR14` writer - Port reset bit 14"]
pub use CR0_W as CR14_W;
#[doc = "Field `CR13` writer - Port reset bit 13"]
pub use CR0_W as CR13_W;
#[doc = "Field `CR12` writer - Port reset bit 12"]
pub use CR0_W as CR12_W;
#[doc = "Field `CR11` writer - Port reset bit 11"]
pub use CR0_W as CR11_W;
#[doc = "Field `CR10` writer - Port reset bit 10"]
pub use CR0_W as CR10_W;
#[doc = "Field `CR9` writer - Port reset bit 9"]
pub use CR0_W as CR9_W;
#[doc = "Field `CR8` writer - Port reset bit 8"]
pub use CR0_W as CR8_W;
#[doc = "Field `CR7` writer - Port reset bit 7"]
pub use CR0_W as CR7_W;
#[doc = "Field `CR6` writer - Port reset bit 6"]
pub use CR0_W as CR6_W;
#[doc = "Field `CR5` writer - Port reset bit 5"]
pub use CR0_W as CR5_W;
#[doc = "Field `CR4` writer - Port reset bit 4"]
pub use CR0_W as CR4_W;
#[doc = "Field `CR3` writer - Port reset bit 3"]
pub use CR0_W as CR3_W;
#[doc = "Field `CR2` writer - Port reset bit 2"]
pub use CR0_W as CR2_W;
#[doc = "Field `CR1` writer - Port reset bit 1"]
pub use CR0_W as CR1_W;
#[doc = "Port reset bit 0\n\nValue on reset: 0"]
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
#[doc = "Field `CR0` writer - Port reset bit 0"]
pub type CR0_W<'a> = crate::BitWriter<'a, u32, BOP_SPEC, CR0_AW, 16>;
impl<'a> CR0_W<'a> {
    #[doc = "Resets the corresponding OCTLx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CR0_AW::RESET)
    }
}
#[doc = "Port set bit15"]
pub use BOP0_AW as BOP15_AW;
#[doc = "Port set bit 14"]
pub use BOP0_AW as BOP14_AW;
#[doc = "Port set bit 13"]
pub use BOP0_AW as BOP13_AW;
#[doc = "Port set bit 12"]
pub use BOP0_AW as BOP12_AW;
#[doc = "Port set bit 11"]
pub use BOP0_AW as BOP11_AW;
#[doc = "Port set bit 10"]
pub use BOP0_AW as BOP10_AW;
#[doc = "Port set bit 9"]
pub use BOP0_AW as BOP9_AW;
#[doc = "Port set bit 8"]
pub use BOP0_AW as BOP8_AW;
#[doc = "Port set bit 7"]
pub use BOP0_AW as BOP7_AW;
#[doc = "Port set bit 6"]
pub use BOP0_AW as BOP6_AW;
#[doc = "Port set bit 5"]
pub use BOP0_AW as BOP5_AW;
#[doc = "Port set bit 4"]
pub use BOP0_AW as BOP4_AW;
#[doc = "Port set bit 3"]
pub use BOP0_AW as BOP3_AW;
#[doc = "Port set bit 2"]
pub use BOP0_AW as BOP2_AW;
#[doc = "Port set bit 1"]
pub use BOP0_AW as BOP1_AW;
#[doc = "Field `BOP15` writer - Port set bit15"]
pub use BOP0_W as BOP15_W;
#[doc = "Field `BOP14` writer - Port set bit 14"]
pub use BOP0_W as BOP14_W;
#[doc = "Field `BOP13` writer - Port set bit 13"]
pub use BOP0_W as BOP13_W;
#[doc = "Field `BOP12` writer - Port set bit 12"]
pub use BOP0_W as BOP12_W;
#[doc = "Field `BOP11` writer - Port set bit 11"]
pub use BOP0_W as BOP11_W;
#[doc = "Field `BOP10` writer - Port set bit 10"]
pub use BOP0_W as BOP10_W;
#[doc = "Field `BOP9` writer - Port set bit 9"]
pub use BOP0_W as BOP9_W;
#[doc = "Field `BOP8` writer - Port set bit 8"]
pub use BOP0_W as BOP8_W;
#[doc = "Field `BOP7` writer - Port set bit 7"]
pub use BOP0_W as BOP7_W;
#[doc = "Field `BOP6` writer - Port set bit 6"]
pub use BOP0_W as BOP6_W;
#[doc = "Field `BOP5` writer - Port set bit 5"]
pub use BOP0_W as BOP5_W;
#[doc = "Field `BOP4` writer - Port set bit 4"]
pub use BOP0_W as BOP4_W;
#[doc = "Field `BOP3` writer - Port set bit 3"]
pub use BOP0_W as BOP3_W;
#[doc = "Field `BOP2` writer - Port set bit 2"]
pub use BOP0_W as BOP2_W;
#[doc = "Field `BOP1` writer - Port set bit 1"]
pub use BOP0_W as BOP1_W;
#[doc = "Port set bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOP0_AW {
    #[doc = "1: Sets the corresponding OCTLx bit"]
    SET = 1,
}
impl From<BOP0_AW> for bool {
    #[inline(always)]
    fn from(variant: BOP0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOP0` writer - Port set bit 0"]
pub type BOP0_W<'a> = crate::BitWriter<'a, u32, BOP_SPEC, BOP0_AW, 0>;
impl<'a> BOP0_W<'a> {
    #[doc = "Sets the corresponding OCTLx bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BOP0_AW::SET)
    }
}
impl W {
    #[doc = "Bit 31 - Port reset bit 15"]
    #[inline(always)]
    pub fn cr15(&mut self) -> CR15_W {
        CR15_W::new(self)
    }
    #[doc = "Bit 30 - Port reset bit 14"]
    #[inline(always)]
    pub fn cr14(&mut self) -> CR14_W {
        CR14_W::new(self)
    }
    #[doc = "Bit 29 - Port reset bit 13"]
    #[inline(always)]
    pub fn cr13(&mut self) -> CR13_W {
        CR13_W::new(self)
    }
    #[doc = "Bit 28 - Port reset bit 12"]
    #[inline(always)]
    pub fn cr12(&mut self) -> CR12_W {
        CR12_W::new(self)
    }
    #[doc = "Bit 27 - Port reset bit 11"]
    #[inline(always)]
    pub fn cr11(&mut self) -> CR11_W {
        CR11_W::new(self)
    }
    #[doc = "Bit 26 - Port reset bit 10"]
    #[inline(always)]
    pub fn cr10(&mut self) -> CR10_W {
        CR10_W::new(self)
    }
    #[doc = "Bit 25 - Port reset bit 9"]
    #[inline(always)]
    pub fn cr9(&mut self) -> CR9_W {
        CR9_W::new(self)
    }
    #[doc = "Bit 24 - Port reset bit 8"]
    #[inline(always)]
    pub fn cr8(&mut self) -> CR8_W {
        CR8_W::new(self)
    }
    #[doc = "Bit 23 - Port reset bit 7"]
    #[inline(always)]
    pub fn cr7(&mut self) -> CR7_W {
        CR7_W::new(self)
    }
    #[doc = "Bit 22 - Port reset bit 6"]
    #[inline(always)]
    pub fn cr6(&mut self) -> CR6_W {
        CR6_W::new(self)
    }
    #[doc = "Bit 21 - Port reset bit 5"]
    #[inline(always)]
    pub fn cr5(&mut self) -> CR5_W {
        CR5_W::new(self)
    }
    #[doc = "Bit 20 - Port reset bit 4"]
    #[inline(always)]
    pub fn cr4(&mut self) -> CR4_W {
        CR4_W::new(self)
    }
    #[doc = "Bit 19 - Port reset bit 3"]
    #[inline(always)]
    pub fn cr3(&mut self) -> CR3_W {
        CR3_W::new(self)
    }
    #[doc = "Bit 18 - Port reset bit 2"]
    #[inline(always)]
    pub fn cr2(&mut self) -> CR2_W {
        CR2_W::new(self)
    }
    #[doc = "Bit 17 - Port reset bit 1"]
    #[inline(always)]
    pub fn cr1(&mut self) -> CR1_W {
        CR1_W::new(self)
    }
    #[doc = "Bit 16 - Port reset bit 0"]
    #[inline(always)]
    pub fn cr0(&mut self) -> CR0_W {
        CR0_W::new(self)
    }
    #[doc = "Bit 15 - Port set bit15"]
    #[inline(always)]
    pub fn bop15(&mut self) -> BOP15_W {
        BOP15_W::new(self)
    }
    #[doc = "Bit 14 - Port set bit 14"]
    #[inline(always)]
    pub fn bop14(&mut self) -> BOP14_W {
        BOP14_W::new(self)
    }
    #[doc = "Bit 13 - Port set bit 13"]
    #[inline(always)]
    pub fn bop13(&mut self) -> BOP13_W {
        BOP13_W::new(self)
    }
    #[doc = "Bit 12 - Port set bit 12"]
    #[inline(always)]
    pub fn bop12(&mut self) -> BOP12_W {
        BOP12_W::new(self)
    }
    #[doc = "Bit 11 - Port set bit 11"]
    #[inline(always)]
    pub fn bop11(&mut self) -> BOP11_W {
        BOP11_W::new(self)
    }
    #[doc = "Bit 10 - Port set bit 10"]
    #[inline(always)]
    pub fn bop10(&mut self) -> BOP10_W {
        BOP10_W::new(self)
    }
    #[doc = "Bit 9 - Port set bit 9"]
    #[inline(always)]
    pub fn bop9(&mut self) -> BOP9_W {
        BOP9_W::new(self)
    }
    #[doc = "Bit 8 - Port set bit 8"]
    #[inline(always)]
    pub fn bop8(&mut self) -> BOP8_W {
        BOP8_W::new(self)
    }
    #[doc = "Bit 7 - Port set bit 7"]
    #[inline(always)]
    pub fn bop7(&mut self) -> BOP7_W {
        BOP7_W::new(self)
    }
    #[doc = "Bit 6 - Port set bit 6"]
    #[inline(always)]
    pub fn bop6(&mut self) -> BOP6_W {
        BOP6_W::new(self)
    }
    #[doc = "Bit 5 - Port set bit 5"]
    #[inline(always)]
    pub fn bop5(&mut self) -> BOP5_W {
        BOP5_W::new(self)
    }
    #[doc = "Bit 4 - Port set bit 4"]
    #[inline(always)]
    pub fn bop4(&mut self) -> BOP4_W {
        BOP4_W::new(self)
    }
    #[doc = "Bit 3 - Port set bit 3"]
    #[inline(always)]
    pub fn bop3(&mut self) -> BOP3_W {
        BOP3_W::new(self)
    }
    #[doc = "Bit 2 - Port set bit 2"]
    #[inline(always)]
    pub fn bop2(&mut self) -> BOP2_W {
        BOP2_W::new(self)
    }
    #[doc = "Bit 1 - Port set bit 1"]
    #[inline(always)]
    pub fn bop1(&mut self) -> BOP1_W {
        BOP1_W::new(self)
    }
    #[doc = "Bit 0 - Port set bit 0"]
    #[inline(always)]
    pub fn bop0(&mut self) -> BOP0_W {
        BOP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bop](index.html) module"]
pub struct BOP_SPEC;
impl crate::RegisterSpec for BOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bop::W](W) writer structure"]
impl crate::Writable for BOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOP to value 0"]
impl crate::Resettable for BOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
