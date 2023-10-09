#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DMAINTEN_SPEC>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DMAINTEN_SPEC>;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub type UPIE_R = crate::BitReader<UPIE_A>;
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPIE_A {
    #[doc = "0: Update interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Update interrupt enabled"]
    ENABLED = 1,
}
impl From<UPIE_A> for bool {
    #[inline(always)]
    fn from(variant: UPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPIE_A {
        match self.bits {
            false => UPIE_A::DISABLED,
            true => UPIE_A::ENABLED,
        }
    }
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPIE_A::DISABLED
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPIE_A::ENABLED
    }
}
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UPIE_A>;
impl<'a, REG, const O: u8> UPIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPIE_A::DISABLED)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPIE_A::ENABLED)
    }
}
#[doc = "Field `CH0IE` reader - Channel 0 Capture/Compare interrupt enable"]
pub type CH0IE_R = crate::BitReader<CH0IE_A>;
#[doc = "Channel 0 Capture/Compare interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0IE_A {
    #[doc = "0: Capture/compare interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Capture/compare interrupt enabled"]
    ENABLED = 1,
}
impl From<CH0IE_A> for bool {
    #[inline(always)]
    fn from(variant: CH0IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0IE_A {
        match self.bits {
            false => CH0IE_A::DISABLED,
            true => CH0IE_A::ENABLED,
        }
    }
    #[doc = "Capture/compare interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0IE_A::DISABLED
    }
    #[doc = "Capture/compare interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0IE_A::ENABLED
    }
}
#[doc = "Field `CH0IE` writer - Channel 0 Capture/Compare interrupt enable"]
pub type CH0IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0IE_A>;
impl<'a, REG, const O: u8> CH0IE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0IE_A::DISABLED)
    }
    #[doc = "Capture/compare interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0IE_A::ENABLED)
    }
}
#[doc = "Field `CH1IE` reader - Channel 1 Capture/Compare interrupt enable"]
pub use CH0IE_R as CH1IE_R;
#[doc = "Field `CH2IE` reader - Channel 2 Capture/Compare interrupt enable"]
pub use CH0IE_R as CH2IE_R;
#[doc = "Field `CH3IE` reader - Channel 3 Capture/Compare interrupt enable"]
pub use CH0IE_R as CH3IE_R;
#[doc = "Field `CH1IE` writer - Channel 1 Capture/Compare interrupt enable"]
pub use CH0IE_W as CH1IE_W;
#[doc = "Field `CH2IE` writer - Channel 2 Capture/Compare interrupt enable"]
pub use CH0IE_W as CH2IE_W;
#[doc = "Field `CH3IE` writer - Channel 3 Capture/Compare interrupt enable"]
pub use CH0IE_W as CH3IE_W;
#[doc = "Field `CMTIE` reader - CMT interrupt enable"]
pub type CMTIE_R = crate::BitReader<CMTIE_A>;
#[doc = "CMT interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMTIE_A {
    #[doc = "0: Commutation interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Commutation interrupt enabled"]
    ENABLED = 1,
}
impl From<CMTIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTIE_A {
        match self.bits {
            false => CMTIE_A::DISABLED,
            true => CMTIE_A::ENABLED,
        }
    }
    #[doc = "Commutation interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMTIE_A::DISABLED
    }
    #[doc = "Commutation interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMTIE_A::ENABLED
    }
}
#[doc = "Field `CMTIE` writer - CMT interrupt enable"]
pub type CMTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMTIE_A>;
impl<'a, REG, const O: u8> CMTIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Commutation interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMTIE_A::DISABLED)
    }
    #[doc = "Commutation interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMTIE_A::ENABLED)
    }
}
#[doc = "Field `TRGIE` reader - Trigger interrupt enable"]
pub type TRGIE_R = crate::BitReader<TRGIE_A>;
#[doc = "Trigger interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGIE_A {
    #[doc = "0: Trigger interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger interrupt enabled"]
    ENABLED = 1,
}
impl From<TRGIE_A> for bool {
    #[inline(always)]
    fn from(variant: TRGIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGIE_A {
        match self.bits {
            false => TRGIE_A::DISABLED,
            true => TRGIE_A::ENABLED,
        }
    }
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRGIE_A::DISABLED
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRGIE_A::ENABLED
    }
}
#[doc = "Field `TRGIE` writer - Trigger interrupt enable"]
pub type TRGIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRGIE_A>;
impl<'a, REG, const O: u8> TRGIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRGIE_A::DISABLED)
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRGIE_A::ENABLED)
    }
}
#[doc = "Field `BRKIE` reader - Break interrupt enable"]
pub type BRKIE_R = crate::BitReader<BRKIE_A>;
#[doc = "Break interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRKIE_A {
    #[doc = "0: Break interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Break interrupt enabled"]
    ENABLED = 1,
}
impl From<BRKIE_A> for bool {
    #[inline(always)]
    fn from(variant: BRKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BRKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKIE_A {
        match self.bits {
            false => BRKIE_A::DISABLED,
            true => BRKIE_A::ENABLED,
        }
    }
    #[doc = "Break interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRKIE_A::DISABLED
    }
    #[doc = "Break interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRKIE_A::ENABLED
    }
}
#[doc = "Field `BRKIE` writer - Break interrupt enable"]
pub type BRKIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BRKIE_A>;
impl<'a, REG, const O: u8> BRKIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRKIE_A::DISABLED)
    }
    #[doc = "Break interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRKIE_A::ENABLED)
    }
}
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UPDEN_R = crate::BitReader<UPDEN_A>;
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDEN_A {
    #[doc = "0: Update DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Update DMA request enabled"]
    ENABLED = 1,
}
impl From<UPDEN_A> for bool {
    #[inline(always)]
    fn from(variant: UPDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDEN_A {
        match self.bits {
            false => UPDEN_A::DISABLED,
            true => UPDEN_A::ENABLED,
        }
    }
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDEN_A::DISABLED
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDEN_A::ENABLED
    }
}
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UPDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UPDEN_A>;
impl<'a, REG, const O: u8> UPDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDEN_A::DISABLED)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDEN_A::ENABLED)
    }
}
#[doc = "Field `CH0DEN` reader - Channel 0 Capture/Compare DMA request enable"]
pub type CH0DEN_R = crate::BitReader<CH0DEN_A>;
#[doc = "Channel 0 Capture/Compare DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0DEN_A {
    #[doc = "0: Capture/compare DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Capture/compare DMA request enabled"]
    ENABLED = 1,
}
impl From<CH0DEN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0DEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0DEN_A {
        match self.bits {
            false => CH0DEN_A::DISABLED,
            true => CH0DEN_A::ENABLED,
        }
    }
    #[doc = "Capture/compare DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0DEN_A::DISABLED
    }
    #[doc = "Capture/compare DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0DEN_A::ENABLED
    }
}
#[doc = "Field `CH0DEN` writer - Channel 0 Capture/Compare DMA request enable"]
pub type CH0DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0DEN_A>;
impl<'a, REG, const O: u8> CH0DEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0DEN_A::DISABLED)
    }
    #[doc = "Capture/compare DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0DEN_A::ENABLED)
    }
}
#[doc = "Field `CH1DEN` reader - Channel 1 Capture/Compare DMA request enable"]
pub use CH0DEN_R as CH1DEN_R;
#[doc = "Field `CH2DEN` reader - Channel 2 Capture/Compare DMA request enable"]
pub use CH0DEN_R as CH2DEN_R;
#[doc = "Field `CH3DEN` reader - Channel 3 Capture/Compare DMA request enable"]
pub use CH0DEN_R as CH3DEN_R;
#[doc = "Field `CH1DEN` writer - Channel 1 Capture/Compare DMA request enable"]
pub use CH0DEN_W as CH1DEN_W;
#[doc = "Field `CH2DEN` writer - Channel 2 Capture/Compare DMA request enable"]
pub use CH0DEN_W as CH2DEN_W;
#[doc = "Field `CH3DEN` writer - Channel 3 Capture/Compare DMA request enable"]
pub use CH0DEN_W as CH3DEN_W;
#[doc = "Field `CMTDEN` reader - CMT DMA request enable"]
pub type CMTDEN_R = crate::BitReader<CMTDEN_A>;
#[doc = "CMT DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMTDEN_A {
    #[doc = "0: Commutation DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Commutation DMA request enabled"]
    ENABLED = 1,
}
impl From<CMTDEN_A> for bool {
    #[inline(always)]
    fn from(variant: CMTDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CMTDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTDEN_A {
        match self.bits {
            false => CMTDEN_A::DISABLED,
            true => CMTDEN_A::ENABLED,
        }
    }
    #[doc = "Commutation DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMTDEN_A::DISABLED
    }
    #[doc = "Commutation DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMTDEN_A::ENABLED
    }
}
#[doc = "Field `CMTDEN` writer - CMT DMA request enable"]
pub type CMTDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMTDEN_A>;
impl<'a, REG, const O: u8> CMTDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Commutation DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMTDEN_A::DISABLED)
    }
    #[doc = "Commutation DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMTDEN_A::ENABLED)
    }
}
#[doc = "Field `TRGDEN` reader - Trigger DMA request enable"]
pub type TRGDEN_R = crate::BitReader<TRGDEN_A>;
#[doc = "Trigger DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGDEN_A {
    #[doc = "0: Trigger DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger DMA request enabled"]
    ENABLED = 1,
}
impl From<TRGDEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRGDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGDEN_A {
        match self.bits {
            false => TRGDEN_A::DISABLED,
            true => TRGDEN_A::ENABLED,
        }
    }
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRGDEN_A::DISABLED
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRGDEN_A::ENABLED
    }
}
#[doc = "Field `TRGDEN` writer - Trigger DMA request enable"]
pub type TRGDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRGDEN_A>;
impl<'a, REG, const O: u8> TRGDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRGDEN_A::DISABLED)
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRGDEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> CH0IE_R {
        CH0IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&self) -> CH1IE_R {
        CH1IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch2ie(&self) -> CH2IE_R {
        CH2IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 3 Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch3ie(&self) -> CH3IE_R {
        CH3IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMT interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&self) -> CMTIE_R {
        CMTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&self) -> TRGIE_R {
        TRGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkie(&self) -> BRKIE_R {
        BRKIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UPDEN_R {
        UPDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 Capture/Compare DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&self) -> CH0DEN_R {
        CH0DEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 Capture/Compare DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&self) -> CH1DEN_R {
        CH1DEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 Capture/Compare DMA request enable"]
    #[inline(always)]
    pub fn ch2den(&self) -> CH2DEN_R {
        CH2DEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 Capture/Compare DMA request enable"]
    #[inline(always)]
    pub fn ch3den(&self) -> CH3DEN_R {
        CH3DEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CMT DMA request enable"]
    #[inline(always)]
    pub fn cmtden(&self) -> CMTDEN_R {
        CMTDEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&self) -> TRGDEN_R {
        TRGDEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<DMAINTEN_SPEC, 0> {
        UPIE_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 Capture/Compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ie(&mut self) -> CH0IE_W<DMAINTEN_SPEC, 1> {
        CH0IE_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 Capture/Compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ie(&mut self) -> CH1IE_W<DMAINTEN_SPEC, 2> {
        CH1IE_W::new(self)
    }
    #[doc = "Bit 3 - Channel 2 Capture/Compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ie(&mut self) -> CH2IE_W<DMAINTEN_SPEC, 3> {
        CH2IE_W::new(self)
    }
    #[doc = "Bit 4 - Channel 3 Capture/Compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ie(&mut self) -> CH3IE_W<DMAINTEN_SPEC, 4> {
        CH3IE_W::new(self)
    }
    #[doc = "Bit 5 - CMT interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtie(&mut self) -> CMTIE_W<DMAINTEN_SPEC, 5> {
        CMTIE_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgie(&mut self) -> TRGIE_W<DMAINTEN_SPEC, 6> {
        TRGIE_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn brkie(&mut self) -> BRKIE_W<DMAINTEN_SPEC, 7> {
        BRKIE_W::new(self)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn upden(&mut self) -> UPDEN_W<DMAINTEN_SPEC, 8> {
        UPDEN_W::new(self)
    }
    #[doc = "Bit 9 - Channel 0 Capture/Compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0den(&mut self) -> CH0DEN_W<DMAINTEN_SPEC, 9> {
        CH0DEN_W::new(self)
    }
    #[doc = "Bit 10 - Channel 1 Capture/Compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1den(&mut self) -> CH1DEN_W<DMAINTEN_SPEC, 10> {
        CH1DEN_W::new(self)
    }
    #[doc = "Bit 11 - Channel 2 Capture/Compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2den(&mut self) -> CH2DEN_W<DMAINTEN_SPEC, 11> {
        CH2DEN_W::new(self)
    }
    #[doc = "Bit 12 - Channel 3 Capture/Compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3den(&mut self) -> CH3DEN_W<DMAINTEN_SPEC, 12> {
        CH3DEN_W::new(self)
    }
    #[doc = "Bit 13 - CMT DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtden(&mut self) -> CMTDEN_W<DMAINTEN_SPEC, 13> {
        CMTDEN_W::new(self)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgden(&mut self) -> TRGDEN_W<DMAINTEN_SPEC, 14> {
        TRGDEN_W::new(self)
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
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmainten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAINTEN_SPEC;
impl crate::RegisterSpec for DMAINTEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DMAINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DMAINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DMAINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
