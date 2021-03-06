#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sleep working interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPWIE_A {
    #[doc = "0: Sleep working interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Sleep working interrupt is enabled"]
    ENABLED = 1,
}
impl From<SLPWIE_A> for bool {
    #[inline(always)]
    fn from(variant: SLPWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPWIE` reader - Sleep working interrupt enable"]
pub type SLPWIE_R = crate::BitReader<SLPWIE_A>;
impl SLPWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPWIE_A {
        match self.bits {
            false => SLPWIE_A::DISABLED,
            true => SLPWIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLPWIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLPWIE_A::ENABLED
    }
}
#[doc = "Field `SLPWIE` writer - Sleep working interrupt enable"]
pub type SLPWIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, SLPWIE_A, 17>;
impl<'a> SLPWIE_W<'a> {
    #[doc = "Sleep working interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLPWIE_A::DISABLED)
    }
    #[doc = "Sleep working interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLPWIE_A::ENABLED)
    }
}
#[doc = "Wakeup interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIE_A {
    #[doc = "0: Wakeup interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Wakeup interrupt is enabled"]
    ENABLED = 1,
}
impl From<WIE_A> for bool {
    #[inline(always)]
    fn from(variant: WIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIE` reader - Wakeup interrupt enable"]
pub type WIE_R = crate::BitReader<WIE_A>;
impl WIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIE_A {
        match self.bits {
            false => WIE_A::DISABLED,
            true => WIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WIE_A::ENABLED
    }
}
#[doc = "Field `WIE` writer - Wakeup interrupt enable"]
pub type WIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, WIE_A, 16>;
impl<'a> WIE_W<'a> {
    #[doc = "Wakeup interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WIE_A::DISABLED)
    }
    #[doc = "Wakeup interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WIE_A::ENABLED)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Error interrupt is enabled"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, ERRIE_A, 15>;
impl<'a> ERRIE_W<'a> {
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
    }
}
#[doc = "Error number interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRNIE_A {
    #[doc = "0: Error number interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Error number interrupt is enabled"]
    ENABLED = 1,
}
impl From<ERRNIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRNIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRNIE` reader - Error number interrupt enable"]
pub type ERRNIE_R = crate::BitReader<ERRNIE_A>;
impl ERRNIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRNIE_A {
        match self.bits {
            false => ERRNIE_A::DISABLED,
            true => ERRNIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRNIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRNIE_A::ENABLED
    }
}
#[doc = "Field `ERRNIE` writer - Error number interrupt enable"]
pub type ERRNIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, ERRNIE_A, 11>;
impl<'a> ERRNIE_W<'a> {
    #[doc = "Error number interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRNIE_A::DISABLED)
    }
    #[doc = "Error number interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRNIE_A::ENABLED)
    }
}
#[doc = "Bus-off interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOIE_A {
    #[doc = "0: Bus-off interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Bus-off interrupt is enabled"]
    ENABLED = 1,
}
impl From<BOIE_A> for bool {
    #[inline(always)]
    fn from(variant: BOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOIE` reader - Bus-off interrupt enable"]
pub type BOIE_R = crate::BitReader<BOIE_A>;
impl BOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOIE_A {
        match self.bits {
            false => BOIE_A::DISABLED,
            true => BOIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOIE_A::ENABLED
    }
}
#[doc = "Field `BOIE` writer - Bus-off interrupt enable"]
pub type BOIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, BOIE_A, 10>;
impl<'a> BOIE_W<'a> {
    #[doc = "Bus-off interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOIE_A::DISABLED)
    }
    #[doc = "Bus-off interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOIE_A::ENABLED)
    }
}
#[doc = "Passive error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERRIE_A {
    #[doc = "0: Passive error interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Passive error interrupt is enabled"]
    ENABLED = 1,
}
impl From<PERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: PERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERRIE` reader - Passive error interrupt enable"]
pub type PERRIE_R = crate::BitReader<PERRIE_A>;
impl PERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERRIE_A {
        match self.bits {
            false => PERRIE_A::DISABLED,
            true => PERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PERRIE_A::ENABLED
    }
}
#[doc = "Field `PERRIE` writer - Passive error interrupt enable"]
pub type PERRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, PERRIE_A, 9>;
impl<'a> PERRIE_W<'a> {
    #[doc = "Passive error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PERRIE_A::DISABLED)
    }
    #[doc = "Passive error interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PERRIE_A::ENABLED)
    }
}
#[doc = "Warning error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WERRIE_A {
    #[doc = "0: Warning error interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Warning error interrupt is enabled"]
    ENABLED = 1,
}
impl From<WERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: WERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WERRIE` reader - Warning error interrupt enable"]
pub type WERRIE_R = crate::BitReader<WERRIE_A>;
impl WERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WERRIE_A {
        match self.bits {
            false => WERRIE_A::DISABLED,
            true => WERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WERRIE_A::ENABLED
    }
}
#[doc = "Field `WERRIE` writer - Warning error interrupt enable"]
pub type WERRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, WERRIE_A, 8>;
impl<'a> WERRIE_W<'a> {
    #[doc = "Warning error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WERRIE_A::DISABLED)
    }
    #[doc = "Warning error interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WERRIE_A::ENABLED)
    }
}
#[doc = "Receive FIFO1 full interrupt enable"]
pub use RFFIE0_A as RFFIE1_A;
#[doc = "Field `RFFIE1` reader - Receive FIFO1 full interrupt enable"]
pub use RFFIE0_R as RFFIE1_R;
#[doc = "Field `RFFIE1` writer - Receive FIFO1 full interrupt enable"]
pub use RFFIE0_W as RFFIE1_W;
#[doc = "Receive FIFO1 not empty interrupt enable"]
pub use RFNEIE0_A as RFNEIE1_A;
#[doc = "Field `RFNEIE1` reader - Receive FIFO1 not empty interrupt enable"]
pub use RFNEIE0_R as RFNEIE1_R;
#[doc = "Field `RFNEIE1` writer - Receive FIFO1 not empty interrupt enable"]
pub use RFNEIE0_W as RFNEIE1_W;
#[doc = "Receive FIFO1 overfull interrupt enable"]
pub use RFOIE0_A as RFOIE1_A;
#[doc = "Field `RFOIE1` reader - Receive FIFO1 overfull interrupt enable"]
pub use RFOIE0_R as RFOIE1_R;
#[doc = "Field `RFOIE1` writer - Receive FIFO1 overfull interrupt enable"]
pub use RFOIE0_W as RFOIE1_W;
#[doc = "Receive FIFO0 overfull interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFOIE0_A {
    #[doc = "0: Receive FIFO overfull interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Receive FIFO overfull interrupt is enabled"]
    ENABLED = 1,
}
impl From<RFOIE0_A> for bool {
    #[inline(always)]
    fn from(variant: RFOIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOIE0` reader - Receive FIFO0 overfull interrupt enable"]
pub type RFOIE0_R = crate::BitReader<RFOIE0_A>;
impl RFOIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOIE0_A {
        match self.bits {
            false => RFOIE0_A::DISABLED,
            true => RFOIE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFOIE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFOIE0_A::ENABLED
    }
}
#[doc = "Field `RFOIE0` writer - Receive FIFO0 overfull interrupt enable"]
pub type RFOIE0_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, RFOIE0_A, 3>;
impl<'a> RFOIE0_W<'a> {
    #[doc = "Receive FIFO overfull interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFOIE0_A::DISABLED)
    }
    #[doc = "Receive FIFO overfull interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFOIE0_A::ENABLED)
    }
}
#[doc = "Receive FIFO0 full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFFIE0_A {
    #[doc = "0: Receive FIFO full interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Receive FIFO full interrupt is enabled"]
    ENABLED = 1,
}
impl From<RFFIE0_A> for bool {
    #[inline(always)]
    fn from(variant: RFFIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFIE0` reader - Receive FIFO0 full interrupt enable"]
pub type RFFIE0_R = crate::BitReader<RFFIE0_A>;
impl RFFIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFFIE0_A {
        match self.bits {
            false => RFFIE0_A::DISABLED,
            true => RFFIE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFFIE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFFIE0_A::ENABLED
    }
}
#[doc = "Field `RFFIE0` writer - Receive FIFO0 full interrupt enable"]
pub type RFFIE0_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, RFFIE0_A, 2>;
impl<'a> RFFIE0_W<'a> {
    #[doc = "Receive FIFO full interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFFIE0_A::DISABLED)
    }
    #[doc = "Receive FIFO full interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFFIE0_A::ENABLED)
    }
}
#[doc = "Receive FIFO0 not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFNEIE0_A {
    #[doc = "0: Receive FIFO not empty interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Receive FIFO not empty interrupt is enabled"]
    ENABLED = 1,
}
impl From<RFNEIE0_A> for bool {
    #[inline(always)]
    fn from(variant: RFNEIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFNEIE0` reader - Receive FIFO0 not empty interrupt enable"]
pub type RFNEIE0_R = crate::BitReader<RFNEIE0_A>;
impl RFNEIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFNEIE0_A {
        match self.bits {
            false => RFNEIE0_A::DISABLED,
            true => RFNEIE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFNEIE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFNEIE0_A::ENABLED
    }
}
#[doc = "Field `RFNEIE0` writer - Receive FIFO0 not empty interrupt enable"]
pub type RFNEIE0_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, RFNEIE0_A, 1>;
impl<'a> RFNEIE0_W<'a> {
    #[doc = "Receive FIFO not empty interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFNEIE0_A::DISABLED)
    }
    #[doc = "Receive FIFO not empty interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFNEIE0_A::ENABLED)
    }
}
#[doc = "Transmit mailbox empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMEIE_A {
    #[doc = "0: Transmit mailbox empty interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Transmit mailbox empty interrupt is enabled"]
    ENABLED = 1,
}
impl From<TMEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TMEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMEIE` reader - Transmit mailbox empty interrupt enable"]
pub type TMEIE_R = crate::BitReader<TMEIE_A>;
impl TMEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMEIE_A {
        match self.bits {
            false => TMEIE_A::DISABLED,
            true => TMEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TMEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TMEIE_A::ENABLED
    }
}
#[doc = "Field `TMEIE` writer - Transmit mailbox empty interrupt enable"]
pub type TMEIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, TMEIE_A, 0>;
impl<'a> TMEIE_W<'a> {
    #[doc = "Transmit mailbox empty interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TMEIE_A::DISABLED)
    }
    #[doc = "Transmit mailbox empty interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TMEIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 17 - Sleep working interrupt enable"]
    #[inline(always)]
    pub fn slpwie(&self) -> SLPWIE_R {
        SLPWIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Wakeup interrupt enable"]
    #[inline(always)]
    pub fn wie(&self) -> WIE_R {
        WIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 11 - Error number interrupt enable"]
    #[inline(always)]
    pub fn errnie(&self) -> ERRNIE_R {
        ERRNIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    pub fn boie(&self) -> BOIE_R {
        BOIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Passive error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Warning error interrupt enable"]
    #[inline(always)]
    pub fn werrie(&self) -> WERRIE_R {
        WERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO1 overfull interrupt enable"]
    #[inline(always)]
    pub fn rfoie1(&self) -> RFOIE1_R {
        RFOIE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO1 full interrupt enable"]
    #[inline(always)]
    pub fn rffie1(&self) -> RFFIE1_R {
        RFFIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO1 not empty interrupt enable"]
    #[inline(always)]
    pub fn rfneie1(&self) -> RFNEIE1_R {
        RFNEIE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO0 overfull interrupt enable"]
    #[inline(always)]
    pub fn rfoie0(&self) -> RFOIE0_R {
        RFOIE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO0 full interrupt enable"]
    #[inline(always)]
    pub fn rffie0(&self) -> RFFIE0_R {
        RFFIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO0 not empty interrupt enable"]
    #[inline(always)]
    pub fn rfneie0(&self) -> RFNEIE0_R {
        RFNEIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Transmit mailbox empty interrupt enable"]
    #[inline(always)]
    pub fn tmeie(&self) -> TMEIE_R {
        TMEIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Sleep working interrupt enable"]
    #[inline(always)]
    pub fn slpwie(&mut self) -> SLPWIE_W {
        SLPWIE_W::new(self)
    }
    #[doc = "Bit 16 - Wakeup interrupt enable"]
    #[inline(always)]
    pub fn wie(&mut self) -> WIE_W {
        WIE_W::new(self)
    }
    #[doc = "Bit 15 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 11 - Error number interrupt enable"]
    #[inline(always)]
    pub fn errnie(&mut self) -> ERRNIE_W {
        ERRNIE_W::new(self)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    pub fn boie(&mut self) -> BOIE_W {
        BOIE_W::new(self)
    }
    #[doc = "Bit 9 - Passive error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&mut self) -> PERRIE_W {
        PERRIE_W::new(self)
    }
    #[doc = "Bit 8 - Warning error interrupt enable"]
    #[inline(always)]
    pub fn werrie(&mut self) -> WERRIE_W {
        WERRIE_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO1 overfull interrupt enable"]
    #[inline(always)]
    pub fn rfoie1(&mut self) -> RFOIE1_W {
        RFOIE1_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO1 full interrupt enable"]
    #[inline(always)]
    pub fn rffie1(&mut self) -> RFFIE1_W {
        RFFIE1_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO1 not empty interrupt enable"]
    #[inline(always)]
    pub fn rfneie1(&mut self) -> RFNEIE1_W {
        RFNEIE1_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO0 overfull interrupt enable"]
    #[inline(always)]
    pub fn rfoie0(&mut self) -> RFOIE0_W {
        RFOIE0_W::new(self)
    }
    #[doc = "Bit 2 - Receive FIFO0 full interrupt enable"]
    #[inline(always)]
    pub fn rffie0(&mut self) -> RFFIE0_W {
        RFFIE0_W::new(self)
    }
    #[doc = "Bit 1 - Receive FIFO0 not empty interrupt enable"]
    #[inline(always)]
    pub fn rfneie0(&mut self) -> RFNEIE0_W {
        RFNEIE0_W::new(self)
    }
    #[doc = "Bit 0 - Transmit mailbox empty interrupt enable"]
    #[inline(always)]
    pub fn tmeie(&mut self) -> TMEIE_W {
        TMEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
