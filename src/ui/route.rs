#![allow(non_snake_case)]

use crate::ui::page::{
    dummy1::Dummy1, dummy2::Dummy2, home::HomePage, tychentropy::final_layout::TychentropyPage,
};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/tychentropy")]
    TychentropyPage {},
    #[route("/dummy1")]
    Dummy1 {},
    #[route("/dummy2")]
    Dummy2 {},
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    Tychentropy,
    Dummy1,
    Dummy2,
}
