#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `IWMOD` reader - Initial working mode"]
pub type IWMOD_R = crate::BitReader<IWMOD_A>;
#[doc = "Initial working mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl IWMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWMOD_A {
        match self.bits {
            false => IWMOD_A::DISABLED,
            true => IWMOD_A::ENABLED,
        }
    }
    #[doc = "Disable initial working mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IWMOD_A::DISABLED
    }
    #[doc = "Enable initial working mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IWMOD_A::ENABLED
    }
}
#[doc = "Field `IWMOD` writer - Initial working mode"]
pub type IWMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IWMOD_A>;
impl<'a, REG, const O: u8> IWMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable initial working mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IWMOD_A::DISABLED)
    }
    #[doc = "Enable initial working mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IWMOD_A::ENABLED)
    }
}
#[doc = "Field `SLPWMOD` reader - Sleep working mode"]
pub type SLPWMOD_R = crate::BitReader<SLPWMOD_A>;
#[doc = "Sleep working mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SLPWMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPWMOD_A {
        match self.bits {
            false => SLPWMOD_A::ACTIVE,
            true => SLPWMOD_A::SLEEP,
        }
    }
    #[doc = "Disable sleep mode, bus activity detected"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SLPWMOD_A::ACTIVE
    }
    #[doc = "Enable sleep mode"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == SLPWMOD_A::SLEEP
    }
}
#[doc = "Field `SLPWMOD` writer - Sleep working mode"]
pub type SLPWMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLPWMOD_A>;
impl<'a, REG, const O: u8> SLPWMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable sleep mode, bus activity detected"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(SLPWMOD_A::ACTIVE)
    }
    #[doc = "Enable sleep mode"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(SLPWMOD_A::SLEEP)
    }
}
#[doc = "Field `TFO` reader - Transmit FIFO order"]
pub type TFO_R = crate::BitReader<TFO_A>;
#[doc = "Transmit FIFO order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFO_A {
        match self.bits {
            false => TFO_A::IDENTIFIER,
            true => TFO_A::FIFO,
        }
    }
    #[doc = "Order by identifier of the frame"]
    #[inline(always)]
    pub fn is_identifier(&self) -> bool {
        *self == TFO_A::IDENTIFIER
    }
    #[doc = "First in first out order"]
    #[inline(always)]
    pub fn is_fifo(&self) -> bool {
        *self == TFO_A::FIFO
    }
}
#[doc = "Field `TFO` writer - Transmit FIFO order"]
pub type TFO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TFO_A>;
impl<'a, REG, const O: u8> TFO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Order by identifier of the frame"]
    #[inline(always)]
    pub fn identifier(self) -> &'a mut crate::W<REG> {
        self.variant(TFO_A::IDENTIFIER)
    }
    #[doc = "First in first out order"]
    #[inline(always)]
    pub fn fifo(self) -> &'a mut crate::W<REG> {
        self.variant(TFO_A::FIFO)
    }
}
#[doc = "Field `RFOD` reader - Receive FIFO overwrite disable"]
pub type RFOD_R = crate::BitReader<RFOD_A>;
#[doc = "Receive FIFO overwrite disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RFOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOD_A {
        match self.bits {
            false => RFOD_A::OVERWRITE,
            true => RFOD_A::DISCARD,
        }
    }
    #[doc = "Overwrite full receive FIFO with incoming frame"]
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == RFOD_A::OVERWRITE
    }
    #[doc = "Discard incoming frame when receive FIFO is full"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == RFOD_A::DISCARD
    }
}
#[doc = "Field `RFOD` writer - Receive FIFO overwrite disable"]
pub type RFOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFOD_A>;
impl<'a, REG, const O: u8> RFOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overwrite full receive FIFO with incoming frame"]
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut crate::W<REG> {
        self.variant(RFOD_A::OVERWRITE)
    }
    #[doc = "Discard incoming frame when receive FIFO is full"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(RFOD_A::DISCARD)
    }
}
#[doc = "Field `ARD` reader - Automatic retransmission disable"]
pub type ARD_R = crate::BitReader<ARD_A>;
#[doc = "Automatic retransmission disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ARD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARD_A {
        match self.bits {
            false => ARD_A::ENABLED,
            true => ARD_A::DISABLED,
        }
    }
    #[doc = "Enable automatic retransmission"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARD_A::ENABLED
    }
    #[doc = "Disable automatic retransmission"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARD_A::DISABLED
    }
}
#[doc = "Field `ARD` writer - Automatic retransmission disable"]
pub type ARD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ARD_A>;
impl<'a, REG, const O: u8> ARD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable automatic retransmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARD_A::ENABLED)
    }
    #[doc = "Disable automatic retransmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARD_A::DISABLED)
    }
}
#[doc = "Field `AWU` reader - Automatic wakeup"]
pub type AWU_R = crate::BitReader<AWU_A>;
#[doc = "Automatic wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl AWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWU_A {
        match self.bits {
            false => AWU_A::MANUAL,
            true => AWU_A::AUTOMATIC,
        }
    }
    #[doc = "Sleep state is set by software"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == AWU_A::MANUAL
    }
    #[doc = "Sleep state is set automatically by hardware"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AWU_A::AUTOMATIC
    }
}
#[doc = "Field `AWU` writer - Automatic wakeup"]
pub type AWU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AWU_A>;
impl<'a, REG, const O: u8> AWU_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sleep state is set by software"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(AWU_A::MANUAL)
    }
    #[doc = "Sleep state is set automatically by hardware"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(AWU_A::AUTOMATIC)
    }
}
#[doc = "Field `ABOR` reader - Automatic bus-off recovery"]
pub type ABOR_R = crate::BitReader<ABOR_A>;
#[doc = "Automatic bus-off recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ABOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABOR_A {
        match self.bits {
            false => ABOR_A::MANUAL,
            true => ABOR_A::AUTOMATIC,
        }
    }
    #[doc = "Bus off state is set by software"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == ABOR_A::MANUAL
    }
    #[doc = "Bus off state is set automatically by hardware"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == ABOR_A::AUTOMATIC
    }
}
#[doc = "Field `ABOR` writer - Automatic bus-off recovery"]
pub type ABOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABOR_A>;
impl<'a, REG, const O: u8> ABOR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus off state is set by software"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(ABOR_A::MANUAL)
    }
    #[doc = "Bus off state is set automatically by hardware"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(ABOR_A::AUTOMATIC)
    }
}
#[doc = "Field `TTC` reader - Time-triggered communication"]
pub type TTC_R = crate::BitReader<TTC_A>;
#[doc = "Time-triggered communication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTC_A {
        match self.bits {
            false => TTC_A::DISABLED,
            true => TTC_A::ENABLED,
        }
    }
    #[doc = "Disable time-triggered communication"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TTC_A::DISABLED
    }
    #[doc = "Enable time-triggered communication"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TTC_A::ENABLED
    }
}
#[doc = "Field `TTC` writer - Time-triggered communication"]
pub type TTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TTC_A>;
impl<'a, REG, const O: u8> TTC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable time-triggered communication"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TTC_A::DISABLED)
    }
    #[doc = "Enable time-triggered communication"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TTC_A::ENABLED)
    }
}
#[doc = "Field `SWRST` reader - Software reset"]
pub type SWRST_R = crate::BitReader<SWRSTR_A>;
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTR_A {
    #[doc = "0: Finished resetting"]
    NOT_RESETTING = 0,
    #[doc = "1: Reset in progress"]
    RESETTING = 1,
}
impl From<SWRSTR_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTR_A {
        match self.bits {
            false => SWRSTR_A::NOT_RESETTING,
            true => SWRSTR_A::RESETTING,
        }
    }
    #[doc = "Finished resetting"]
    #[inline(always)]
    pub fn is_not_resetting(&self) -> bool {
        *self == SWRSTR_A::NOT_RESETTING
    }
    #[doc = "Reset in progress"]
    #[inline(always)]
    pub fn is_resetting(&self) -> bool {
        *self == SWRSTR_A::RESETTING
    }
}
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTW_AW {
    #[doc = "1: Reset CAN"]
    RESET = 1,
}
impl From<SWRSTW_AW> for bool {
    #[inline(always)]
    fn from(variant: SWRSTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` writer - Software reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWRSTW_AW>;
impl<'a, REG, const O: u8> SWRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset CAN"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SWRSTW_AW::RESET)
    }
}
#[doc = "Field `DFZ` reader - Debug freeze"]
pub type DFZ_R = crate::BitReader<DFZ_A>;
#[doc = "Debug freeze\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DFZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFZ_A {
        match self.bits {
            false => DFZ_A::CONTINUE,
            true => DFZ_A::STOP,
        }
    }
    #[doc = "Continue running CAN during debug"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DFZ_A::CONTINUE
    }
    #[doc = "Stop CAN reception and transmission during debug hold"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DFZ_A::STOP
    }
}
#[doc = "Field `DFZ` writer - Debug freeze"]
pub type DFZ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DFZ_A>;
impl<'a, REG, const O: u8> DFZ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running CAN during debug"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DFZ_A::CONTINUE)
    }
    #[doc = "Stop CAN reception and transmission during debug hold"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DFZ_A::STOP)
    }
}
impl R {
    #[doc = "Bit 0 - Initial working mode"]
    #[inline(always)]
    pub fn iwmod(&self) -> IWMOD_R {
        IWMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep working mode"]
    #[inline(always)]
    pub fn slpwmod(&self) -> SLPWMOD_R {
        SLPWMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO order"]
    #[inline(always)]
    pub fn tfo(&self) -> TFO_R {
        TFO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overwrite disable"]
    #[inline(always)]
    pub fn rfod(&self) -> RFOD_R {
        RFOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic retransmission disable"]
    #[inline(always)]
    pub fn ard(&self) -> ARD_R {
        ARD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic wakeup"]
    #[inline(always)]
    pub fn awu(&self) -> AWU_R {
        AWU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic bus-off recovery"]
    #[inline(always)]
    pub fn abor(&self) -> ABOR_R {
        ABOR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Time-triggered communication"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Debug freeze"]
    #[inline(always)]
    pub fn dfz(&self) -> DFZ_R {
        DFZ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initial working mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwmod(&mut self) -> IWMOD_W<CTL_SPEC, 0> {
        IWMOD_W::new(self)
    }
    #[doc = "Bit 1 - Sleep working mode"]
    #[inline(always)]
    #[must_use]
    pub fn slpwmod(&mut self) -> SLPWMOD_W<CTL_SPEC, 1> {
        SLPWMOD_W::new(self)
    }
    #[doc = "Bit 2 - Transmit FIFO order"]
    #[inline(always)]
    #[must_use]
    pub fn tfo(&mut self) -> TFO_W<CTL_SPEC, 2> {
        TFO_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO overwrite disable"]
    #[inline(always)]
    #[must_use]
    pub fn rfod(&mut self) -> RFOD_W<CTL_SPEC, 3> {
        RFOD_W::new(self)
    }
    #[doc = "Bit 4 - Automatic retransmission disable"]
    #[inline(always)]
    #[must_use]
    pub fn ard(&mut self) -> ARD_W<CTL_SPEC, 4> {
        ARD_W::new(self)
    }
    #[doc = "Bit 5 - Automatic wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn awu(&mut self) -> AWU_W<CTL_SPEC, 5> {
        AWU_W::new(self)
    }
    #[doc = "Bit 6 - Automatic bus-off recovery"]
    #[inline(always)]
    #[must_use]
    pub fn abor(&mut self) -> ABOR_W<CTL_SPEC, 6> {
        ABOR_W::new(self)
    }
    #[doc = "Bit 7 - Time-triggered communication"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<CTL_SPEC, 7> {
        TTC_W::new(self)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CTL_SPEC, 15> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 16 - Debug freeze"]
    #[inline(always)]
    #[must_use]
    pub fn dfz(&mut self) -> DFZ_W<CTL_SPEC, 16> {
        DFZ_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0001_0002"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0002;
}
