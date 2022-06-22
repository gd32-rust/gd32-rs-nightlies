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
#[doc = "Start flag of regular channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRC_A {
    #[doc = "0: No regular channel conversion started"]
    NOTSTARTED = 0,
    #[doc = "1: Regular channel conversion has started"]
    STARTED = 1,
}
impl From<STRC_A> for bool {
    #[inline(always)]
    fn from(variant: STRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRC` reader - Start flag of regular channel group"]
pub type STRC_R = crate::BitReader<STRC_A>;
impl STRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRC_A {
        match self.bits {
            false => STRC_A::NOTSTARTED,
            true => STRC_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRC_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRC_A::STARTED
    }
}
#[doc = "Start flag of regular channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRC_AW {
    #[doc = "0: Clear the regular channel start flag"]
    CLEAR = 0,
}
impl From<STRC_AW> for bool {
    #[inline(always)]
    fn from(variant: STRC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRC` writer - Start flag of regular channel group"]
pub type STRC_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, STRC_AW, 4>;
impl<'a> STRC_W<'a> {
    #[doc = "Clear the regular channel start flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STRC_AW::CLEAR)
    }
}
#[doc = "Start flag of inserted channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STIC_A {
    #[doc = "0: No inserted channel group conversion started"]
    NOTSTARTED = 0,
    #[doc = "1: Inserted channel group conversion has started"]
    STARTED = 1,
}
impl From<STIC_A> for bool {
    #[inline(always)]
    fn from(variant: STIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STIC` reader - Start flag of inserted channel group"]
pub type STIC_R = crate::BitReader<STIC_A>;
impl STIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STIC_A {
        match self.bits {
            false => STIC_A::NOTSTARTED,
            true => STIC_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STIC_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STIC_A::STARTED
    }
}
#[doc = "Start flag of inserted channel group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STIC_AW {
    #[doc = "0: Clear the inserted channel group start flag"]
    CLEAR = 0,
}
impl From<STIC_AW> for bool {
    #[inline(always)]
    fn from(variant: STIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STIC` writer - Start flag of inserted channel group"]
pub type STIC_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, STIC_AW, 3>;
impl<'a> STIC_W<'a> {
    #[doc = "Clear the inserted channel group start flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STIC_AW::CLEAR)
    }
}
#[doc = "End of inserted group conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOIC_A {
    #[doc = "0: Conversion is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Conversion complete"]
    COMPLETE = 1,
}
impl From<EOIC_A> for bool {
    #[inline(always)]
    fn from(variant: EOIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOIC` reader - End of inserted group conversion flag"]
pub type EOIC_R = crate::BitReader<EOIC_A>;
impl EOIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOIC_A {
        match self.bits {
            false => EOIC_A::NOTCOMPLETE,
            true => EOIC_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOIC_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOIC_A::COMPLETE
    }
}
#[doc = "End of inserted group conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOIC_AW {
    #[doc = "0: Clear end of inserted group conversion flag"]
    CLEAR = 0,
}
impl From<EOIC_AW> for bool {
    #[inline(always)]
    fn from(variant: EOIC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOIC` writer - End of inserted group conversion flag"]
pub type EOIC_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, EOIC_AW, 2>;
impl<'a> EOIC_W<'a> {
    #[doc = "Clear end of inserted group conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOIC_AW::CLEAR)
    }
}
#[doc = "End of group conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_A {
    #[doc = "0: Conversion is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Conversion complete"]
    COMPLETE = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` reader - End of group conversion flag"]
pub type EOC_R = crate::BitReader<EOC_A>;
impl EOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::NOTCOMPLETE,
            true => EOC_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC_A::COMPLETE
    }
}
#[doc = "End of group conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_AW {
    #[doc = "0: Clear end of group conversion flag"]
    CLEAR = 0,
}
impl From<EOC_AW> for bool {
    #[inline(always)]
    fn from(variant: EOC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` writer - End of group conversion flag"]
pub type EOC_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, EOC_AW, 1>;
impl<'a> EOC_W<'a> {
    #[doc = "Clear end of group conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOC_AW::CLEAR)
    }
}
#[doc = "Analog watchdog event flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDE_A {
    #[doc = "0: No analog watchdog event occurred"]
    NOEVENT = 0,
    #[doc = "1: Analog watchdog event occurred"]
    EVENT = 1,
}
impl From<WDE_A> for bool {
    #[inline(always)]
    fn from(variant: WDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDE` reader - Analog watchdog event flag"]
pub type WDE_R = crate::BitReader<WDE_A>;
impl WDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDE_A {
        match self.bits {
            false => WDE_A::NOEVENT,
            true => WDE_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == WDE_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == WDE_A::EVENT
    }
}
#[doc = "Analog watchdog event flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDE_AW {
    #[doc = "0: Clear the analog watchdog event flag"]
    CLEAR = 0,
}
impl From<WDE_AW> for bool {
    #[inline(always)]
    fn from(variant: WDE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDE` writer - Analog watchdog event flag"]
pub type WDE_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, WDE_AW, 0>;
impl<'a> WDE_W<'a> {
    #[doc = "Clear the analog watchdog event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WDE_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 4 - Start flag of regular channel group"]
    #[inline(always)]
    pub fn strc(&self) -> STRC_R {
        STRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Start flag of inserted channel group"]
    #[inline(always)]
    pub fn stic(&self) -> STIC_R {
        STIC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - End of inserted group conversion flag"]
    #[inline(always)]
    pub fn eoic(&self) -> EOIC_R {
        EOIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - End of group conversion flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Analog watchdog event flag"]
    #[inline(always)]
    pub fn wde(&self) -> WDE_R {
        WDE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Start flag of regular channel group"]
    #[inline(always)]
    pub fn strc(&mut self) -> STRC_W {
        STRC_W::new(self)
    }
    #[doc = "Bit 3 - Start flag of inserted channel group"]
    #[inline(always)]
    pub fn stic(&mut self) -> STIC_W {
        STIC_W::new(self)
    }
    #[doc = "Bit 2 - End of inserted group conversion flag"]
    #[inline(always)]
    pub fn eoic(&mut self) -> EOIC_W {
        EOIC_W::new(self)
    }
    #[doc = "Bit 1 - End of group conversion flag"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W {
        EOC_W::new(self)
    }
    #[doc = "Bit 0 - Analog watchdog event flag"]
    #[inline(always)]
    pub fn wde(&mut self) -> WDE_W {
        WDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
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
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
