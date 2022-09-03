#[doc = "Register `PWR_WKUPCR` reader"]
pub struct R(crate::R<PWR_WKUPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_WKUPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_WKUPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_WKUPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_WKUPCR` writer"]
pub struct W(crate::W<PWR_WKUPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_WKUPCR_SPEC>;
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
impl From<crate::W<PWR_WKUPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_WKUPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPC1` reader - WKUPC1"]
pub type WKUPC1_R = crate::BitReader<bool>;
#[doc = "Field `WKUPC1` writer - WKUPC1"]
pub type WKUPC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPC2` reader - WKUPC2"]
pub type WKUPC2_R = crate::BitReader<bool>;
#[doc = "Field `WKUPC2` writer - WKUPC2"]
pub type WKUPC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPC3` reader - WKUPC3"]
pub type WKUPC3_R = crate::BitReader<bool>;
#[doc = "Field `WKUPC3` writer - WKUPC3"]
pub type WKUPC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPC4` reader - WKUPC4"]
pub type WKUPC4_R = crate::BitReader<bool>;
#[doc = "Field `WKUPC4` writer - WKUPC4"]
pub type WKUPC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPC5` reader - WKUPC5"]
pub type WKUPC5_R = crate::BitReader<bool>;
#[doc = "Field `WKUPC5` writer - WKUPC5"]
pub type WKUPC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPC6` reader - WKUPC6"]
pub type WKUPC6_R = crate::BitReader<bool>;
#[doc = "Field `WKUPC6` writer - WKUPC6"]
pub type WKUPC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPP1` reader - WKUPP1"]
pub type WKUPP1_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP1` writer - WKUPP1"]
pub type WKUPP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPP2` reader - WKUPP2"]
pub type WKUPP2_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP2` writer - WKUPP2"]
pub type WKUPP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPP3` reader - WKUPP3"]
pub type WKUPP3_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP3` writer - WKUPP3"]
pub type WKUPP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPP4` reader - WKUPP4"]
pub type WKUPP4_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP4` writer - WKUPP4"]
pub type WKUPP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPP5` reader - WKUPP5"]
pub type WKUPP5_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP5` writer - WKUPP5"]
pub type WKUPP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPP6` reader - WKUPP6"]
pub type WKUPP6_R = crate::BitReader<bool>;
#[doc = "Field `WKUPP6` writer - WKUPP6"]
pub type WKUPP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WKUPCR_SPEC, bool, O>;
#[doc = "Field `WKUPPUPD1` reader - WKUPPUPD1"]
pub type WKUPPUPD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD1` writer - WKUPPUPD1"]
pub type WKUPPUPD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WKUPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WKUPPUPD2` reader - WKUPPUPD2"]
pub type WKUPPUPD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD2` writer - WKUPPUPD2"]
pub type WKUPPUPD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WKUPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WKUPPUPD3` reader - WKUPPUPD3"]
pub type WKUPPUPD3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD3` writer - WKUPPUPD3"]
pub type WKUPPUPD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WKUPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WKUPPUPD4` reader - WKUPPUPD4"]
pub type WKUPPUPD4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD4` writer - WKUPPUPD4"]
pub type WKUPPUPD4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WKUPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WKUPPUPD5` reader - WKUPPUPD5"]
pub type WKUPPUPD5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD5` writer - WKUPPUPD5"]
pub type WKUPPUPD5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WKUPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WKUPPUPD6` reader - WKUPPUPD6"]
pub type WKUPPUPD6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPPUPD6` writer - WKUPPUPD6"]
pub type WKUPPUPD6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WKUPCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - WKUPC1"]
    #[inline(always)]
    pub fn wkupc1(&self) -> WKUPC1_R {
        WKUPC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WKUPC2"]
    #[inline(always)]
    pub fn wkupc2(&self) -> WKUPC2_R {
        WKUPC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WKUPC3"]
    #[inline(always)]
    pub fn wkupc3(&self) -> WKUPC3_R {
        WKUPC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUPC4"]
    #[inline(always)]
    pub fn wkupc4(&self) -> WKUPC4_R {
        WKUPC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WKUPC5"]
    #[inline(always)]
    pub fn wkupc5(&self) -> WKUPC5_R {
        WKUPC5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WKUPC6"]
    #[inline(always)]
    pub fn wkupc6(&self) -> WKUPC6_R {
        WKUPC6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - WKUPP1"]
    #[inline(always)]
    pub fn wkupp1(&self) -> WKUPP1_R {
        WKUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WKUPP2"]
    #[inline(always)]
    pub fn wkupp2(&self) -> WKUPP2_R {
        WKUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WKUPP3"]
    #[inline(always)]
    pub fn wkupp3(&self) -> WKUPP3_R {
        WKUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WKUPP4"]
    #[inline(always)]
    pub fn wkupp4(&self) -> WKUPP4_R {
        WKUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WKUPP5"]
    #[inline(always)]
    pub fn wkupp5(&self) -> WKUPP5_R {
        WKUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WKUPP6"]
    #[inline(always)]
    pub fn wkupp6(&self) -> WKUPP6_R {
        WKUPP6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - WKUPPUPD1"]
    #[inline(always)]
    pub fn wkuppupd1(&self) -> WKUPPUPD1_R {
        WKUPPUPD1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - WKUPPUPD2"]
    #[inline(always)]
    pub fn wkuppupd2(&self) -> WKUPPUPD2_R {
        WKUPPUPD2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - WKUPPUPD3"]
    #[inline(always)]
    pub fn wkuppupd3(&self) -> WKUPPUPD3_R {
        WKUPPUPD3_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - WKUPPUPD4"]
    #[inline(always)]
    pub fn wkuppupd4(&self) -> WKUPPUPD4_R {
        WKUPPUPD4_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - WKUPPUPD5"]
    #[inline(always)]
    pub fn wkuppupd5(&self) -> WKUPPUPD5_R {
        WKUPPUPD5_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - WKUPPUPD6"]
    #[inline(always)]
    pub fn wkuppupd6(&self) -> WKUPPUPD6_R {
        WKUPPUPD6_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - WKUPC1"]
    #[inline(always)]
    pub fn wkupc1(&mut self) -> WKUPC1_W<0> {
        WKUPC1_W::new(self)
    }
    #[doc = "Bit 1 - WKUPC2"]
    #[inline(always)]
    pub fn wkupc2(&mut self) -> WKUPC2_W<1> {
        WKUPC2_W::new(self)
    }
    #[doc = "Bit 2 - WKUPC3"]
    #[inline(always)]
    pub fn wkupc3(&mut self) -> WKUPC3_W<2> {
        WKUPC3_W::new(self)
    }
    #[doc = "Bit 3 - WKUPC4"]
    #[inline(always)]
    pub fn wkupc4(&mut self) -> WKUPC4_W<3> {
        WKUPC4_W::new(self)
    }
    #[doc = "Bit 4 - WKUPC5"]
    #[inline(always)]
    pub fn wkupc5(&mut self) -> WKUPC5_W<4> {
        WKUPC5_W::new(self)
    }
    #[doc = "Bit 5 - WKUPC6"]
    #[inline(always)]
    pub fn wkupc6(&mut self) -> WKUPC6_W<5> {
        WKUPC6_W::new(self)
    }
    #[doc = "Bit 8 - WKUPP1"]
    #[inline(always)]
    pub fn wkupp1(&mut self) -> WKUPP1_W<8> {
        WKUPP1_W::new(self)
    }
    #[doc = "Bit 9 - WKUPP2"]
    #[inline(always)]
    pub fn wkupp2(&mut self) -> WKUPP2_W<9> {
        WKUPP2_W::new(self)
    }
    #[doc = "Bit 10 - WKUPP3"]
    #[inline(always)]
    pub fn wkupp3(&mut self) -> WKUPP3_W<10> {
        WKUPP3_W::new(self)
    }
    #[doc = "Bit 11 - WKUPP4"]
    #[inline(always)]
    pub fn wkupp4(&mut self) -> WKUPP4_W<11> {
        WKUPP4_W::new(self)
    }
    #[doc = "Bit 12 - WKUPP5"]
    #[inline(always)]
    pub fn wkupp5(&mut self) -> WKUPP5_W<12> {
        WKUPP5_W::new(self)
    }
    #[doc = "Bit 13 - WKUPP6"]
    #[inline(always)]
    pub fn wkupp6(&mut self) -> WKUPP6_W<13> {
        WKUPP6_W::new(self)
    }
    #[doc = "Bits 16:17 - WKUPPUPD1"]
    #[inline(always)]
    pub fn wkuppupd1(&mut self) -> WKUPPUPD1_W<16> {
        WKUPPUPD1_W::new(self)
    }
    #[doc = "Bits 18:19 - WKUPPUPD2"]
    #[inline(always)]
    pub fn wkuppupd2(&mut self) -> WKUPPUPD2_W<18> {
        WKUPPUPD2_W::new(self)
    }
    #[doc = "Bits 20:21 - WKUPPUPD3"]
    #[inline(always)]
    pub fn wkuppupd3(&mut self) -> WKUPPUPD3_W<20> {
        WKUPPUPD3_W::new(self)
    }
    #[doc = "Bits 22:23 - WKUPPUPD4"]
    #[inline(always)]
    pub fn wkuppupd4(&mut self) -> WKUPPUPD4_W<22> {
        WKUPPUPD4_W::new(self)
    }
    #[doc = "Bits 24:25 - WKUPPUPD5"]
    #[inline(always)]
    pub fn wkuppupd5(&mut self) -> WKUPPUPD5_W<24> {
        WKUPPUPD5_W::new(self)
    }
    #[doc = "Bits 26:27 - WKUPPUPD6"]
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
#[doc = "Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\\[6:1\\], WKUPP\\[6:1\\]
bits and WKUPPUPD\\[6:1\\]
bit pairs are discarded when the corresponding WKUPEN\\[6:1\\]
bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_wkupcr](index.html) module"]
pub struct PWR_WKUPCR_SPEC;
impl crate::RegisterSpec for PWR_WKUPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_wkupcr::R](R) reader structure"]
impl crate::Readable for PWR_WKUPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_wkupcr::W](W) writer structure"]
impl crate::Writable for PWR_WKUPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_WKUPCR to value 0"]
impl crate::Resettable for PWR_WKUPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
