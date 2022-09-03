#[doc = "Register `CRC_INIT` reader"]
pub struct R(crate::R<CRC_INIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_INIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_INIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_INIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_INIT` writer"]
pub struct W(crate::W<CRC_INIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_INIT_SPEC>;
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
impl From<crate::W<CRC_INIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_INIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_INIT` reader - CRC_INIT"]
pub type CRC_INIT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRC_INIT` writer - CRC_INIT"]
pub type CRC_INIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_INIT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CRC_INIT"]
    #[inline(always)]
    pub fn crc_init(&self) -> CRC_INIT_R {
        CRC_INIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC_INIT"]
    #[inline(always)]
    pub fn crc_init(&mut self) -> CRC_INIT_W<0> {
        CRC_INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC initial value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_init](index.html) module"]
pub struct CRC_INIT_SPEC;
impl crate::RegisterSpec for CRC_INIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_init::R](R) reader structure"]
impl crate::Readable for CRC_INIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_init::W](W) writer structure"]
impl crate::Writable for CRC_INIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_INIT to value 0xffff_ffff"]
impl crate::Resettable for CRC_INIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
