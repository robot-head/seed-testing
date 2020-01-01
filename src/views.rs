mod all_customers;
mod customer;
mod home;
mod loading;
use crate::model::{Model, Msg, Page};
use seed::{prelude::*, *};

pub fn index_view(model: &Model) -> impl View<Msg> {
  div![
    h1!["GraphQL Rust Example"],
    match model.page {
      Page::Customers => all_customers::all_customers_view(&model),
      Page::Customer => customer::customer_view(&model),
      Page::Loading => loading::loading_view(&model),
      _ => home::home_view(&model),
    },
  ]
}
