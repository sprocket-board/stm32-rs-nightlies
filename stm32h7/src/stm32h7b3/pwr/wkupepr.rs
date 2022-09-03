#[doc = "Register `WKUPEPR` reader"]
pub struct R(crate::R<WKUPEPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUPEPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUPEPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUPEPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUPEPR` writer"]
pub struct W(crate::W<WKUPEPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUPEPR_SPEC>;
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
impl From<crate::W<WKUPEPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUPEPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPEN1` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN1_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN1` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPEN2` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN2_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN2` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPEN3` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN3_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN3` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPEN4` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN4_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN4` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPEN5` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN5_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN5` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPEN6` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN6_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN6` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type WKUPEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPP1` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP1_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP1` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPP2` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP2_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP2` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPP3` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP3_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP3` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPP4` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP4_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP4` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPP5` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP5_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP5` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPP6` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP6_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP6` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type WKUPP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPEPR_SPEC, bool, O>;
#[doc = "Field `WKUPPUPD1` reader - Wakeup pin pull configuration"]
pub type WKUPPUPD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD1` writer - Wakeup pin pull configuration"]
pub type WKUPPUPD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUPEPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WKUPPUPD2` reader - Wakeup pin pull configuration"]
pub type WKUPPUPD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD2` writer - Wakeup pin pull configuration"]
pub type WKUPPUPD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUPEPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WKUPPUPD3` reader - Wakeup pin pull configuration"]
pub type WKUPPUPD3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD3` writer - Wakeup pin pull configuration"]
pub type WKUPPUPD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUPEPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WKUPPUPD4` reader - Wakeup pin pull configuration"]
pub type WKUPPUPD4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD4` writer - Wakeup pin pull configuration"]
pub type WKUPPUPD4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUPEPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WKUPPUPD5` reader - Wakeup pin pull configuration"]
pub type WKUPPUPD5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD5` writer - Wakeup pin pull configuration"]
pub type WKUPPUPD5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUPEPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WKUPPUPD6` reader - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
pub type WKUPPUPD6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD6` writer - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
pub type WKUPPUPD6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUPEPR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp1(&self) -> WKUPP1_R {
        WKUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp2(&self) -> WKUPP2_R {
        WKUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp3(&self) -> WKUPP3_R {
        WKUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp4(&self) -> WKUPP4_R {
        WKUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp5(&self) -> WKUPP5_R {
        WKUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp6(&self) -> WKUPP6_R {
        WKUPP6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd1(&self) -> WKUPPUPD1_R {
        WKUPPUPD1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd2(&self) -> WKUPPUPD2_R {
        WKUPPUPD2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd3(&self) -> WKUPPUPD3_R {
        WKUPPUPD3_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd4(&self) -> WKUPPUPD4_R {
        WKUPPUPD4_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd5(&self) -> WKUPPUPD5_R {
        WKUPPUPD5_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wkuppupd6(&self) -> WKUPPUPD6_R {
        WKUPPUPD6_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen1(&mut self) -> WKUPEN1_W<0> {
        WKUPEN1_W::new(self)
    }
    #[doc = "Bit 1 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen2(&mut self) -> WKUPEN2_W<1> {
        WKUPEN2_W::new(self)
    }
    #[doc = "Bit 2 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen3(&mut self) -> WKUPEN3_W<2> {
        WKUPEN3_W::new(self)
    }
    #[doc = "Bit 3 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen4(&mut self) -> WKUPEN4_W<3> {
        WKUPEN4_W::new(self)
    }
    #[doc = "Bit 4 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen5(&mut self) -> WKUPEN5_W<4> {
        WKUPEN5_W::new(self)
    }
    #[doc = "Bit 5 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen6(&mut self) -> WKUPEN6_W<5> {
        WKUPEN6_W::new(self)
    }
    #[doc = "Bit 8 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp1(&mut self) -> WKUPP1_W<8> {
        WKUPP1_W::new(self)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp2(&mut self) -> WKUPP2_W<9> {
        WKUPP2_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp3(&mut self) -> WKUPP3_W<10> {
        WKUPP3_W::new(self)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp4(&mut self) -> WKUPP4_W<11> {
        WKUPP4_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp5(&mut self) -> WKUPP5_W<12> {
        WKUPP5_W::new(self)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp6(&mut self) -> WKUPP6_W<13> {
        WKUPP6_W::new(self)
    }
    #[doc = "Bits 16:17 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd1(&mut self) -> WKUPPUPD1_W<16> {
        WKUPPUPD1_W::new(self)
    }
    #[doc = "Bits 18:19 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd2(&mut self) -> WKUPPUPD2_W<18> {
        WKUPPUPD2_W::new(self)
    }
    #[doc = "Bits 20:21 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd3(&mut self) -> WKUPPUPD3_W<20> {
        WKUPPUPD3_W::new(self)
    }
    #[doc = "Bits 22:23 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd4(&mut self) -> WKUPPUPD4_W<22> {
        WKUPPUPD4_W::new(self)
    }
    #[doc = "Bits 24:25 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd5(&mut self) -> WKUPPUPD5_W<24> {
        WKUPPUPD5_W::new(self)
    }
    #[doc = "Bits 26:27 - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wkuppupd6(&mut self) -> WKUPPUPD6_W<26> {
        WKUPPUPD6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset only by system reset, not reset by wakeup from Standby mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkupepr](index.html) module"]
pub struct WKUPEPR_SPEC;
impl crate::RegisterSpec for WKUPEPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkupepr::R](R) reader structure"]
impl crate::Readable for WKUPEPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkupepr::W](W) writer structure"]
impl crate::Writable for WKUPEPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WKUPEPR to value 0"]
impl crate::Resettable for WKUPEPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
