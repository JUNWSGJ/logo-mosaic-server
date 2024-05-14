use dashmap::DashMap;
use anyhow::Result;

use crate::{ImageDO, ImageRepo};

pub struct ImageMemoryRepo{
    images: DashMap<String, ImageDO>
}

impl ImageMemoryRepo{
    pub fn new() -> Self{
        Self{
            images: DashMap::new()
        }
    }
}

impl ImageRepo for ImageMemoryRepo {
    fn get_image(&self, id: &str) -> Option<ImageDO> {
        self.images.get(id).map(|item| item.value().clone())
    }

    fn list_images(&self) -> Vec<ImageDO> {
        self.images.iter().map(|item| {
            let image = item.value();
            image.clone()
        }).collect()
    }
    
    fn insert_image(&self, image: ImageDO) -> Result<(), crate::ApiError> {
        self.images.insert(image.id.clone(), image);
        Ok(())
    }
}

