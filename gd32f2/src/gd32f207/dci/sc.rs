#[doc = "Register `SC` reader"]
pub type R = crate::R<SC_SPEC>;
#[doc = "Register `SC` writer"]
pub type W = crate::W<SC_SPEC>;
#[doc = "Field `FS` reader - Frame start code in embedded synchronous mode"]
pub type FS_R = crate::FieldReader;
#[doc = "Field `FS` writer - Frame start code in embedded synchronous mode"]
pub type FS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LS` reader - Line start code in embedded synchronous mode"]
pub type LS_R = crate::FieldReader;
#[doc = "Field `LS` writer - Line start code in embedded synchronous mode"]
pub type LS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LE` reader - Line end code in embedded synchronous mode"]
pub type LE_R = crate::FieldReader;
#[doc = "Field `LE` writer - Line end code in embedded synchronous mode"]
pub type LE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FE` reader - Frame end code in embedded synchronous mode"]
pub type FE_R = crate::FieldReader;
#[doc = "Field `FE` writer - Frame end code in embedded synchronous mode"]
pub type FE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Frame start code in embedded synchronous mode"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start code in embedded synchronous mode"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end code in embedded synchronous mode"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end code in embedded synchronous mode"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start code in embedded synchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn fs(&mut self) -> FS_W<SC_SPEC, 0> {
        FS_W::new(self)
    }
    #[doc = "Bits 8:15 - Line start code in embedded synchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn ls(&mut self) -> LS_W<SC_SPEC, 8> {
        LS_W::new(self)
    }
    #[doc = "Bits 16:23 - Line end code in embedded synchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn le(&mut self) -> LE_W<SC_SPEC, 16> {
        LE_W::new(self)
    }
    #[doc = "Bits 24:31 - Frame end code in embedded synchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<SC_SPEC, 24> {
        FE_W::new(self)
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
#[doc = "DCI Synchronization codes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc::R`](R) reader structure"]
impl crate::Readable for SC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sc::W`](W) writer structure"]
impl crate::Writable for SC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
