use abi_stable::{
    declare_root_module_statics,
    library::RootModule,
    package_version_strings,
    sabi_types::VersionStrings,
    std_types::{RBoxError, ROption, RResult, RStr, RString, RVec},
    StableAbi,
};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix))]
pub struct Plugin {
    pub on_mount: extern "C" fn() -> RResult<(), RBoxError>,
    pub entities: extern "C" fn() -> RResult<RVec<REntity>, RBoxError>,
    pub on_entity_action: extern "C" fn(u64, ROption<RStr>) -> RResult<(), RBoxError>,
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
    pub icon_path: ROption<RString>,
    pub icon_data: ROption<RVec<u8>>,
}

impl REntity {
    pub fn builder(id: u64, name: &str) -> REntityBuilder {
        REntityBuilder::new(id, name)
    }
}

pub struct REntityBuilder<'a> {
    id: u64,
    name: &'a str,
    description: Option<&'a str>,
    alias: Option<&'a str>,
    pub icon_path: Option<String>,
    pub icon_data: Option<Vec<u8>>,
}

impl<'a> REntityBuilder<'a> {
    pub fn new(id: u64, name: &'a str) -> Self {
        REntityBuilder {
            id,
            name,
            description: None,
            alias: None,
            icon_data: None,
            icon_path: None,
        }
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn alias(mut self, alias: &'a str) -> Self {
        self.alias = Some(alias);
        self
    }

    pub fn icon_path(mut self, icon_path: String) -> Self {
        self.icon_path = Some(icon_path);
        self
    }

    pub fn icon_data(mut self, icon_data: Vec<u8>) -> Self {
        self.icon_data = Some(icon_data);
        self
    }

    pub fn build(self) -> REntity {
        REntity {
            id: self.id,
            name: RString::from(self.name),
            description: self.description.map(RString::from).into(),
            alias: self.alias.map(RString::from).into(),
            icon_path: self.icon_path.map(|v| v.into()).into(),
            icon_data: self.icon_data.map(|v| v.into()).into(),
        }
    }
}
