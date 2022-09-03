#[doc = "Register `HUFFENC_DC0%s` reader"]
pub struct R(crate::R<HUFFENC_DC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFENC_DC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFENC_DC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFENC_DC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HUFFENC_DC0%s` writer"]
pub struct W(crate::W<HUFFENC_DC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HUFFENC_DC0_SPEC>;
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
impl From<crate::W<HUFFENC_DC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HUFFENC_DC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DHTMem_RAM` reader - DHTMem RAM"]
pub type DHTMEM_RAM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DHTMem_RAM` writer - DHTMem RAM"]
pub type DHTMEM_RAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HUFFENC_DC0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DHTMem RAM"]
    #[inline(always)]
    pub fn dhtmem_ram(&self) -> DHTMEM_RAM_R {
        DHTMEM_RAM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DHTMem RAM"]
    #[inline(always)]
    pub fn dhtmem_ram(&mut self) -> DHTMEM_RAM_W<0> {
        DHTMEM_RAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG encoder, DC Huffman table %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc0](index.html) module"]
pub struct HUFFENC_DC0_SPEC;
impl crate::RegisterSpec for HUFFENC_DC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffenc_dc0::R](R) reader structure"]
impl crate::Readable for HUFFENC_DC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [huffenc_dc0::W](W) writer structure"]
impl crate::Writable for HUFFENC_DC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HUFFENC_DC0%s to value 0"]
impl crate::Resettable for HUFFENC_DC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
