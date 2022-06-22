#[doc = "Register `CPU_IRQ_LAT` reader"]
pub struct R(crate::R<CPU_IRQ_LAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_IRQ_LAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_IRQ_LAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_IRQ_LAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_IRQ_LAT` writer"]
pub struct W(crate::W<CPU_IRQ_LAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_IRQ_LAT_SPEC>;
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
impl From<crate::W<CPU_IRQ_LAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_IRQ_LAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_LATENCY` reader - specifies the minimum number of cycles between an interrupt"]
pub type IRQ_LATENCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQ_LATENCY` writer - specifies the minimum number of cycles between an interrupt"]
pub type IRQ_LATENCY_W<'a> = crate::FieldWriter<'a, u32, CPU_IRQ_LAT_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - specifies the minimum number of cycles between an interrupt"]
    #[inline(always)]
    pub fn irq_latency(&self) -> IRQ_LATENCY_R {
        IRQ_LATENCY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - specifies the minimum number of cycles between an interrupt"]
    #[inline(always)]
    pub fn irq_latency(&mut self) -> IRQ_LATENCY_W {
        IRQ_LATENCY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRQ Latency register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_irq_lat](index.html) module"]
pub struct CPU_IRQ_LAT_SPEC;
impl crate::RegisterSpec for CPU_IRQ_LAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_irq_lat::R](R) reader structure"]
impl crate::Readable for CPU_IRQ_LAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_irq_lat::W](W) writer structure"]
impl crate::Writable for CPU_IRQ_LAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_IRQ_LAT to value 0"]
impl crate::Resettable for CPU_IRQ_LAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
