use super::*;

pub trait ColorTransition: Clone + Copy + From<Rgb> + Into<Rgb> {}
pub trait ColorSpace: ColorTransition {}
pub trait NonRgbSpace: ColorTransition {}
pub trait NonRadialSpace: ColorTransition {}
pub trait NonSaturationSpace: ColorTransition {}

impl ColorTransition for Rgb {}
impl ColorTransition for RgbColor {}
impl ColorTransition for RgbaColor {}
impl ColorTransition for HslColor {}
impl ColorTransition for HsvColor {}
impl ColorTransition for CmykColor {}
impl ColorTransition for CmyColor {}
#[cfg(feature = "experimental")]
impl ColorTransition for LabColor {}
#[cfg(feature = "experimental")]
impl ColorTransition for XyzColor {}

impl ColorSpace for RgbColor {}
impl ColorSpace for RgbaColor {}
impl ColorSpace for HslColor {}
impl ColorSpace for HsvColor {}
impl ColorSpace for CmykColor {}
impl ColorSpace for CmyColor {}
#[cfg(feature = "experimental")]
impl ColorSpace for LabColor {}
#[cfg(feature = "experimental")]
impl ColorSpace for XyzColor {}

impl NonRgbSpace for HslColor {}
impl NonRgbSpace for HsvColor {}
impl NonRgbSpace for CmykColor {}
impl NonRgbSpace for CmyColor {}
#[cfg(feature = "experimental")]
impl NonRgbSpace for LabColor {}
#[cfg(feature = "experimental")]
impl NonRgbSpace for XyzColor {}

impl NonRadialSpace for Rgb {}
impl NonRadialSpace for RgbColor {}
impl NonRadialSpace for RgbaColor {}
impl NonRadialSpace for CmykColor {}
impl NonRadialSpace for CmyColor {}
#[cfg(feature = "experimental")]
impl NonRadialSpace for LabColor {}
#[cfg(feature = "experimental")]
impl NonRadialSpace for XyzColor {}

impl NonSaturationSpace for Rgb {}
impl NonSaturationSpace for RgbColor {}
impl NonSaturationSpace for RgbaColor {}
impl NonSaturationSpace for CmykColor {}
impl NonSaturationSpace for CmyColor {}
#[cfg(feature = "experimental")]
impl NonSaturationSpace for LabColor {}
#[cfg(feature = "experimental")]
impl NonSaturationSpace for XyzColor {}
