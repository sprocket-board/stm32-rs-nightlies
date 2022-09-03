#[doc = "Register `RCC_BR_RSTSCLRR` reader"]
pub struct R(crate::R<RCC_BR_RSTSCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_BR_RSTSCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_BR_RSTSCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_BR_RSTSCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_BR_RSTSCLRR` writer"]
pub struct W(crate::W<RCC_BR_RSTSCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_BR_RSTSCLRR_SPEC>;
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
impl From<crate::W<RCC_BR_RSTSCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_BR_RSTSCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORRSTF` reader - PORRSTF"]
pub type PORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PORRSTF` writer - PORRSTF"]
pub type PORRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
#[doc = "Field `BORRSTF` reader - BORRSTF"]
pub type BORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `BORRSTF` writer - BORRSTF"]
pub type BORRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
#[doc = "Field `PADRSTF` reader - PADRSTF"]
pub type PADRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PADRSTF` writer - PADRSTF"]
pub type PADRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
#[doc = "Field `HCSSRSTF` reader - HCSSRSTF"]
pub type HCSSRSTF_R = crate::BitReader<bool>;
#[doc = "Field `HCSSRSTF` writer - HCSSRSTF"]
pub type HCSSRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
#[doc = "Field `VCORERSTF` reader - VCORERSTF"]
pub type VCORERSTF_R = crate::BitReader<bool>;
#[doc = "Field `VCORERSTF` writer - VCORERSTF"]
pub type VCORERSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
#[doc = "Field `MPSYSRSTF` reader - MPSYSRSTF"]
pub type MPSYSRSTF_R = crate::BitReader<bool>;
#[doc = "Field `MPSYSRSTF` writer - MPSYSRSTF"]
pub type MPSYSRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
#[doc = "Field `MCSYSRSTF` reader - MCSYSRSTF"]
pub type MCSYSRSTF_R = crate::BitReader<bool>;
#[doc = "Field `MCSYSRSTF` writer - MCSYSRSTF"]
pub type MCSYSRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
#[doc = "Field `IWDG1RSTF` reader - IWDG1RSTF"]
pub type IWDG1RSTF_R = crate::BitReader<bool>;
#[doc = "Field `IWDG1RSTF` writer - IWDG1RSTF"]
pub type IWDG1RSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
#[doc = "Field `IWDG2RSTF` reader - IWDG2RSTF"]
pub type IWDG2RSTF_R = crate::BitReader<bool>;
#[doc = "Field `IWDG2RSTF` writer - IWDG2RSTF"]
pub type IWDG2RSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
#[doc = "Field `MPUP0RSTF` reader - MPUP0RSTF"]
pub type MPUP0RSTF_R = crate::BitReader<bool>;
#[doc = "Field `MPUP0RSTF` writer - MPUP0RSTF"]
pub type MPUP0RSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
#[doc = "Field `MPUP1RSTF` reader - MPUP1RSTF"]
pub type MPUP1RSTF_R = crate::BitReader<bool>;
#[doc = "Field `MPUP1RSTF` writer - MPUP1RSTF"]
pub type MPUP1RSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BR_RSTSCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PORRSTF"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BORRSTF"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PADRSTF"]
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HCSSRSTF"]
    #[inline(always)]
    pub fn hcssrstf(&self) -> HCSSRSTF_R {
        HCSSRSTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VCORERSTF"]
    #[inline(always)]
    pub fn vcorerstf(&self) -> VCORERSTF_R {
        VCORERSTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - MPSYSRSTF"]
    #[inline(always)]
    pub fn mpsysrstf(&self) -> MPSYSRSTF_R {
        MPSYSRSTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MCSYSRSTF"]
    #[inline(always)]
    pub fn mcsysrstf(&self) -> MCSYSRSTF_R {
        MCSYSRSTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IWDG1RSTF"]
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IWDG2RSTF"]
    #[inline(always)]
    pub fn iwdg2rstf(&self) -> IWDG2RSTF_R {
        IWDG2RSTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - MPUP0RSTF"]
    #[inline(always)]
    pub fn mpup0rstf(&self) -> MPUP0RSTF_R {
        MPUP0RSTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MPUP1RSTF"]
    #[inline(always)]
    pub fn mpup1rstf(&self) -> MPUP1RSTF_R {
        MPUP1RSTF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORRSTF"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W<0> {
        PORRSTF_W::new(self)
    }
    #[doc = "Bit 1 - BORRSTF"]
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W<1> {
        BORRSTF_W::new(self)
    }
    #[doc = "Bit 2 - PADRSTF"]
    #[inline(always)]
    pub fn padrstf(&mut self) -> PADRSTF_W<2> {
        PADRSTF_W::new(self)
    }
    #[doc = "Bit 3 - HCSSRSTF"]
    #[inline(always)]
    pub fn hcssrstf(&mut self) -> HCSSRSTF_W<3> {
        HCSSRSTF_W::new(self)
    }
    #[doc = "Bit 4 - VCORERSTF"]
    #[inline(always)]
    pub fn vcorerstf(&mut self) -> VCORERSTF_W<4> {
        VCORERSTF_W::new(self)
    }
    #[doc = "Bit 6 - MPSYSRSTF"]
    #[inline(always)]
    pub fn mpsysrstf(&mut self) -> MPSYSRSTF_W<6> {
        MPSYSRSTF_W::new(self)
    }
    #[doc = "Bit 7 - MCSYSRSTF"]
    #[inline(always)]
    pub fn mcsysrstf(&mut self) -> MCSYSRSTF_W<7> {
        MCSYSRSTF_W::new(self)
    }
    #[doc = "Bit 8 - IWDG1RSTF"]
    #[inline(always)]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W<8> {
        IWDG1RSTF_W::new(self)
    }
    #[doc = "Bit 9 - IWDG2RSTF"]
    #[inline(always)]
    pub fn iwdg2rstf(&mut self) -> IWDG2RSTF_W<9> {
        IWDG2RSTF_W::new(self)
    }
    #[doc = "Bit 13 - MPUP0RSTF"]
    #[inline(always)]
    pub fn mpup0rstf(&mut self) -> MPUP0RSTF_W<13> {
        MPUP0RSTF_W::new(self)
    }
    #[doc = "Bit 14 - MPUP1RSTF"]
    #[inline(always)]
    pub fn mpup1rstf(&mut self) -> MPUP1RSTF_W<14> {
        MPUP1RSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\\[1:0\\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\\[1:0\\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_br_rstsclrr](index.html) module"]
pub struct RCC_BR_RSTSCLRR_SPEC;
impl crate::RegisterSpec for RCC_BR_RSTSCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_br_rstsclrr::R](R) reader structure"]
impl crate::Readable for RCC_BR_RSTSCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_br_rstsclrr::W](W) writer structure"]
impl crate::Writable for RCC_BR_RSTSCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_BR_RSTSCLRR to value 0x15"]
impl crate::Resettable for RCC_BR_RSTSCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15
    }
}
