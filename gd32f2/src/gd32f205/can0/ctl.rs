#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debug freeze\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFZ_A {
    #[doc = "0: Continue running CAN during debug"]
    CONTINUE = 0,
    #[doc = "1: Stop CAN reception and transmission during debug hold"]
    STOP = 1,
}
impl From<DFZ_A> for bool {
    #[inline(always)]
    fn from(variant: DFZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFZ` reader - Debug freeze"]
pub type DFZ_R = crate::BitReader<DFZ_A>;
impl DFZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFZ_A {
        match self.bits {
            false => DFZ_A::CONTINUE,
            true => DFZ_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DFZ_A::CONTINUE
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DFZ_A::STOP
    }
}
#[doc = "Field `DFZ` writer - Debug freeze"]
pub type DFZ_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, DFZ_A, 16>;
impl<'a> DFZ_W<'a> {
    #[doc = "Continue running CAN during debug"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DFZ_A::CONTINUE)
    }
    #[doc = "Stop CAN reception and transmission during debug hold"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DFZ_A::STOP)
    }
}
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRST_A {
    #[doc = "0: Finished resetting"]
    NOTRESETTING = 0,
    #[doc = "1: Reset in progress"]
    RESETTING = 1,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` reader - Software reset"]
pub type SWRST_R = crate::BitReader<SWRST_A>;
impl SWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRST_A {
        match self.bits {
            false => SWRST_A::NOTRESETTING,
            true => SWRST_A::RESETTING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRESETTING`"]
    #[inline(always)]
    pub fn is_not_resetting(&self) -> bool {
        *self == SWRST_A::NOTRESETTING
    }
    #[doc = "Checks if the value of the field is `RESETTING`"]
    #[inline(always)]
    pub fn is_resetting(&self) -> bool {
        *self == SWRST_A::RESETTING
    }
}
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRST_AW {
    #[doc = "1: Reset CAN"]
    RESET = 1,
}
impl From<SWRST_AW> for bool {
    #[inline(always)]
    fn from(variant: SWRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` writer - Software reset"]
pub type SWRST_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, SWRST_AW, 15>;
impl<'a> SWRST_W<'a> {
    #[doc = "Reset CAN"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRST_AW::RESET)
    }
}
#[doc = "Time-triggered communication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTC_A {
    #[doc = "0: Disable time-triggered communication"]
    DISABLED = 0,
    #[doc = "1: Enable time-triggered communication"]
    ENABLED = 1,
}
impl From<TTC_A> for bool {
    #[inline(always)]
    fn from(variant: TTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTC` reader - Time-triggered communication"]
pub type TTC_R = crate::BitReader<TTC_A>;
impl TTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTC_A {
        match self.bits {
            false => TTC_A::DISABLED,
            true => TTC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TTC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TTC_A::ENABLED
    }
}
#[doc = "Field `TTC` writer - Time-triggered communication"]
pub type TTC_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, TTC_A, 7>;
impl<'a> TTC_W<'a> {
    #[doc = "Disable time-triggered communication"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TTC_A::DISABLED)
    }
    #[doc = "Enable time-triggered communication"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TTC_A::ENABLED)
    }
}
#[doc = "Automatic bus-off recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABOR_A {
    #[doc = "0: Bus off state is set by software"]
    MANUAL = 0,
    #[doc = "1: Bus off state is set automatically by hardware"]
    AUTOMATIC = 1,
}
impl From<ABOR_A> for bool {
    #[inline(always)]
    fn from(variant: ABOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABOR` reader - Automatic bus-off recovery"]
pub type ABOR_R = crate::BitReader<ABOR_A>;
impl ABOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABOR_A {
        match self.bits {
            false => ABOR_A::MANUAL,
            true => ABOR_A::AUTOMATIC,
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == ABOR_A::MANUAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == ABOR_A::AUTOMATIC
    }
}
#[doc = "Field `ABOR` writer - Automatic bus-off recovery"]
pub type ABOR_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, ABOR_A, 6>;
impl<'a> ABOR_W<'a> {
    #[doc = "Bus off state is set by software"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(ABOR_A::MANUAL)
    }
    #[doc = "Bus off state is set automatically by hardware"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(ABOR_A::AUTOMATIC)
    }
}
#[doc = "Automatic wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWU_A {
    #[doc = "0: Sleep state is set by software"]
    MANUAL = 0,
    #[doc = "1: Sleep state is set automatically by hardware"]
    AUTOMATIC = 1,
}
impl From<AWU_A> for bool {
    #[inline(always)]
    fn from(variant: AWU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWU` reader - Automatic wakeup"]
pub type AWU_R = crate::BitReader<AWU_A>;
impl AWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWU_A {
        match self.bits {
            false => AWU_A::MANUAL,
            true => AWU_A::AUTOMATIC,
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == AWU_A::MANUAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AWU_A::AUTOMATIC
    }
}
#[doc = "Field `AWU` writer - Automatic wakeup"]
pub type AWU_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, AWU_A, 5>;
impl<'a> AWU_W<'a> {
    #[doc = "Sleep state is set by software"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(AWU_A::MANUAL)
    }
    #[doc = "Sleep state is set automatically by hardware"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(AWU_A::AUTOMATIC)
    }
}
#[doc = "Automatic retransmission disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARD_A {
    #[doc = "0: Enable automatic retransmission"]
    ENABLED = 0,
    #[doc = "1: Disable automatic retransmission"]
    DISABLED = 1,
}
impl From<ARD_A> for bool {
    #[inline(always)]
    fn from(variant: ARD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARD` reader - Automatic retransmission disable"]
pub type ARD_R = crate::BitReader<ARD_A>;
impl ARD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARD_A {
        match self.bits {
            false => ARD_A::ENABLED,
            true => ARD_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARD_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARD_A::DISABLED
    }
}
#[doc = "Field `ARD` writer - Automatic retransmission disable"]
pub type ARD_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, ARD_A, 4>;
impl<'a> ARD_W<'a> {
    #[doc = "Enable automatic retransmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARD_A::ENABLED)
    }
    #[doc = "Disable automatic retransmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARD_A::DISABLED)
    }
}
#[doc = "Receive FIFO overwrite disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFOD_A {
    #[doc = "0: Overwrite full receive FIFO with incoming frame"]
    OVERWRITE = 0,
    #[doc = "1: Discard incoming frame when receive FIFO is full"]
    DISCARD = 1,
}
impl From<RFOD_A> for bool {
    #[inline(always)]
    fn from(variant: RFOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOD` reader - Receive FIFO overwrite disable"]
pub type RFOD_R = crate::BitReader<RFOD_A>;
impl RFOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOD_A {
        match self.bits {
            false => RFOD_A::OVERWRITE,
            true => RFOD_A::DISCARD,
        }
    }
    #[doc = "Checks if the value of the field is `OVERWRITE`"]
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == RFOD_A::OVERWRITE
    }
    #[doc = "Checks if the value of the field is `DISCARD`"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == RFOD_A::DISCARD
    }
}
#[doc = "Field `RFOD` writer - Receive FIFO overwrite disable"]
pub type RFOD_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, RFOD_A, 3>;
impl<'a> RFOD_W<'a> {
    #[doc = "Overwrite full receive FIFO with incoming frame"]
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut W {
        self.variant(RFOD_A::OVERWRITE)
    }
    #[doc = "Discard incoming frame when receive FIFO is full"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(RFOD_A::DISCARD)
    }
}
#[doc = "Transmit FIFO order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFO_A {
    #[doc = "0: Order by identifier of the frame"]
    IDENTIFIER = 0,
    #[doc = "1: First in first out order"]
    FIFO = 1,
}
impl From<TFO_A> for bool {
    #[inline(always)]
    fn from(variant: TFO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFO` reader - Transmit FIFO order"]
pub type TFO_R = crate::BitReader<TFO_A>;
impl TFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFO_A {
        match self.bits {
            false => TFO_A::IDENTIFIER,
            true => TFO_A::FIFO,
        }
    }
    #[doc = "Checks if the value of the field is `IDENTIFIER`"]
    #[inline(always)]
    pub fn is_identifier(&self) -> bool {
        *self == TFO_A::IDENTIFIER
    }
    #[doc = "Checks if the value of the field is `FIFO`"]
    #[inline(always)]
    pub fn is_fifo(&self) -> bool {
        *self == TFO_A::FIFO
    }
}
#[doc = "Field `TFO` writer - Transmit FIFO order"]
pub type TFO_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, TFO_A, 2>;
impl<'a> TFO_W<'a> {
    #[doc = "Order by identifier of the frame"]
    #[inline(always)]
    pub fn identifier(self) -> &'a mut W {
        self.variant(TFO_A::IDENTIFIER)
    }
    #[doc = "First in first out order"]
    #[inline(always)]
    pub fn fifo(self) -> &'a mut W {
        self.variant(TFO_A::FIFO)
    }
}
#[doc = "Sleep working mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPWMOD_A {
    #[doc = "0: Disable sleep mode, bus activity detected"]
    ACTIVE = 0,
    #[doc = "1: Enable sleep mode"]
    SLEEP = 1,
}
impl From<SLPWMOD_A> for bool {
    #[inline(always)]
    fn from(variant: SLPWMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPWMOD` reader - Sleep working mode"]
pub type SLPWMOD_R = crate::BitReader<SLPWMOD_A>;
impl SLPWMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPWMOD_A {
        match self.bits {
            false => SLPWMOD_A::ACTIVE,
            true => SLPWMOD_A::SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SLPWMOD_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == SLPWMOD_A::SLEEP
    }
}
#[doc = "Field `SLPWMOD` writer - Sleep working mode"]
pub type SLPWMOD_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, SLPWMOD_A, 1>;
impl<'a> SLPWMOD_W<'a> {
    #[doc = "Disable sleep mode, bus activity detected"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SLPWMOD_A::ACTIVE)
    }
    #[doc = "Enable sleep mode"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(SLPWMOD_A::SLEEP)
    }
}
#[doc = "Initial working mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWMOD_A {
    #[doc = "0: Disable initial working mode"]
    DISABLED = 0,
    #[doc = "1: Enable initial working mode"]
    ENABLED = 1,
}
impl From<IWMOD_A> for bool {
    #[inline(always)]
    fn from(variant: IWMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWMOD` reader - Initial working mode"]
pub type IWMOD_R = crate::BitReader<IWMOD_A>;
impl IWMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWMOD_A {
        match self.bits {
            false => IWMOD_A::DISABLED,
            true => IWMOD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IWMOD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IWMOD_A::ENABLED
    }
}
#[doc = "Field `IWMOD` writer - Initial working mode"]
pub type IWMOD_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, IWMOD_A, 0>;
impl<'a> IWMOD_W<'a> {
    #[doc = "Disable initial working mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IWMOD_A::DISABLED)
    }
    #[doc = "Enable initial working mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IWMOD_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 16 - Debug freeze"]
    #[inline(always)]
    pub fn dfz(&self) -> DFZ_R {
        DFZ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 7 - Time-triggered communication"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic bus-off recovery"]
    #[inline(always)]
    pub fn abor(&self) -> ABOR_R {
        ABOR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic wakeup"]
    #[inline(always)]
    pub fn awu(&self) -> AWU_R {
        AWU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic retransmission disable"]
    #[inline(always)]
    pub fn ard(&self) -> ARD_R {
        ARD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overwrite disable"]
    #[inline(always)]
    pub fn rfod(&self) -> RFOD_R {
        RFOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO order"]
    #[inline(always)]
    pub fn tfo(&self) -> TFO_R {
        TFO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep working mode"]
    #[inline(always)]
    pub fn slpwmod(&self) -> SLPWMOD_R {
        SLPWMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Initial working mode"]
    #[inline(always)]
    pub fn iwmod(&self) -> IWMOD_R {
        IWMOD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Debug freeze"]
    #[inline(always)]
    pub fn dfz(&mut self) -> DFZ_W {
        DFZ_W::new(self)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W::new(self)
    }
    #[doc = "Bit 7 - Time-triggered communication"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W {
        TTC_W::new(self)
    }
    #[doc = "Bit 6 - Automatic bus-off recovery"]
    #[inline(always)]
    pub fn abor(&mut self) -> ABOR_W {
        ABOR_W::new(self)
    }
    #[doc = "Bit 5 - Automatic wakeup"]
    #[inline(always)]
    pub fn awu(&mut self) -> AWU_W {
        AWU_W::new(self)
    }
    #[doc = "Bit 4 - Automatic retransmission disable"]
    #[inline(always)]
    pub fn ard(&mut self) -> ARD_W {
        ARD_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO overwrite disable"]
    #[inline(always)]
    pub fn rfod(&mut self) -> RFOD_W {
        RFOD_W::new(self)
    }
    #[doc = "Bit 2 - Transmit FIFO order"]
    #[inline(always)]
    pub fn tfo(&mut self) -> TFO_W {
        TFO_W::new(self)
    }
    #[doc = "Bit 1 - Sleep working mode"]
    #[inline(always)]
    pub fn slpwmod(&mut self) -> SLPWMOD_W {
        SLPWMOD_W::new(self)
    }
    #[doc = "Bit 0 - Initial working mode"]
    #[inline(always)]
    pub fn iwmod(&mut self) -> IWMOD_W {
        IWMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0x0001_0002"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0002
    }
}
