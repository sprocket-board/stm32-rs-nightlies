#[doc = "Register `CH2CFGR2` reader"]
pub struct R(crate::R<CH2CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CFGR2` writer"]
pub struct W(crate::W<CH2CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CFGR2_SPEC>;
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
impl From<crate::W<CH2CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTRBS` reader - DTRBS"]
pub type DTRBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTRBS` writer - DTRBS"]
pub type DTRBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2CFGR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `OFFSET` reader - OFFSET"]
pub type OFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFSET` writer - OFFSET"]
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2CFGR2_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    pub fn dtrbs(&self) -> DTRBS_R {
        DTRBS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    pub fn dtrbs(&mut self) -> DTRBS_W<3> {
        DTRBS_W::new(self)
    }
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<8> {
        OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH2CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cfgr2](index.html) module"]
pub struct CH2CFGR2_SPEC;
impl crate::RegisterSpec for CH2CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2cfgr2::R](R) reader structure"]
impl crate::Readable for CH2CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2cfgr2::W](W) writer structure"]
impl crate::Writable for CH2CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2CFGR2 to value 0"]
impl crate::Resettable for CH2CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
