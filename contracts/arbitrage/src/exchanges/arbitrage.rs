use cosmwasm_std::{ Uint128, StdResult };

const COMMISSION_FEE:u128 = 3;
const COMMISSION_BASE:u128 = 1000;

fn calculate_a(a1: Uint128, b1: Uint128, b2: Uint128) -> StdResult<Uint128> {
        //Exchange commission
        let r: Uint128 = Uint128::from(COMMISSION_BASE).checked_sub(Uint128::from(COMMISSION_FEE))?;
        //a
        let a_numerator: Uint128 = a1.checked_mul(b2)?;
        let a_denominator: Uint128 = b1.checked_mul(r)?.checked_div(Uint128::from(COMMISSION_BASE))?.checked_add(b2)?;
        let a: Uint128 = a_numerator.checked_div(a_denominator)?;
        Ok(a)
}

fn calculate_a_(b1: Uint128, a2: Uint128, b2: Uint128) -> StdResult<Uint128> {
    //Exchange commission
    let r: Uint128 = Uint128::from(COMMISSION_BASE).checked_sub(Uint128::from(COMMISSION_FEE))?;
    //a'
    let _a_numerator: Uint128 = a2.checked_mul(b1)?.checked_mul(r)?.checked_div(Uint128::from(COMMISSION_BASE))?;
    let _a_denominator: Uint128 = b1.checked_mul(r)?.checked_div(Uint128::from(COMMISSION_BASE))?.checked_add(b2)?;
    let _a: Uint128 = _a_numerator.checked_div(_a_denominator)?;
    Ok(_a)
}

fn calculate_square_root(num :Uint128) -> StdResult<Uint128> {
    let mut x0: Uint128 = num.checked_div(Uint128::from(2u128))?;
    if x0 != Uint128::zero() {
        let mut x1: Uint128 = num.checked_div(x0)?.checked_add(x0)?.checked_div(Uint128::from(2u128))?;
        while x1 < x0 {
            x0 = x1;
            x1 = num.checked_div(x0)?.checked_add(x0)?.checked_div(Uint128::from(2u128))?;
        }
        Ok(x0)
    } else {
        Ok(num)
    }
}

pub fn calculate_optimal_starting_token_amount(a1: Uint128, b1: Uint128, a2: Uint128, b2: Uint128) -> StdResult<Uint128> {
    //Exchange commission
    let r: Uint128 = Uint128::from(COMMISSION_BASE).checked_sub(Uint128::from(COMMISSION_FEE))?;
    let a: Uint128 = calculate_a(a1, b1, b2)?;
    let a_: Uint128 = calculate_a_(b1, a2, b2)?;
    //Calculate the optimal starting amount
    let to_be_sqrt: Uint128 = a_.checked_mul(a)?.checked_mul(r)?.checked_div(Uint128::from(COMMISSION_BASE))?;
    let sqrt: Uint128 = calculate_square_root(to_be_sqrt)?.checked_sub(a)?;
    let optimal_starting_amount: Uint128 = sqrt.checked_div(r)?.checked_mul(Uint128::from(COMMISSION_BASE))?;
    Ok(optimal_starting_amount)
}

pub fn calculate_profit(starting_amount: Uint128, a1: Uint128, a2: Uint128, b1: Uint128, b2: Uint128) -> StdResult<Uint128> {
    let r: Uint128 = Uint128::from(COMMISSION_BASE).checked_sub(Uint128::from(COMMISSION_FEE))?;
    let a: Uint128 = calculate_a(a1, b1, b2)?;
    let a_: Uint128 = calculate_a_(b1, a2, b2)?;
    let finishing_amount_numerator: Uint128 = a_.checked_mul(starting_amount)?.checked_mul(r)?.checked_div(Uint128::from(COMMISSION_BASE))?;
    let finishing_amount_denominator: Uint128 = starting_amount.checked_mul(r)?.checked_div(Uint128::from(COMMISSION_BASE))?.checked_add(a)?;
    let finishing_amount = finishing_amount_numerator.checked_div(finishing_amount_denominator)?;
    let profit: Uint128 = finishing_amount.checked_sub(starting_amount)?;
    Ok(profit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_root_function() {
        assert_eq!(Uint128::from(2u128), calculate_square_root(Uint128::from(4u128)).unwrap());
        assert_eq!(Uint128::from(4u128), calculate_square_root(Uint128::from(16u128)).unwrap());
        assert_eq!(Uint128::from(16u128), calculate_square_root(Uint128::from(256u128)).unwrap());
    }
}
