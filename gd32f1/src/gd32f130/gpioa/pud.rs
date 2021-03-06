#[doc = "Register `PUD` reader"]
pub struct R(crate::R<PUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUD` writer"]
pub struct W(crate::W<PUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUD_SPEC>;
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
impl From<crate::W<PUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pin 15 pull-up or pull-down bits"]
pub use PUD0_A as PUD15_A;
#[doc = "Pin 14 pull-up or pull-down bits"]
pub use PUD0_A as PUD14_A;
#[doc = "Pin 13 pull-up or pull-down bits"]
pub use PUD0_A as PUD13_A;
#[doc = "Pin 12 pull-up or pull-down bits"]
pub use PUD0_A as PUD12_A;
#[doc = "Pin 11 pull-up or pull-down bits"]
pub use PUD0_A as PUD11_A;
#[doc = "Pin 10 pull-up or pull-down bits"]
pub use PUD0_A as PUD10_A;
#[doc = "Pin 9 pull-up or pull-down bits"]
pub use PUD0_A as PUD9_A;
#[doc = "Pin 8 pull-up or pull-down bits"]
pub use PUD0_A as PUD8_A;
#[doc = "Pin 7 pull-up or pull-down bits"]
pub use PUD0_A as PUD7_A;
#[doc = "Pin 6 pull-up or pull-down bits"]
pub use PUD0_A as PUD6_A;
#[doc = "Pin 5 pull-up or pull-down bits"]
pub use PUD0_A as PUD5_A;
#[doc = "Pin 4 pull-up or pull-down bits"]
pub use PUD0_A as PUD4_A;
#[doc = "Pin 3 pull-up or pull-down bits"]
pub use PUD0_A as PUD3_A;
#[doc = "Pin 2 pull-up or pull-down bits"]
pub use PUD0_A as PUD2_A;
#[doc = "Pin 1 pull-up or pull-down bits"]
pub use PUD0_A as PUD1_A;
#[doc = "Field `PUD15` reader - Pin 15 pull-up or pull-down bits"]
pub use PUD0_R as PUD15_R;
#[doc = "Field `PUD14` reader - Pin 14 pull-up or pull-down bits"]
pub use PUD0_R as PUD14_R;
#[doc = "Field `PUD13` reader - Pin 13 pull-up or pull-down bits"]
pub use PUD0_R as PUD13_R;
#[doc = "Field `PUD12` reader - Pin 12 pull-up or pull-down bits"]
pub use PUD0_R as PUD12_R;
#[doc = "Field `PUD11` reader - Pin 11 pull-up or pull-down bits"]
pub use PUD0_R as PUD11_R;
#[doc = "Field `PUD10` reader - Pin 10 pull-up or pull-down bits"]
pub use PUD0_R as PUD10_R;
#[doc = "Field `PUD9` reader - Pin 9 pull-up or pull-down bits"]
pub use PUD0_R as PUD9_R;
#[doc = "Field `PUD8` reader - Pin 8 pull-up or pull-down bits"]
pub use PUD0_R as PUD8_R;
#[doc = "Field `PUD7` reader - Pin 7 pull-up or pull-down bits"]
pub use PUD0_R as PUD7_R;
#[doc = "Field `PUD6` reader - Pin 6 pull-up or pull-down bits"]
pub use PUD0_R as PUD6_R;
#[doc = "Field `PUD5` reader - Pin 5 pull-up or pull-down bits"]
pub use PUD0_R as PUD5_R;
#[doc = "Field `PUD4` reader - Pin 4 pull-up or pull-down bits"]
pub use PUD0_R as PUD4_R;
#[doc = "Field `PUD3` reader - Pin 3 pull-up or pull-down bits"]
pub use PUD0_R as PUD3_R;
#[doc = "Field `PUD2` reader - Pin 2 pull-up or pull-down bits"]
pub use PUD0_R as PUD2_R;
#[doc = "Field `PUD1` reader - Pin 1 pull-up or pull-down bits"]
pub use PUD0_R as PUD1_R;
#[doc = "Field `PUD15` writer - Pin 15 pull-up or pull-down bits"]
pub use PUD0_W as PUD15_W;
#[doc = "Field `PUD14` writer - Pin 14 pull-up or pull-down bits"]
pub use PUD0_W as PUD14_W;
#[doc = "Field `PUD13` writer - Pin 13 pull-up or pull-down bits"]
pub use PUD0_W as PUD13_W;
#[doc = "Field `PUD12` writer - Pin 12 pull-up or pull-down bits"]
pub use PUD0_W as PUD12_W;
#[doc = "Field `PUD11` writer - Pin 11 pull-up or pull-down bits"]
pub use PUD0_W as PUD11_W;
#[doc = "Field `PUD10` writer - Pin 10 pull-up or pull-down bits"]
pub use PUD0_W as PUD10_W;
#[doc = "Field `PUD9` writer - Pin 9 pull-up or pull-down bits"]
pub use PUD0_W as PUD9_W;
#[doc = "Field `PUD8` writer - Pin 8 pull-up or pull-down bits"]
pub use PUD0_W as PUD8_W;
#[doc = "Field `PUD7` writer - Pin 7 pull-up or pull-down bits"]
pub use PUD0_W as PUD7_W;
#[doc = "Field `PUD6` writer - Pin 6 pull-up or pull-down bits"]
pub use PUD0_W as PUD6_W;
#[doc = "Field `PUD5` writer - Pin 5 pull-up or pull-down bits"]
pub use PUD0_W as PUD5_W;
#[doc = "Field `PUD4` writer - Pin 4 pull-up or pull-down bits"]
pub use PUD0_W as PUD4_W;
#[doc = "Field `PUD3` writer - Pin 3 pull-up or pull-down bits"]
pub use PUD0_W as PUD3_W;
#[doc = "Field `PUD2` writer - Pin 2 pull-up or pull-down bits"]
pub use PUD0_W as PUD2_W;
#[doc = "Field `PUD1` writer - Pin 1 pull-up or pull-down bits"]
pub use PUD0_W as PUD1_W;
#[doc = "Pin 0 pull-up or pull-down bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PUD0_A {
    #[doc = "0: No pull-up, pull-down (reset state)"]
    FLOATING = 0,
    #[doc = "1: Pull-up"]
    PULLUP = 1,
    #[doc = "2: Pull-down"]
    PULLDOWN = 2,
}
impl From<PUD0_A> for u8 {
    #[inline(always)]
    fn from(variant: PUD0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PUD0` reader - Pin 0 pull-up or pull-down bits"]
pub type PUD0_R = crate::FieldReader<u8, PUD0_A>;
impl PUD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PUD0_A> {
        match self.bits {
            0 => Some(PUD0_A::FLOATING),
            1 => Some(PUD0_A::PULLUP),
            2 => Some(PUD0_A::PULLDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLOATING`"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PUD0_A::FLOATING
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PUD0_A::PULLUP
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PUD0_A::PULLDOWN
    }
}
#[doc = "Field `PUD0` writer - Pin 0 pull-up or pull-down bits"]
pub type PUD0_W<'a> = crate::FieldWriter<'a, u32, PUD_SPEC, u8, PUD0_A, 2, 0>;
impl<'a> PUD0_W<'a> {
    #[doc = "No pull-up, pull-down (reset state)"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUD0_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUD0_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUD0_A::PULLDOWN)
    }
}
impl R {
    #[doc = "Bits 30:31 - Pin 15 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud15(&self) -> PUD15_R {
        PUD15_R::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Pin 14 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud14(&self) -> PUD14_R {
        PUD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin 13 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud13(&self) -> PUD13_R {
        PUD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pin 12 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud12(&self) -> PUD12_R {
        PUD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin 11 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud11(&self) -> PUD11_R {
        PUD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin 10 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud10(&self) -> PUD10_R {
        PUD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin 9 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud9(&self) -> PUD9_R {
        PUD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin 8 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud8(&self) -> PUD8_R {
        PUD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin 7 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud7(&self) -> PUD7_R {
        PUD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin 6 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud6(&self) -> PUD6_R {
        PUD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin 5 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud5(&self) -> PUD5_R {
        PUD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin 4 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud4(&self) -> PUD4_R {
        PUD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin 3 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud3(&self) -> PUD3_R {
        PUD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin 2 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud2(&self) -> PUD2_R {
        PUD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin 1 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud1(&self) -> PUD1_R {
        PUD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Pin 0 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud0(&self) -> PUD0_R {
        PUD0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Pin 15 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud15(&mut self) -> PUD15_W {
        PUD15_W::new(self)
    }
    #[doc = "Bits 28:29 - Pin 14 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud14(&mut self) -> PUD14_W {
        PUD14_W::new(self)
    }
    #[doc = "Bits 26:27 - Pin 13 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud13(&mut self) -> PUD13_W {
        PUD13_W::new(self)
    }
    #[doc = "Bits 24:25 - Pin 12 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud12(&mut self) -> PUD12_W {
        PUD12_W::new(self)
    }
    #[doc = "Bits 22:23 - Pin 11 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud11(&mut self) -> PUD11_W {
        PUD11_W::new(self)
    }
    #[doc = "Bits 20:21 - Pin 10 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud10(&mut self) -> PUD10_W {
        PUD10_W::new(self)
    }
    #[doc = "Bits 18:19 - Pin 9 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud9(&mut self) -> PUD9_W {
        PUD9_W::new(self)
    }
    #[doc = "Bits 16:17 - Pin 8 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud8(&mut self) -> PUD8_W {
        PUD8_W::new(self)
    }
    #[doc = "Bits 14:15 - Pin 7 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud7(&mut self) -> PUD7_W {
        PUD7_W::new(self)
    }
    #[doc = "Bits 12:13 - Pin 6 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud6(&mut self) -> PUD6_W {
        PUD6_W::new(self)
    }
    #[doc = "Bits 10:11 - Pin 5 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud5(&mut self) -> PUD5_W {
        PUD5_W::new(self)
    }
    #[doc = "Bits 8:9 - Pin 4 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud4(&mut self) -> PUD4_W {
        PUD4_W::new(self)
    }
    #[doc = "Bits 6:7 - Pin 3 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud3(&mut self) -> PUD3_W {
        PUD3_W::new(self)
    }
    #[doc = "Bits 4:5 - Pin 2 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud2(&mut self) -> PUD2_W {
        PUD2_W::new(self)
    }
    #[doc = "Bits 2:3 - Pin 1 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud1(&mut self) -> PUD1_W {
        PUD1_W::new(self)
    }
    #[doc = "Bits 0:1 - Pin 0 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud0(&mut self) -> PUD0_W {
        PUD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pud](index.html) module"]
pub struct PUD_SPEC;
impl crate::RegisterSpec for PUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pud::R](R) reader structure"]
impl crate::Readable for PUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pud::W](W) writer structure"]
impl crate::Writable for PUD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUD to value 0x2400_0000"]
impl crate::Resettable for PUD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2400_0000
    }
}
