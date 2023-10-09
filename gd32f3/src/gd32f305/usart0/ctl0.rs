#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `SBKCMD` reader - Send break command"]
pub type SBKCMD_R = crate::BitReader<SBKCMD_A>;
#[doc = "Send break command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKCMD_A {
    #[doc = "0: Do not transmit a break frame"]
    DISABLED = 0,
    #[doc = "1: Do not transmit a break frame"]
    ENABLED = 1,
}
impl From<SBKCMD_A> for bool {
    #[inline(always)]
    fn from(variant: SBKCMD_A) -> Self {
        variant as u8 != 0
    }
}
impl SBKCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBKCMD_A {
        match self.bits {
            false => SBKCMD_A::DISABLED,
            true => SBKCMD_A::ENABLED,
        }
    }
    #[doc = "Do not transmit a break frame"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SBKCMD_A::DISABLED
    }
    #[doc = "Do not transmit a break frame"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SBKCMD_A::ENABLED
    }
}
#[doc = "Field `SBKCMD` writer - Send break command"]
pub type SBKCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SBKCMD_A>;
impl<'a, REG, const O: u8> SBKCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not transmit a break frame"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBKCMD_A::DISABLED)
    }
    #[doc = "Do not transmit a break frame"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBKCMD_A::ENABLED)
    }
}
#[doc = "Field `RWU` reader - Receiver wakeup from mute mode"]
pub type RWU_R = crate::BitReader<RWU_A>;
#[doc = "Receiver wakeup from mute mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWU_A {
    #[doc = "0: Receiver wakeup from mute mode"]
    DISABLED = 0,
    #[doc = "1: Receiver in mute mode"]
    ENABLED = 1,
}
impl From<RWU_A> for bool {
    #[inline(always)]
    fn from(variant: RWU_A) -> Self {
        variant as u8 != 0
    }
}
impl RWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWU_A {
        match self.bits {
            false => RWU_A::DISABLED,
            true => RWU_A::ENABLED,
        }
    }
    #[doc = "Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWU_A::DISABLED
    }
    #[doc = "Receiver in mute mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWU_A::ENABLED
    }
}
#[doc = "Field `RWU` writer - Receiver wakeup from mute mode"]
pub type RWU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RWU_A>;
impl<'a, REG, const O: u8> RWU_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWU_A::DISABLED)
    }
    #[doc = "Receiver in mute mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWU_A::ENABLED)
    }
}
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader<REN_A>;
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REN_A {
    #[doc = "0: Receiver is disabled"]
    DISABLED = 0,
    #[doc = "1: Receiver is enabled"]
    ENABLED = 1,
}
impl From<REN_A> for bool {
    #[inline(always)]
    fn from(variant: REN_A) -> Self {
        variant as u8 != 0
    }
}
impl REN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REN_A {
        match self.bits {
            false => REN_A::DISABLED,
            true => REN_A::ENABLED,
        }
    }
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REN_A::DISABLED
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REN_A::ENABLED
    }
}
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, REN_A>;
impl<'a, REG, const O: u8> REN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REN_A::DISABLED)
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REN_A::ENABLED)
    }
}
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader<TEN_A>;
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN_A {
    #[doc = "0: Transmitter is disabled"]
    DISABLED = 0,
    #[doc = "1: Transmitter is enabled"]
    ENABLED = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::DISABLED,
            true => TEN_A::ENABLED,
        }
    }
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN_A::DISABLED
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN_A::ENABLED
    }
}
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TEN_A>;
impl<'a, REG, const O: u8> TEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN_A::DISABLED)
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN_A::ENABLED)
    }
}
#[doc = "Field `IDLEIE` reader - IDLE line detected interrupt enable"]
pub type IDLEIE_R = crate::BitReader<IDLEIE_A>;
#[doc = "IDLE line detected interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIE_A {
    #[doc = "0: Idle line detected interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever IDLEF=1 in the STAT register"]
    ENABLED = 1,
}
impl From<IDLEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLEIE_A {
        match self.bits {
            false => IDLEIE_A::DISABLED,
            true => IDLEIE_A::ENABLED,
        }
    }
    #[doc = "Idle line detected interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIE_A::DISABLED
    }
    #[doc = "Interrupt is generated whenever IDLEF=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIE_A::ENABLED
    }
}
#[doc = "Field `IDLEIE` writer - IDLE line detected interrupt enable"]
pub type IDLEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IDLEIE_A>;
impl<'a, REG, const O: u8> IDLEIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle line detected interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever IDLEF=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE_A::ENABLED)
    }
}
#[doc = "Field `RBNEIE` reader - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RBNEIE_R = crate::BitReader<RBNEIE_A>;
#[doc = "Read data buffer not empty interrupt and overrun error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBNEIE_A {
    #[doc = "0: Read data buffer not empty and overrrun error interrupts are disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever ORERR=1 or RBNE=1 in the STAT register"]
    ENABLED = 1,
}
impl From<RBNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBNEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RBNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBNEIE_A {
        match self.bits {
            false => RBNEIE_A::DISABLED,
            true => RBNEIE_A::ENABLED,
        }
    }
    #[doc = "Read data buffer not empty and overrrun error interrupts are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RBNEIE_A::DISABLED
    }
    #[doc = "Interrupt is generated whenever ORERR=1 or RBNE=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RBNEIE_A::ENABLED
    }
}
#[doc = "Field `RBNEIE` writer - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RBNEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RBNEIE_A>;
impl<'a, REG, const O: u8> RBNEIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read data buffer not empty and overrrun error interrupts are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RBNEIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever ORERR=1 or RBNE=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RBNEIE_A::ENABLED)
    }
}
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: Transmission complete interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever TC=1 in the STAT register"]
    ENABLED = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::DISABLED,
            true => TCIE_A::ENABLED,
        }
    }
    #[doc = "Transmission complete interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::DISABLED
    }
    #[doc = "Interrupt is generated whenever TC=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::ENABLED
    }
}
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TCIE_A>;
impl<'a, REG, const O: u8> TCIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission complete interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever TC=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE_A::ENABLED)
    }
}
#[doc = "Field `TBEIE` reader - Transmitter buffer empty interrupt enable"]
pub type TBEIE_R = crate::BitReader<TBEIE_A>;
#[doc = "Transmitter buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBEIE_A {
    #[doc = "0: Transmission register empty interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever TBE=1 in the STAT register"]
    ENABLED = 1,
}
impl From<TBEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TBEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TBEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBEIE_A {
        match self.bits {
            false => TBEIE_A::DISABLED,
            true => TBEIE_A::ENABLED,
        }
    }
    #[doc = "Transmission register empty interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TBEIE_A::DISABLED
    }
    #[doc = "Interrupt is generated whenever TBE=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TBEIE_A::ENABLED
    }
}
#[doc = "Field `TBEIE` writer - Transmitter buffer empty interrupt enable"]
pub type TBEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TBEIE_A>;
impl<'a, REG, const O: u8> TBEIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission register empty interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBEIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever TBE=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBEIE_A::ENABLED)
    }
}
#[doc = "Field `PERRIE` reader - Parity error interrupt enable"]
pub type PERRIE_R = crate::BitReader<PERRIE_A>;
#[doc = "Parity error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERRIE_A {
    #[doc = "0: Parity error interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever PERR=1 in the STAT register"]
    ENABLED = 1,
}
impl From<PERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: PERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERRIE_A {
        match self.bits {
            false => PERRIE_A::DISABLED,
            true => PERRIE_A::ENABLED,
        }
    }
    #[doc = "Parity error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PERRIE_A::DISABLED
    }
    #[doc = "Interrupt is generated whenever PERR=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PERRIE_A::ENABLED
    }
}
#[doc = "Field `PERRIE` writer - Parity error interrupt enable"]
pub type PERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PERRIE_A>;
impl<'a, REG, const O: u8> PERRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PERRIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever PERR=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PERRIE_A::ENABLED)
    }
}
#[doc = "Field `PM` reader - Parity mode"]
pub type PM_R = crate::BitReader<PM_A>;
#[doc = "Parity mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    #[doc = "0: Even parity"]
    EVEN = 0,
    #[doc = "1: Odd parity"]
    ODD = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::EVEN,
            true => PM_A::ODD,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PM_A::EVEN
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PM_A::ODD
    }
}
#[doc = "Field `PM` writer - Parity mode"]
pub type PM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PM_A>;
impl<'a, REG, const O: u8> PM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::EVEN)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::ODD)
    }
}
#[doc = "Field `PCEN` reader - Parity check function enable"]
pub type PCEN_R = crate::BitReader<PCEN_A>;
#[doc = "Parity check function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCEN_A {
    #[doc = "0: Parity control disabled"]
    DISABLED = 0,
    #[doc = "1: Parity control enabled"]
    ENABLED = 1,
}
impl From<PCEN_A> for bool {
    #[inline(always)]
    fn from(variant: PCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCEN_A {
        match self.bits {
            false => PCEN_A::DISABLED,
            true => PCEN_A::ENABLED,
        }
    }
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCEN_A::DISABLED
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCEN_A::ENABLED
    }
}
#[doc = "Field `PCEN` writer - Parity check function enable"]
pub type PCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PCEN_A>;
impl<'a, REG, const O: u8> PCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PCEN_A::DISABLED)
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PCEN_A::ENABLED)
    }
}
#[doc = "Field `WM` reader - Wakeup method in mute mode"]
pub type WM_R = crate::BitReader<WM_A>;
#[doc = "Wakeup method in mute mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WM_A {
    #[doc = "0: Idle line"]
    IDLE = 0,
    #[doc = "1: Address mask"]
    ADDRESS = 1,
}
impl From<WM_A> for bool {
    #[inline(always)]
    fn from(variant: WM_A) -> Self {
        variant as u8 != 0
    }
}
impl WM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WM_A {
        match self.bits {
            false => WM_A::IDLE,
            true => WM_A::ADDRESS,
        }
    }
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WM_A::IDLE
    }
    #[doc = "Address mask"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WM_A::ADDRESS
    }
}
#[doc = "Field `WM` writer - Wakeup method in mute mode"]
pub type WM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WM_A>;
impl<'a, REG, const O: u8> WM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(WM_A::IDLE)
    }
    #[doc = "Address mask"]
    #[inline(always)]
    pub fn address(self) -> &'a mut crate::W<REG> {
        self.variant(WM_A::ADDRESS)
    }
}
#[doc = "Field `WL` reader - Word length"]
pub type WL_R = crate::BitReader<WL_A>;
#[doc = "Word length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WL_A {
    #[doc = "0: 8 data bits"]
    BIT8 = 0,
    #[doc = "1: 9 data bits"]
    BIT9 = 1,
}
impl From<WL_A> for bool {
    #[inline(always)]
    fn from(variant: WL_A) -> Self {
        variant as u8 != 0
    }
}
impl WL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WL_A {
        match self.bits {
            false => WL_A::BIT8,
            true => WL_A::BIT9,
        }
    }
    #[doc = "8 data bits"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == WL_A::BIT8
    }
    #[doc = "9 data bits"]
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == WL_A::BIT9
    }
}
#[doc = "Field `WL` writer - Word length"]
pub type WL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WL_A>;
impl<'a, REG, const O: u8> WL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8 data bits"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(WL_A::BIT8)
    }
    #[doc = "9 data bits"]
    #[inline(always)]
    pub fn bit9(self) -> &'a mut crate::W<REG> {
        self.variant(WL_A::BIT9)
    }
}
#[doc = "Field `UEN` reader - USART enable"]
pub type UEN_R = crate::BitReader<UEN_A>;
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UEN_A {
    #[doc = "0: USART prescaler and outputs disabled"]
    DISABLED = 0,
    #[doc = "1: USART prescaler and outputs enabled"]
    ENABLED = 1,
}
impl From<UEN_A> for bool {
    #[inline(always)]
    fn from(variant: UEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UEN_A {
        match self.bits {
            false => UEN_A::DISABLED,
            true => UEN_A::ENABLED,
        }
    }
    #[doc = "USART prescaler and outputs disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UEN_A::DISABLED
    }
    #[doc = "USART prescaler and outputs enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UEN_A::ENABLED
    }
}
#[doc = "Field `UEN` writer - USART enable"]
pub type UEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UEN_A>;
impl<'a, REG, const O: u8> UEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART prescaler and outputs disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UEN_A::DISABLED)
    }
    #[doc = "USART prescaler and outputs enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Send break command"]
    #[inline(always)]
    pub fn sbkcmd(&self) -> SBKCMD_R {
        SBKCMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity check function enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn wl(&self) -> WL_R {
        WL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UEN_R {
        UEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break command"]
    #[inline(always)]
    #[must_use]
    pub fn sbkcmd(&mut self) -> SBKCMD_W<CTL0_SPEC, 0> {
        SBKCMD_W::new(self)
    }
    #[doc = "Bit 1 - Receiver wakeup from mute mode"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RWU_W<CTL0_SPEC, 1> {
        RWU_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<CTL0_SPEC, 2> {
        REN_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<CTL0_SPEC, 3> {
        TEN_W::new(self)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<CTL0_SPEC, 4> {
        IDLEIE_W::new(self)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbneie(&mut self) -> RBNEIE_W<CTL0_SPEC, 5> {
        RBNEIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CTL0_SPEC, 6> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbeie(&mut self) -> TBEIE_W<CTL0_SPEC, 7> {
        TBEIE_W::new(self)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrie(&mut self) -> PERRIE_W<CTL0_SPEC, 8> {
        PERRIE_W::new(self)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<CTL0_SPEC, 9> {
        PM_W::new(self)
    }
    #[doc = "Bit 10 - Parity check function enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<CTL0_SPEC, 10> {
        PCEN_W::new(self)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    #[must_use]
    pub fn wm(&mut self) -> WM_W<CTL0_SPEC, 11> {
        WM_W::new(self)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    #[must_use]
    pub fn wl(&mut self) -> WL_W<CTL0_SPEC, 12> {
        WL_W::new(self)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn uen(&mut self) -> UEN_W<CTL0_SPEC, 13> {
        UEN_W::new(self)
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
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}