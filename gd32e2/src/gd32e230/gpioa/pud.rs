#[doc = "Register `PUD` reader"]
pub type R = crate::R<PUD_SPEC>;
#[doc = "Register `PUD` writer"]
pub type W = crate::W<PUD_SPEC>;
#[doc = "Field `PUD0` reader - Port x configuration bits (y = 0..15)"]
pub type PUD0_R = crate::FieldReader<PUD0_A>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUD0_A {
    #[doc = "0: No pull-up, pull-down (reset state)"]
    FLOATING = 0,
    #[doc = "1: Pull-up"]
    PULL_UP = 1,
    #[doc = "2: Pull-down"]
    PULL_DOWN = 2,
}
impl From<PUD0_A> for u8 {
    #[inline(always)]
    fn from(variant: PUD0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUD0_A {
    type Ux = u8;
}
impl PUD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PUD0_A> {
        match self.bits {
            0 => Some(PUD0_A::FLOATING),
            1 => Some(PUD0_A::PULL_UP),
            2 => Some(PUD0_A::PULL_DOWN),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down (reset state)"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PUD0_A::FLOATING
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PUD0_A::PULL_UP
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PUD0_A::PULL_DOWN
    }
}
#[doc = "Field `PUD0` writer - Port x configuration bits (y = 0..15)"]
pub type PUD0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, PUD0_A>;
impl<'a, REG, const O: u8> PUD0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down (reset state)"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut crate::W<REG> {
        self.variant(PUD0_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PUD0_A::PULL_UP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PUD0_A::PULL_DOWN)
    }
}
#[doc = "Field `PUD1` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD1_R;
#[doc = "Field `PUD2` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD2_R;
#[doc = "Field `PUD3` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD3_R;
#[doc = "Field `PUD4` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD4_R;
#[doc = "Field `PUD5` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD5_R;
#[doc = "Field `PUD6` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD6_R;
#[doc = "Field `PUD7` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD7_R;
#[doc = "Field `PUD8` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD8_R;
#[doc = "Field `PUD9` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD9_R;
#[doc = "Field `PUD10` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD10_R;
#[doc = "Field `PUD11` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD11_R;
#[doc = "Field `PUD12` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD12_R;
#[doc = "Field `PUD13` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD13_R;
#[doc = "Field `PUD14` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD14_R;
#[doc = "Field `PUD15` reader - Port x configuration bits (y = 0..15)"]
pub use PUD0_R as PUD15_R;
#[doc = "Field `PUD1` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD1_W;
#[doc = "Field `PUD2` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD2_W;
#[doc = "Field `PUD3` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD3_W;
#[doc = "Field `PUD4` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD4_W;
#[doc = "Field `PUD5` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD5_W;
#[doc = "Field `PUD6` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD6_W;
#[doc = "Field `PUD7` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD7_W;
#[doc = "Field `PUD8` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD8_W;
#[doc = "Field `PUD9` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD9_W;
#[doc = "Field `PUD10` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD10_W;
#[doc = "Field `PUD11` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD11_W;
#[doc = "Field `PUD12` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD12_W;
#[doc = "Field `PUD13` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD13_W;
#[doc = "Field `PUD14` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD14_W;
#[doc = "Field `PUD15` writer - Port x configuration bits (y = 0..15)"]
pub use PUD0_W as PUD15_W;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud0(&self) -> PUD0_R {
        PUD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud1(&self) -> PUD1_R {
        PUD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud2(&self) -> PUD2_R {
        PUD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud3(&self) -> PUD3_R {
        PUD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud4(&self) -> PUD4_R {
        PUD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud5(&self) -> PUD5_R {
        PUD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud6(&self) -> PUD6_R {
        PUD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud7(&self) -> PUD7_R {
        PUD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud8(&self) -> PUD8_R {
        PUD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud9(&self) -> PUD9_R {
        PUD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud10(&self) -> PUD10_R {
        PUD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud11(&self) -> PUD11_R {
        PUD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud12(&self) -> PUD12_R {
        PUD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud13(&self) -> PUD13_R {
        PUD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud14(&self) -> PUD14_R {
        PUD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud15(&self) -> PUD15_R {
        PUD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud0(&mut self) -> PUD0_W<PUD_SPEC, 0> {
        PUD0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud1(&mut self) -> PUD1_W<PUD_SPEC, 2> {
        PUD1_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud2(&mut self) -> PUD2_W<PUD_SPEC, 4> {
        PUD2_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud3(&mut self) -> PUD3_W<PUD_SPEC, 6> {
        PUD3_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud4(&mut self) -> PUD4_W<PUD_SPEC, 8> {
        PUD4_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud5(&mut self) -> PUD5_W<PUD_SPEC, 10> {
        PUD5_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud6(&mut self) -> PUD6_W<PUD_SPEC, 12> {
        PUD6_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud7(&mut self) -> PUD7_W<PUD_SPEC, 14> {
        PUD7_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud8(&mut self) -> PUD8_W<PUD_SPEC, 16> {
        PUD8_W::new(self)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud9(&mut self) -> PUD9_W<PUD_SPEC, 18> {
        PUD9_W::new(self)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud10(&mut self) -> PUD10_W<PUD_SPEC, 20> {
        PUD10_W::new(self)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud11(&mut self) -> PUD11_W<PUD_SPEC, 22> {
        PUD11_W::new(self)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud12(&mut self) -> PUD12_W<PUD_SPEC, 24> {
        PUD12_W::new(self)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud13(&mut self) -> PUD13_W<PUD_SPEC, 26> {
        PUD13_W::new(self)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud14(&mut self) -> PUD14_W<PUD_SPEC, 28> {
        PUD14_W::new(self)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pud15(&mut self) -> PUD15_W<PUD_SPEC, 30> {
        PUD15_W::new(self)
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
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pud::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pud::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUD_SPEC;
impl crate::RegisterSpec for PUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pud::R`](R) reader structure"]
impl crate::Readable for PUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pud::W`](W) writer structure"]
impl crate::Writable for PUD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUD to value 0x2400_0000"]
impl crate::Resettable for PUD_SPEC {
    const RESET_VALUE: Self::Ux = 0x2400_0000;
}
