use crate::currency_getter::CurrencyGetter;

mod currency_getter;
#[tokio::main]
async fn main() {
    let t = CurrencyGetter::new();
    let date = time::Date::from_calendar_date(1999, time::Month::January, 4).unwrap();
    let tt = t.currencies_at_date("USD", date).await;
    println!("{:?}", tt.unwrap());
}
