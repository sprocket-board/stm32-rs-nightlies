#[doc = "Register `APB3FZ1` reader"]
pub struct R(crate::R<APB3FZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3FZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3FZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3FZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3FZ1` writer"]
pub struct W(crate::W<APB3FZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3FZ1_SPEC>;
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
impl From<crate::W<APB3FZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3FZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WWDG` reader - WWDG stop in debug"]
pub type WWDG_R = crate::BitReader<bool>;
#[doc = "Field `WWDG` writer - WWDG stop in debug"]
pub type WWDG_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZ1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - WWDG stop in debug"]
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - WWDG stop in debug"]
    #[inline(always)]
    pub fn wwdg(&mut self) -> WWDG_W<6> {
        WWDG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB3 peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3fz1](index.html) module"]
pub struct APB3FZ1_SPEC;
impl crate::RegisterSpec for APB3FZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3fz1::R](R) reader structure"]
impl crate::Readable for APB3FZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3fz1::W](W) writer structure"]
impl crate::Writable for APB3FZ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB3FZ1 to value 0"]
impl crate::Resettable for APB3FZ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
