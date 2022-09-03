#[doc = "Register `AEEFR3` reader"]
pub struct R(crate::R<AEEFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AEEFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AEEFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AEEFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AEEFR3` writer"]
pub struct W(crate::W<AEEFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AEEFR3_SPEC>;
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
impl From<crate::W<AEEFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AEEFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEVACE` reader - External Event A Counter Enable"]
pub type EEVACE_R = crate::BitReader<bool>;
#[doc = "Field `EEVACE` writer - External Event A Counter Enable"]
pub type EEVACE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AEEFR3_SPEC, bool, O>;
#[doc = "Field `EEVACRES` reader - External Event A Counter Reset"]
pub type EEVACRES_R = crate::BitReader<bool>;
#[doc = "Field `EEVACRES` writer - External Event A Counter Reset"]
pub type EEVACRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, AEEFR3_SPEC, bool, O>;
#[doc = "Field `EEVARSTM` reader - External Event A Reset Mode"]
pub type EEVARSTM_R = crate::BitReader<bool>;
#[doc = "Field `EEVARSTM` writer - External Event A Reset Mode"]
pub type EEVARSTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, AEEFR3_SPEC, bool, O>;
#[doc = "Field `EEVASEL` reader - External Event A Selection"]
pub type EEVASEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EEVASEL` writer - External Event A Selection"]
pub type EEVASEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AEEFR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `EEVACNT` reader - External Event A counter"]
pub type EEVACNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EEVACNT` writer - External Event A counter"]
pub type EEVACNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AEEFR3_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - External Event A Counter Enable"]
    #[inline(always)]
    pub fn eevace(&self) -> EEVACE_R {
        EEVACE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Event A Counter Reset"]
    #[inline(always)]
    pub fn eevacres(&self) -> EEVACRES_R {
        EEVACRES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Event A Reset Mode"]
    #[inline(always)]
    pub fn eevarstm(&self) -> EEVARSTM_R {
        EEVARSTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - External Event A Selection"]
    #[inline(always)]
    pub fn eevasel(&self) -> EEVASEL_R {
        EEVASEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - External Event A counter"]
    #[inline(always)]
    pub fn eevacnt(&self) -> EEVACNT_R {
        EEVACNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Event A Counter Enable"]
    #[inline(always)]
    pub fn eevace(&mut self) -> EEVACE_W<0> {
        EEVACE_W::new(self)
    }
    #[doc = "Bit 1 - External Event A Counter Reset"]
    #[inline(always)]
    pub fn eevacres(&mut self) -> EEVACRES_W<1> {
        EEVACRES_W::new(self)
    }
    #[doc = "Bit 2 - External Event A Reset Mode"]
    #[inline(always)]
    pub fn eevarstm(&mut self) -> EEVARSTM_W<2> {
        EEVARSTM_W::new(self)
    }
    #[doc = "Bits 4:7 - External Event A Selection"]
    #[inline(always)]
    pub fn eevasel(&mut self) -> EEVASEL_W<4> {
        EEVASEL_W::new(self)
    }
    #[doc = "Bits 8:13 - External Event A counter"]
    #[inline(always)]
    pub fn eevacnt(&mut self) -> EEVACNT_W<8> {
        EEVACNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Timerx External Event Filtering Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeefr3](index.html) module"]
pub struct AEEFR3_SPEC;
impl crate::RegisterSpec for AEEFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aeefr3::R](R) reader structure"]
impl crate::Readable for AEEFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aeefr3::W](W) writer structure"]
impl crate::Writable for AEEFR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AEEFR3 to value 0"]
impl crate::Resettable for AEEFR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
