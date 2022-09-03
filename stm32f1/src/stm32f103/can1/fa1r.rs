#[doc = "Register `FA1R` reader"]
pub struct R(crate::R<FA1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FA1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FA1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FA1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FA1R` writer"]
pub struct W(crate::W<FA1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FA1R_SPEC>;
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
impl From<crate::W<FA1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FA1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FACT0` reader - Filter active"]
pub type FACT0_R = crate::BitReader<bool>;
#[doc = "Field `FACT0` writer - Filter active"]
pub type FACT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT1` reader - Filter active"]
pub type FACT1_R = crate::BitReader<bool>;
#[doc = "Field `FACT1` writer - Filter active"]
pub type FACT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT2` reader - Filter active"]
pub type FACT2_R = crate::BitReader<bool>;
#[doc = "Field `FACT2` writer - Filter active"]
pub type FACT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT3` reader - Filter active"]
pub type FACT3_R = crate::BitReader<bool>;
#[doc = "Field `FACT3` writer - Filter active"]
pub type FACT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT4` reader - Filter active"]
pub type FACT4_R = crate::BitReader<bool>;
#[doc = "Field `FACT4` writer - Filter active"]
pub type FACT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT5` reader - Filter active"]
pub type FACT5_R = crate::BitReader<bool>;
#[doc = "Field `FACT5` writer - Filter active"]
pub type FACT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT6` reader - Filter active"]
pub type FACT6_R = crate::BitReader<bool>;
#[doc = "Field `FACT6` writer - Filter active"]
pub type FACT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT7` reader - Filter active"]
pub type FACT7_R = crate::BitReader<bool>;
#[doc = "Field `FACT7` writer - Filter active"]
pub type FACT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT8` reader - Filter active"]
pub type FACT8_R = crate::BitReader<bool>;
#[doc = "Field `FACT8` writer - Filter active"]
pub type FACT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT9` reader - Filter active"]
pub type FACT9_R = crate::BitReader<bool>;
#[doc = "Field `FACT9` writer - Filter active"]
pub type FACT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT10` reader - Filter active"]
pub type FACT10_R = crate::BitReader<bool>;
#[doc = "Field `FACT10` writer - Filter active"]
pub type FACT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT11` reader - Filter active"]
pub type FACT11_R = crate::BitReader<bool>;
#[doc = "Field `FACT11` writer - Filter active"]
pub type FACT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT12` reader - Filter active"]
pub type FACT12_R = crate::BitReader<bool>;
#[doc = "Field `FACT12` writer - Filter active"]
pub type FACT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
#[doc = "Field `FACT13` reader - Filter active"]
pub type FACT13_R = crate::BitReader<bool>;
#[doc = "Field `FACT13` writer - Filter active"]
pub type FACT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FA1R_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    pub fn fact0(&self) -> FACT0_R {
        FACT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    pub fn fact1(&self) -> FACT1_R {
        FACT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    pub fn fact2(&self) -> FACT2_R {
        FACT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    pub fn fact3(&self) -> FACT3_R {
        FACT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    pub fn fact4(&self) -> FACT4_R {
        FACT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    pub fn fact5(&self) -> FACT5_R {
        FACT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    pub fn fact6(&self) -> FACT6_R {
        FACT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    pub fn fact7(&self) -> FACT7_R {
        FACT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    pub fn fact8(&self) -> FACT8_R {
        FACT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    pub fn fact9(&self) -> FACT9_R {
        FACT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    pub fn fact10(&self) -> FACT10_R {
        FACT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    pub fn fact11(&self) -> FACT11_R {
        FACT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    pub fn fact12(&self) -> FACT12_R {
        FACT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    pub fn fact13(&self) -> FACT13_R {
        FACT13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    pub fn fact0(&mut self) -> FACT0_W<0> {
        FACT0_W::new(self)
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    pub fn fact1(&mut self) -> FACT1_W<1> {
        FACT1_W::new(self)
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    pub fn fact2(&mut self) -> FACT2_W<2> {
        FACT2_W::new(self)
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    pub fn fact3(&mut self) -> FACT3_W<3> {
        FACT3_W::new(self)
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    pub fn fact4(&mut self) -> FACT4_W<4> {
        FACT4_W::new(self)
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    pub fn fact5(&mut self) -> FACT5_W<5> {
        FACT5_W::new(self)
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    pub fn fact6(&mut self) -> FACT6_W<6> {
        FACT6_W::new(self)
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    pub fn fact7(&mut self) -> FACT7_W<7> {
        FACT7_W::new(self)
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    pub fn fact8(&mut self) -> FACT8_W<8> {
        FACT8_W::new(self)
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    pub fn fact9(&mut self) -> FACT9_W<9> {
        FACT9_W::new(self)
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    pub fn fact10(&mut self) -> FACT10_W<10> {
        FACT10_W::new(self)
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    pub fn fact11(&mut self) -> FACT11_W<11> {
        FACT11_W::new(self)
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    pub fn fact12(&mut self) -> FACT12_W<12> {
        FACT12_W::new(self)
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    pub fn fact13(&mut self) -> FACT13_W<13> {
        FACT13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_FA1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fa1r](index.html) module"]
pub struct FA1R_SPEC;
impl crate::RegisterSpec for FA1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fa1r::R](R) reader structure"]
impl crate::Readable for FA1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fa1r::W](W) writer structure"]
impl crate::Writable for FA1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FA1R to value 0"]
impl crate::Resettable for FA1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
