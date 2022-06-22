#[doc = "Register `STAT0` reader"]
pub struct R(crate::R<STAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT0` writer"]
pub struct W(crate::W<STAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT0_SPEC>;
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
impl From<crate::W<STAT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SMBus Alert status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBALT_A {
    #[doc = "0: SMBA not pulled down or no alert occured"]
    NOALERT = 0,
    #[doc = "1: SMBA pulled down or alert occurred"]
    ALERT = 1,
}
impl From<SMBALT_A> for bool {
    #[inline(always)]
    fn from(variant: SMBALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBALT` reader - SMBus Alert status"]
pub type SMBALT_R = crate::BitReader<SMBALT_A>;
impl SMBALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBALT_A {
        match self.bits {
            false => SMBALT_A::NOALERT,
            true => SMBALT_A::ALERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOALERT`"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == SMBALT_A::NOALERT
    }
    #[doc = "Checks if the value of the field is `ALERT`"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == SMBALT_A::ALERT
    }
}
#[doc = "Field `SMBALT` writer - SMBus Alert status"]
pub type SMBALT_W<'a> = crate::BitWriter<'a, u32, STAT0_SPEC, SMBALT_A, 15>;
impl<'a> SMBALT_W<'a> {
    #[doc = "SMBA not pulled down or no alert occured"]
    #[inline(always)]
    pub fn no_alert(self) -> &'a mut W {
        self.variant(SMBALT_A::NOALERT)
    }
    #[doc = "SMBA pulled down or alert occurred"]
    #[inline(always)]
    pub fn alert(self) -> &'a mut W {
        self.variant(SMBALT_A::ALERT)
    }
}
#[doc = "Timeout signal in SMBus mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBTO_A {
    #[doc = "0: No Timeout error"]
    NOTIMEOUT = 0,
    #[doc = "1: SCL remained low for 25 ms"]
    TIMEOUT = 1,
}
impl From<SMBTO_A> for bool {
    #[inline(always)]
    fn from(variant: SMBTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBTO` reader - Timeout signal in SMBus mode"]
pub type SMBTO_R = crate::BitReader<SMBTO_A>;
impl SMBTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBTO_A {
        match self.bits {
            false => SMBTO_A::NOTIMEOUT,
            true => SMBTO_A::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == SMBTO_A::NOTIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == SMBTO_A::TIMEOUT
    }
}
#[doc = "Field `SMBTO` writer - Timeout signal in SMBus mode"]
pub type SMBTO_W<'a> = crate::BitWriter<'a, u32, STAT0_SPEC, SMBTO_A, 14>;
impl<'a> SMBTO_W<'a> {
    #[doc = "No Timeout error"]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut W {
        self.variant(SMBTO_A::NOTIMEOUT)
    }
    #[doc = "SCL remained low for 25 ms"]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(SMBTO_A::TIMEOUT)
    }
}
#[doc = "PEC error when receiving data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECERR_A {
    #[doc = "0: No PEC error"]
    NOERROR = 0,
    #[doc = "1: PEC error"]
    ERROR = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECERR` reader - PEC error when receiving data"]
pub type PECERR_R = crate::BitReader<PECERR_A>;
impl PECERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::NOERROR,
            true => PECERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PECERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PECERR_A::ERROR
    }
}
#[doc = "Field `PECERR` writer - PEC error when receiving data"]
pub type PECERR_W<'a> = crate::BitWriter<'a, u32, STAT0_SPEC, PECERR_A, 12>;
impl<'a> PECERR_W<'a> {
    #[doc = "No PEC error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(PECERR_A::NOERROR)
    }
    #[doc = "PEC error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(PECERR_A::ERROR)
    }
}
#[doc = "Over-run or under-run situation occurs in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUERR_A {
    #[doc = "0: No overrun/underrun occured"]
    NOOVERRUN = 0,
    #[doc = "1: Overrun/underrun occured"]
    OVERRUN = 1,
}
impl From<OUERR_A> for bool {
    #[inline(always)]
    fn from(variant: OUERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUERR` reader - Over-run or under-run situation occurs in slave mode"]
pub type OUERR_R = crate::BitReader<OUERR_A>;
impl OUERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUERR_A {
        match self.bits {
            false => OUERR_A::NOOVERRUN,
            true => OUERR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OUERR_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OUERR_A::OVERRUN
    }
}
#[doc = "Field `OUERR` writer - Over-run or under-run situation occurs in slave mode"]
pub type OUERR_W<'a> = crate::BitWriter<'a, u32, STAT0_SPEC, OUERR_A, 11>;
impl<'a> OUERR_W<'a> {
    #[doc = "No overrun/underrun occured"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OUERR_A::NOOVERRUN)
    }
    #[doc = "Overrun/underrun occured"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OUERR_A::OVERRUN)
    }
}
#[doc = "Acknowledge error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AERR_A {
    #[doc = "0: No acknowledge error"]
    NOERROR = 0,
    #[doc = "1: Acknowledge error"]
    ERROR = 1,
}
impl From<AERR_A> for bool {
    #[inline(always)]
    fn from(variant: AERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AERR` reader - Acknowledge error"]
pub type AERR_R = crate::BitReader<AERR_A>;
impl AERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AERR_A {
        match self.bits {
            false => AERR_A::NOERROR,
            true => AERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == AERR_A::ERROR
    }
}
#[doc = "Field `AERR` writer - Acknowledge error"]
pub type AERR_W<'a> = crate::BitWriter<'a, u32, STAT0_SPEC, AERR_A, 10>;
impl<'a> AERR_W<'a> {
    #[doc = "No acknowledge error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(AERR_A::NOERROR)
    }
    #[doc = "Acknowledge error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(AERR_A::ERROR)
    }
}
#[doc = "Arbitration Lost in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOSTARB_A {
    #[doc = "0: No Arbitration Lost detected"]
    NOLOST = 0,
    #[doc = "1: Arbitration Lost detected"]
    LOST = 1,
}
impl From<LOSTARB_A> for bool {
    #[inline(always)]
    fn from(variant: LOSTARB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOSTARB` reader - Arbitration Lost in master mode"]
pub type LOSTARB_R = crate::BitReader<LOSTARB_A>;
impl LOSTARB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOSTARB_A {
        match self.bits {
            false => LOSTARB_A::NOLOST,
            true => LOSTARB_A::LOST,
        }
    }
    #[doc = "Checks if the value of the field is `NOLOST`"]
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == LOSTARB_A::NOLOST
    }
    #[doc = "Checks if the value of the field is `LOST`"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == LOSTARB_A::LOST
    }
}
#[doc = "Field `LOSTARB` writer - Arbitration Lost in master mode"]
pub type LOSTARB_W<'a> = crate::BitWriter<'a, u32, STAT0_SPEC, LOSTARB_A, 9>;
impl<'a> LOSTARB_W<'a> {
    #[doc = "No Arbitration Lost detected"]
    #[inline(always)]
    pub fn no_lost(self) -> &'a mut W {
        self.variant(LOSTARB_A::NOLOST)
    }
    #[doc = "Arbitration Lost detected"]
    #[inline(always)]
    pub fn lost(self) -> &'a mut W {
        self.variant(LOSTARB_A::LOST)
    }
}
#[doc = "A bus error occurs indication a unexpected START or STOP condition on I2C bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERR_A {
    #[doc = "0: No misplaced Start or Stop condition"]
    NOERROR = 0,
    #[doc = "1: Misplaced Start or Stop condition"]
    ERROR = 1,
}
impl From<BERR_A> for bool {
    #[inline(always)]
    fn from(variant: BERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERR` reader - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
pub type BERR_R = crate::BitReader<BERR_A>;
impl BERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BERR_A {
        match self.bits {
            false => BERR_A::NOERROR,
            true => BERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERR_A::ERROR
    }
}
#[doc = "Field `BERR` writer - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
pub type BERR_W<'a> = crate::BitWriter<'a, u32, STAT0_SPEC, BERR_A, 8>;
impl<'a> BERR_W<'a> {
    #[doc = "No misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(BERR_A::NOERROR)
    }
    #[doc = "Misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(BERR_A::ERROR)
    }
}
#[doc = "I2C_DATA is Empty during transmitting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBE_A {
    #[doc = "0: Data register not empty"]
    NOTEMPTY = 0,
    #[doc = "1: Data register empty, software can write"]
    EMPTY = 1,
}
impl From<TBE_A> for bool {
    #[inline(always)]
    fn from(variant: TBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBE` reader - I2C_DATA is Empty during transmitting"]
pub type TBE_R = crate::BitReader<TBE_A>;
impl TBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBE_A {
        match self.bits {
            false => TBE_A::NOTEMPTY,
            true => TBE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TBE_A::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TBE_A::EMPTY
    }
}
#[doc = "I2C_DATA is not Empty during receiving\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBNE_A {
    #[doc = "0: Data register empty"]
    EMPTY = 0,
    #[doc = "1: Data register not empty, software can read"]
    NOTEMPTY = 1,
}
impl From<RBNE_A> for bool {
    #[inline(always)]
    fn from(variant: RBNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBNE` reader - I2C_DATA is not Empty during receiving"]
pub type RBNE_R = crate::BitReader<RBNE_A>;
impl RBNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBNE_A {
        match self.bits {
            false => RBNE_A::EMPTY,
            true => RBNE_A::NOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RBNE_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RBNE_A::NOTEMPTY
    }
}
#[doc = "STOP condition detected in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPDET_A {
    #[doc = "0: No Stop condition detected"]
    NOSTOP = 0,
    #[doc = "1: Stop condition detected"]
    STOP = 1,
}
impl From<STPDET_A> for bool {
    #[inline(always)]
    fn from(variant: STPDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPDET` reader - STOP condition detected in slave mode"]
pub type STPDET_R = crate::BitReader<STPDET_A>;
impl STPDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPDET_A {
        match self.bits {
            false => STPDET_A::NOSTOP,
            true => STPDET_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTOP`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STPDET_A::NOSTOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STPDET_A::STOP
    }
}
#[doc = "Header of 10-bit address is sent in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD10SEND_A {
    #[doc = "0: No header of 10-bit address is sent"]
    NOHEADER = 0,
    #[doc = "1: Header of 10-bit address is sent"]
    HEADER = 1,
}
impl From<ADD10SEND_A> for bool {
    #[inline(always)]
    fn from(variant: ADD10SEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD10SEND` reader - Header of 10-bit address is sent in master mode"]
pub type ADD10SEND_R = crate::BitReader<ADD10SEND_A>;
impl ADD10SEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADD10SEND_A {
        match self.bits {
            false => ADD10SEND_A::NOHEADER,
            true => ADD10SEND_A::HEADER,
        }
    }
    #[doc = "Checks if the value of the field is `NOHEADER`"]
    #[inline(always)]
    pub fn is_no_header(&self) -> bool {
        *self == ADD10SEND_A::NOHEADER
    }
    #[doc = "Checks if the value of the field is `HEADER`"]
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == ADD10SEND_A::HEADER
    }
}
#[doc = "Byte transmission completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTC_A {
    #[doc = "0: Data byte transfer not done"]
    NOTFINISHED = 0,
    #[doc = "1: Data byte transfer successful"]
    FINISHED = 1,
}
impl From<BTC_A> for bool {
    #[inline(always)]
    fn from(variant: BTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTC` reader - Byte transmission completed"]
pub type BTC_R = crate::BitReader<BTC_A>;
impl BTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTC_A {
        match self.bits {
            false => BTC_A::NOTFINISHED,
            true => BTC_A::FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFINISHED`"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == BTC_A::NOTFINISHED
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == BTC_A::FINISHED
    }
}
#[doc = "Address is sent in master mode or received and matches in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDSEND_A {
    #[doc = "0: Adress mismatched or not received"]
    NOTMATCH = 0,
    #[doc = "1: Received slave address matched with one of the enabled slave addresses"]
    MATCH = 1,
}
impl From<ADDSEND_A> for bool {
    #[inline(always)]
    fn from(variant: ADDSEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDSEND` reader - Address is sent in master mode or received and matches in slave mode"]
pub type ADDSEND_R = crate::BitReader<ADDSEND_A>;
impl ADDSEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDSEND_A {
        match self.bits {
            false => ADDSEND_A::NOTMATCH,
            true => ADDSEND_A::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NOTMATCH`"]
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDSEND_A::NOTMATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ADDSEND_A::MATCH
    }
}
#[doc = "START condition sent out in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBSEND_A {
    #[doc = "0: No Start condition"]
    NOSTART = 0,
    #[doc = "1: Start condition generated"]
    START = 1,
}
impl From<SBSEND_A> for bool {
    #[inline(always)]
    fn from(variant: SBSEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBSEND` reader - START condition sent out in master mode"]
pub type SBSEND_R = crate::BitReader<SBSEND_A>;
impl SBSEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBSEND_A {
        match self.bits {
            false => SBSEND_A::NOSTART,
            true => SBSEND_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTART`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == SBSEND_A::NOSTART
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SBSEND_A::START
    }
}
impl R {
    #[doc = "Bit 15 - SMBus Alert status"]
    #[inline(always)]
    pub fn smbalt(&self) -> SMBALT_R {
        SMBALT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout signal in SMBus mode"]
    #[inline(always)]
    pub fn smbto(&self) -> SMBTO_R {
        SMBTO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC error when receiving data"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Over-run or under-run situation occurs in slave mode"]
    #[inline(always)]
    pub fn ouerr(&self) -> OUERR_R {
        OUERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge error"]
    #[inline(always)]
    pub fn aerr(&self) -> AERR_R {
        AERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost in master mode"]
    #[inline(always)]
    pub fn lostarb(&self) -> LOSTARB_R {
        LOSTARB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C_DATA is Empty during transmitting"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C_DATA is not Empty during receiving"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - STOP condition detected in slave mode"]
    #[inline(always)]
    pub fn stpdet(&self) -> STPDET_R {
        STPDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Header of 10-bit address is sent in master mode"]
    #[inline(always)]
    pub fn add10send(&self) -> ADD10SEND_R {
        ADD10SEND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transmission completed"]
    #[inline(always)]
    pub fn btc(&self) -> BTC_R {
        BTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Address is sent in master mode or received and matches in slave mode"]
    #[inline(always)]
    pub fn addsend(&self) -> ADDSEND_R {
        ADDSEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - START condition sent out in master mode"]
    #[inline(always)]
    pub fn sbsend(&self) -> SBSEND_R {
        SBSEND_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - SMBus Alert status"]
    #[inline(always)]
    pub fn smbalt(&mut self) -> SMBALT_W {
        SMBALT_W::new(self)
    }
    #[doc = "Bit 14 - Timeout signal in SMBus mode"]
    #[inline(always)]
    pub fn smbto(&mut self) -> SMBTO_W {
        SMBTO_W::new(self)
    }
    #[doc = "Bit 12 - PEC error when receiving data"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W {
        PECERR_W::new(self)
    }
    #[doc = "Bit 11 - Over-run or under-run situation occurs in slave mode"]
    #[inline(always)]
    pub fn ouerr(&mut self) -> OUERR_W {
        OUERR_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge error"]
    #[inline(always)]
    pub fn aerr(&mut self) -> AERR_W {
        AERR_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration Lost in master mode"]
    #[inline(always)]
    pub fn lostarb(&mut self) -> LOSTARB_W {
        LOSTARB_W::new(self)
    }
    #[doc = "Bit 8 - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat0](index.html) module"]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat0::R](R) reader structure"]
impl crate::Readable for STAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat0::W](W) writer structure"]
impl crate::Writable for STAT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for STAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
