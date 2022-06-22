#[doc = "Register `MAC_INTMSK` reader"]
pub struct R(crate::R<MAC_INTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_INTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_INTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_INTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_INTMSK` writer"]
pub struct W(crate::W<MAC_INTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_INTMSK_SPEC>;
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
impl From<crate::W<MAC_INTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_INTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUMIM` reader - WUM interrupt mask"]
pub type WUMIM_R = crate::BitReader<bool>;
#[doc = "Field `WUMIM` writer - WUM interrupt mask"]
pub type WUMIM_W<'a> = crate::BitWriter<'a, u32, MAC_INTMSK_SPEC, bool, 3>;
#[doc = "Field `TMSTIM` reader - Time stamp trigger interrupt mask"]
pub type TMSTIM_R = crate::BitReader<bool>;
#[doc = "Field `TMSTIM` writer - Time stamp trigger interrupt mask"]
pub type TMSTIM_W<'a> = crate::BitWriter<'a, u32, MAC_INTMSK_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 3 - WUM interrupt mask"]
    #[inline(always)]
    pub fn wumim(&self) -> WUMIM_R {
        WUMIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    pub fn tmstim(&self) -> TMSTIM_R {
        TMSTIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - WUM interrupt mask"]
    #[inline(always)]
    pub fn wumim(&mut self) -> WUMIM_W {
        WUMIM_W::new(self)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    pub fn tmstim(&mut self) -> TMSTIM_W {
        TMSTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC interrupt mask register (MAC_INTMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_intmsk](index.html) module"]
pub struct MAC_INTMSK_SPEC;
impl crate::RegisterSpec for MAC_INTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_intmsk::R](R) reader structure"]
impl crate::Readable for MAC_INTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_intmsk::W](W) writer structure"]
impl crate::Writable for MAC_INTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_INTMSK to value 0"]
impl crate::Resettable for MAC_INTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
