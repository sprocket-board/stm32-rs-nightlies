#[doc = "Register `SPI_CRCPOLY` reader"]
pub struct R(crate::R<SPI_CRCPOLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CRCPOLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CRCPOLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CRCPOLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CRCPOLY` writer"]
pub struct W(crate::W<SPI_CRCPOLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CRCPOLY_SPEC>;
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
impl From<crate::W<SPI_CRCPOLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CRCPOLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCPOLY` reader - CRCPOLY"]
pub type CRCPOLY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRCPOLY` writer - CRCPOLY"]
pub type CRCPOLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_CRCPOLY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CRCPOLY"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRCPOLY"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<0> {
        CRCPOLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_crcpoly](index.html) module"]
pub struct SPI_CRCPOLY_SPEC;
impl crate::RegisterSpec for SPI_CRCPOLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_crcpoly::R](R) reader structure"]
impl crate::Readable for SPI_CRCPOLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_crcpoly::W](W) writer structure"]
impl crate::Writable for SPI_CRCPOLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CRCPOLY to value 0x0107"]
impl crate::Resettable for SPI_CRCPOLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0107
    }
}
