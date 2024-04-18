use abi_stable::{
    std_types::{RBox, ROption, RString, RVec},
    StableAbi,
};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, StableAbi)]
pub enum RComponent {
    Container {
        child: ROption<RBox<RComponent>>,
    },
    Column {
        children: ROption<RVec<RComponent>>,
    },
    Row {
        children: ROption<RVec<RComponent>>,
    },
    Clickable {
        on_click: ROption<RString>,
        child: ROption<RBox<RComponent>>,
    },
}
