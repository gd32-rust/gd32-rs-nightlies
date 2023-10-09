#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `WDE` reader - Analog watchdog event flag"]
pub type WDE_R = crate::BitReader<WDER_A>;
#[doc = "Analog watchdog event flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDER_A {
    #[doc = "0: No analog watchdog event occurred"]
    NO_EVENT = 0,
    #[doc = "1: Analog watchdog event occurred"]
    EVENT = 1,
}
impl From<WDER_A> for bool {
    #[inline(always)]
    fn from(variant: WDER_A) -> Self {
        variant as u8 != 0
    }
}
impl WDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDER_A {
        match self.bits {
            false => WDER_A::NO_EVENT,
            true => WDER_A::EVENT,
        }
    }
    #[doc = "No analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == WDER_A::NO_EVENT
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == WDER_A::EVENT
    }
}
#[doc = "Analog watchdog event flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDEW_AW {
    #[doc = "0: Clear the analog watchdog event flag"]
    CLEAR = 0,
}
impl From<WDEW_AW> for bool {
    #[inline(always)]
    fn from(variant: WDEW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDE` writer - Analog watchdog event flag"]
pub type WDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDEW_AW>;
impl<'a, REG, const O: u8> WDE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the analog watchdog event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WDEW_AW::CLEAR)
    }
}
#[doc = "Field `EOC` reader - End of group conversion flag"]
pub type EOC_R = crate::BitReader<EOCR_A>;
#[doc = "End of group conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR_A {
    #[doc = "0: Conversion is not complete"]
    NOT_COMPLETE = 0,
    #[doc = "1: Conversion complete"]
    COMPLETE = 1,
}
impl From<EOCR_A> for bool {
    #[inline(always)]
    fn from(variant: EOCR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCR_A {
        match self.bits {
            false => EOCR_A::NOT_COMPLETE,
            true => EOCR_A::COMPLETE,
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR_A::NOT_COMPLETE
    }
    #[doc = "Conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR_A::COMPLETE
    }
}
#[doc = "End of group conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW_AW {
    #[doc = "0: Clear end of group conversion flag"]
    CLEAR = 0,
}
impl From<EOCW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` writer - End of group conversion flag"]
pub type EOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EOCW_AW>;
impl<'a, REG, const O: u8> EOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear end of group conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCW_AW::CLEAR)
    }
}
#[doc = "Field `EOIC` reader - End of inserted group conversion flag"]
pub type EOIC_R = crate::BitReader<EOICR_A>;
#[doc = "End of inserted group conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOICR_A {
    #[doc = "0: Conversion is not complete"]
    NOT_COMPLETE = 0,
    #[doc = "1: Conversion complete"]
    COMPLETE = 1,
}
impl From<EOICR_A> for bool {
    #[inline(always)]
    fn from(variant: EOICR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOICR_A {
        match self.bits {
            false => EOICR_A::NOT_COMPLETE,
            true => EOICR_A::COMPLETE,
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOICR_A::NOT_COMPLETE
    }
    #[doc = "Conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOICR_A::COMPLETE
    }
}
#[doc = "End of inserted group conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOICW_AW {
    #[doc = "0: Clear end of inserted group conversion flag"]
    CLEAR = 0,
}
impl From<EOICW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOICW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOIC` writer - End of inserted group conversion flag"]
pub type EOIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EOICW_AW>;
impl<'a, REG, const O: u8> EOIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear end of inserted group conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOICW_AW::CLEAR)
    }
}
#[doc = "Field `STIC` reader - Start flag of inserted channel group"]
pub type STIC_R = crate::BitReader<STICR_A>;
#[doc = "Start flag of inserted channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STICR_A {
    #[doc = "0: No inserted channel group conversion started"]
    NOT_STARTED = 0,
    #[doc = "1: Inserted channel group conversion has started"]
    STARTED = 1,
}
impl From<STICR_A> for bool {
    #[inline(always)]
    fn from(variant: STICR_A) -> Self {
        variant as u8 != 0
    }
}
impl STIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STICR_A {
        match self.bits {
            false => STICR_A::NOT_STARTED,
            true => STICR_A::STARTED,
        }
    }
    #[doc = "No inserted channel group conversion started"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STICR_A::NOT_STARTED
    }
    #[doc = "Inserted channel group conversion has started"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STICR_A::STARTED
    }
}
#[doc = "Start flag of inserted channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STICW_AW {
    #[doc = "0: Clear the inserted channel group start flag"]
    CLEAR = 0,
}
impl From<STICW_AW> for bool {
    #[inline(always)]
    fn from(variant: STICW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STIC` writer - Start flag of inserted channel group"]
pub type STIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STICW_AW>;
impl<'a, REG, const O: u8> STIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the inserted channel group start flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(STICW_AW::CLEAR)
    }
}
#[doc = "Field `STRC` reader - Start flag of regular channel group"]
pub type STRC_R = crate::BitReader<STRCR_A>;
#[doc = "Start flag of regular channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRCR_A {
    #[doc = "0: No regular channel conversion started"]
    NOT_STARTED = 0,
    #[doc = "1: Regular channel conversion has started"]
    STARTED = 1,
}
impl From<STRCR_A> for bool {
    #[inline(always)]
    fn from(variant: STRCR_A) -> Self {
        variant as u8 != 0
    }
}
impl STRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRCR_A {
        match self.bits {
            false => STRCR_A::NOT_STARTED,
            true => STRCR_A::STARTED,
        }
    }
    #[doc = "No regular channel conversion started"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRCR_A::NOT_STARTED
    }
    #[doc = "Regular channel conversion has started"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRCR_A::STARTED
    }
}
#[doc = "Start flag of regular channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRCW_AW {
    #[doc = "0: Clear the regular channel start flag"]
    CLEAR = 0,
}
impl From<STRCW_AW> for bool {
    #[inline(always)]
    fn from(variant: STRCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRC` writer - Start flag of regular channel group"]
pub type STRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STRCW_AW>;
impl<'a, REG, const O: u8> STRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the regular channel start flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(STRCW_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Analog watchdog event flag"]
    #[inline(always)]
    pub fn wde(&self) -> WDE_R {
        WDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of group conversion flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of inserted group conversion flag"]
    #[inline(always)]
    pub fn eoic(&self) -> EOIC_R {
        EOIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start flag of inserted channel group"]
    #[inline(always)]
    pub fn stic(&self) -> STIC_R {
        STIC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Start flag of regular channel group"]
    #[inline(always)]
    pub fn strc(&self) -> STRC_R {
        STRC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog watchdog event flag"]
    #[inline(always)]
    #[must_use]
    pub fn wde(&mut self) -> WDE_W<STAT_SPEC, 0> {
        WDE_W::new(self)
    }
    #[doc = "Bit 1 - End of group conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<STAT_SPEC, 1> {
        EOC_W::new(self)
    }
    #[doc = "Bit 2 - End of inserted group conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoic(&mut self) -> EOIC_W<STAT_SPEC, 2> {
        EOIC_W::new(self)
    }
    #[doc = "Bit 3 - Start flag of inserted channel group"]
    #[inline(always)]
    #[must_use]
    pub fn stic(&mut self) -> STIC_W<STAT_SPEC, 3> {
        STIC_W::new(self)
    }
    #[doc = "Bit 4 - Start flag of regular channel group"]
    #[inline(always)]
    #[must_use]
    pub fn strc(&mut self) -> STRC_W<STAT_SPEC, 4> {
        STRC_W::new(self)
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
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
