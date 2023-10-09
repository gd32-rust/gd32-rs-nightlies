#[doc = "Register `CH4CTL` reader"]
pub type R = crate::R<CH4CTL_SPEC>;
#[doc = "Register `CH4CTL` writer"]
pub type W = crate::W<CH4CTL_SPEC>;
#[doc = "Field `CHEN` reader - Channel enable"]
pub type CHEN_R = crate::BitReader<CHEN_A>;
#[doc = "Channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHEN_A {
    #[doc = "0: Channel disabled"]
    DISABLED = 0,
    #[doc = "1: Channel enabled"]
    ENABLED = 1,
}
impl From<CHEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN_A {
        match self.bits {
            false => CHEN_A::DISABLED,
            true => CHEN_A::ENABLED,
        }
    }
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHEN_A::DISABLED
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHEN_A::ENABLED
    }
}
#[doc = "Field `CHEN` writer - Channel enable"]
pub type CHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHEN_A>;
impl<'a, REG, const O: u8> CHEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHEN_A::DISABLED)
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHEN_A::ENABLED)
    }
}
#[doc = "Field `FTFIE` reader - Enable bit for full transfer finish interrupt"]
pub type FTFIE_R = crate::BitReader<FTFIE_A>;
#[doc = "Enable bit for full transfer finish interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTFIE_A {
    #[doc = "0: Full transfer interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Full transfer interrupt enabled"]
    ENABLED = 1,
}
impl From<FTFIE_A> for bool {
    #[inline(always)]
    fn from(variant: FTFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FTFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTFIE_A {
        match self.bits {
            false => FTFIE_A::DISABLED,
            true => FTFIE_A::ENABLED,
        }
    }
    #[doc = "Full transfer interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FTFIE_A::DISABLED
    }
    #[doc = "Full transfer interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FTFIE_A::ENABLED
    }
}
#[doc = "Field `FTFIE` writer - Enable bit for full transfer finish interrupt"]
pub type FTFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FTFIE_A>;
impl<'a, REG, const O: u8> FTFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full transfer interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FTFIE_A::DISABLED)
    }
    #[doc = "Full transfer interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FTFIE_A::ENABLED)
    }
}
#[doc = "Field `HTFIE` reader - Enable bit for half transfer finish interrupt"]
pub type HTFIE_R = crate::BitReader<HTFIE_A>;
#[doc = "Enable bit for half transfer finish interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTFIE_A {
    #[doc = "0: Half transfer interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Half transfer interrupt enabled"]
    ENABLED = 1,
}
impl From<HTFIE_A> for bool {
    #[inline(always)]
    fn from(variant: HTFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HTFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTFIE_A {
        match self.bits {
            false => HTFIE_A::DISABLED,
            true => HTFIE_A::ENABLED,
        }
    }
    #[doc = "Half transfer interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTFIE_A::DISABLED
    }
    #[doc = "Half transfer interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTFIE_A::ENABLED
    }
}
#[doc = "Field `HTFIE` writer - Enable bit for half transfer finish interrupt"]
pub type HTFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HTFIE_A>;
impl<'a, REG, const O: u8> HTFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half transfer interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HTFIE_A::DISABLED)
    }
    #[doc = "Half transfer interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HTFIE_A::ENABLED)
    }
}
#[doc = "Field `ERRIE` reader - Enable bit for tranfer access error interrupt"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
#[doc = "Enable bit for tranfer access error interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: Transfer error interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Transfer error interrupt enabled"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Transfer error interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Transfer error interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Enable bit for tranfer access error interrupt"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERRIE_A>;
impl<'a, REG, const O: u8> ERRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Transfer error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::ENABLED)
    }
}
#[doc = "Field `DIR` reader - Transfer mode"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Read from peripheral"]
    FROM_PERIPHERAL = 0,
    #[doc = "1: Read from memory"]
    FROM_MEMORY = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::FROM_PERIPHERAL,
            true => DIR_A::FROM_MEMORY,
        }
    }
    #[doc = "Read from peripheral"]
    #[inline(always)]
    pub fn is_from_peripheral(&self) -> bool {
        *self == DIR_A::FROM_PERIPHERAL
    }
    #[doc = "Read from memory"]
    #[inline(always)]
    pub fn is_from_memory(&self) -> bool {
        *self == DIR_A::FROM_MEMORY
    }
}
#[doc = "Field `DIR` writer - Transfer mode"]
pub type DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DIR_A>;
impl<'a, REG, const O: u8> DIR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from peripheral"]
    #[inline(always)]
    pub fn from_peripheral(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::FROM_PERIPHERAL)
    }
    #[doc = "Read from memory"]
    #[inline(always)]
    pub fn from_memory(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::FROM_MEMORY)
    }
}
#[doc = "Field `CMEN` reader - Circular mode enable"]
pub type CMEN_R = crate::BitReader<CMEN_A>;
#[doc = "Circular mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMEN_A {
    #[doc = "0: Circular buffer disabled"]
    DISABLED = 0,
    #[doc = "1: Circular buffer enabled"]
    ENABLED = 1,
}
impl From<CMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMEN_A {
        match self.bits {
            false => CMEN_A::DISABLED,
            true => CMEN_A::ENABLED,
        }
    }
    #[doc = "Circular buffer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMEN_A::DISABLED
    }
    #[doc = "Circular buffer enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMEN_A::ENABLED
    }
}
#[doc = "Field `CMEN` writer - Circular mode enable"]
pub type CMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMEN_A>;
impl<'a, REG, const O: u8> CMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Circular buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMEN_A::DISABLED)
    }
    #[doc = "Circular buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMEN_A::ENABLED)
    }
}
#[doc = "Field `PNAGA` reader - Next address generation algorithm of peripheral"]
pub type PNAGA_R = crate::BitReader<PNAGA_A>;
#[doc = "Next address generation algorithm of peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PNAGA_A {
    #[doc = "0: Fixed address mode"]
    FIXED = 0,
    #[doc = "1: Increment address mode"]
    INCREMENT = 1,
}
impl From<PNAGA_A> for bool {
    #[inline(always)]
    fn from(variant: PNAGA_A) -> Self {
        variant as u8 != 0
    }
}
impl PNAGA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PNAGA_A {
        match self.bits {
            false => PNAGA_A::FIXED,
            true => PNAGA_A::INCREMENT,
        }
    }
    #[doc = "Fixed address mode"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == PNAGA_A::FIXED
    }
    #[doc = "Increment address mode"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == PNAGA_A::INCREMENT
    }
}
#[doc = "Field `PNAGA` writer - Next address generation algorithm of peripheral"]
pub type PNAGA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PNAGA_A>;
impl<'a, REG, const O: u8> PNAGA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed address mode"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(PNAGA_A::FIXED)
    }
    #[doc = "Increment address mode"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(PNAGA_A::INCREMENT)
    }
}
#[doc = "Field `MNAGA` reader - Next address generation algorithm of memory"]
pub use PNAGA_R as MNAGA_R;
#[doc = "Field `MNAGA` writer - Next address generation algorithm of memory"]
pub use PNAGA_W as MNAGA_W;
#[doc = "Field `PWIDTH` reader - Transfer data size of peripheral"]
pub type PWIDTH_R = crate::FieldReader<PWIDTH_A>;
#[doc = "Transfer data size of peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWIDTH_A {
    #[doc = "0: 8-bit size"]
    BITS8 = 0,
    #[doc = "1: 16-bit size"]
    BITS16 = 1,
    #[doc = "2: 32-bit size"]
    BITS32 = 2,
}
impl From<PWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: PWIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWIDTH_A {
    type Ux = u8;
}
impl PWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWIDTH_A> {
        match self.bits {
            0 => Some(PWIDTH_A::BITS8),
            1 => Some(PWIDTH_A::BITS16),
            2 => Some(PWIDTH_A::BITS32),
            _ => None,
        }
    }
    #[doc = "8-bit size"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PWIDTH_A::BITS8
    }
    #[doc = "16-bit size"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PWIDTH_A::BITS16
    }
    #[doc = "32-bit size"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PWIDTH_A::BITS32
    }
}
#[doc = "Field `PWIDTH` writer - Transfer data size of peripheral"]
pub type PWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, PWIDTH_A>;
impl<'a, REG, const O: u8> PWIDTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit size"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(PWIDTH_A::BITS8)
    }
    #[doc = "16-bit size"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(PWIDTH_A::BITS16)
    }
    #[doc = "32-bit size"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(PWIDTH_A::BITS32)
    }
}
#[doc = "Field `MWIDTH` reader - Transfer data size of memory"]
pub use PWIDTH_R as MWIDTH_R;
#[doc = "Field `MWIDTH` writer - Transfer data size of memory"]
pub use PWIDTH_W as MWIDTH_W;
#[doc = "Field `PRIO` reader - Priority Level of this channel"]
pub type PRIO_R = crate::FieldReader<PRIO_A>;
#[doc = "Priority Level of this channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIO_A {
    #[doc = "0: Low priority"]
    LOW = 0,
    #[doc = "1: Medium priority"]
    MEDIUM = 1,
    #[doc = "2: High priority"]
    HIGH = 2,
    #[doc = "3: Very high priority"]
    VERY_HIGH = 3,
}
impl From<PRIO_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIO_A {
    type Ux = u8;
}
impl PRIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIO_A {
        match self.bits {
            0 => PRIO_A::LOW,
            1 => PRIO_A::MEDIUM,
            2 => PRIO_A::HIGH,
            3 => PRIO_A::VERY_HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Low priority"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PRIO_A::LOW
    }
    #[doc = "Medium priority"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PRIO_A::MEDIUM
    }
    #[doc = "High priority"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PRIO_A::HIGH
    }
    #[doc = "Very high priority"]
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == PRIO_A::VERY_HIGH
    }
}
#[doc = "Field `PRIO` writer - Priority Level of this channel"]
pub type PRIO_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PRIO_A>;
impl<'a, REG, const O: u8> PRIO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low priority"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO_A::LOW)
    }
    #[doc = "Medium priority"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO_A::MEDIUM)
    }
    #[doc = "High priority"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO_A::HIGH)
    }
    #[doc = "Very high priority"]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO_A::VERY_HIGH)
    }
}
#[doc = "Field `M2M` reader - Memory to memory mode"]
pub type M2M_R = crate::BitReader<M2M_A>;
#[doc = "Memory to memory mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M2M_A {
    #[doc = "0: Memory to memory mode disabled"]
    DISABLED = 0,
    #[doc = "1: Memory to memory mode enabled"]
    ENABLED = 1,
}
impl From<M2M_A> for bool {
    #[inline(always)]
    fn from(variant: M2M_A) -> Self {
        variant as u8 != 0
    }
}
impl M2M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M2M_A {
        match self.bits {
            false => M2M_A::DISABLED,
            true => M2M_A::ENABLED,
        }
    }
    #[doc = "Memory to memory mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M2M_A::DISABLED
    }
    #[doc = "Memory to memory mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M2M_A::ENABLED
    }
}
#[doc = "Field `M2M` writer - Memory to memory mode"]
pub type M2M_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, M2M_A>;
impl<'a, REG, const O: u8> M2M_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory to memory mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M2M_A::DISABLED)
    }
    #[doc = "Memory to memory mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M2M_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable bit for full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&self) -> FTFIE_R {
        FTFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for half transfer finish interrupt"]
    #[inline(always)]
    pub fn htfie(&self) -> HTFIE_R {
        HTFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable bit for tranfer access error interrupt"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer mode"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    pub fn cmen(&self) -> CMEN_R {
        CMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&self) -> PNAGA_R {
        PNAGA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&self) -> MNAGA_R {
        MNAGA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    pub fn pwidth(&self) -> PWIDTH_R {
        PWIDTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    pub fn mwidth(&self) -> MWIDTH_R {
        MWIDTH_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Priority Level of this channel"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn m2m(&self) -> M2M_R {
        M2M_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<CH4CTL_SPEC, 0> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable bit for full transfer finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ftfie(&mut self) -> FTFIE_W<CH4CTL_SPEC, 1> {
        FTFIE_W::new(self)
    }
    #[doc = "Bit 2 - Enable bit for half transfer finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn htfie(&mut self) -> HTFIE_W<CH4CTL_SPEC, 2> {
        HTFIE_W::new(self)
    }
    #[doc = "Bit 3 - Enable bit for tranfer access error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CH4CTL_SPEC, 3> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 4 - Transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CH4CTL_SPEC, 4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmen(&mut self) -> CMEN_W<CH4CTL_SPEC, 5> {
        CMEN_W::new(self)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn pnaga(&mut self) -> PNAGA_W<CH4CTL_SPEC, 6> {
        PNAGA_W::new(self)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    #[must_use]
    pub fn mnaga(&mut self) -> MNAGA_W<CH4CTL_SPEC, 7> {
        MNAGA_W::new(self)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PWIDTH_W<CH4CTL_SPEC, 8> {
        PWIDTH_W::new(self)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MWIDTH_W<CH4CTL_SPEC, 10> {
        MWIDTH_W::new(self)
    }
    #[doc = "Bits 12:13 - Priority Level of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<CH4CTL_SPEC, 12> {
        PRIO_W::new(self)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn m2m(&mut self) -> M2M_W<CH4CTL_SPEC, 14> {
        M2M_W::new(self)
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
#[doc = "DMA channel configuration register (DMA_CH4CTL0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4CTL_SPEC;
impl crate::RegisterSpec for CH4CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4ctl::R`](R) reader structure"]
impl crate::Readable for CH4CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch4ctl::W`](W) writer structure"]
impl crate::Writable for CH4CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4CTL to value 0"]
impl crate::Resettable for CH4CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
