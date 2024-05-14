mod activity_repo;
mod image_repo;

use anyhow::Result;
use crate::{ApiError, GridShape, Point};

pub use activity_repo::*;
pub use image_repo::*;

#[derive(Debug, Clone)]
pub struct ActivityDO{
    pub id: String,
    pub name: String,
    pub grids: Vec<ActivityGridDO>,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub canvas_color: String,
}


#[derive(Debug, Clone)]
pub struct ActivityGridDO{
    pub seq: String,
    pub points: Vec<Point>,
    pub shape: GridShape,
    pub marked: bool,
    pub unmarked_color: String,
    pub marked_color: String,
}

#[derive(Debug, Clone)]
pub struct ActivityInfoResult{
    pub id: String,
    pub name: String,
}


pub trait ActivityRepo{
    fn get_activity(&self, id: &str) -> Option<ActivityDO>;
    fn list_activities(&self) -> Vec<ActivityInfoResult>;

    fn insert_activity(&self, activity: ActivityDO) -> Result<(), ApiError>;
    fn mark_grid_of_activity(&self, activity_id: &str, seq: &str) -> Result<()>;
    fn reset_activity(&self, activity_id: &str) -> Result<(), ApiError>;
}


#[derive(Debug, Clone)]
pub struct ImageDO{
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub path: String,
    pub bg_color: (u8, u8, u8),
}


pub trait ImageRepo{
    fn get_image(&self, id: &str) -> Option<ImageDO>;
    fn list_images(&self) -> Vec<ImageDO>;
    fn insert_image(&self, image: ImageDO) -> Result<(), ApiError>;
}


