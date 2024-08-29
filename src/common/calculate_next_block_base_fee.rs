use alloy::primitives::U256;
use rand::Rng;
// Calculates the next block base fee given the previous block's gas usage / limits
// Refer to: https://www.blocknative.com/blog/eip-1559-fees
pub fn calculate_next_block_base_fee(
    gas_used: U256,
    gas_limit: U256,
    base_fee_per_gas: U256,
) -> U256 {
    let gas_used = gas_used;

    let mut target_gas_used = gas_limit / U256::from(2u64);
    target_gas_used = if target_gas_used == U256::ZERO {
        U256::from(1)
    } else {
        target_gas_used
    };

    let new_base_fee = {
        if gas_used > target_gas_used {
            base_fee_per_gas
                + ((base_fee_per_gas * (gas_used - target_gas_used)) / target_gas_used)
                    / U256::from(8u64)
        } else {
            base_fee_per_gas
                - ((base_fee_per_gas * (target_gas_used - gas_used)) / target_gas_used)
                    / U256::from(8u64)
        }
    };

    let seed = U256::from(rand::thread_rng().gen_range(0..9));
    new_base_fee + seed
}
