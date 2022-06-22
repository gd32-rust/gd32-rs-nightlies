#[doc = "Register `PCF4` reader"]
pub struct R(crate::R<PCF4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCF4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCF4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCF4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCF4` writer"]
pub struct W(crate::W<PCF4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCF4_SPEC>;
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
impl From<crate::W<PCF4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCF4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI2_MOSI_REMAP` reader - SPI2_MOSI remapping"]
pub type SPI2_MOSI_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `SPI2_MOSI_REMAP` writer - SPI2_MOSI remapping"]
pub type SPI2_MOSI_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 24>;
#[doc = "Field `SPI1_SCK_REMAP` reader - SPI1_SCK remapping"]
pub type SPI1_SCK_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `SPI1_SCK_REMAP` writer - SPI1_SCK remapping"]
pub type SPI1_SCK_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 23>;
#[doc = "Field `TLI_R1_PI3_REMAP` reader - TLI_R1_PI3 remapping"]
pub type TLI_R1_PI3_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R1_PI3_REMAP` writer - TLI_R1_PI3 remapping"]
pub type TLI_R1_PI3_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 22>;
#[doc = "Field `TLI_R0_PH4_REMAP` reader - TLI_R0_PH4 remapping"]
pub type TLI_R0_PH4_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R0_PH4_REMAP` writer - TLI_R0_PH4 remapping"]
pub type TLI_R0_PH4_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 21>;
#[doc = "Field `TLI_HSYNC_PI10_REMAP` reader - TLI_HSYNC_PI10 remapping"]
pub type TLI_HSYNC_PI10_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_HSYNC_PI10_REMAP` writer - TLI_HSYNC_PI10 remapping"]
pub type TLI_HSYNC_PI10_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 20>;
#[doc = "Field `TLI_VSYNC_PI9_REMAP` reader - TLI_VSYNC_PI9 remapping"]
pub type TLI_VSYNC_PI9_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_VSYNC_PI9_REMAP` writer - TLI_VSYNC_PI9 remapping"]
pub type TLI_VSYNC_PI9_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 19>;
#[doc = "Field `TLI_B7_PI7_REMAP` reader - TLI_B7_PI7 remapping"]
pub type TLI_B7_PI7_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B7_PI7_REMAP` writer - TLI_B7_PI7 remapping"]
pub type TLI_B7_PI7_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 18>;
#[doc = "Field `TLI_B6_PI6_REMAP` reader - TLI_B6_PI6 remapping"]
pub type TLI_B6_PI6_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B6_PI6_REMAP` writer - TLI_B6_PI6 remapping"]
pub type TLI_B6_PI6_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 17>;
#[doc = "Field `TLI_B5_PI5_REMAP` reader - TLI_B5_PI5 remapping"]
pub type TLI_B5_PI5_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B5_PI5_REMAP` writer - TLI_B5_PI5 remapping"]
pub type TLI_B5_PI5_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 16>;
#[doc = "Field `TLI_B4_PI4_REMAP` reader - TLI_B4_PI4 remapping"]
pub type TLI_B4_PI4_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B4_PI4_REMAP` writer - TLI_B4_PI4 remapping"]
pub type TLI_B4_PI4_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 15>;
#[doc = "Field `TLI_G7_PI2_REMAP` reader - TLI_G7_PI2 remapping"]
pub type TLI_G7_PI2_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G7_PI2_REMAP` writer - TLI_G7_PI2 remapping"]
pub type TLI_G7_PI2_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 14>;
#[doc = "Field `TLI_G6_PI1_REMAP` reader - TLI_G6_PI1 remapping"]
pub type TLI_G6_PI1_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G6_PI1_REMAP` writer - TLI_G6_PI1 remapping"]
pub type TLI_G6_PI1_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 13>;
#[doc = "Field `TLI_G5_PI0_REMAP` reader - TLI_G5_PI0 remapping"]
pub type TLI_G5_PI0_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G5_PI0_REMAP` writer - TLI_G5_PI0 remapping"]
pub type TLI_G5_PI0_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 12>;
#[doc = "Field `TLI_G4_PH15_REMAP` reader - TLI_G4_PH15 remapping"]
pub type TLI_G4_PH15_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G4_PH15_REMAP` writer - TLI_G4_PH15 remapping"]
pub type TLI_G4_PH15_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 11>;
#[doc = "Field `TLI_G3_PH14_REMAP` reader - TLI_G3_PH14 remapping"]
pub type TLI_G3_PH14_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G3_PH14_REMAP` writer - TLI_G3_PH14 remapping"]
pub type TLI_G3_PH14_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 10>;
#[doc = "Field `TLI_G2_PH13_REMAP` reader - TLI_G2_PH13 remapping"]
pub type TLI_G2_PH13_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G2_PH13_REMAP` writer - TLI_G2_PH13 remapping"]
pub type TLI_G2_PH13_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 9>;
#[doc = "Field `TLI_R6_PH12_REMAP` reader - TLI_R6_PH12 remapping"]
pub type TLI_R6_PH12_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R6_PH12_REMAP` writer - TLI_R6_PH12 remapping"]
pub type TLI_R6_PH12_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 8>;
#[doc = "Field `TLI_R5_PH11_REMAP` reader - TLI_R5_PH11 remapping"]
pub type TLI_R5_PH11_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R5_PH11_REMAP` writer - TLI_R5_PH11 remapping"]
pub type TLI_R5_PH11_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 7>;
#[doc = "Field `TLI_R4_PH10_REMAP` reader - TLI_R4_PH10 remapping"]
pub type TLI_R4_PH10_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R4_PH10_REMAP` writer - TLI_R4_PH10 remapping"]
pub type TLI_R4_PH10_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 6>;
#[doc = "Field `TLI_R3_PH9_REMAP` reader - TLI_R3_PH9 remapping"]
pub type TLI_R3_PH9_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R3_PH9_REMAP` writer - TLI_R3_PH9 remapping"]
pub type TLI_R3_PH9_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 5>;
#[doc = "Field `TLI_R2_PH8_REMAP` reader - TLI_R2_PH8 remapping"]
pub type TLI_R2_PH8_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R2_PH8_REMAP` writer - TLI_R2_PH8 remapping"]
pub type TLI_R2_PH8_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 4>;
#[doc = "Field `TLI_R1_PH3_REMAP` reader - TLI_R1_PH3 remapping"]
pub type TLI_R1_PH3_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R1_PH3_REMAP` writer - TLI_R1_PH3 remapping"]
pub type TLI_R1_PH3_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 3>;
#[doc = "Field `TLI_R0_PH2_REMAP` reader - TLI_R0_PH2 remapping"]
pub type TLI_R0_PH2_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R0_PH2_REMAP` writer - TLI_R0_PH2 remapping"]
pub type TLI_R0_PH2_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 2>;
#[doc = "Field `TLI_B1_PG12_REMAP` reader - TLI_B1_PG12 remapping"]
pub type TLI_B1_PG12_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B1_PG12_REMAP` writer - TLI_B1_PG12 remapping"]
pub type TLI_B1_PG12_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 1>;
#[doc = "Field `TLI_B4_PG12_REMAP` reader - TLI_B4_PG12 remapping"]
pub type TLI_B4_PG12_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B4_PG12_REMAP` writer - TLI_B4_PG12 remapping"]
pub type TLI_B4_PG12_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF4_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 24 - SPI2_MOSI remapping"]
    #[inline(always)]
    pub fn spi2_mosi_remap(&self) -> SPI2_MOSI_REMAP_R {
        SPI2_MOSI_REMAP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - SPI1_SCK remapping"]
    #[inline(always)]
    pub fn spi1_sck_remap(&self) -> SPI1_SCK_REMAP_R {
        SPI1_SCK_REMAP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - TLI_R1_PI3 remapping"]
    #[inline(always)]
    pub fn tli_r1_pi3_remap(&self) -> TLI_R1_PI3_REMAP_R {
        TLI_R1_PI3_REMAP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - TLI_R0_PH4 remapping"]
    #[inline(always)]
    pub fn tli_r0_ph4_remap(&self) -> TLI_R0_PH4_REMAP_R {
        TLI_R0_PH4_REMAP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - TLI_HSYNC_PI10 remapping"]
    #[inline(always)]
    pub fn tli_hsync_pi10_remap(&self) -> TLI_HSYNC_PI10_REMAP_R {
        TLI_HSYNC_PI10_REMAP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - TLI_VSYNC_PI9 remapping"]
    #[inline(always)]
    pub fn tli_vsync_pi9_remap(&self) -> TLI_VSYNC_PI9_REMAP_R {
        TLI_VSYNC_PI9_REMAP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - TLI_B7_PI7 remapping"]
    #[inline(always)]
    pub fn tli_b7_pi7_remap(&self) -> TLI_B7_PI7_REMAP_R {
        TLI_B7_PI7_REMAP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - TLI_B6_PI6 remapping"]
    #[inline(always)]
    pub fn tli_b6_pi6_remap(&self) -> TLI_B6_PI6_REMAP_R {
        TLI_B6_PI6_REMAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - TLI_B5_PI5 remapping"]
    #[inline(always)]
    pub fn tli_b5_pi5_remap(&self) -> TLI_B5_PI5_REMAP_R {
        TLI_B5_PI5_REMAP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - TLI_B4_PI4 remapping"]
    #[inline(always)]
    pub fn tli_b4_pi4_remap(&self) -> TLI_B4_PI4_REMAP_R {
        TLI_B4_PI4_REMAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - TLI_G7_PI2 remapping"]
    #[inline(always)]
    pub fn tli_g7_pi2_remap(&self) -> TLI_G7_PI2_REMAP_R {
        TLI_G7_PI2_REMAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - TLI_G6_PI1 remapping"]
    #[inline(always)]
    pub fn tli_g6_pi1_remap(&self) -> TLI_G6_PI1_REMAP_R {
        TLI_G6_PI1_REMAP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - TLI_G5_PI0 remapping"]
    #[inline(always)]
    pub fn tli_g5_pi0_remap(&self) -> TLI_G5_PI0_REMAP_R {
        TLI_G5_PI0_REMAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - TLI_G4_PH15 remapping"]
    #[inline(always)]
    pub fn tli_g4_ph15_remap(&self) -> TLI_G4_PH15_REMAP_R {
        TLI_G4_PH15_REMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - TLI_G3_PH14 remapping"]
    #[inline(always)]
    pub fn tli_g3_ph14_remap(&self) -> TLI_G3_PH14_REMAP_R {
        TLI_G3_PH14_REMAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - TLI_G2_PH13 remapping"]
    #[inline(always)]
    pub fn tli_g2_ph13_remap(&self) -> TLI_G2_PH13_REMAP_R {
        TLI_G2_PH13_REMAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - TLI_R6_PH12 remapping"]
    #[inline(always)]
    pub fn tli_r6_ph12_remap(&self) -> TLI_R6_PH12_REMAP_R {
        TLI_R6_PH12_REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - TLI_R5_PH11 remapping"]
    #[inline(always)]
    pub fn tli_r5_ph11_remap(&self) -> TLI_R5_PH11_REMAP_R {
        TLI_R5_PH11_REMAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - TLI_R4_PH10 remapping"]
    #[inline(always)]
    pub fn tli_r4_ph10_remap(&self) -> TLI_R4_PH10_REMAP_R {
        TLI_R4_PH10_REMAP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - TLI_R3_PH9 remapping"]
    #[inline(always)]
    pub fn tli_r3_ph9_remap(&self) -> TLI_R3_PH9_REMAP_R {
        TLI_R3_PH9_REMAP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - TLI_R2_PH8 remapping"]
    #[inline(always)]
    pub fn tli_r2_ph8_remap(&self) -> TLI_R2_PH8_REMAP_R {
        TLI_R2_PH8_REMAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - TLI_R1_PH3 remapping"]
    #[inline(always)]
    pub fn tli_r1_ph3_remap(&self) -> TLI_R1_PH3_REMAP_R {
        TLI_R1_PH3_REMAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - TLI_R0_PH2 remapping"]
    #[inline(always)]
    pub fn tli_r0_ph2_remap(&self) -> TLI_R0_PH2_REMAP_R {
        TLI_R0_PH2_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - TLI_B1_PG12 remapping"]
    #[inline(always)]
    pub fn tli_b1_pg12_remap(&self) -> TLI_B1_PG12_REMAP_R {
        TLI_B1_PG12_REMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TLI_B4_PG12 remapping"]
    #[inline(always)]
    pub fn tli_b4_pg12_remap(&self) -> TLI_B4_PG12_REMAP_R {
        TLI_B4_PG12_REMAP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - SPI2_MOSI remapping"]
    #[inline(always)]
    pub fn spi2_mosi_remap(&mut self) -> SPI2_MOSI_REMAP_W {
        SPI2_MOSI_REMAP_W::new(self)
    }
    #[doc = "Bit 23 - SPI1_SCK remapping"]
    #[inline(always)]
    pub fn spi1_sck_remap(&mut self) -> SPI1_SCK_REMAP_W {
        SPI1_SCK_REMAP_W::new(self)
    }
    #[doc = "Bit 22 - TLI_R1_PI3 remapping"]
    #[inline(always)]
    pub fn tli_r1_pi3_remap(&mut self) -> TLI_R1_PI3_REMAP_W {
        TLI_R1_PI3_REMAP_W::new(self)
    }
    #[doc = "Bit 21 - TLI_R0_PH4 remapping"]
    #[inline(always)]
    pub fn tli_r0_ph4_remap(&mut self) -> TLI_R0_PH4_REMAP_W {
        TLI_R0_PH4_REMAP_W::new(self)
    }
    #[doc = "Bit 20 - TLI_HSYNC_PI10 remapping"]
    #[inline(always)]
    pub fn tli_hsync_pi10_remap(&mut self) -> TLI_HSYNC_PI10_REMAP_W {
        TLI_HSYNC_PI10_REMAP_W::new(self)
    }
    #[doc = "Bit 19 - TLI_VSYNC_PI9 remapping"]
    #[inline(always)]
    pub fn tli_vsync_pi9_remap(&mut self) -> TLI_VSYNC_PI9_REMAP_W {
        TLI_VSYNC_PI9_REMAP_W::new(self)
    }
    #[doc = "Bit 18 - TLI_B7_PI7 remapping"]
    #[inline(always)]
    pub fn tli_b7_pi7_remap(&mut self) -> TLI_B7_PI7_REMAP_W {
        TLI_B7_PI7_REMAP_W::new(self)
    }
    #[doc = "Bit 17 - TLI_B6_PI6 remapping"]
    #[inline(always)]
    pub fn tli_b6_pi6_remap(&mut self) -> TLI_B6_PI6_REMAP_W {
        TLI_B6_PI6_REMAP_W::new(self)
    }
    #[doc = "Bit 16 - TLI_B5_PI5 remapping"]
    #[inline(always)]
    pub fn tli_b5_pi5_remap(&mut self) -> TLI_B5_PI5_REMAP_W {
        TLI_B5_PI5_REMAP_W::new(self)
    }
    #[doc = "Bit 15 - TLI_B4_PI4 remapping"]
    #[inline(always)]
    pub fn tli_b4_pi4_remap(&mut self) -> TLI_B4_PI4_REMAP_W {
        TLI_B4_PI4_REMAP_W::new(self)
    }
    #[doc = "Bit 14 - TLI_G7_PI2 remapping"]
    #[inline(always)]
    pub fn tli_g7_pi2_remap(&mut self) -> TLI_G7_PI2_REMAP_W {
        TLI_G7_PI2_REMAP_W::new(self)
    }
    #[doc = "Bit 13 - TLI_G6_PI1 remapping"]
    #[inline(always)]
    pub fn tli_g6_pi1_remap(&mut self) -> TLI_G6_PI1_REMAP_W {
        TLI_G6_PI1_REMAP_W::new(self)
    }
    #[doc = "Bit 12 - TLI_G5_PI0 remapping"]
    #[inline(always)]
    pub fn tli_g5_pi0_remap(&mut self) -> TLI_G5_PI0_REMAP_W {
        TLI_G5_PI0_REMAP_W::new(self)
    }
    #[doc = "Bit 11 - TLI_G4_PH15 remapping"]
    #[inline(always)]
    pub fn tli_g4_ph15_remap(&mut self) -> TLI_G4_PH15_REMAP_W {
        TLI_G4_PH15_REMAP_W::new(self)
    }
    #[doc = "Bit 10 - TLI_G3_PH14 remapping"]
    #[inline(always)]
    pub fn tli_g3_ph14_remap(&mut self) -> TLI_G3_PH14_REMAP_W {
        TLI_G3_PH14_REMAP_W::new(self)
    }
    #[doc = "Bit 9 - TLI_G2_PH13 remapping"]
    #[inline(always)]
    pub fn tli_g2_ph13_remap(&mut self) -> TLI_G2_PH13_REMAP_W {
        TLI_G2_PH13_REMAP_W::new(self)
    }
    #[doc = "Bit 8 - TLI_R6_PH12 remapping"]
    #[inline(always)]
    pub fn tli_r6_ph12_remap(&mut self) -> TLI_R6_PH12_REMAP_W {
        TLI_R6_PH12_REMAP_W::new(self)
    }
    #[doc = "Bit 7 - TLI_R5_PH11 remapping"]
    #[inline(always)]
    pub fn tli_r5_ph11_remap(&mut self) -> TLI_R5_PH11_REMAP_W {
        TLI_R5_PH11_REMAP_W::new(self)
    }
    #[doc = "Bit 6 - TLI_R4_PH10 remapping"]
    #[inline(always)]
    pub fn tli_r4_ph10_remap(&mut self) -> TLI_R4_PH10_REMAP_W {
        TLI_R4_PH10_REMAP_W::new(self)
    }
    #[doc = "Bit 5 - TLI_R3_PH9 remapping"]
    #[inline(always)]
    pub fn tli_r3_ph9_remap(&mut self) -> TLI_R3_PH9_REMAP_W {
        TLI_R3_PH9_REMAP_W::new(self)
    }
    #[doc = "Bit 4 - TLI_R2_PH8 remapping"]
    #[inline(always)]
    pub fn tli_r2_ph8_remap(&mut self) -> TLI_R2_PH8_REMAP_W {
        TLI_R2_PH8_REMAP_W::new(self)
    }
    #[doc = "Bit 3 - TLI_R1_PH3 remapping"]
    #[inline(always)]
    pub fn tli_r1_ph3_remap(&mut self) -> TLI_R1_PH3_REMAP_W {
        TLI_R1_PH3_REMAP_W::new(self)
    }
    #[doc = "Bit 2 - TLI_R0_PH2 remapping"]
    #[inline(always)]
    pub fn tli_r0_ph2_remap(&mut self) -> TLI_R0_PH2_REMAP_W {
        TLI_R0_PH2_REMAP_W::new(self)
    }
    #[doc = "Bit 1 - TLI_B1_PG12 remapping"]
    #[inline(always)]
    pub fn tli_b1_pg12_remap(&mut self) -> TLI_B1_PG12_REMAP_W {
        TLI_B1_PG12_REMAP_W::new(self)
    }
    #[doc = "Bit 0 - TLI_B4_PG12 remapping"]
    #[inline(always)]
    pub fn tli_b4_pg12_remap(&mut self) -> TLI_B4_PG12_REMAP_W {
        TLI_B4_PG12_REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO port configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcf4](index.html) module"]
pub struct PCF4_SPEC;
impl crate::RegisterSpec for PCF4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcf4::R](R) reader structure"]
impl crate::Readable for PCF4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcf4::W](W) writer structure"]
impl crate::Writable for PCF4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCF4 to value 0"]
impl crate::Resettable for PCF4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
