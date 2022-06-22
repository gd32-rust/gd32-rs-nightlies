#[doc = "Register `DATE` reader"]
pub struct R(crate::R<DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATE` writer"]
pub struct W(crate::W<DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATE_SPEC>;
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
impl From<crate::W<DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YRT` reader - Year tens in BCD code"]
pub type YRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YRT` writer - Year tens in BCD code"]
pub type YRT_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 4, 20>;
#[doc = "Field `YRU` reader - Year units in BCD code"]
pub type YRU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YRU` writer - Year units in BCD code"]
pub type YRU_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 4, 16>;
#[doc = "Field `DOW` reader - Days of the week"]
pub type DOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOW` writer - Days of the week"]
pub type DOW_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 3, 13>;
#[doc = "Field `MONT` reader - Month tens in BCD code"]
pub type MONT_R = crate::BitReader<bool>;
#[doc = "Field `MONT` writer - Month tens in BCD code"]
pub type MONT_W<'a> = crate::BitWriter<'a, u32, DATE_SPEC, bool, 12>;
#[doc = "Field `MONU` reader - Month units in BCD code"]
pub type MONU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MONU` writer - Month units in BCD code"]
pub type MONU_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 4, 8>;
#[doc = "Field `DAYT` reader - Date tens in BCD code"]
pub type DAYT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYT` writer - Date tens in BCD code"]
pub type DAYT_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 2, 4>;
#[doc = "Field `DAYU` reader - Date units in BCD code"]
pub type DAYU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYU` writer - Date units in BCD code"]
pub type DAYU_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 20:23 - Year tens in BCD code"]
    #[inline(always)]
    pub fn yrt(&self) -> YRT_R {
        YRT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Year units in BCD code"]
    #[inline(always)]
    pub fn yru(&self) -> YRU_R {
        YRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Days of the week"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD code"]
    #[inline(always)]
    pub fn mont(&self) -> MONT_R {
        MONT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Month units in BCD code"]
    #[inline(always)]
    pub fn monu(&self) -> MONU_R {
        MONU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD code"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 0:3 - Date units in BCD code"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - Year tens in BCD code"]
    #[inline(always)]
    pub fn yrt(&mut self) -> YRT_W {
        YRT_W::new(self)
    }
    #[doc = "Bits 16:19 - Year units in BCD code"]
    #[inline(always)]
    pub fn yru(&mut self) -> YRU_W {
        YRU_W::new(self)
    }
    #[doc = "Bits 13:15 - Days of the week"]
    #[inline(always)]
    pub fn dow(&mut self) -> DOW_W {
        DOW_W::new(self)
    }
    #[doc = "Bit 12 - Month tens in BCD code"]
    #[inline(always)]
    pub fn mont(&mut self) -> MONT_W {
        MONT_W::new(self)
    }
    #[doc = "Bits 8:11 - Month units in BCD code"]
    #[inline(always)]
    pub fn monu(&mut self) -> MONU_W {
        MONU_W::new(self)
    }
    #[doc = "Bits 4:5 - Date tens in BCD code"]
    #[inline(always)]
    pub fn dayt(&mut self) -> DAYT_W {
        DAYT_W::new(self)
    }
    #[doc = "Bits 0:3 - Date units in BCD code"]
    #[inline(always)]
    pub fn dayu(&mut self) -> DAYU_W {
        DAYU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "date register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](index.html) module"]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [date::R](R) reader structure"]
impl crate::Readable for DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [date::W](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATE to value 0x2101"]
impl crate::Resettable for DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2101
    }
}
