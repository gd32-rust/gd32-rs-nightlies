#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `PUD` reader - Free watchdog timer prescaler value update"]
pub type PUD_R = crate::BitReader<PUD_A>;
#[doc = "Free watchdog timer prescaler value update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUD_A {
    #[doc = "0: The value read from the PSC register is valid"]
    VALID = 0,
    #[doc = "1: A write operation to to the PSC register is ongoing, so the value read is invalid"]
    ONGOING = 1,
}
impl From<PUD_A> for bool {
    #[inline(always)]
    fn from(variant: PUD_A) -> Self {
        variant as u8 != 0
    }
}
impl PUD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUD_A {
        match self.bits {
            false => PUD_A::VALID,
            true => PUD_A::ONGOING,
        }
    }
    #[doc = "The value read from the PSC register is valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == PUD_A::VALID
    }
    #[doc = "A write operation to to the PSC register is ongoing, so the value read is invalid"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == PUD_A::ONGOING
    }
}
#[doc = "Field `RUD` reader - Free watchdog timer counter reload value update"]
pub type RUD_R = crate::BitReader<RUD_A>;
#[doc = "Free watchdog timer counter reload value update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUD_A {
    #[doc = "0: The value read from the RLD register is valid"]
    VALID = 0,
    #[doc = "1: A write operation to to the RLD register is ongoing, so the value read is invalid"]
    ONGOING = 1,
}
impl From<RUD_A> for bool {
    #[inline(always)]
    fn from(variant: RUD_A) -> Self {
        variant as u8 != 0
    }
}
impl RUD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUD_A {
        match self.bits {
            false => RUD_A::VALID,
            true => RUD_A::ONGOING,
        }
    }
    #[doc = "The value read from the RLD register is valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == RUD_A::VALID
    }
    #[doc = "A write operation to to the RLD register is ongoing, so the value read is invalid"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == RUD_A::ONGOING
    }
}
#[doc = "Field `WUD` reader - Watchdog counter window value update"]
pub type WUD_R = crate::BitReader<WUD_A>;
#[doc = "Watchdog counter window value update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUD_A {
    #[doc = "0: The value read from the WND register is valid"]
    VALID = 0,
    #[doc = "1: A write operation to to the WND register is ongoing, so the value read is invalid"]
    ONGOING = 1,
}
impl From<WUD_A> for bool {
    #[inline(always)]
    fn from(variant: WUD_A) -> Self {
        variant as u8 != 0
    }
}
impl WUD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUD_A {
        match self.bits {
            false => WUD_A::VALID,
            true => WUD_A::ONGOING,
        }
    }
    #[doc = "The value read from the WND register is valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == WUD_A::VALID
    }
    #[doc = "A write operation to to the WND register is ongoing, so the value read is invalid"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == WUD_A::ONGOING
    }
}
impl R {
    #[doc = "Bit 0 - Free watchdog timer prescaler value update"]
    #[inline(always)]
    pub fn pud(&self) -> PUD_R {
        PUD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free watchdog timer counter reload value update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog counter window value update"]
    #[inline(always)]
    pub fn wud(&self) -> WUD_R {
        WUD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
