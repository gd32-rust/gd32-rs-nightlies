#[doc = "Register `INT` reader"]
pub type R = crate::R<INT_SPEC>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<INT_SPEC>;
#[doc = "Field `IRC40KSTBIF` reader - IRC40K stabilization interrupt flag"]
pub type IRC40KSTBIF_R = crate::BitReader<IRC40KSTBIFR_A>;
#[doc = "IRC40K stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRC40KSTBIFR_A {
    #[doc = "0: No interrupt generated"]
    NOT_INTERRUPTED = 0,
    #[doc = "1: IRC40K stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<IRC40KSTBIFR_A> for bool {
    #[inline(always)]
    fn from(variant: IRC40KSTBIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl IRC40KSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC40KSTBIFR_A {
        match self.bits {
            false => IRC40KSTBIFR_A::NOT_INTERRUPTED,
            true => IRC40KSTBIFR_A::INTERRUPTED,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == IRC40KSTBIFR_A::NOT_INTERRUPTED
    }
    #[doc = "IRC40K stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == IRC40KSTBIFR_A::INTERRUPTED
    }
}
#[doc = "Field `LXTALSTBIF` reader - LXTAL stabilization interrupt flag"]
pub type LXTALSTBIF_R = crate::BitReader<LXTALSTBIFR_A>;
#[doc = "LXTAL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LXTALSTBIFR_A {
    #[doc = "0: No interrupt generated"]
    NOT_INTERRUPTED = 0,
    #[doc = "1: LXTAL stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<LXTALSTBIFR_A> for bool {
    #[inline(always)]
    fn from(variant: LXTALSTBIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl LXTALSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALSTBIFR_A {
        match self.bits {
            false => LXTALSTBIFR_A::NOT_INTERRUPTED,
            true => LXTALSTBIFR_A::INTERRUPTED,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LXTALSTBIFR_A::NOT_INTERRUPTED
    }
    #[doc = "LXTAL stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LXTALSTBIFR_A::INTERRUPTED
    }
}
#[doc = "Field `IRC8MSTBIF` reader - IRC8M stabilization interrupt flag"]
pub type IRC8MSTBIF_R = crate::BitReader<IRC8MSTBIFR_A>;
#[doc = "IRC8M stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRC8MSTBIFR_A {
    #[doc = "0: No interrupt generated"]
    NOT_INTERRUPTED = 0,
    #[doc = "1: IRC8M stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<IRC8MSTBIFR_A> for bool {
    #[inline(always)]
    fn from(variant: IRC8MSTBIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl IRC8MSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC8MSTBIFR_A {
        match self.bits {
            false => IRC8MSTBIFR_A::NOT_INTERRUPTED,
            true => IRC8MSTBIFR_A::INTERRUPTED,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == IRC8MSTBIFR_A::NOT_INTERRUPTED
    }
    #[doc = "IRC8M stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == IRC8MSTBIFR_A::INTERRUPTED
    }
}
#[doc = "Field `HXTALSTBIF` reader - HXTAL stabilization interrupt flag"]
pub type HXTALSTBIF_R = crate::BitReader<HXTALSTBIFR_A>;
#[doc = "HXTAL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HXTALSTBIFR_A {
    #[doc = "0: No interrupt generated"]
    NOT_INTERRUPTED = 0,
    #[doc = "1: HXTAL stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<HXTALSTBIFR_A> for bool {
    #[inline(always)]
    fn from(variant: HXTALSTBIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl HXTALSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTALSTBIFR_A {
        match self.bits {
            false => HXTALSTBIFR_A::NOT_INTERRUPTED,
            true => HXTALSTBIFR_A::INTERRUPTED,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == HXTALSTBIFR_A::NOT_INTERRUPTED
    }
    #[doc = "HXTAL stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == HXTALSTBIFR_A::INTERRUPTED
    }
}
#[doc = "Field `PLLSTBIF` reader - PLL stabilization interrupt flag"]
pub type PLLSTBIF_R = crate::BitReader<PLLSTBIFR_A>;
#[doc = "PLL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSTBIFR_A {
    #[doc = "0: No interrupt generated"]
    NOT_INTERRUPTED = 0,
    #[doc = "1: PLL stabilisation interrupt generated"]
    INTERRUPTED = 1,
}
impl From<PLLSTBIFR_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTBIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSTBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTBIFR_A {
        match self.bits {
            false => PLLSTBIFR_A::NOT_INTERRUPTED,
            true => PLLSTBIFR_A::INTERRUPTED,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == PLLSTBIFR_A::NOT_INTERRUPTED
    }
    #[doc = "PLL stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == PLLSTBIFR_A::INTERRUPTED
    }
}
#[doc = "Field `PLL1STBIF` reader - PLL1 stabilization interrupt flag"]
pub use PLLSTBIF_R as PLL1STBIF_R;
#[doc = "Field `PLL2STBIF` reader - PLL2 stabilization interrupt flag"]
pub use PLLSTBIF_R as PLL2STBIF_R;
#[doc = "Field `CKMIF` reader - HXTAL Clock Stuck Interrupt Flag"]
pub type CKMIF_R = crate::BitReader<CKMIFR_A>;
#[doc = "HXTAL Clock Stuck Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKMIFR_A {
    #[doc = "0: Clock operating normally"]
    NOT_INTERRUPTED = 0,
    #[doc = "1: HXTAL clock stuck"]
    INTERRUPTED = 1,
}
impl From<CKMIFR_A> for bool {
    #[inline(always)]
    fn from(variant: CKMIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CKMIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMIFR_A {
        match self.bits {
            false => CKMIFR_A::NOT_INTERRUPTED,
            true => CKMIFR_A::INTERRUPTED,
        }
    }
    #[doc = "Clock operating normally"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CKMIFR_A::NOT_INTERRUPTED
    }
    #[doc = "HXTAL clock stuck"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CKMIFR_A::INTERRUPTED
    }
}
#[doc = "Field `IRC40KSTBIE` reader - IRC40K Stabilization interrupt enable"]
pub type IRC40KSTBIE_R = crate::BitReader<IRC40KSTBIE_A>;
#[doc = "IRC40K Stabilization interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRC40KSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<IRC40KSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: IRC40KSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl IRC40KSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC40KSTBIE_A {
        match self.bits {
            false => IRC40KSTBIE_A::DISABLED,
            true => IRC40KSTBIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRC40KSTBIE_A::DISABLED
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRC40KSTBIE_A::ENABLED
    }
}
#[doc = "Field `IRC40KSTBIE` writer - IRC40K Stabilization interrupt enable"]
pub type IRC40KSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRC40KSTBIE_A>;
impl<'a, REG, const O: u8> IRC40KSTBIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IRC40KSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IRC40KSTBIE_A::ENABLED)
    }
}
#[doc = "Field `LXTALSTBIE` reader - LXTAL Stabilization Interrupt Enable"]
pub type LXTALSTBIE_R = crate::BitReader<LXTALSTBIE_A>;
#[doc = "LXTAL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LXTALSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<LXTALSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: LXTALSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LXTALSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTALSTBIE_A {
        match self.bits {
            false => LXTALSTBIE_A::DISABLED,
            true => LXTALSTBIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LXTALSTBIE_A::DISABLED
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LXTALSTBIE_A::ENABLED
    }
}
#[doc = "Field `LXTALSTBIE` writer - LXTAL Stabilization Interrupt Enable"]
pub type LXTALSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LXTALSTBIE_A>;
impl<'a, REG, const O: u8> LXTALSTBIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALSTBIE_A::ENABLED)
    }
}
#[doc = "Field `IRC8MSTBIE` reader - IRC8M Stabilization Interrupt Enable"]
pub type IRC8MSTBIE_R = crate::BitReader<IRC8MSTBIE_A>;
#[doc = "IRC8M Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRC8MSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<IRC8MSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: IRC8MSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl IRC8MSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC8MSTBIE_A {
        match self.bits {
            false => IRC8MSTBIE_A::DISABLED,
            true => IRC8MSTBIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRC8MSTBIE_A::DISABLED
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRC8MSTBIE_A::ENABLED
    }
}
#[doc = "Field `IRC8MSTBIE` writer - IRC8M Stabilization Interrupt Enable"]
pub type IRC8MSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRC8MSTBIE_A>;
impl<'a, REG, const O: u8> IRC8MSTBIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IRC8MSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IRC8MSTBIE_A::ENABLED)
    }
}
#[doc = "Field `HXTALSTBIE` reader - HXTAL Stabilization Interrupt Enable"]
pub type HXTALSTBIE_R = crate::BitReader<HXTALSTBIE_A>;
#[doc = "HXTAL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HXTALSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<HXTALSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: HXTALSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HXTALSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTALSTBIE_A {
        match self.bits {
            false => HXTALSTBIE_A::DISABLED,
            true => HXTALSTBIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HXTALSTBIE_A::DISABLED
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HXTALSTBIE_A::ENABLED
    }
}
#[doc = "Field `HXTALSTBIE` writer - HXTAL Stabilization Interrupt Enable"]
pub type HXTALSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HXTALSTBIE_A>;
impl<'a, REG, const O: u8> HXTALSTBIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALSTBIE_A::ENABLED)
    }
}
#[doc = "Field `PLLSTBIE` reader - PLL Stabilization Interrupt Enable"]
pub type PLLSTBIE_R = crate::BitReader<PLLSTBIE_A>;
#[doc = "PLL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSTBIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<PLLSTBIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSTBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTBIE_A {
        match self.bits {
            false => PLLSTBIE_A::DISABLED,
            true => PLLSTBIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSTBIE_A::DISABLED
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSTBIE_A::ENABLED
    }
}
#[doc = "Field `PLLSTBIE` writer - PLL Stabilization Interrupt Enable"]
pub type PLLSTBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PLLSTBIE_A>;
impl<'a, REG, const O: u8> PLLSTBIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSTBIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSTBIE_A::ENABLED)
    }
}
#[doc = "Field `PLL1STBIE` reader - PLL1 Stabilization Interrupt Enable"]
pub use PLLSTBIE_R as PLL1STBIE_R;
#[doc = "Field `PLL2STBIE` reader - PLL2 Stabilization Interrupt Enable"]
pub use PLLSTBIE_R as PLL2STBIE_R;
#[doc = "Field `PLL1STBIE` writer - PLL1 Stabilization Interrupt Enable"]
pub use PLLSTBIE_W as PLL1STBIE_W;
#[doc = "Field `PLL2STBIE` writer - PLL2 Stabilization Interrupt Enable"]
pub use PLLSTBIE_W as PLL2STBIE_W;
#[doc = "IRC40K Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRC40KSTBICW_AW {
    #[doc = "1: Clear IRC40KSTBIF flag"]
    CLEAR = 1,
}
impl From<IRC40KSTBICW_AW> for bool {
    #[inline(always)]
    fn from(variant: IRC40KSTBICW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC40KSTBIC` writer - IRC40K Stabilization Interrupt Clear"]
pub type IRC40KSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRC40KSTBICW_AW>;
impl<'a, REG, const O: u8> IRC40KSTBIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IRC40KSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IRC40KSTBICW_AW::CLEAR)
    }
}
#[doc = "LXTAL Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LXTALSTBICW_AW {
    #[doc = "1: Clear LXTALSTBIF flag"]
    CLEAR = 1,
}
impl From<LXTALSTBICW_AW> for bool {
    #[inline(always)]
    fn from(variant: LXTALSTBICW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTBIC` writer - LXTAL Stabilization Interrupt Clear"]
pub type LXTALSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LXTALSTBICW_AW>;
impl<'a, REG, const O: u8> LXTALSTBIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear LXTALSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LXTALSTBICW_AW::CLEAR)
    }
}
#[doc = "IRC8M Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRC8MSTBICW_AW {
    #[doc = "1: Clear IRC8MSTBIF flag"]
    CLEAR = 1,
}
impl From<IRC8MSTBICW_AW> for bool {
    #[inline(always)]
    fn from(variant: IRC8MSTBICW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MSTBIC` writer - IRC8M Stabilization Interrupt Clear"]
pub type IRC8MSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRC8MSTBICW_AW>;
impl<'a, REG, const O: u8> IRC8MSTBIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IRC8MSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IRC8MSTBICW_AW::CLEAR)
    }
}
#[doc = "HXTAL Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HXTALSTBICW_AW {
    #[doc = "1: Clear HXTALSTBIF flag"]
    CLEAR = 1,
}
impl From<HXTALSTBICW_AW> for bool {
    #[inline(always)]
    fn from(variant: HXTALSTBICW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTBIC` writer - HXTAL Stabilization Interrupt Clear"]
pub type HXTALSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HXTALSTBICW_AW>;
impl<'a, REG, const O: u8> HXTALSTBIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear HXTALSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALSTBICW_AW::CLEAR)
    }
}
#[doc = "PLL stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSTBICW_AW {
    #[doc = "1: Clear PLLSTBIF flag"]
    CLEAR = 1,
}
impl From<PLLSTBICW_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLSTBICW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBIC` writer - PLL stabilization Interrupt Clear"]
pub type PLLSTBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PLLSTBICW_AW>;
impl<'a, REG, const O: u8> PLLSTBIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear PLLSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSTBICW_AW::CLEAR)
    }
}
#[doc = "Field `PLL1STBIC` writer - PLL1 stabilization Interrupt Clear"]
pub use PLLSTBIC_W as PLL1STBIC_W;
#[doc = "Field `PLL2STBIC` writer - PLL2 stabilization Interrupt Clear"]
pub use PLLSTBIC_W as PLL2STBIC_W;
#[doc = "HXTAL Clock Stuck Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKMICW_AW {
    #[doc = "1: Clear CKMIF flag"]
    CLEAR = 1,
}
impl From<CKMICW_AW> for bool {
    #[inline(always)]
    fn from(variant: CKMICW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMIC` writer - HXTAL Clock Stuck Interrupt Clear"]
pub type CKMIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CKMICW_AW>;
impl<'a, REG, const O: u8> CKMIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CKMIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CKMICW_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - IRC40K stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc40kstbif(&self) -> IRC40KSTBIF_R {
        IRC40KSTBIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn lxtalstbif(&self) -> LXTALSTBIF_R {
        LXTALSTBIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRC8M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc8mstbif(&self) -> IRC8MSTBIF_R {
        IRC8MSTBIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn hxtalstbif(&self) -> HXTALSTBIF_R {
        HXTALSTBIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllstbif(&self) -> PLLSTBIF_R {
        PLLSTBIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL1 stabilization interrupt flag"]
    #[inline(always)]
    pub fn pll1stbif(&self) -> PLL1STBIF_R {
        PLL1STBIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL2 stabilization interrupt flag"]
    #[inline(always)]
    pub fn pll2stbif(&self) -> PLL2STBIF_R {
        PLL2STBIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HXTAL Clock Stuck Interrupt Flag"]
    #[inline(always)]
    pub fn ckmif(&self) -> CKMIF_R {
        CKMIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc40kstbie(&self) -> IRC40KSTBIE_R {
        IRC40KSTBIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&self) -> LXTALSTBIE_R {
        LXTALSTBIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc8mstbie(&self) -> IRC8MSTBIE_R {
        IRC8MSTBIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&self) -> HXTALSTBIE_R {
        HXTALSTBIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&self) -> PLLSTBIE_R {
        PLLSTBIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLL1 Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pll1stbie(&self) -> PLL1STBIE_R {
        PLL1STBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLL2 Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pll2stbie(&self) -> PLL2STBIE_R {
        PLL2STBIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc40kstbie(&mut self) -> IRC40KSTBIE_W<INT_SPEC, 8> {
        IRC40KSTBIE_W::new(self)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalstbie(&mut self) -> LXTALSTBIE_W<INT_SPEC, 9> {
        LXTALSTBIE_W::new(self)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc8mstbie(&mut self) -> IRC8MSTBIE_W<INT_SPEC, 10> {
        IRC8MSTBIE_W::new(self)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalstbie(&mut self) -> HXTALSTBIE_W<INT_SPEC, 11> {
        HXTALSTBIE_W::new(self)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllstbie(&mut self) -> PLLSTBIE_W<INT_SPEC, 12> {
        PLLSTBIE_W::new(self)
    }
    #[doc = "Bit 13 - PLL1 Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1stbie(&mut self) -> PLL1STBIE_W<INT_SPEC, 13> {
        PLL1STBIE_W::new(self)
    }
    #[doc = "Bit 14 - PLL2 Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2stbie(&mut self) -> PLL2STBIE_W<INT_SPEC, 14> {
        PLL2STBIE_W::new(self)
    }
    #[doc = "Bit 16 - IRC40K Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc40kstbic(&mut self) -> IRC40KSTBIC_W<INT_SPEC, 16> {
        IRC40KSTBIC_W::new(self)
    }
    #[doc = "Bit 17 - LXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalstbic(&mut self) -> LXTALSTBIC_W<INT_SPEC, 17> {
        LXTALSTBIC_W::new(self)
    }
    #[doc = "Bit 18 - IRC8M Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc8mstbic(&mut self) -> IRC8MSTBIC_W<INT_SPEC, 18> {
        IRC8MSTBIC_W::new(self)
    }
    #[doc = "Bit 19 - HXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalstbic(&mut self) -> HXTALSTBIC_W<INT_SPEC, 19> {
        HXTALSTBIC_W::new(self)
    }
    #[doc = "Bit 20 - PLL stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllstbic(&mut self) -> PLLSTBIC_W<INT_SPEC, 20> {
        PLLSTBIC_W::new(self)
    }
    #[doc = "Bit 21 - PLL1 stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pll1stbic(&mut self) -> PLL1STBIC_W<INT_SPEC, 21> {
        PLL1STBIC_W::new(self)
    }
    #[doc = "Bit 22 - PLL2 stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pll2stbic(&mut self) -> PLL2STBIC_W<INT_SPEC, 22> {
        PLL2STBIC_W::new(self)
    }
    #[doc = "Bit 23 - HXTAL Clock Stuck Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ckmic(&mut self) -> CKMIC_W<INT_SPEC, 23> {
        CKMIC_W::new(self)
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
#[doc = "Clock interrupt register (RCU_INT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
