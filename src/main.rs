fn main() {
    println!("APIS");
    let body = get_products();
    println!("{}", body);
}

pub fn get_products() -> String {
    let url = format!("https://api.360beautyscents.co.ke/api/products");
    reqwest::blocking::get(url)
        .expect("Request failed")
        .text()
        .expect("body failed")
}
