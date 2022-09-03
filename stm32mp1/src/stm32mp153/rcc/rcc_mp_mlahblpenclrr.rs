#[doc = "Register `RCC_MP_MLAHBLPENCLRR` reader"]
pub struct R(crate::R<RCC_MP_MLAHBLPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_MLAHBLPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_MLAHBLPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_MLAHBLPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_MLAHBLPENCLRR` writer"]
pub struct W(crate::W<RCC_MP_MLAHBLPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_MLAHBLPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_MLAHBLPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_MLAHBLPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM1LPEN` reader - SRAM1LPEN"]
pub type SRAM1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1LPEN` writer - SRAM1LPEN"]
pub type SRAM1LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_MLAHBLPENCLRR_SPEC, bool, O>;
#[doc = "Field `SRAM2LPEN` reader - SRAM2LPEN"]
pub type SRAM2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2LPEN` writer - SRAM2LPEN"]
pub type SRAM2LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_MLAHBLPENCLRR_SPEC, bool, O>;
#[doc = "Field `SRAM34LPEN` reader - SRAM34LPEN"]
pub type SRAM34LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM34LPEN` writer - SRAM34LPEN"]
pub type SRAM34LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_MLAHBLPENCLRR_SPEC, bool, O>;
#[doc = "Field `RETRAMLPEN` reader - RETRAMLPEN"]
pub type RETRAMLPEN_R = crate::BitReader<bool>;
#[doc = "Field `RETRAMLPEN` writer - RETRAMLPEN"]
pub type RETRAMLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_MLAHBLPENCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SRAM1LPEN"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2LPEN"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM34LPEN"]
    #[inline(always)]
    pub fn sram34lpen(&self) -> SRAM34LPEN_R {
        SRAM34LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RETRAMLPEN"]
    #[inline(always)]
    pub fn retramlpen(&self) -> RETRAMLPEN_R {
        RETRAMLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM1LPEN"]
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<0> {
        SRAM1LPEN_W::new(self)
    }
    #[doc = "Bit 1 - SRAM2LPEN"]
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<1> {
        SRAM2LPEN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM34LPEN"]
    #[inline(always)]
    pub fn sram34lpen(&mut self) -> SRAM34LPEN_W<2> {
        SRAM34LPEN_W::new(self)
    }
    #[doc = "Bit 4 - RETRAMLPEN"]
    #[inline(always)]
    pub fn retramlpen(&mut self) -> RETRAMLPEN_W<4> {
        RETRAMLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_mlahblpenclrr](index.html) module"]
pub struct RCC_MP_MLAHBLPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_MLAHBLPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_mlahblpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_MLAHBLPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_mlahblpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_MLAHBLPENCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_MLAHBLPENCLRR to value 0x17"]
impl crate::Resettable for RCC_MP_MLAHBLPENCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x17
    }
}
