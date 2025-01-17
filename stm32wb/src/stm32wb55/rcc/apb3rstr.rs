#[doc = "Register `APB3RSTR` reader"]
pub struct R(crate::R<APB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3RSTR` writer"]
pub struct W(crate::W<APB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3RSTR_SPEC>;
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
impl From<crate::W<APB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFRST` reader - Radio system BLE reset"]
pub type RFRST_R = crate::BitReader<bool>;
#[doc = "Field `RFRST` writer - Radio system BLE reset"]
pub type RFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Radio system BLE reset"]
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Radio system BLE reset"]
    #[inline(always)]
    pub fn rfrst(&mut self) -> RFRST_W<0> {
        RFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB3 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3rstr](index.html) module"]
pub struct APB3RSTR_SPEC;
impl crate::RegisterSpec for APB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3rstr::R](R) reader structure"]
impl crate::Readable for APB3RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3rstr::W](W) writer structure"]
impl crate::Writable for APB3RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB3RSTR to value 0"]
impl crate::Resettable for APB3RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
