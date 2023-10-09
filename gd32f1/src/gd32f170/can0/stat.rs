#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `IWS` reader - Initial working state"]
pub type IWS_R = crate::BitReader<IWS_A>;
#[doc = "Initial working state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWS_A {
    #[doc = "0: CAN is not in initial working mode"]
    NORMAL = 0,
    #[doc = "1: CAN is in initial working mode"]
    INITIAL = 1,
}
impl From<IWS_A> for bool {
    #[inline(always)]
    fn from(variant: IWS_A) -> Self {
        variant as u8 != 0
    }
}
impl IWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWS_A {
        match self.bits {
            false => IWS_A::NORMAL,
            true => IWS_A::INITIAL,
        }
    }
    #[doc = "CAN is not in initial working mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IWS_A::NORMAL
    }
    #[doc = "CAN is in initial working mode"]
    #[inline(always)]
    pub fn is_initial(&self) -> bool {
        *self == IWS_A::INITIAL
    }
}
#[doc = "Field `SLPWS` reader - Sleep working state"]
pub type SLPWS_R = crate::BitReader<SLPWS_A>;
#[doc = "Sleep working state\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLPWS_A {
    #[doc = "0: CAN is not in sleep working mode"]
    NORMAL = 0,
    #[doc = "1: CAN is in sleep working mode"]
    SLEEPING = 1,
}
impl From<SLPWS_A> for bool {
    #[inline(always)]
    fn from(variant: SLPWS_A) -> Self {
        variant as u8 != 0
    }
}
impl SLPWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPWS_A {
        match self.bits {
            false => SLPWS_A::NORMAL,
            true => SLPWS_A::SLEEPING,
        }
    }
    #[doc = "CAN is not in sleep working mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SLPWS_A::NORMAL
    }
    #[doc = "CAN is in sleep working mode"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == SLPWS_A::SLEEPING
    }
}
#[doc = "Field `ERRIF` reader - Error interrupt flag"]
pub type ERRIF_R = crate::BitReader<ERRIFR_A>;
#[doc = "Error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIFR_A {
    #[doc = "0: There was no error"]
    NO_ERROR = 0,
    #[doc = "1: An error was detected"]
    ERROR = 1,
}
impl From<ERRIFR_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIFR_A {
        match self.bits {
            false => ERRIFR_A::NO_ERROR,
            true => ERRIFR_A::ERROR,
        }
    }
    #[doc = "There was no error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERRIFR_A::NO_ERROR
    }
    #[doc = "An error was detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERRIFR_A::ERROR
    }
}
#[doc = "Error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIFW_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<ERRIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIF` writer - Error interrupt flag"]
pub type ERRIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERRIFW_AW>;
impl<'a, REG, const O: u8> ERRIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIFW_AW::CLEAR)
    }
}
#[doc = "Field `WUIF` reader - Status change interrupt flag of wakeup from sleep working mode"]
pub type WUIF_R = crate::BitReader<WUIFR_A>;
#[doc = "Status change interrupt flag of wakeup from sleep working mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUIFR_A {
    #[doc = "0: No wakeup event"]
    NO_WAKEUP = 0,
    #[doc = "1: Wakeup event"]
    WAKEUP = 1,
}
impl From<WUIFR_A> for bool {
    #[inline(always)]
    fn from(variant: WUIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl WUIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUIFR_A {
        match self.bits {
            false => WUIFR_A::NO_WAKEUP,
            true => WUIFR_A::WAKEUP,
        }
    }
    #[doc = "No wakeup event"]
    #[inline(always)]
    pub fn is_no_wakeup(&self) -> bool {
        *self == WUIFR_A::NO_WAKEUP
    }
    #[doc = "Wakeup event"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUIFR_A::WAKEUP
    }
}
#[doc = "Status change interrupt flag of wakeup from sleep working mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUIFW_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<WUIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: WUIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUIF` writer - Status change interrupt flag of wakeup from sleep working mode"]
pub type WUIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WUIFW_AW>;
impl<'a, REG, const O: u8> WUIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WUIFW_AW::CLEAR)
    }
}
#[doc = "Field `SLPIF` reader - Status change interrupt flag of sleep working mode entering"]
pub type SLPIF_R = crate::BitReader<SLPIFR_A>;
#[doc = "Status change interrupt flag of sleep working mode entering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLPIFR_A {
    #[doc = "0: CAN is not entering sleep working mode"]
    AWAKE = 0,
    #[doc = "1: CAN is entering sleep working mode"]
    SLEEPING = 1,
}
impl From<SLPIFR_A> for bool {
    #[inline(always)]
    fn from(variant: SLPIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SLPIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPIFR_A {
        match self.bits {
            false => SLPIFR_A::AWAKE,
            true => SLPIFR_A::SLEEPING,
        }
    }
    #[doc = "CAN is not entering sleep working mode"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == SLPIFR_A::AWAKE
    }
    #[doc = "CAN is entering sleep working mode"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == SLPIFR_A::SLEEPING
    }
}
#[doc = "Status change interrupt flag of sleep working mode entering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLPIFW_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<SLPIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: SLPIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPIF` writer - Status change interrupt flag of sleep working mode entering"]
pub type SLPIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLPIFW_AW>;
impl<'a, REG, const O: u8> SLPIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SLPIFW_AW::CLEAR)
    }
}
#[doc = "Field `TS` reader - Transmitting state"]
pub type TS_R = crate::BitReader<TS_A>;
#[doc = "Transmitting state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS_A {
    #[doc = "0: CAN is not working in transmitting state"]
    NOT_WORKING = 0,
    #[doc = "1: CAN is working in transmitting state"]
    WORKING = 1,
}
impl From<TS_A> for bool {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as u8 != 0
    }
}
impl TS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS_A {
        match self.bits {
            false => TS_A::NOT_WORKING,
            true => TS_A::WORKING,
        }
    }
    #[doc = "CAN is not working in transmitting state"]
    #[inline(always)]
    pub fn is_not_working(&self) -> bool {
        *self == TS_A::NOT_WORKING
    }
    #[doc = "CAN is working in transmitting state"]
    #[inline(always)]
    pub fn is_working(&self) -> bool {
        *self == TS_A::WORKING
    }
}
#[doc = "Field `RS` reader - Receiving state"]
pub type RS_R = crate::BitReader<RS_A>;
#[doc = "Receiving state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RS_A {
    #[doc = "0: CAN is not working in receiving state"]
    NOT_WORKING = 0,
    #[doc = "1: CAN is working in receiving state"]
    WORKING = 1,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
impl RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS_A {
        match self.bits {
            false => RS_A::NOT_WORKING,
            true => RS_A::WORKING,
        }
    }
    #[doc = "CAN is not working in receiving state"]
    #[inline(always)]
    pub fn is_not_working(&self) -> bool {
        *self == RS_A::NOT_WORKING
    }
    #[doc = "CAN is working in receiving state"]
    #[inline(always)]
    pub fn is_working(&self) -> bool {
        *self == RS_A::WORKING
    }
}
#[doc = "Field `LASTRX` reader - Last sample value of Rx pin"]
pub type LASTRX_R = crate::BitReader;
#[doc = "Field `RXL` reader - RX level"]
pub type RXL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Initial working state"]
    #[inline(always)]
    pub fn iws(&self) -> IWS_R {
        IWS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep working state"]
    #[inline(always)]
    pub fn slpws(&self) -> SLPWS_R {
        SLPWS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status change interrupt flag of wakeup from sleep working mode"]
    #[inline(always)]
    pub fn wuif(&self) -> WUIF_R {
        WUIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status change interrupt flag of sleep working mode entering"]
    #[inline(always)]
    pub fn slpif(&self) -> SLPIF_R {
        SLPIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitting state"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receiving state"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Last sample value of Rx pin"]
    #[inline(always)]
    pub fn lastrx(&self) -> LASTRX_R {
        LASTRX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX level"]
    #[inline(always)]
    pub fn rxl(&self) -> RXL_R {
        RXL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn errif(&mut self) -> ERRIF_W<STAT_SPEC, 2> {
        ERRIF_W::new(self)
    }
    #[doc = "Bit 3 - Status change interrupt flag of wakeup from sleep working mode"]
    #[inline(always)]
    #[must_use]
    pub fn wuif(&mut self) -> WUIF_W<STAT_SPEC, 3> {
        WUIF_W::new(self)
    }
    #[doc = "Bit 4 - Status change interrupt flag of sleep working mode entering"]
    #[inline(always)]
    #[must_use]
    pub fn slpif(&mut self) -> SLPIF_W<STAT_SPEC, 4> {
        SLPIF_W::new(self)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0x0c02"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c02;
}
