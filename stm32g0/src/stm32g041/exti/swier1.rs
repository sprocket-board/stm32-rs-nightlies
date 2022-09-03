#[doc = "Register `SWIER1` reader"]
pub struct R(crate::R<SWIER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER1` writer"]
pub struct W(crate::W<SWIER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER1_SPEC>;
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
impl From<crate::W<SWIER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWIER0` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type SWIER0_R = crate::BitReader<SWIER0W_A>;
#[doc = "Rising trigger event configuration bit of Configurable Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIER0W_A {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWIER0W_A> for bool {
    #[inline(always)]
    fn from(variant: SWIER0W_A) -> Self {
        variant as u8 != 0
    }
}
impl SWIER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIER0W_A> {
        match self.bits {
            true => Some(SWIER0W_A::Pend),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pend`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER0W_A::Pend
    }
}
#[doc = "Field `SWIER0` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type SWIER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER1_SPEC, SWIER0W_A, O>;
impl<'a, const O: u8> SWIER0_W<'a, O> {
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER0W_A::Pend)
    }
}
#[doc = "Field `SWIER1` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER1_R;
#[doc = "Field `SWIER2` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER2_R;
#[doc = "Field `SWIER3` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER3_R;
#[doc = "Field `SWIER4` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER4_R;
#[doc = "Field `SWIER5` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER5_R;
#[doc = "Field `SWIER6` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER6_R;
#[doc = "Field `SWIER7` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER7_R;
#[doc = "Field `SWIER8` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER8_R;
#[doc = "Field `SWIER9` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER9_R;
#[doc = "Field `SWIER10` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER10_R;
#[doc = "Field `SWIER11` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER11_R;
#[doc = "Field `SWIER12` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER12_R;
#[doc = "Field `SWIER13` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER13_R;
#[doc = "Field `SWIER14` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER14_R;
#[doc = "Field `SWIER15` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER15_R;
#[doc = "Field `SWIER16` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_R as SWIER16_R;
#[doc = "Field `SWIER1` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER1_W;
#[doc = "Field `SWIER2` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER2_W;
#[doc = "Field `SWIER3` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER3_W;
#[doc = "Field `SWIER4` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER4_W;
#[doc = "Field `SWIER5` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER5_W;
#[doc = "Field `SWIER6` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER6_W;
#[doc = "Field `SWIER7` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER7_W;
#[doc = "Field `SWIER8` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER8_W;
#[doc = "Field `SWIER9` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER9_W;
#[doc = "Field `SWIER10` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER10_W;
#[doc = "Field `SWIER11` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER11_W;
#[doc = "Field `SWIER12` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER12_W;
#[doc = "Field `SWIER13` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER13_W;
#[doc = "Field `SWIER14` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER14_W;
#[doc = "Field `SWIER15` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER15_W;
#[doc = "Field `SWIER16` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use SWIER0_W as SWIER16_W;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier0(&self) -> SWIER0_R {
        SWIER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier1(&self) -> SWIER1_R {
        SWIER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier2(&self) -> SWIER2_R {
        SWIER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier3(&self) -> SWIER3_R {
        SWIER3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier4(&self) -> SWIER4_R {
        SWIER4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier5(&self) -> SWIER5_R {
        SWIER5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier6(&self) -> SWIER6_R {
        SWIER6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier7(&self) -> SWIER7_R {
        SWIER7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier8(&self) -> SWIER8_R {
        SWIER8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier9(&self) -> SWIER9_R {
        SWIER9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier10(&self) -> SWIER10_R {
        SWIER10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier11(&self) -> SWIER11_R {
        SWIER11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier12(&self) -> SWIER12_R {
        SWIER12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier13(&self) -> SWIER13_R {
        SWIER13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier14(&self) -> SWIER14_R {
        SWIER14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier15(&self) -> SWIER15_R {
        SWIER15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier16(&self) -> SWIER16_R {
        SWIER16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier0(&mut self) -> SWIER0_W<0> {
        SWIER0_W::new(self)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier1(&mut self) -> SWIER1_W<1> {
        SWIER1_W::new(self)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier2(&mut self) -> SWIER2_W<2> {
        SWIER2_W::new(self)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier3(&mut self) -> SWIER3_W<3> {
        SWIER3_W::new(self)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier4(&mut self) -> SWIER4_W<4> {
        SWIER4_W::new(self)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier5(&mut self) -> SWIER5_W<5> {
        SWIER5_W::new(self)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier6(&mut self) -> SWIER6_W<6> {
        SWIER6_W::new(self)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier7(&mut self) -> SWIER7_W<7> {
        SWIER7_W::new(self)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier8(&mut self) -> SWIER8_W<8> {
        SWIER8_W::new(self)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier9(&mut self) -> SWIER9_W<9> {
        SWIER9_W::new(self)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier10(&mut self) -> SWIER10_W<10> {
        SWIER10_W::new(self)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier11(&mut self) -> SWIER11_W<11> {
        SWIER11_W::new(self)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier12(&mut self) -> SWIER12_W<12> {
        SWIER12_W::new(self)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier13(&mut self) -> SWIER13_W<13> {
        SWIER13_W::new(self)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier14(&mut self) -> SWIER14_W<14> {
        SWIER14_W::new(self)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier15(&mut self) -> SWIER15_W<15> {
        SWIER15_W::new(self)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swier16(&mut self) -> SWIER16_W<16> {
        SWIER16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier1](index.html) module"]
pub struct SWIER1_SPEC;
impl crate::RegisterSpec for SWIER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier1::R](R) reader structure"]
impl crate::Readable for SWIER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier1::W](W) writer structure"]
impl crate::Writable for SWIER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIER1 to value 0"]
impl crate::Resettable for SWIER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
