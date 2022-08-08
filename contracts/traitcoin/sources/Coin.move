module NamedAddr::Coin{
    // Only included in compilation for testing. Similar to #[cfg(testing)]
    // in Rust. Imports the `Signer` module from the MoveStdlib package.
    use std::signer;

    /// Address of the owner of this module
    const MODULE_OWNER: address = @NamedAddr;

    /// Error codes
    const ENOT_MODULE_OWNER: u64 = 0;
    const EINSUFFICIENT_BALANCE: u64 = 1;
    const EALREADY_HAS_BALANCE: u64 = 2;

    struct Coin<phantom CoinType> has store{
        value: u64,
    }

    struct Balance<phantom CoinType> has key {
        coin: Coin<CoinType>, 
    }

    /// Publish an empty balance resource under `account`'s address. This function must be called before
    /// minting or transferring to the account.
    public fun publish_balance<CoinType>(account: &signer) { 
        let empty_coin = Coin<CoinType>{value:0};
        assert!(!exists<Balance<CoinType>>(signer::address_of(account)),EALREADY_HAS_BALANCE);
        move_to(account, Balance<CoinType>{coin:empty_coin});
     }

    /// Initialize this module.
    // public fun mint<CoinType>(module_owner: &signer, mint_addr: address, amount: u64) acquires Balance{
    //     // Only the owner of the module can initialize this module
    //     assert!(signer::address_of(module_owner) == MODULE_OWNER, ENOT_MODULE_OWNER);
    //     // Deposit `amount` of tokens to `mint_addr`'s balance
    //     deposit<CoinType>(mint_addr, Coin { value: amount });
    // }
    /// Mint `amount` tokens to `mint_addr`. This method requires a witness with `CoinType` so that the
    /// module that owns `CoinType` can decide the minting policy.
    public fun mint<CoinType: drop>(mint_addr: address, amount: u64, _witness: CoinType) acquires Balance {
        // Deposit `total_value` amount of tokens to mint_addr's balance
        deposit(mint_addr, Coin<CoinType> { value: amount });
    }

    /// Returns the balance of `owner`.
    public fun balance_of<CoinType>(owner: address): u64 acquires Balance { 
        borrow_global<Balance<CoinType>>(owner).coin.value
     }

    spec balance_of {
        pragma aborts_if_is_strict;
        aborts_if !exists<Balance<CoinType>>(owner);
    }

    /// Transfers `amount` of tokens from `from` to `to`. This method requires a witness with `CoinType` so that the
    /// module that owns `CoinType` can decide the transferring policy.
    public fun transfer<CoinType: drop>(from: &signer, to: address, amount: u64, _witness: CoinType) acquires Balance {
        let check = withdraw<CoinType>(signer::address_of(from), amount);
        deposit<CoinType>(to, check);
    }

      /// Transfers `amount` of tokens from `from` to `to`.
    public fun withdraw<CoinType>(addr: address, amount: u64): Coin<CoinType> acquires Balance { 
        let balance  = balance_of<CoinType>(addr);
        assert!(balance>=amount,EINSUFFICIENT_BALANCE);
        let balance_ref = &mut borrow_global_mut<Balance<CoinType>>(addr).coin.value;
        *balance_ref = balance - amount;
        Coin{value: amount}
    }

    /// Deposit `amount` number of tokens to the balance under `addr`.
    fun deposit<CoinType>(addr: address, check: Coin<CoinType>) acquires Balance{
        let balance = balance_of<CoinType>(addr);
        let balance_ref = &mut borrow_global_mut<Balance<CoinType>>(addr).coin.value;
        let Coin { value } = check; // unpacks the check
        *balance_ref = balance + value;
    }
}