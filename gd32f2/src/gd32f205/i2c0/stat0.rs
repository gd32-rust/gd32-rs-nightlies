#[doc = "Register `STAT0` reader"]
pub type R = crate::R<STAT0_SPEC>;
#[doc = "Register `STAT0` writer"]
pub type W = crate::W<STAT0_SPEC>;
#[doc = "Field `SBSEND` reader - START condition sent out in master mode"]
pub type SBSEND_R = crate::BitReader<SBSEND_A>;
#[doc = "START condition sent out in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSEND_A {
    #[doc = "0: No Start condition"]
    NO_START = 0,
    #[doc = "1: Start condition generated"]
    START = 1,
}
impl From<SBSEND_A> for bool {
    #[inline(always)]
    fn from(variant: SBSEND_A) -> Self {
        variant as u8 != 0
    }
}
impl SBSEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBSEND_A {
        match self.bits {
            false => SBSEND_A::NO_START,
            true => SBSEND_A::START,
        }
    }
    #[doc = "No Start condition"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == SBSEND_A::NO_START
    }
    #[doc = "Start condition generated"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SBSEND_A::START
    }
}
#[doc = "Field `ADDSEND` reader - Address is sent in master mode or received and matches in slave mode"]
pub type ADDSEND_R = crate::BitReader<ADDSEND_A>;
#[doc = "Address is sent in master mode or received and matches in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDSEND_A {
    #[doc = "0: Adress mismatched or not received"]
    NOT_MATCH = 0,
    #[doc = "1: Received slave address matched with one of the enabled slave addresses"]
    MATCH = 1,
}
impl From<ADDSEND_A> for bool {
    #[inline(always)]
    fn from(variant: ADDSEND_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDSEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDSEND_A {
        match self.bits {
            false => ADDSEND_A::NOT_MATCH,
            true => ADDSEND_A::MATCH,
        }
    }
    #[doc = "Adress mismatched or not received"]
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDSEND_A::NOT_MATCH
    }
    #[doc = "Received slave address matched with one of the enabled slave addresses"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ADDSEND_A::MATCH
    }
}
#[doc = "Field `BTC` reader - Byte transmission completed"]
pub type BTC_R = crate::BitReader<BTC_A>;
#[doc = "Byte transmission completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BTC_A {
    #[doc = "0: Data byte transfer not done"]
    NOT_FINISHED = 0,
    #[doc = "1: Data byte transfer successful"]
    FINISHED = 1,
}
impl From<BTC_A> for bool {
    #[inline(always)]
    fn from(variant: BTC_A) -> Self {
        variant as u8 != 0
    }
}
impl BTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTC_A {
        match self.bits {
            false => BTC_A::NOT_FINISHED,
            true => BTC_A::FINISHED,
        }
    }
    #[doc = "Data byte transfer not done"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == BTC_A::NOT_FINISHED
    }
    #[doc = "Data byte transfer successful"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == BTC_A::FINISHED
    }
}
#[doc = "Field `ADD10SEND` reader - Header of 10-bit address is sent in master mode"]
pub type ADD10SEND_R = crate::BitReader<ADD10SEND_A>;
#[doc = "Header of 10-bit address is sent in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD10SEND_A {
    #[doc = "0: No header of 10-bit address is sent"]
    NO_HEADER = 0,
    #[doc = "1: Header of 10-bit address is sent"]
    HEADER = 1,
}
impl From<ADD10SEND_A> for bool {
    #[inline(always)]
    fn from(variant: ADD10SEND_A) -> Self {
        variant as u8 != 0
    }
}
impl ADD10SEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADD10SEND_A {
        match self.bits {
            false => ADD10SEND_A::NO_HEADER,
            true => ADD10SEND_A::HEADER,
        }
    }
    #[doc = "No header of 10-bit address is sent"]
    #[inline(always)]
    pub fn is_no_header(&self) -> bool {
        *self == ADD10SEND_A::NO_HEADER
    }
    #[doc = "Header of 10-bit address is sent"]
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == ADD10SEND_A::HEADER
    }
}
#[doc = "Field `STPDET` reader - STOP condition detected in slave mode"]
pub type STPDET_R = crate::BitReader<STPDET_A>;
#[doc = "STOP condition detected in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STPDET_A {
    #[doc = "0: No Stop condition detected"]
    NO_STOP = 0,
    #[doc = "1: Stop condition detected"]
    STOP = 1,
}
impl From<STPDET_A> for bool {
    #[inline(always)]
    fn from(variant: STPDET_A) -> Self {
        variant as u8 != 0
    }
}
impl STPDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPDET_A {
        match self.bits {
            false => STPDET_A::NO_STOP,
            true => STPDET_A::STOP,
        }
    }
    #[doc = "No Stop condition detected"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STPDET_A::NO_STOP
    }
    #[doc = "Stop condition detected"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STPDET_A::STOP
    }
}
#[doc = "Field `RBNE` reader - I2C_DATA is not Empty during receiving"]
pub type RBNE_R = crate::BitReader<RBNE_A>;
#[doc = "I2C_DATA is not Empty during receiving\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBNE_A {
    #[doc = "0: Data register empty"]
    EMPTY = 0,
    #[doc = "1: Data register not empty, software can read"]
    NOT_EMPTY = 1,
}
impl From<RBNE_A> for bool {
    #[inline(always)]
    fn from(variant: RBNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RBNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBNE_A {
        match self.bits {
            false => RBNE_A::EMPTY,
            true => RBNE_A::NOT_EMPTY,
        }
    }
    #[doc = "Data register empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RBNE_A::EMPTY
    }
    #[doc = "Data register not empty, software can read"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RBNE_A::NOT_EMPTY
    }
}
#[doc = "Field `TBE` reader - I2C_DATA is Empty during transmitting"]
pub type TBE_R = crate::BitReader<TBE_A>;
#[doc = "I2C_DATA is Empty during transmitting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBE_A {
    #[doc = "0: Data register not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: Data register empty, software can write"]
    EMPTY = 1,
}
impl From<TBE_A> for bool {
    #[inline(always)]
    fn from(variant: TBE_A) -> Self {
        variant as u8 != 0
    }
}
impl TBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBE_A {
        match self.bits {
            false => TBE_A::NOT_EMPTY,
            true => TBE_A::EMPTY,
        }
    }
    #[doc = "Data register not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TBE_A::NOT_EMPTY
    }
    #[doc = "Data register empty, software can write"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TBE_A::EMPTY
    }
}
#[doc = "Field `BERR` reader - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
pub type BERR_R = crate::BitReader<BERR_A>;
#[doc = "A bus error occurs indication a unexpected START or STOP condition on I2C bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERR_A {
    #[doc = "0: No misplaced Start or Stop condition"]
    NO_ERROR = 0,
    #[doc = "1: Misplaced Start or Stop condition"]
    ERROR = 1,
}
impl From<BERR_A> for bool {
    #[inline(always)]
    fn from(variant: BERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BERR_A {
        match self.bits {
            false => BERR_A::NO_ERROR,
            true => BERR_A::ERROR,
        }
    }
    #[doc = "No misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERR_A::NO_ERROR
    }
    #[doc = "Misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERR_A::ERROR
    }
}
#[doc = "Field `BERR` writer - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
pub type BERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BERR_A>;
impl<'a, REG, const O: u8> BERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(BERR_A::NO_ERROR)
    }
    #[doc = "Misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(BERR_A::ERROR)
    }
}
#[doc = "Field `LOSTARB` reader - Arbitration Lost in master mode"]
pub type LOSTARB_R = crate::BitReader<LOSTARB_A>;
#[doc = "Arbitration Lost in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOSTARB_A {
    #[doc = "0: No Arbitration Lost detected"]
    NO_LOST = 0,
    #[doc = "1: Arbitration Lost detected"]
    LOST = 1,
}
impl From<LOSTARB_A> for bool {
    #[inline(always)]
    fn from(variant: LOSTARB_A) -> Self {
        variant as u8 != 0
    }
}
impl LOSTARB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOSTARB_A {
        match self.bits {
            false => LOSTARB_A::NO_LOST,
            true => LOSTARB_A::LOST,
        }
    }
    #[doc = "No Arbitration Lost detected"]
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == LOSTARB_A::NO_LOST
    }
    #[doc = "Arbitration Lost detected"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == LOSTARB_A::LOST
    }
}
#[doc = "Field `LOSTARB` writer - Arbitration Lost in master mode"]
pub type LOSTARB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LOSTARB_A>;
impl<'a, REG, const O: u8> LOSTARB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Arbitration Lost detected"]
    #[inline(always)]
    pub fn no_lost(self) -> &'a mut crate::W<REG> {
        self.variant(LOSTARB_A::NO_LOST)
    }
    #[doc = "Arbitration Lost detected"]
    #[inline(always)]
    pub fn lost(self) -> &'a mut crate::W<REG> {
        self.variant(LOSTARB_A::LOST)
    }
}
#[doc = "Field `AERR` reader - Acknowledge error"]
pub type AERR_R = crate::BitReader<AERR_A>;
#[doc = "Acknowledge error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AERR_A {
    #[doc = "0: No acknowledge error"]
    NO_ERROR = 0,
    #[doc = "1: Acknowledge error"]
    ERROR = 1,
}
impl From<AERR_A> for bool {
    #[inline(always)]
    fn from(variant: AERR_A) -> Self {
        variant as u8 != 0
    }
}
impl AERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AERR_A {
        match self.bits {
            false => AERR_A::NO_ERROR,
            true => AERR_A::ERROR,
        }
    }
    #[doc = "No acknowledge error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AERR_A::NO_ERROR
    }
    #[doc = "Acknowledge error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == AERR_A::ERROR
    }
}
#[doc = "Field `AERR` writer - Acknowledge error"]
pub type AERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AERR_A>;
impl<'a, REG, const O: u8> AERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledge error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(AERR_A::NO_ERROR)
    }
    #[doc = "Acknowledge error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(AERR_A::ERROR)
    }
}
#[doc = "Field `OUERR` reader - Over-run or under-run situation occurs in slave mode"]
pub type OUERR_R = crate::BitReader<OUERR_A>;
#[doc = "Over-run or under-run situation occurs in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUERR_A {
    #[doc = "0: No overrun/underrun occured"]
    NO_OVERRUN = 0,
    #[doc = "1: Overrun/underrun occured"]
    OVERRUN = 1,
}
impl From<OUERR_A> for bool {
    #[inline(always)]
    fn from(variant: OUERR_A) -> Self {
        variant as u8 != 0
    }
}
impl OUERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUERR_A {
        match self.bits {
            false => OUERR_A::NO_OVERRUN,
            true => OUERR_A::OVERRUN,
        }
    }
    #[doc = "No overrun/underrun occured"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OUERR_A::NO_OVERRUN
    }
    #[doc = "Overrun/underrun occured"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OUERR_A::OVERRUN
    }
}
#[doc = "Field `OUERR` writer - Over-run or under-run situation occurs in slave mode"]
pub type OUERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OUERR_A>;
impl<'a, REG, const O: u8> OUERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun/underrun occured"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut crate::W<REG> {
        self.variant(OUERR_A::NO_OVERRUN)
    }
    #[doc = "Overrun/underrun occured"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut crate::W<REG> {
        self.variant(OUERR_A::OVERRUN)
    }
}
#[doc = "Field `PECERR` reader - PEC error when receiving data"]
pub type PECERR_R = crate::BitReader<PECERR_A>;
#[doc = "PEC error when receiving data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECERR_A {
    #[doc = "0: No PEC error"]
    NO_ERROR = 0,
    #[doc = "1: PEC error"]
    ERROR = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PECERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::NO_ERROR,
            true => PECERR_A::ERROR,
        }
    }
    #[doc = "No PEC error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PECERR_A::NO_ERROR
    }
    #[doc = "PEC error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PECERR_A::ERROR
    }
}
#[doc = "Field `PECERR` writer - PEC error when receiving data"]
pub type PECERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PECERR_A>;
impl<'a, REG, const O: u8> PECERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No PEC error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(PECERR_A::NO_ERROR)
    }
    #[doc = "PEC error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(PECERR_A::ERROR)
    }
}
#[doc = "Field `SMBTO` reader - Timeout signal in SMBus mode"]
pub type SMBTO_R = crate::BitReader<SMBTO_A>;
#[doc = "Timeout signal in SMBus mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBTO_A {
    #[doc = "0: No Timeout error"]
    NO_TIMEOUT = 0,
    #[doc = "1: SCL remained low for 25 ms"]
    TIMEOUT = 1,
}
impl From<SMBTO_A> for bool {
    #[inline(always)]
    fn from(variant: SMBTO_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBTO_A {
        match self.bits {
            false => SMBTO_A::NO_TIMEOUT,
            true => SMBTO_A::TIMEOUT,
        }
    }
    #[doc = "No Timeout error"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == SMBTO_A::NO_TIMEOUT
    }
    #[doc = "SCL remained low for 25 ms"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == SMBTO_A::TIMEOUT
    }
}
#[doc = "Field `SMBTO` writer - Timeout signal in SMBus mode"]
pub type SMBTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SMBTO_A>;
impl<'a, REG, const O: u8> SMBTO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Timeout error"]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut crate::W<REG> {
        self.variant(SMBTO_A::NO_TIMEOUT)
    }
    #[doc = "SCL remained low for 25 ms"]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut crate::W<REG> {
        self.variant(SMBTO_A::TIMEOUT)
    }
}
#[doc = "Field `SMBALT` reader - SMBus Alert status"]
pub type SMBALT_R = crate::BitReader<SMBALT_A>;
#[doc = "SMBus Alert status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBALT_A {
    #[doc = "0: SMBA not pulled down or no alert occured"]
    NO_ALERT = 0,
    #[doc = "1: SMBA pulled down or alert occurred"]
    ALERT = 1,
}
impl From<SMBALT_A> for bool {
    #[inline(always)]
    fn from(variant: SMBALT_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBALT_A {
        match self.bits {
            false => SMBALT_A::NO_ALERT,
            true => SMBALT_A::ALERT,
        }
    }
    #[doc = "SMBA not pulled down or no alert occured"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == SMBALT_A::NO_ALERT
    }
    #[doc = "SMBA pulled down or alert occurred"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == SMBALT_A::ALERT
    }
}
#[doc = "Field `SMBALT` writer - SMBus Alert status"]
pub type SMBALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SMBALT_A>;
impl<'a, REG, const O: u8> SMBALT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBA not pulled down or no alert occured"]
    #[inline(always)]
    pub fn no_alert(self) -> &'a mut crate::W<REG> {
        self.variant(SMBALT_A::NO_ALERT)
    }
    #[doc = "SMBA pulled down or alert occurred"]
    #[inline(always)]
    pub fn alert(self) -> &'a mut crate::W<REG> {
        self.variant(SMBALT_A::ALERT)
    }
}
impl R {
    #[doc = "Bit 0 - START condition sent out in master mode"]
    #[inline(always)]
    pub fn sbsend(&self) -> SBSEND_R {
        SBSEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address is sent in master mode or received and matches in slave mode"]
    #[inline(always)]
    pub fn addsend(&self) -> ADDSEND_R {
        ADDSEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transmission completed"]
    #[inline(always)]
    pub fn btc(&self) -> BTC_R {
        BTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Header of 10-bit address is sent in master mode"]
    #[inline(always)]
    pub fn add10send(&self) -> ADD10SEND_R {
        ADD10SEND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STOP condition detected in slave mode"]
    #[inline(always)]
    pub fn stpdet(&self) -> STPDET_R {
        STPDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C_DATA is not Empty during receiving"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C_DATA is Empty during transmitting"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost in master mode"]
    #[inline(always)]
    pub fn lostarb(&self) -> LOSTARB_R {
        LOSTARB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge error"]
    #[inline(always)]
    pub fn aerr(&self) -> AERR_R {
        AERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Over-run or under-run situation occurs in slave mode"]
    #[inline(always)]
    pub fn ouerr(&self) -> OUERR_R {
        OUERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC error when receiving data"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout signal in SMBus mode"]
    #[inline(always)]
    pub fn smbto(&self) -> SMBTO_R {
        SMBTO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus Alert status"]
    #[inline(always)]
    pub fn smbalt(&self) -> SMBALT_R {
        SMBALT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<STAT0_SPEC, 8> {
        BERR_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration Lost in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn lostarb(&mut self) -> LOSTARB_W<STAT0_SPEC, 9> {
        LOSTARB_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge error"]
    #[inline(always)]
    #[must_use]
    pub fn aerr(&mut self) -> AERR_W<STAT0_SPEC, 10> {
        AERR_W::new(self)
    }
    #[doc = "Bit 11 - Over-run or under-run situation occurs in slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn ouerr(&mut self) -> OUERR_W<STAT0_SPEC, 11> {
        OUERR_W::new(self)
    }
    #[doc = "Bit 12 - PEC error when receiving data"]
    #[inline(always)]
    #[must_use]
    pub fn pecerr(&mut self) -> PECERR_W<STAT0_SPEC, 12> {
        PECERR_W::new(self)
    }
    #[doc = "Bit 14 - Timeout signal in SMBus mode"]
    #[inline(always)]
    #[must_use]
    pub fn smbto(&mut self) -> SMBTO_W<STAT0_SPEC, 14> {
        SMBTO_W::new(self)
    }
    #[doc = "Bit 15 - SMBus Alert status"]
    #[inline(always)]
    #[must_use]
    pub fn smbalt(&mut self) -> SMBALT_W<STAT0_SPEC, 15> {
        SMBALT_W::new(self)
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
#[doc = "Transfer status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for STAT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat0::W`](W) writer structure"]
impl crate::Writable for STAT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for STAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
