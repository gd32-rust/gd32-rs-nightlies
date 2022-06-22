#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMEN_A {
    #[doc = "0: LIN mode disabled"]
    DISABLED = 0,
    #[doc = "1: LIN mode enabled"]
    ENABLED = 1,
}
impl From<LMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LMEN` reader - LIN mode enable"]
pub type LMEN_R = crate::BitReader<LMEN_A>;
impl LMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMEN_A {
        match self.bits {
            false => LMEN_A::DISABLED,
            true => LMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LMEN_A::ENABLED
    }
}
#[doc = "Field `LMEN` writer - LIN mode enable"]
pub type LMEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, LMEN_A, 14>;
impl<'a> LMEN_W<'a> {
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LMEN_A::DISABLED)
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LMEN_A::ENABLED)
    }
}
#[doc = "STOP bits length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STB_A {
    #[doc = "0: 1 stop bit"]
    STOP1 = 0,
    #[doc = "1: 0.5 stop bit"]
    STOP0P5 = 1,
    #[doc = "2: 2 stop bit"]
    STOP2 = 2,
    #[doc = "3: 1.5 stop bit"]
    STOP1P5 = 3,
}
impl From<STB_A> for u8 {
    #[inline(always)]
    fn from(variant: STB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STB` reader - STOP bits length"]
pub type STB_R = crate::FieldReader<u8, STB_A>;
impl STB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STB_A {
        match self.bits {
            0 => STB_A::STOP1,
            1 => STB_A::STOP0P5,
            2 => STB_A::STOP2,
            3 => STB_A::STOP1P5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP1`"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STB_A::STOP1
    }
    #[doc = "Checks if the value of the field is `STOP0P5`"]
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STB_A::STOP0P5
    }
    #[doc = "Checks if the value of the field is `STOP2`"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STB_A::STOP2
    }
    #[doc = "Checks if the value of the field is `STOP1P5`"]
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STB_A::STOP1P5
    }
}
#[doc = "Field `STB` writer - STOP bits length"]
pub type STB_W<'a> = crate::FieldWriterSafe<'a, u32, CTL1_SPEC, u8, STB_A, 2, 12>;
impl<'a> STB_W<'a> {
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(STB_A::STOP1)
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut W {
        self.variant(STB_A::STOP0P5)
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(STB_A::STOP2)
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut W {
        self.variant(STB_A::STOP1P5)
    }
}
#[doc = "CK pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKEN_A {
    #[doc = "0: CK pin disabled"]
    DISABLED = 0,
    #[doc = "1: CK pin enabled"]
    ENABLED = 1,
}
impl From<CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKEN` reader - CK pin enable"]
pub type CKEN_R = crate::BitReader<CKEN_A>;
impl CKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKEN_A {
        match self.bits {
            false => CKEN_A::DISABLED,
            true => CKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CKEN_A::ENABLED
    }
}
#[doc = "Field `CKEN` writer - CK pin enable"]
pub type CKEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, CKEN_A, 11>;
impl<'a> CKEN_W<'a> {
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CKEN_A::DISABLED)
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CKEN_A::ENABLED)
    }
}
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPL_A {
    #[doc = "0: Steady low value on CK pin outside tranmission window"]
    NOTINVERTED = 0,
    #[doc = "1: Steady high value on CK pin outside tranmission window"]
    INVERTED = 1,
}
impl From<CPL_A> for bool {
    #[inline(always)]
    fn from(variant: CPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPL` reader - Clock polarity"]
pub type CPL_R = crate::BitReader<CPL_A>;
impl CPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPL_A {
        match self.bits {
            false => CPL_A::NOTINVERTED,
            true => CPL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == CPL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CPL_A::INVERTED
    }
}
#[doc = "Field `CPL` writer - Clock polarity"]
pub type CPL_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, CPL_A, 10>;
impl<'a> CPL_W<'a> {
    #[doc = "Steady low value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(CPL_A::NOTINVERTED)
    }
    #[doc = "Steady high value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CPL_A::INVERTED)
    }
}
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPH_A {
    #[doc = "0: The first clock transition is the first data capture edge"]
    FIRST = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    SECOND = 1,
}
impl From<CPH_A> for bool {
    #[inline(always)]
    fn from(variant: CPH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPH` reader - Clock phase"]
pub type CPH_R = crate::BitReader<CPH_A>;
impl CPH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPH_A {
        match self.bits {
            false => CPH_A::FIRST,
            true => CPH_A::SECOND,
        }
    }
    #[doc = "Checks if the value of the field is `FIRST`"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CPH_A::FIRST
    }
    #[doc = "Checks if the value of the field is `SECOND`"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CPH_A::SECOND
    }
}
#[doc = "Field `CPH` writer - Clock phase"]
pub type CPH_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, CPH_A, 9>;
impl<'a> CPH_W<'a> {
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first(self) -> &'a mut W {
        self.variant(CPH_A::FIRST)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second(self) -> &'a mut W {
        self.variant(CPH_A::SECOND)
    }
}
#[doc = "CK Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEN_A {
    #[doc = "0: The clock pulse of the last data bit is not output to the CK pin"]
    NOTOUTPUT = 0,
    #[doc = "1: The clock pulse of the last data bit is output to the CK pin"]
    OUTPUT = 1,
}
impl From<CLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEN` reader - CK Length"]
pub type CLEN_R = crate::BitReader<CLEN_A>;
impl CLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLEN_A {
        match self.bits {
            false => CLEN_A::NOTOUTPUT,
            true => CLEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTOUTPUT`"]
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == CLEN_A::NOTOUTPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CLEN_A::OUTPUT
    }
}
#[doc = "Field `CLEN` writer - CK Length"]
pub type CLEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, CLEN_A, 8>;
impl<'a> CLEN_W<'a> {
    #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
    #[inline(always)]
    pub fn not_output(self) -> &'a mut W {
        self.variant(CLEN_A::NOTOUTPUT)
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CLEN_A::OUTPUT)
    }
}
#[doc = "LIN break detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: An interrupt is generated whenever LBDF=1 in the STAT register"]
    ENABLED = 1,
}
impl From<LBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader<LBDIE_A>;
impl LBDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBDIE_A {
        match self.bits {
            false => LBDIE_A::DISABLED,
            true => LBDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIE_A::ENABLED
    }
}
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, LBDIE_A, 6>;
impl<'a> LBDIE_W<'a> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBDIE_A::DISABLED)
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBDIE_A::ENABLED)
    }
}
#[doc = "LIN break frame length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBLEN_A {
    #[doc = "0: 10-bit break detection"]
    BIT10 = 0,
    #[doc = "1: 11-bit break detection"]
    BIT11 = 1,
}
impl From<LBLEN_A> for bool {
    #[inline(always)]
    fn from(variant: LBLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBLEN` reader - LIN break frame length"]
pub type LBLEN_R = crate::BitReader<LBLEN_A>;
impl LBLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBLEN_A {
        match self.bits {
            false => LBLEN_A::BIT10,
            true => LBLEN_A::BIT11,
        }
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == LBLEN_A::BIT10
    }
    #[doc = "Checks if the value of the field is `BIT11`"]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == LBLEN_A::BIT11
    }
}
#[doc = "Field `LBLEN` writer - LIN break frame length"]
pub type LBLEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, LBLEN_A, 5>;
impl<'a> LBLEN_W<'a> {
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(LBLEN_A::BIT10)
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut W {
        self.variant(LBLEN_A::BIT11)
    }
}
#[doc = "Address of the USART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADDR_A {
    #[doc = "0: 4-bit address detection"]
    BIT4 = 0,
    #[doc = "1: Full-bit address detection"]
    FULL = 1,
}
impl From<ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADDR` reader - Address of the USART"]
pub type ADDR_R = crate::FieldReader<u8, ADDR_A>;
impl ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDR_A> {
        match self.bits {
            0 => Some(ADDR_A::BIT4),
            1 => Some(ADDR_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BIT4`"]
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == ADDR_A::BIT4
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == ADDR_A::FULL
    }
}
#[doc = "Field `ADDR` writer - Address of the USART"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, ADDR_A, 4, 0>;
impl<'a> ADDR_W<'a> {
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn bit4(self) -> &'a mut W {
        self.variant(ADDR_A::BIT4)
    }
    #[doc = "Full-bit address detection"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(ADDR_A::FULL)
    }
}
impl R {
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&self) -> LMEN_R {
        LMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    pub fn stb(&self) -> STB_R {
        STB_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&self) -> CPH_R {
        CPH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - CK Length"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    pub fn lblen(&self) -> LBLEN_R {
        LBLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:3 - Address of the USART"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&mut self) -> LMEN_W {
        LMEN_W::new(self)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    pub fn stb(&mut self) -> STB_W {
        STB_W::new(self)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    pub fn cken(&mut self) -> CKEN_W {
        CKEN_W::new(self)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W {
        CPL_W::new(self)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&mut self) -> CPH_W {
        CPH_W::new(self)
    }
    #[doc = "Bit 8 - CK Length"]
    #[inline(always)]
    pub fn clen(&mut self) -> CLEN_W {
        CLEN_W::new(self)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W {
        LBDIE_W::new(self)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    pub fn lblen(&mut self) -> LBLEN_W {
        LBLEN_W::new(self)
    }
    #[doc = "Bits 0:3 - Address of the USART"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
