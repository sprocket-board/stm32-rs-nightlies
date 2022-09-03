#[doc = "Register `DDRCTRL_DBGCAM` reader"]
pub struct R(crate::R<DDRCTRL_DBGCAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DBGCAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DBGCAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DBGCAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DBG_HPR_Q_DEPTH` reader - DBG_HPR_Q_DEPTH"]
pub type DBG_HPR_Q_DEPTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBG_LPR_Q_DEPTH` reader - DBG_LPR_Q_DEPTH"]
pub type DBG_LPR_Q_DEPTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBG_W_Q_DEPTH` reader - DBG_W_Q_DEPTH"]
pub type DBG_W_Q_DEPTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBG_STALL` reader - DBG_STALL"]
pub type DBG_STALL_R = crate::BitReader<bool>;
#[doc = "Field `DBG_RD_Q_EMPTY` reader - DBG_RD_Q_EMPTY"]
pub type DBG_RD_Q_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `DBG_WR_Q_EMPTY` reader - DBG_WR_Q_EMPTY"]
pub type DBG_WR_Q_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RD_DATA_PIPELINE_EMPTY` reader - RD_DATA_PIPELINE_EMPTY"]
pub type RD_DATA_PIPELINE_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `WR_DATA_PIPELINE_EMPTY` reader - WR_DATA_PIPELINE_EMPTY"]
pub type WR_DATA_PIPELINE_EMPTY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:4 - DBG_HPR_Q_DEPTH"]
    #[inline(always)]
    pub fn dbg_hpr_q_depth(&self) -> DBG_HPR_Q_DEPTH_R {
        DBG_HPR_Q_DEPTH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DBG_LPR_Q_DEPTH"]
    #[inline(always)]
    pub fn dbg_lpr_q_depth(&self) -> DBG_LPR_Q_DEPTH_R {
        DBG_LPR_Q_DEPTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DBG_W_Q_DEPTH"]
    #[inline(always)]
    pub fn dbg_w_q_depth(&self) -> DBG_W_Q_DEPTH_R {
        DBG_W_Q_DEPTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - DBG_STALL"]
    #[inline(always)]
    pub fn dbg_stall(&self) -> DBG_STALL_R {
        DBG_STALL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DBG_RD_Q_EMPTY"]
    #[inline(always)]
    pub fn dbg_rd_q_empty(&self) -> DBG_RD_Q_EMPTY_R {
        DBG_RD_Q_EMPTY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DBG_WR_Q_EMPTY"]
    #[inline(always)]
    pub fn dbg_wr_q_empty(&self) -> DBG_WR_Q_EMPTY_R {
        DBG_WR_Q_EMPTY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - RD_DATA_PIPELINE_EMPTY"]
    #[inline(always)]
    pub fn rd_data_pipeline_empty(&self) -> RD_DATA_PIPELINE_EMPTY_R {
        RD_DATA_PIPELINE_EMPTY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - WR_DATA_PIPELINE_EMPTY"]
    #[inline(always)]
    pub fn wr_data_pipeline_empty(&self) -> WR_DATA_PIPELINE_EMPTY_R {
        WR_DATA_PIPELINE_EMPTY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "DDRCTRL CAM debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dbgcam](index.html) module"]
pub struct DDRCTRL_DBGCAM_SPEC;
impl crate::RegisterSpec for DDRCTRL_DBGCAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dbgcam::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DBGCAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_DBGCAM to value 0"]
impl crate::Resettable for DDRCTRL_DBGCAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
