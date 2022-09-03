#[doc = "Register `RCC_SPI2S23CKSELR` reader"]
pub struct R(crate::R<RCC_SPI2S23CKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_SPI2S23CKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_SPI2S23CKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_SPI2S23CKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_SPI2S23CKSELR` writer"]
pub struct W(crate::W<RCC_SPI2S23CKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_SPI2S23CKSELR_SPEC>;
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
impl From<crate::W<RCC_SPI2S23CKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_SPI2S23CKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI23SRC` reader - SPI23SRC"]
pub type SPI23SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI23SRC` writer - SPI23SRC"]
pub type SPI23SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_SPI2S23CKSELR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - SPI23SRC"]
    #[inline(always)]
    pub fn spi23src(&self) -> SPI23SRC_R {
        SPI23SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI23SRC"]
    #[inline(always)]
    pub fn spi23src(&mut self) -> SPI23SRC_W<0> {
        SPI23SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_spi2s23ckselr](index.html) module"]
pub struct RCC_SPI2S23CKSELR_SPEC;
impl crate::RegisterSpec for RCC_SPI2S23CKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_spi2s23ckselr::R](R) reader structure"]
impl crate::Readable for RCC_SPI2S23CKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_spi2s23ckselr::W](W) writer structure"]
impl crate::Writable for RCC_SPI2S23CKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_SPI2S23CKSELR to value 0"]
impl crate::Resettable for RCC_SPI2S23CKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}