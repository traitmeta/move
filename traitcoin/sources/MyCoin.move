module NamedAddr::MyCoin{
    // Only included in compilation for testing. Similar to #[cfg(testing)]
    // in Rust. Imports the `Signer` module from the MoveStdlib package.
    use std::signer;
    use NamedAddr::Coin;

    struct MyCoin has drop {}

    const ENOT_ODD: u64 = 0;

    public fun setup_and_mint(account: &signer, amount: u64) {
        Coin::publish_balance<MyCoin>(account);
        Coin::mint<MyCoin>(signer::address_of(account), amount,MyCoin {});
    }

    public fun transfer(from: &signer, to: address, amount: u64) {
        // amount must be odd.
        assert!(amount % 2 == 1, ENOT_ODD);
        Coin::transfer<MyCoin>(from, to, amount,MyCoin {});
    }
    
    /*
        Unit tests
    */
    #[test(from = @0x42, to = @0x10)]
    fun test_odd_success(from: signer, to: signer) {
        setup_and_mint(&from, 42);
        setup_and_mint(&to, 10);

        // transfer an odd number of coins so this should succeed.
        transfer(&from, @0x10, 7);

        assert!(Coin::balance_of<MyCoin>(@0x42) == 35, 0);
        assert!(Coin::balance_of<MyCoin>(@0x10) == 17, 0);
    }

    #[test(from = @0x42, to = @0x10)]
    #[expected_failure]
    fun test_not_odd_failure(from: signer, to: signer) {
        setup_and_mint(&from, 42);
        setup_and_mint(&to, 10);

        // transfer an even number of coins so this should fail.
        transfer(&from, @0x10, 8);
    }
}