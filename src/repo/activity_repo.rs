use dashmap::DashMap;
use anyhow::Result;
use crate::{ActivityDO, ActivityInfoResult, ActivityRepo, ApiError};

pub struct ActivityMemoryRepo{
    activities: DashMap<String, ActivityDO>
}

impl ActivityMemoryRepo{
    pub fn new() -> Self{
        Self{
            activities: DashMap::new()
        }
    }
}

impl ActivityRepo for ActivityMemoryRepo {
    fn insert_activity(&self, activity: ActivityDO) -> Result<(), ApiError> {
        self.activities.insert(activity.id.clone(), activity);
        Ok(())
    }

    fn get_activity(&self, id: &str) -> Option<ActivityDO> {
        self.activities.get(id).map(|item| item.value().clone())
    }

    fn mark_grid_of_activity(&self, activity_id: &str, seq: &str) -> Result<()> {
        if let Some(mut activity) = self.activities.get_mut(activity_id) {
            if let Some(grid_index) = activity.value().grids.iter().position(|grid| grid.seq == seq) {
                // 更新找到的网格的marked属性
                activity.value_mut().grids[grid_index].marked = true;
                
                // 由于使用了DashMap，此处无需显式保存，更新已自动反映在内存中
                return Ok(());
            } else {
                // 如果没有找到匹配的网格序列号，可以考虑返回一个错误或日志记录
                return Err(anyhow::anyhow!("Grid with seq {} not found in activity {}", seq, activity_id));
            }
        } else {
            // 活动ID未找到时的处理
            return Err(anyhow::anyhow!("Activity with id {} not found", activity_id));
        }
    }
    
    fn reset_activity(&self, activity_id: &str) -> Result<(), ApiError> {
        if let Some(mut activity) = self.activities.get_mut(activity_id) {
            activity.value_mut().grids.iter_mut().for_each(|grid| {
                grid.marked = false;
            });
            Ok(())
        } else {
            // 活动ID未找到时的处理
            return Err(ApiError::BizError("ACTIVITY_NOT_FOUND".to_string(), format!("Activity with id {} not found", activity_id)));
        }
    }
    
    fn list_activities(&self) -> Vec<ActivityInfoResult> {
        self.activities.iter().map(|item| {
            let activity = item.value();
            ActivityInfoResult{
                id: activity.id.clone(),
                name: activity.name.clone(),
            }
        }).collect()    
    }

}
