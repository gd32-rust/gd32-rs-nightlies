#[doc = "Register `TIME` reader"]
pub struct R(crate::R<TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIME` writer"]
pub struct W(crate::W<TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIME_SPEC>;
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
impl From<crate::W<TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM` reader - AM/PM mark"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - AM/PM mark"]
pub type PM_W<'a> = crate::BitWriter<'a, u32, TIME_SPEC, bool, 22>;
#[doc = "Field `HRT` reader - Hour tens in BCD format"]
pub type HRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HRT` writer - Hour tens in BCD format"]
pub type HRT_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 2, 20>;
#[doc = "Field `HRU` reader - Hour units in BCD format"]
pub type HRU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HRU` writer - Hour units in BCD format"]
pub type HRU_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 4, 16>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MNT_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 3, 12>;
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MNU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MNU_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 4, 8>;
#[doc = "Field `SCT` reader - Second tens in BCD format"]
pub type SCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCT` writer - Second tens in BCD format"]
pub type SCT_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 3, 4>;
#[doc = "Field `SCU` reader - Second units in BCD format"]
pub type SCU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCU` writer - Second units in BCD format"]
pub type SCU_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bit 22 - AM/PM mark"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn hrt(&self) -> HRT_R {
        HRT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hru(&self) -> HRU_R {
        HRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn scu(&self) -> SCU_R {
        SCU_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 22 - AM/PM mark"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W::new(self)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn hrt(&mut self) -> HRT_W {
        HRT_W::new(self)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hru(&mut self) -> HRU_W {
        HRU_W::new(self)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W {
        MNT_W::new(self)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W {
        MNU_W::new(self)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn sct(&mut self) -> SCT_W {
        SCT_W::new(self)
    }
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn scu(&mut self) -> SCU_W {
        SCU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time of day register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time](index.html) module"]
pub struct TIME_SPEC;
impl crate::RegisterSpec for TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time::R](R) reader structure"]
impl crate::Readable for TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [time::W](W) writer structure"]
impl crate::Writable for TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
