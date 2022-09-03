#[doc = "Register `SAI_BSLOTR` reader"]
pub struct R(crate::R<SAI_BSLOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_BSLOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_BSLOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_BSLOTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_BSLOTR` writer"]
pub struct W(crate::W<SAI_BSLOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_BSLOTR_SPEC>;
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
impl From<crate::W<SAI_BSLOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_BSLOTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBOFF` reader - FBOFF"]
pub type FBOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FBOFF` writer - FBOFF"]
pub type FBOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_BSLOTR_SPEC, u8, u8, 5, O>;
#[doc = "Field `SLOTSZ` reader - SLOTSZ"]
pub type SLOTSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLOTSZ` writer - SLOTSZ"]
pub type SLOTSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_BSLOTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NBSLOT` reader - NBSLOT"]
pub type NBSLOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBSLOT` writer - NBSLOT"]
pub type NBSLOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_BSLOTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SLOTEN` reader - SLOTEN"]
pub type SLOTEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLOTEN` writer - SLOTEN"]
pub type SLOTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_BSLOTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:4 - FBOFF"]
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - SLOTSZ"]
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - NBSLOT"]
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - SLOTEN"]
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - FBOFF"]
    #[inline(always)]
    pub fn fboff(&mut self) -> FBOFF_W<0> {
        FBOFF_W::new(self)
    }
    #[doc = "Bits 6:7 - SLOTSZ"]
    #[inline(always)]
    pub fn slotsz(&mut self) -> SLOTSZ_W<6> {
        SLOTSZ_W::new(self)
    }
    #[doc = "Bits 8:11 - NBSLOT"]
    #[inline(always)]
    pub fn nbslot(&mut self) -> NBSLOT_W<8> {
        NBSLOT_W::new(self)
    }
    #[doc = "Bits 16:31 - SLOTEN"]
    #[inline(always)]
    pub fn sloten(&mut self) -> SLOTEN_W<16> {
        SLOTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register has no meaning in and SPDIF audio protocol\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bslotr](index.html) module"]
pub struct SAI_BSLOTR_SPEC;
impl crate::RegisterSpec for SAI_BSLOTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_bslotr::R](R) reader structure"]
impl crate::Readable for SAI_BSLOTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_bslotr::W](W) writer structure"]
impl crate::Writable for SAI_BSLOTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_BSLOTR to value 0"]
impl crate::Resettable for SAI_BSLOTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
