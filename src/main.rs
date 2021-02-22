
// use warp::Filter;

// #[tokio::main]
// async fn main() {
    //     // println!("value: {}", handlers::number_to_words((999999999999999999999999999999999333333).parse::<u128>()));
//     let route = warp::path!("number" / String)
//         .map(|name:String| format!("{}", handlers::number_to_words(name.parse::<u128>().unwrap())));

//     warp::serve(route)
//         .run(([127, 0, 0, 1], 3030))
//         .await;
// }

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod handlers;

#[post("/", data = "<input>")]
fn index(input: String) -> String{ 
    let response = handlers::number_to_words(input.parse::<u128>().unwrap());
    return response;
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
