#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SWEVG_SPEC>;
#[doc = "Update event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPG_AW {
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers."]
    UPDATE = 1,
}
impl From<UPG_AW> for bool {
    #[inline(always)]
    fn from(variant: UPG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPG` writer - Update event generation"]
pub type UPG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UPG_AW>;
impl<'a, REG, const O: u8> UPG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UPG_AW::UPDATE)
    }
}
#[doc = "Channel 0's capture or compare event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0G_AW {
    #[doc = "1: Generate a capture or compare event"]
    CAPTURE_COMPARE = 1,
}
impl From<CH0G_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0G_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0G` writer - Channel 0's capture or compare event generation"]
pub type CH0G_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0G_AW>;
impl<'a, REG, const O: u8> CH0G_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a capture or compare event"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut crate::W<REG> {
        self.variant(CH0G_AW::CAPTURE_COMPARE)
    }
}
#[doc = "Field `CH1G` writer - Channel 1's capture or compare event generation"]
pub use CH0G_W as CH1G_W;
#[doc = "Field `CH2G` writer - Channel 2's capture or compare event generation"]
pub use CH0G_W as CH2G_W;
#[doc = "Field `CH3G` writer - Channel 3's capture or compare event generation"]
pub use CH0G_W as CH3G_W;
#[doc = "Channel commutation event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMTG_AW {
    #[doc = "1: Generate a channel commutation event, updating capture/compare control registers based on the value of CCSE"]
    UPDATE = 1,
}
impl From<CMTG_AW> for bool {
    #[inline(always)]
    fn from(variant: CMTG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTG` writer - Channel commutation event generation"]
pub type CMTG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMTG_AW>;
impl<'a, REG, const O: u8> CMTG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a channel commutation event, updating capture/compare control registers based on the value of CCSE"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(CMTG_AW::UPDATE)
    }
}
#[doc = "Trigger event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGG_AW {
    #[doc = "1: Generate a trigger event"]
    TRIGGER = 1,
}
impl From<TRGG_AW> for bool {
    #[inline(always)]
    fn from(variant: TRGG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGG` writer - Trigger event generation"]
pub type TRGG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRGG_AW>;
impl<'a, REG, const O: u8> TRGG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TRGG_AW::TRIGGER)
    }
}
#[doc = "Break event generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRKG_AW {
    #[doc = "1: Generate a break event"]
    BREAK = 1,
}
impl From<BRKG_AW> for bool {
    #[inline(always)]
    fn from(variant: BRKG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKG` writer - Break event generation"]
pub type BRKG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BRKG_AW>;
impl<'a, REG, const O: u8> BRKG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a break event"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(BRKG_AW::BREAK)
    }
}
impl W {
    #[doc = "Bit 0 - Update event generation"]
    #[inline(always)]
    #[must_use]
    pub fn upg(&mut self) -> UPG_W<SWEVG_SPEC, 0> {
        UPG_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0's capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0g(&mut self) -> CH0G_W<SWEVG_SPEC, 1> {
        CH0G_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1's capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch1g(&mut self) -> CH1G_W<SWEVG_SPEC, 2> {
        CH1G_W::new(self)
    }
    #[doc = "Bit 3 - Channel 2's capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch2g(&mut self) -> CH2G_W<SWEVG_SPEC, 3> {
        CH2G_W::new(self)
    }
    #[doc = "Bit 4 - Channel 3's capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch3g(&mut self) -> CH3G_W<SWEVG_SPEC, 4> {
        CH3G_W::new(self)
    }
    #[doc = "Bit 5 - Channel commutation event generation"]
    #[inline(always)]
    #[must_use]
    pub fn cmtg(&mut self) -> CMTG_W<SWEVG_SPEC, 5> {
        CMTG_W::new(self)
    }
    #[doc = "Bit 6 - Trigger event generation"]
    #[inline(always)]
    #[must_use]
    pub fn trgg(&mut self) -> TRGG_W<SWEVG_SPEC, 6> {
        TRGG_W::new(self)
    }
    #[doc = "Bit 7 - Break event generation"]
    #[inline(always)]
    #[must_use]
    pub fn brkg(&mut self) -> BRKG_W<SWEVG_SPEC, 7> {
        BRKG_W::new(self)
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
#[doc = "Software event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVG_SPEC;
impl crate::RegisterSpec for SWEVG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevg::W`](W) writer structure"]
impl crate::Writable for SWEVG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SWEVG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
