use anyhow::Result;
use rusty_money::{iso, iso::Currency, Money};

pub fn money_minor(money: Money<Currency>, amount: i64) -> Result<Money<Currency>> {
    let output: Money<Currency> = Money::from_minor(amount, money.currency());

    Ok(output)
}

pub fn money_from_str<'a>() -> Result<Money<'a, Currency>> {
    let usd: Money<Currency> = Money::from_str("-2000.009", iso::USD)?;
    println!("{}", usd);
    Ok(usd)
}

/*
root@18a218f78b76:/# ./balance
-$2,000.01
-€2.000.009,00
let eur = Money::from_str("-2000.009", iso::EUR)?;
println!("{}", eur);
*/

fn money_from_minor() -> Result<()> {
    let amount = 10_000;
    let hundred = Money::from_minor(amount, iso::USD);
    let thousand = Money::from_minor(100_000, iso::USD);

    println!("{}", thousand > hundred); // false
    println!("{}", thousand.is_positive()); // true

    Ok(())
}
