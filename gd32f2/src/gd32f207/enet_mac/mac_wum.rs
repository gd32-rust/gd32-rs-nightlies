#[doc = "Register `MAC_WUM` reader"]
pub struct R(crate::R<MAC_WUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_WUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_WUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_WUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_WUM` writer"]
pub struct W(crate::W<MAC_WUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_WUM_SPEC>;
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
impl From<crate::W<MAC_WUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_WUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWD` reader - Power down"]
pub type PWD_R = crate::BitReader<bool>;
#[doc = "Field `PWD` writer - Power down"]
pub type PWD_W<'a> = crate::BitWriter<'a, u32, MAC_WUM_SPEC, bool, 0>;
#[doc = "Field `MPEN` reader - Magic Packet enable"]
pub type MPEN_R = crate::BitReader<bool>;
#[doc = "Field `MPEN` writer - Magic Packet enable"]
pub type MPEN_W<'a> = crate::BitWriter<'a, u32, MAC_WUM_SPEC, bool, 1>;
#[doc = "Field `WFEN` reader - Wakeup frame enable"]
pub type WFEN_R = crate::BitReader<bool>;
#[doc = "Field `WFEN` writer - Wakeup frame enable"]
pub type WFEN_W<'a> = crate::BitWriter<'a, u32, MAC_WUM_SPEC, bool, 2>;
#[doc = "Field `MPKR` reader - Magic packet received"]
pub type MPKR_R = crate::BitReader<bool>;
#[doc = "Field `MPKR` writer - Magic packet received"]
pub type MPKR_W<'a> = crate::BitWriter<'a, u32, MAC_WUM_SPEC, bool, 5>;
#[doc = "Field `WUFR` reader - Wakeup frame received"]
pub type WUFR_R = crate::BitReader<bool>;
#[doc = "Field `WUFR` writer - Wakeup frame received"]
pub type WUFR_W<'a> = crate::BitWriter<'a, u32, MAC_WUM_SPEC, bool, 6>;
#[doc = "Field `GU` reader - Global unicast"]
pub type GU_R = crate::BitReader<bool>;
#[doc = "Field `GU` writer - Global unicast"]
pub type GU_W<'a> = crate::BitWriter<'a, u32, MAC_WUM_SPEC, bool, 9>;
#[doc = "Field `WUFFRPR` reader - Wakeup frame filter register pointer reset"]
pub type WUFFRPR_R = crate::BitReader<bool>;
#[doc = "Field `WUFFRPR` writer - Wakeup frame filter register pointer reset"]
pub type WUFFRPR_W<'a> = crate::BitWriter<'a, u32, MAC_WUM_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pwd(&self) -> PWD_R {
        PWD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    pub fn mpen(&self) -> MPEN_R {
        MPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfen(&self) -> WFEN_R {
        WFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpkr(&self) -> MPKR_R {
        MPKR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wufr(&self) -> WUFR_R {
        WUFR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wuffrpr(&self) -> WUFFRPR_R {
        WUFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pwd(&mut self) -> PWD_W {
        PWD_W::new(self)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    pub fn mpen(&mut self) -> MPEN_W {
        MPEN_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfen(&mut self) -> WFEN_W {
        WFEN_W::new(self)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpkr(&mut self) -> MPKR_W {
        MPKR_W::new(self)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wufr(&mut self) -> WUFR_W {
        WUFR_W::new(self)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&mut self) -> GU_W {
        GU_W::new(self)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wuffrpr(&mut self) -> WUFFRPR_W {
        WUFFRPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC wakeup management register (MAC_WUM)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_wum](index.html) module"]
pub struct MAC_WUM_SPEC;
impl crate::RegisterSpec for MAC_WUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_wum::R](R) reader structure"]
impl crate::Readable for MAC_WUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_wum::W](W) writer structure"]
impl crate::Writable for MAC_WUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_WUM to value 0"]
impl crate::Resettable for MAC_WUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
