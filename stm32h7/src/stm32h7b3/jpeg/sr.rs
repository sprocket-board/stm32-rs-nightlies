#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IFTF` reader - Input FIFO Threshold Flag This bit is set when the input FIFO is not full and is bellow its threshold."]
pub type IFTF_R = crate::BitReader<bool>;
#[doc = "Field `IFNFF` reader - Input FIFO Not Full Flag This bit is set when the input FIFO is not full (a data can be written)."]
pub type IFNFF_R = crate::BitReader<bool>;
#[doc = "Field `OFTF` reader - Output FIFO Threshold Flag This bit is set when the output FIFO is not empty and has reach its threshold."]
pub type OFTF_R = crate::BitReader<bool>;
#[doc = "Field `OFNEF` reader - Output FIFO Not Empty Flag This bit is set when the output FIFO is not empty (a data is available)."]
pub type OFNEF_R = crate::BitReader<bool>;
#[doc = "Field `EOCF` reader - End of Conversion Flag This bit is set when the JPEG codec core has finished the encoding or the decoding process and than last data has been sent to the output FIFO."]
pub type EOCF_R = crate::BitReader<bool>;
#[doc = "Field `HPDF` reader - Header Parsing Done Flag This bit is set in decode mode when the JPEG codec has finished the parsing of the headers and the internal registers have been updated."]
pub type HPDF_R = crate::BitReader<bool>;
#[doc = "Field `COF` reader - Codec Operation Flag This bit is set when when a JPEG codec operation is on going (encoding or decoding)."]
pub type COF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Input FIFO Threshold Flag This bit is set when the input FIFO is not full and is bellow its threshold."]
    #[inline(always)]
    pub fn iftf(&self) -> IFTF_R {
        IFTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Flag This bit is set when the input FIFO is not full (a data can be written)."]
    #[inline(always)]
    pub fn ifnff(&self) -> IFNFF_R {
        IFNFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Flag This bit is set when the output FIFO is not empty and has reach its threshold."]
    #[inline(always)]
    pub fn oftf(&self) -> OFTF_R {
        OFTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Flag This bit is set when the output FIFO is not empty (a data is available)."]
    #[inline(always)]
    pub fn ofnef(&self) -> OFNEF_R {
        OFNEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Flag This bit is set when the JPEG codec core has finished the encoding or the decoding process and than last data has been sent to the output FIFO."]
    #[inline(always)]
    pub fn eocf(&self) -> EOCF_R {
        EOCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Header Parsing Done Flag This bit is set in decode mode when the JPEG codec has finished the parsing of the headers and the internal registers have been updated."]
    #[inline(always)]
    pub fn hpdf(&self) -> HPDF_R {
        HPDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Codec Operation Flag This bit is set when when a JPEG codec operation is on going (encoding or decoding)."]
    #[inline(always)]
    pub fn cof(&self) -> COF_R {
        COF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "JPEG status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x06"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
