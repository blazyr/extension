use abi_stable::{
    std_types::{RBox, ROption, RString, RVec},
    StableAbi,
};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, StableAbi)]
pub enum RComponent {
    Container {
        child: ROption<RBox<RComponent>>,
        on_click: ROption<RString>,
    },
    Column {
        children: ROption<RVec<RComponent>>,
    },
    Row {
        children: ROption<RVec<RComponent>>,
    },
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, StableAbi)]
pub enum RComponentClickable {
    Container,
    Column,
    Row,
}
