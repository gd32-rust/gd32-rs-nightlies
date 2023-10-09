#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `I2CEN` reader - I2C peripheral enable"]
pub type I2CEN_R = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2C peripheral enable"]
pub type I2CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIE` reader - Receive interrupt enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Receive interrupt enable"]
pub type TIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBNEIE` reader - Receive interrupt enable"]
pub type RBNEIE_R = crate::BitReader;
#[doc = "Field `RBNEIE` writer - Receive interrupt enable"]
pub type RBNEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDMIE` reader - Address match interrupt enable in slave mode"]
pub type ADDMIE_R = crate::BitReader;
#[doc = "Field `ADDMIE` writer - Address match interrupt enable in slave mode"]
pub type ADDMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NACKIE` reader - Not acknowledge received interrupt enable"]
pub type NACKIE_R = crate::BitReader;
#[doc = "Field `NACKIE` writer - Not acknowledge received interrupt enable"]
pub type NACKIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STPDETIE` reader - Stop detection interrupt enable"]
pub type STPDETIE_R = crate::BitReader;
#[doc = "Field `STPDETIE` writer - Stop detection interrupt enable"]
pub type STPDETIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DNF` reader - Digital noise filter"]
pub type DNF_R = crate::FieldReader;
#[doc = "Field `DNF` writer - Digital noise filter"]
pub type DNF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ANOFF` reader - Analog noise filter disable"]
pub type ANOFF_R = crate::BitReader;
#[doc = "Field `ANOFF` writer - Analog noise filter disable"]
pub type ANOFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DENT` reader - DMA enable for transmission"]
pub type DENT_R = crate::BitReader;
#[doc = "Field `DENT` writer - DMA enable for transmission"]
pub type DENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DENR` reader - DMA enable for reception"]
pub type DENR_R = crate::BitReader;
#[doc = "Field `DENR` writer - DMA enable for reception"]
pub type DENR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SBCTL` reader - Slave byte control"]
pub type SBCTL_R = crate::BitReader;
#[doc = "Field `SBCTL` writer - Slave byte control"]
pub type SBCTL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SS` reader - Whether to stretch SCL low when data is not ready in slave mode"]
pub type SS_R = crate::BitReader;
#[doc = "Field `SS` writer - Whether to stretch SCL low when data is not ready in slave mode"]
pub type SS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUEN` reader - Wakeup from Deep-sleep mode enable"]
pub type WUEN_R = crate::BitReader;
#[doc = "Field `WUEN` writer - Wakeup from Deep-sleep mode enable"]
pub type WUEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GCEN` reader - Whether or not to response to a General Call"]
pub type GCEN_R = crate::BitReader;
#[doc = "Field `GCEN` writer - Whether or not to response to a General Call"]
pub type GCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBHAEN` reader - SMBus Host address enable"]
pub type SMBHAEN_R = crate::BitReader;
#[doc = "Field `SMBHAEN` writer - SMBus Host address enable"]
pub type SMBHAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBDAEN` reader - SMBus device default address enable"]
pub type SMBDAEN_R = crate::BitReader;
#[doc = "Field `SMBDAEN` writer - SMBus device default address enable"]
pub type SMBDAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBALTEN` reader - SMBus Alert enable"]
pub type SMBALTEN_R = crate::BitReader;
#[doc = "Field `SMBALTEN` writer - SMBus Alert enable"]
pub type SMBALTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECEN` reader - PEC Calculation Switch"]
pub type PECEN_R = crate::BitReader;
#[doc = "Field `PECEN` writer - PEC Calculation Switch"]
pub type PECEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match interrupt enable in slave mode"]
    #[inline(always)]
    pub fn addmie(&self) -> ADDMIE_R {
        ADDMIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection interrupt enable"]
    #[inline(always)]
    pub fn stpdetie(&self) -> STPDETIE_R {
        STPDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Analog noise filter disable"]
    #[inline(always)]
    pub fn anoff(&self) -> ANOFF_R {
        ANOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA enable for transmission"]
    #[inline(always)]
    pub fn dent(&self) -> DENT_R {
        DENT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA enable for reception"]
    #[inline(always)]
    pub fn denr(&self) -> DENR_R {
        DENR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave byte control"]
    #[inline(always)]
    pub fn sbctl(&self) -> SBCTL_R {
        SBCTL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Whether to stretch SCL low when data is not ready in slave mode"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wakeup from Deep-sleep mode enable"]
    #[inline(always)]
    pub fn wuen(&self) -> WUEN_R {
        WUEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Whether or not to response to a General Call"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Host address enable"]
    #[inline(always)]
    pub fn smbhaen(&self) -> SMBHAEN_R {
        SMBHAEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus device default address enable"]
    #[inline(always)]
    pub fn smbdaen(&self) -> SMBDAEN_R {
        SMBDAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBus Alert enable"]
    #[inline(always)]
    pub fn smbalten(&self) -> SMBALTEN_R {
        SMBALTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC Calculation Switch"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<CTL0_SPEC, 0> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 1 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<CTL0_SPEC, 1> {
        TIE_W::new(self)
    }
    #[doc = "Bit 2 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbneie(&mut self) -> RBNEIE_W<CTL0_SPEC, 2> {
        RBNEIE_W::new(self)
    }
    #[doc = "Bit 3 - Address match interrupt enable in slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn addmie(&mut self) -> ADDMIE_W<CTL0_SPEC, 3> {
        ADDMIE_W::new(self)
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackie(&mut self) -> NACKIE_W<CTL0_SPEC, 4> {
        NACKIE_W::new(self)
    }
    #[doc = "Bit 5 - Stop detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stpdetie(&mut self) -> STPDETIE_W<CTL0_SPEC, 5> {
        STPDETIE_W::new(self)
    }
    #[doc = "Bit 6 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CTL0_SPEC, 6> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 7 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL0_SPEC, 7> {
        ERRIE_W::new(self)
    }
    #[doc = "Bits 8:11 - Digital noise filter"]
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DNF_W<CTL0_SPEC, 8> {
        DNF_W::new(self)
    }
    #[doc = "Bit 12 - Analog noise filter disable"]
    #[inline(always)]
    #[must_use]
    pub fn anoff(&mut self) -> ANOFF_W<CTL0_SPEC, 12> {
        ANOFF_W::new(self)
    }
    #[doc = "Bit 14 - DMA enable for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn dent(&mut self) -> DENT_W<CTL0_SPEC, 14> {
        DENT_W::new(self)
    }
    #[doc = "Bit 15 - DMA enable for reception"]
    #[inline(always)]
    #[must_use]
    pub fn denr(&mut self) -> DENR_W<CTL0_SPEC, 15> {
        DENR_W::new(self)
    }
    #[doc = "Bit 16 - Slave byte control"]
    #[inline(always)]
    #[must_use]
    pub fn sbctl(&mut self) -> SBCTL_W<CTL0_SPEC, 16> {
        SBCTL_W::new(self)
    }
    #[doc = "Bit 17 - Whether to stretch SCL low when data is not ready in slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<CTL0_SPEC, 17> {
        SS_W::new(self)
    }
    #[doc = "Bit 18 - Wakeup from Deep-sleep mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn wuen(&mut self) -> WUEN_W<CTL0_SPEC, 18> {
        WUEN_W::new(self)
    }
    #[doc = "Bit 19 - Whether or not to response to a General Call"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<CTL0_SPEC, 19> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 20 - SMBus Host address enable"]
    #[inline(always)]
    #[must_use]
    pub fn smbhaen(&mut self) -> SMBHAEN_W<CTL0_SPEC, 20> {
        SMBHAEN_W::new(self)
    }
    #[doc = "Bit 21 - SMBus device default address enable"]
    #[inline(always)]
    #[must_use]
    pub fn smbdaen(&mut self) -> SMBDAEN_W<CTL0_SPEC, 21> {
        SMBDAEN_W::new(self)
    }
    #[doc = "Bit 22 - SMBus Alert enable"]
    #[inline(always)]
    #[must_use]
    pub fn smbalten(&mut self) -> SMBALTEN_W<CTL0_SPEC, 22> {
        SMBALTEN_W::new(self)
    }
    #[doc = "Bit 23 - PEC Calculation Switch"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<CTL0_SPEC, 23> {
        PECEN_W::new(self)
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
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
