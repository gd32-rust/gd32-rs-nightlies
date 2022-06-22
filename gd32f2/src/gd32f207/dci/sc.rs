#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl From<crate::W<SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FE` reader - Frame end code in embedded synchronous mode"]
pub type FE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FE` writer - Frame end code in embedded synchronous mode"]
pub type FE_W<'a> = crate::FieldWriter<'a, u32, SC_SPEC, u8, u8, 8, 24>;
#[doc = "Field `LE` reader - Line end code in embedded synchronous mode"]
pub type LE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LE` writer - Line end code in embedded synchronous mode"]
pub type LE_W<'a> = crate::FieldWriter<'a, u32, SC_SPEC, u8, u8, 8, 16>;
#[doc = "Field `LS` reader - Line start code in embedded synchronous mode"]
pub type LS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LS` writer - Line start code in embedded synchronous mode"]
pub type LS_W<'a> = crate::FieldWriter<'a, u32, SC_SPEC, u8, u8, 8, 8>;
#[doc = "Field `FS` reader - Frame start code in embedded synchronous mode"]
pub type FS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS` writer - Frame start code in embedded synchronous mode"]
pub type FS_W<'a> = crate::FieldWriter<'a, u32, SC_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 24:31 - Frame end code in embedded synchronous mode"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end code in embedded synchronous mode"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start code in embedded synchronous mode"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Frame start code in embedded synchronous mode"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Frame end code in embedded synchronous mode"]
    #[inline(always)]
    pub fn fe(&mut self) -> FE_W {
        FE_W::new(self)
    }
    #[doc = "Bits 16:23 - Line end code in embedded synchronous mode"]
    #[inline(always)]
    pub fn le(&mut self) -> LE_W {
        LE_W::new(self)
    }
    #[doc = "Bits 8:15 - Line start code in embedded synchronous mode"]
    #[inline(always)]
    pub fn ls(&mut self) -> LS_W {
        LS_W::new(self)
    }
    #[doc = "Bits 0:7 - Frame start code in embedded synchronous mode"]
    #[inline(always)]
    pub fn fs(&mut self) -> FS_W {
        FS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCI Synchronization codes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
