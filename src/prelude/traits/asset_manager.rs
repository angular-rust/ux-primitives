use bytes::Bytes;

/// Provides functions to interact with media assets embedded or loaded in the application.
/// 
/// Use with caution, there are usually more type safe ways to utilise assets.
///
pub trait AssetManager {
    /// Request an embedded or loaded media asset.  E.g. bitmap or sound.
    /// Return the asset - can be of any type for type inference (or cast as appropriate)
    /// 
    /// # Arguments
    /// 
    /// * `id` - The uniqie reference of the requested asset.  E.g. className.
    /// * `package_id` - The package of the requested asset.  Will default to "assets" if not provided. (optional)
    /// * `args` - Some assets may require additional arguments, provide them here.(optional)
    ///
    fn get_asset(&self, id: String, package_id: Option<String>, args: Vec<String>) -> Bytes;
}
