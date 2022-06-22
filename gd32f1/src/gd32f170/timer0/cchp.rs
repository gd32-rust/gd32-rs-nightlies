#[doc = "Register `CCHP` reader"]
pub struct R(crate::R<CCHP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCHP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCHP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCHP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCHP` writer"]
pub struct W(crate::W<CCHP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCHP_SPEC>;
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
impl From<crate::W<CCHP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCHP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Primary output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN_A {
    #[doc = "0: Channel outputs are disabled"]
    DISABLED = 0,
    #[doc = "1: Channel outputs are enabled"]
    ENABLED = 1,
}
impl From<POEN_A> for bool {
    #[inline(always)]
    fn from(variant: POEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POEN` reader - Primary output enable"]
pub type POEN_R = crate::BitReader<POEN_A>;
impl POEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN_A {
        match self.bits {
            false => POEN_A::DISABLED,
            true => POEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == POEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POEN_A::ENABLED
    }
}
#[doc = "Field `POEN` writer - Primary output enable"]
pub type POEN_W<'a> = crate::BitWriter<'a, u16, CCHP_SPEC, POEN_A, 15>;
impl<'a> POEN_W<'a> {
    #[doc = "Channel outputs are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POEN_A::DISABLED)
    }
    #[doc = "Channel outputs are enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POEN_A::ENABLED)
    }
}
#[doc = "Output automatic enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OAEN_A {
    #[doc = "0: POEN cannot be set by hardware"]
    MANUAL = 0,
    #[doc = "1: POEN can be set by hardware automatically at the next update event"]
    AUTOMATIC = 1,
}
impl From<OAEN_A> for bool {
    #[inline(always)]
    fn from(variant: OAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAEN` reader - Output automatic enable"]
pub type OAEN_R = crate::BitReader<OAEN_A>;
impl OAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAEN_A {
        match self.bits {
            false => OAEN_A::MANUAL,
            true => OAEN_A::AUTOMATIC,
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == OAEN_A::MANUAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == OAEN_A::AUTOMATIC
    }
}
#[doc = "Field `OAEN` writer - Output automatic enable"]
pub type OAEN_W<'a> = crate::BitWriter<'a, u16, CCHP_SPEC, OAEN_A, 14>;
impl<'a> OAEN_W<'a> {
    #[doc = "POEN cannot be set by hardware"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(OAEN_A::MANUAL)
    }
    #[doc = "POEN can be set by hardware automatically at the next update event"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(OAEN_A::AUTOMATIC)
    }
}
#[doc = "Break polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKP_A {
    #[doc = "0: BRKIN is active low"]
    INVERTED = 0,
    #[doc = "1: BRKIN is active high"]
    NOTINVERTED = 1,
}
impl From<BRKP_A> for bool {
    #[inline(always)]
    fn from(variant: BRKP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKP` reader - Break polarity"]
pub type BRKP_R = crate::BitReader<BRKP_A>;
impl BRKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKP_A {
        match self.bits {
            false => BRKP_A::INVERTED,
            true => BRKP_A::NOTINVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BRKP_A::INVERTED
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BRKP_A::NOTINVERTED
    }
}
#[doc = "Field `BRKP` writer - Break polarity"]
pub type BRKP_W<'a> = crate::BitWriter<'a, u16, CCHP_SPEC, BRKP_A, 13>;
impl<'a> BRKP_W<'a> {
    #[doc = "BRKIN is active low"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(BRKP_A::INVERTED)
    }
    #[doc = "BRKIN is active high"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(BRKP_A::NOTINVERTED)
    }
}
#[doc = "Break enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKEN_A {
    #[doc = "0: Break inputs disabled"]
    DISABLED = 0,
    #[doc = "1: Break inputs enabled"]
    ENABLED = 1,
}
impl From<BRKEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEN` reader - Break enable"]
pub type BRKEN_R = crate::BitReader<BRKEN_A>;
impl BRKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKEN_A {
        match self.bits {
            false => BRKEN_A::DISABLED,
            true => BRKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRKEN_A::ENABLED
    }
}
#[doc = "Field `BRKEN` writer - Break enable"]
pub type BRKEN_W<'a> = crate::BitWriter<'a, u16, CCHP_SPEC, BRKEN_A, 12>;
impl<'a> BRKEN_W<'a> {
    #[doc = "Break inputs disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BRKEN_A::DISABLED)
    }
    #[doc = "Break inputs enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BRKEN_A::ENABLED)
    }
}
#[doc = "Run mode off-state configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROS_A {
    #[doc = "0: When POEN is set, the channel output signals are disabled"]
    DISABLED = 0,
    #[doc = "1: When POEN is set, the channel output signals are enabled"]
    ENABLED = 1,
}
impl From<ROS_A> for bool {
    #[inline(always)]
    fn from(variant: ROS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROS` reader - Run mode off-state configure"]
pub type ROS_R = crate::BitReader<ROS_A>;
impl ROS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROS_A {
        match self.bits {
            false => ROS_A::DISABLED,
            true => ROS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROS_A::ENABLED
    }
}
#[doc = "Field `ROS` writer - Run mode off-state configure"]
pub type ROS_W<'a> = crate::BitWriter<'a, u16, CCHP_SPEC, ROS_A, 11>;
impl<'a> ROS_W<'a> {
    #[doc = "When POEN is set, the channel output signals are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROS_A::DISABLED)
    }
    #[doc = "When POEN is set, the channel output signals are enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROS_A::ENABLED)
    }
}
#[doc = "Idle mode off-state configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOS_A {
    #[doc = "0: When POEN is reset, the channel output signals are disabled"]
    DISABLED = 0,
    #[doc = "1: When POEN is reset, the channel output signals are enabled"]
    ENABLED = 1,
}
impl From<IOS_A> for bool {
    #[inline(always)]
    fn from(variant: IOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOS` reader - Idle mode off-state configure"]
pub type IOS_R = crate::BitReader<IOS_A>;
impl IOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOS_A {
        match self.bits {
            false => IOS_A::DISABLED,
            true => IOS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOS_A::ENABLED
    }
}
#[doc = "Field `IOS` writer - Idle mode off-state configure"]
pub type IOS_W<'a> = crate::BitWriter<'a, u16, CCHP_SPEC, IOS_A, 10>;
impl<'a> IOS_W<'a> {
    #[doc = "When POEN is reset, the channel output signals are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOS_A::DISABLED)
    }
    #[doc = "When POEN is reset, the channel output signals are enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOS_A::ENABLED)
    }
}
#[doc = "Complementary register protect control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PROT_A {
    #[doc = "0: Write protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection mode 0"]
    MODE0 = 1,
    #[doc = "2: Protection mode 1"]
    MODE1 = 2,
    #[doc = "3: Protection mode 2"]
    MODE2 = 3,
}
impl From<PROT_A> for u8 {
    #[inline(always)]
    fn from(variant: PROT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PROT` reader - Complementary register protect control"]
pub type PROT_R = crate::FieldReader<u8, PROT_A>;
impl PROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROT_A {
        match self.bits {
            0 => PROT_A::DISABLED,
            1 => PROT_A::MODE0,
            2 => PROT_A::MODE1,
            3 => PROT_A::MODE2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PROT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == PROT_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == PROT_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == PROT_A::MODE2
    }
}
#[doc = "Field `PROT` writer - Complementary register protect control"]
pub type PROT_W<'a> = crate::FieldWriterSafe<'a, u16, CCHP_SPEC, u8, PROT_A, 2, 8>;
impl<'a> PROT_W<'a> {
    #[doc = "Write protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PROT_A::DISABLED)
    }
    #[doc = "Protection mode 0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(PROT_A::MODE0)
    }
    #[doc = "Protection mode 1"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(PROT_A::MODE1)
    }
    #[doc = "Protection mode 2"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(PROT_A::MODE2)
    }
}
#[doc = "Field `DTCFG` reader - Dead time configure"]
pub type DTCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTCFG` writer - Dead time configure"]
pub type DTCFG_W<'a> = crate::FieldWriterSafe<'a, u16, CCHP_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bit 15 - Primary output enable"]
    #[inline(always)]
    pub fn poen(&self) -> POEN_R {
        POEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Output automatic enable"]
    #[inline(always)]
    pub fn oaen(&self) -> OAEN_R {
        OAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn brkp(&self) -> BRKP_R {
        BRKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn brken(&self) -> BRKEN_R {
        BRKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Run mode off-state configure"]
    #[inline(always)]
    pub fn ros(&self) -> ROS_R {
        ROS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Idle mode off-state configure"]
    #[inline(always)]
    pub fn ios(&self) -> IOS_R {
        IOS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Complementary register protect control"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 0:7 - Dead time configure"]
    #[inline(always)]
    pub fn dtcfg(&self) -> DTCFG_R {
        DTCFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Primary output enable"]
    #[inline(always)]
    pub fn poen(&mut self) -> POEN_W {
        POEN_W::new(self)
    }
    #[doc = "Bit 14 - Output automatic enable"]
    #[inline(always)]
    pub fn oaen(&mut self) -> OAEN_W {
        OAEN_W::new(self)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn brkp(&mut self) -> BRKP_W {
        BRKP_W::new(self)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn brken(&mut self) -> BRKEN_W {
        BRKEN_W::new(self)
    }
    #[doc = "Bit 11 - Run mode off-state configure"]
    #[inline(always)]
    pub fn ros(&mut self) -> ROS_W {
        ROS_W::new(self)
    }
    #[doc = "Bit 10 - Idle mode off-state configure"]
    #[inline(always)]
    pub fn ios(&mut self) -> IOS_W {
        IOS_W::new(self)
    }
    #[doc = "Bits 8:9 - Complementary register protect control"]
    #[inline(always)]
    pub fn prot(&mut self) -> PROT_W {
        PROT_W::new(self)
    }
    #[doc = "Bits 0:7 - Dead time configure"]
    #[inline(always)]
    pub fn dtcfg(&mut self) -> DTCFG_W {
        DTCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel complementary protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cchp](index.html) module"]
pub struct CCHP_SPEC;
impl crate::RegisterSpec for CCHP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cchp::R](R) reader structure"]
impl crate::Readable for CCHP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cchp::W](W) writer structure"]
impl crate::Writable for CCHP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCHP to value 0"]
impl crate::Resettable for CCHP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
