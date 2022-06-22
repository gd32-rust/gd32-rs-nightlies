#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXL` reader - RX level"]
pub type RXL_R = crate::BitReader<bool>;
#[doc = "Field `LASTRX` reader - Last sample value of Rx pin"]
pub type LASTRX_R = crate::BitReader<bool>;
#[doc = "Receiving state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RS_A {
    #[doc = "0: CAN is not working in receiving state"]
    NOTWORKING = 0,
    #[doc = "1: CAN is working in receiving state"]
    WORKING = 1,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS` reader - Receiving state"]
pub type RS_R = crate::BitReader<RS_A>;
impl RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS_A {
        match self.bits {
            false => RS_A::NOTWORKING,
            true => RS_A::WORKING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTWORKING`"]
    #[inline(always)]
    pub fn is_not_working(&self) -> bool {
        *self == RS_A::NOTWORKING
    }
    #[doc = "Checks if the value of the field is `WORKING`"]
    #[inline(always)]
    pub fn is_working(&self) -> bool {
        *self == RS_A::WORKING
    }
}
#[doc = "Transmitting state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS_A {
    #[doc = "0: CAN is not working in transmitting state"]
    NOTWORKING = 0,
    #[doc = "1: CAN is working in transmitting state"]
    WORKING = 1,
}
impl From<TS_A> for bool {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS` reader - Transmitting state"]
pub type TS_R = crate::BitReader<TS_A>;
impl TS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS_A {
        match self.bits {
            false => TS_A::NOTWORKING,
            true => TS_A::WORKING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTWORKING`"]
    #[inline(always)]
    pub fn is_not_working(&self) -> bool {
        *self == TS_A::NOTWORKING
    }
    #[doc = "Checks if the value of the field is `WORKING`"]
    #[inline(always)]
    pub fn is_working(&self) -> bool {
        *self == TS_A::WORKING
    }
}
#[doc = "Status change interrupt flag of sleep working mode entering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPIF_A {
    #[doc = "0: CAN is not entering sleep working mode"]
    AWAKE = 0,
    #[doc = "1: CAN is entering sleep working mode"]
    SLEEPING = 1,
}
impl From<SLPIF_A> for bool {
    #[inline(always)]
    fn from(variant: SLPIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPIF` reader - Status change interrupt flag of sleep working mode entering"]
pub type SLPIF_R = crate::BitReader<SLPIF_A>;
impl SLPIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPIF_A {
        match self.bits {
            false => SLPIF_A::AWAKE,
            true => SLPIF_A::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == SLPIF_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == SLPIF_A::SLEEPING
    }
}
#[doc = "Status change interrupt flag of sleep working mode entering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPIF_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<SLPIF_AW> for bool {
    #[inline(always)]
    fn from(variant: SLPIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPIF` writer - Status change interrupt flag of sleep working mode entering"]
pub type SLPIF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, SLPIF_AW, 4>;
impl<'a> SLPIF_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SLPIF_AW::CLEAR)
    }
}
#[doc = "Status change interrupt flag of wakeup from sleep working mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUIF_A {
    #[doc = "0: No wakeup event"]
    NOWAKEUP = 0,
    #[doc = "1: Wakeup event"]
    WAKEUP = 1,
}
impl From<WUIF_A> for bool {
    #[inline(always)]
    fn from(variant: WUIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUIF` reader - Status change interrupt flag of wakeup from sleep working mode"]
pub type WUIF_R = crate::BitReader<WUIF_A>;
impl WUIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUIF_A {
        match self.bits {
            false => WUIF_A::NOWAKEUP,
            true => WUIF_A::WAKEUP,
        }
    }
    #[doc = "Checks if the value of the field is `NOWAKEUP`"]
    #[inline(always)]
    pub fn is_no_wakeup(&self) -> bool {
        *self == WUIF_A::NOWAKEUP
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUIF_A::WAKEUP
    }
}
#[doc = "Status change interrupt flag of wakeup from sleep working mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUIF_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<WUIF_AW> for bool {
    #[inline(always)]
    fn from(variant: WUIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUIF` writer - Status change interrupt flag of wakeup from sleep working mode"]
pub type WUIF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, WUIF_AW, 3>;
impl<'a> WUIF_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUIF_AW::CLEAR)
    }
}
#[doc = "Error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIF_A {
    #[doc = "0: There was no error"]
    NOERROR = 0,
    #[doc = "1: An error was detected"]
    ERROR = 1,
}
impl From<ERRIF_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIF` reader - Error interrupt flag"]
pub type ERRIF_R = crate::BitReader<ERRIF_A>;
impl ERRIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIF_A {
        match self.bits {
            false => ERRIF_A::NOERROR,
            true => ERRIF_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERRIF_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERRIF_A::ERROR
    }
}
#[doc = "Error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIF_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<ERRIF_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIF` writer - Error interrupt flag"]
pub type ERRIF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, ERRIF_AW, 2>;
impl<'a> ERRIF_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERRIF_AW::CLEAR)
    }
}
#[doc = "Sleep working state\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SLPWS` reader - Sleep working state"]
pub type SLPWS_R = crate::BitReader<SLPWS_A>;
impl SLPWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPWS_A {
        match self.bits {
            false => SLPWS_A::NORMAL,
            true => SLPWS_A::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SLPWS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == SLPWS_A::SLEEPING
    }
}
#[doc = "Initial working state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `IWS` reader - Initial working state"]
pub type IWS_R = crate::BitReader<IWS_A>;
impl IWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWS_A {
        match self.bits {
            false => IWS_A::NORMAL,
            true => IWS_A::INITIAL,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IWS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INITIAL`"]
    #[inline(always)]
    pub fn is_initial(&self) -> bool {
        *self == IWS_A::INITIAL
    }
}
impl R {
    #[doc = "Bit 11 - RX level"]
    #[inline(always)]
    pub fn rxl(&self) -> RXL_R {
        RXL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Last sample value of Rx pin"]
    #[inline(always)]
    pub fn lastrx(&self) -> LASTRX_R {
        LASTRX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Receiving state"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitting state"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 4 - Status change interrupt flag of sleep working mode entering"]
    #[inline(always)]
    pub fn slpif(&self) -> SLPIF_R {
        SLPIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Status change interrupt flag of wakeup from sleep working mode"]
    #[inline(always)]
    pub fn wuif(&self) -> WUIF_R {
        WUIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep working state"]
    #[inline(always)]
    pub fn slpws(&self) -> SLPWS_R {
        SLPWS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Initial working state"]
    #[inline(always)]
    pub fn iws(&self) -> IWS_R {
        IWS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Status change interrupt flag of sleep working mode entering"]
    #[inline(always)]
    pub fn slpif(&mut self) -> SLPIF_W {
        SLPIF_W::new(self)
    }
    #[doc = "Bit 3 - Status change interrupt flag of wakeup from sleep working mode"]
    #[inline(always)]
    pub fn wuif(&mut self) -> WUIF_W {
        WUIF_W::new(self)
    }
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&mut self) -> ERRIF_W {
        ERRIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0x0c02"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c02
    }
}
