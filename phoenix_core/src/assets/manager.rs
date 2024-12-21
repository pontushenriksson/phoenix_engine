use crate::debugger::debugger::*;

pub trait Asset {
  
}

#[derive(Debug)]
pub struct AssetHandle {
  // asset: &Asset or index in Vec<>,
}

pub struct AssetManager {
  assets: Vec<Box<dyn Asset>>,
}

impl AssetManager {
  
  /// Tries to get asset.
  /// 
  /// If asset is loaded it returns an AssetHandle.
  /// If it isn't, it tries to load the asset and returns an AssetHandle.
  pub fn get() -> Option<AssetHandle> {
    None
  }


  pub fn load() -> Option<AssetHandle> {
    None
  }

  /// Tries to unload asset.
  /// 
  /// Returns an error if the asset isn't valid (already unloaded or was never loaded).
  pub fn  unload(&self, asset: AssetHandle) -> Result<u32, PhoenixError> {
    Err(
      PhoenixError::UnloadAsset(format!("Failed to unload asset {:?}", asset))
    )
  }
}
