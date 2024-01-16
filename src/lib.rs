mod extension;
pub use extension::Entity;

use abi_stable::{
    declare_root_module_statics,
    library::RootModule,
    package_version_strings,
    sabi_types::VersionStrings,
    std_types::{RBoxError, ROption, RResult, RString, RVec},
    StableAbi,
};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix))]
pub struct Plugin {
    pub on_mount: extern "C" fn() -> RResult<(), RBoxError>,
    pub entities: extern "C" fn() -> RResult<RVec<REntity>, RBoxError>,
    pub on_entity_action: extern "C" fn(u64) -> RResult<(), RBoxError>,
    #[sabi(last_prefix_field)]
    pub on_dispose: extern "C" fn() -> RResult<(), RBoxError>,
}

impl RootModule for Plugin_Ref {
    // The name of the dynamic library
    const BASE_NAME: &'static str = "plugin";
    // The name of the library for logging and similars
    const NAME: &'static str = "plugin";
    // The version of this plugin's crate
    const VERSION_STRINGS: VersionStrings = package_version_strings!();

    declare_root_module_statics! {Plugin_Ref}
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, StableAbi)]
pub struct REntity {
    pub id: u64,
    pub name: RString,
    pub description: ROption<RString>,
    pub alias: ROption<RString>,
}
