
use warp::Filter;
mod handlers;

    //let res = ;
#[tokio::main]
async fn main() {
    // println!("value: {}", handlers::number_to_words((999999999999999999999999999999999333333).parse::<u128>()));
    let route = warp::path!("number" / String)
        .map(|name:String| format!("{}", handlers::number_to_words(name.parse::<u128>().unwrap())));

    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

    
