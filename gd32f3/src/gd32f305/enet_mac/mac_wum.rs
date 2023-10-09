#[doc = "Register `MAC_WUM` reader"]
pub type R = crate::R<MAC_WUM_SPEC>;
#[doc = "Register `MAC_WUM` writer"]
pub type W = crate::W<MAC_WUM_SPEC>;
#[doc = "Field `PWD` reader - Power down"]
pub type PWD_R = crate::BitReader;
#[doc = "Field `PWD` writer - Power down"]
pub type PWD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPEN` reader - Magic Packet enable"]
pub type MPEN_R = crate::BitReader;
#[doc = "Field `MPEN` writer - Magic Packet enable"]
pub type MPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WFEN` reader - Wakeup frame enable"]
pub type WFEN_R = crate::BitReader;
#[doc = "Field `WFEN` writer - Wakeup frame enable"]
pub type WFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPKR` reader - Magic packet received"]
pub type MPKR_R = crate::BitReader;
#[doc = "Field `MPKR` writer - Magic packet received"]
pub type MPKR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUFR` reader - Wakeup frame received"]
pub type WUFR_R = crate::BitReader;
#[doc = "Field `WUFR` writer - Wakeup frame received"]
pub type WUFR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GU` reader - Global unicast"]
pub type GU_R = crate::BitReader;
#[doc = "Field `GU` writer - Global unicast"]
pub type GU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUFFRPR` reader - Wakeup frame filter register pointer reset"]
pub type WUFFRPR_R = crate::BitReader;
#[doc = "Field `WUFFRPR` writer - Wakeup frame filter register pointer reset"]
pub type WUFFRPR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn pwd(&mut self) -> PWD_W<MAC_WUM_SPEC, 0> {
        PWD_W::new(self)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpen(&mut self) -> MPEN_W<MAC_WUM_SPEC, 1> {
        MPEN_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    #[must_use]
    pub fn wfen(&mut self) -> WFEN_W<MAC_WUM_SPEC, 2> {
        WFEN_W::new(self)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    #[must_use]
    pub fn mpkr(&mut self) -> MPKR_W<MAC_WUM_SPEC, 5> {
        MPKR_W::new(self)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    #[must_use]
    pub fn wufr(&mut self) -> WUFR_W<MAC_WUM_SPEC, 6> {
        WUFR_W::new(self)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    #[must_use]
    pub fn gu(&mut self) -> GU_W<MAC_WUM_SPEC, 9> {
        GU_W::new(self)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    #[must_use]
    pub fn wuffrpr(&mut self) -> WUFFRPR_W<MAC_WUM_SPEC, 31> {
        WUFFRPR_W::new(self)
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
#[doc = "Ethernet MAC wakeup management register (MAC_WUM)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_wum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_wum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_WUM_SPEC;
impl crate::RegisterSpec for MAC_WUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_wum::R`](R) reader structure"]
impl crate::Readable for MAC_WUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_wum::W`](W) writer structure"]
impl crate::Writable for MAC_WUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_WUM to value 0"]
impl crate::Resettable for MAC_WUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
