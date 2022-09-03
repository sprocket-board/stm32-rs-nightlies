#[doc = "Register `QMEM2%s` reader"]
pub struct R(crate::R<QMEM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QMEM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QMEM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QMEM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QMEM2%s` writer"]
pub struct W(crate::W<QMEM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QMEM2_SPEC>;
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
impl From<crate::W<QMEM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QMEM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QMem_RAM` reader - QMem RAM"]
pub type QMEM_RAM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `QMem_RAM` writer - QMem RAM"]
pub type QMEM_RAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QMEM2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - QMem RAM"]
    #[inline(always)]
    pub fn qmem_ram(&self) -> QMEM_RAM_R {
        QMEM_RAM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - QMem RAM"]
    #[inline(always)]
    pub fn qmem_ram(&mut self) -> QMEM_RAM_W<0> {
        QMEM_RAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2](index.html) module"]
pub struct QMEM2_SPEC;
impl crate::RegisterSpec for QMEM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qmem2::R](R) reader structure"]
impl crate::Readable for QMEM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qmem2::W](W) writer structure"]
impl crate::Writable for QMEM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QMEM2%s to value 0"]
impl crate::Resettable for QMEM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
