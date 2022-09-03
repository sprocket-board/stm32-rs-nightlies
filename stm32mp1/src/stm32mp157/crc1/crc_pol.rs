#[doc = "Register `CRC_POL` reader"]
pub struct R(crate::R<CRC_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_POL` writer"]
pub struct W(crate::W<CRC_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_POL_SPEC>;
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
impl From<crate::W<CRC_POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL` reader - POL"]
pub type POL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `POL` writer - POL"]
pub type POL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_POL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - POL"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<0> {
        POL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC polynomial\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_pol](index.html) module"]
pub struct CRC_POL_SPEC;
impl crate::RegisterSpec for CRC_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_pol::R](R) reader structure"]
impl crate::Readable for CRC_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_pol::W](W) writer structure"]
impl crate::Writable for CRC_POL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_POL to value 0x04c1_1db7"]
impl crate::Resettable for CRC_POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04c1_1db7
    }
}