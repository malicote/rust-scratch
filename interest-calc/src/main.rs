use structopt::StructOpt;
use rusty_money::{money, Money};

#[derive(Debug)]
#[derive(StructOpt)]
struct CliArgs {
    principal: u64,
    annual_return: f64,
    annual_contribution: u64,
    max_years: usize,
}

static CURRENT_AGE: usize = 20;

fn calculate_return(principal: f64, annual_return: f64, annual_contribution: f64, max_years: usize) -> Vec<f64> {
    assert!(principal > 0.0);

    let mut returns = vec![principal];
    for year in 0..max_years {
        let yearly_amount = returns[year]*(1.0+annual_return/100.0) + annual_contribution;
        returns.push(yearly_amount);
    }

    returns
}

fn print_results(params: CliArgs, returns: Vec<f64>) {
    println!("====================================");
    println!("Principal:\t\t{}", money!(params.principal, "USD"));
    println!("Annual Return:\t\t{}%", params.annual_return);
    println!("Annual Contribution:\t{}", money!(params.annual_contribution, "USD"));
    println!("Number of years:\t{}", params.max_years);
    println!("");
    println!("Results:");

    for year in 0..returns.len() {
        let amount = returns[year] as u64;
        println!("Age: {}: {}", CURRENT_AGE + year, money!(amount, "USD"));
    }
}

fn main() {
    let args = CliArgs::from_args();

    let returns = calculate_return(args.principal as f64, args.annual_return,
                                   args.annual_contribution as f64, args.max_years);

    print_results(args, returns);
}
