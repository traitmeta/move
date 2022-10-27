module red_package::package{
    use sui::object::{Self, UID};
    use sui::tx_context::{Self,TxContext};
    use sui::transfer;
    use sui::coin::{Self, Coin};
    use std::vector;


    const INSUFFICIENT_AVAILABLE_BALANCE: u64 = 3;
    const INSUFFICIENT_AMOUNT: u64 = 4;
    const REDPACKAGE_NOT_EXISTS: u64 = 5;
    const REDPACKAGE_IS_EMPTY: u64 = 6;
    const ALREADY_CLAIMED: u64 = 7;

    struct RedPackage<phantom T> has key, store {
        id: UID,
        tokens: Coin<T>,
        per_amount :u64,
        claimed: vector<address>,
    }

    public fun create_fair<TokenType: store>(pay_token: Coin<TokenType>, per_amount: u64, count: u64, ctx: &mut TxContext) {
        let amount = per_amount * count;
        assert!(coin::value(&pay_token) > amount, INSUFFICIENT_AMOUNT);

        let redPackage = RedPackage<TokenType> {
            id: object::new(ctx),
            tokens: pay_token,
            per_amount,
            claimed: vector::empty()
        };

        transfer::share_object(redPackage);
    }

    public fun claim<TokenType: store>(red_package_info: &mut RedPackage<TokenType>, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        let rest = coin::value(&red_package_info.tokens);
        assert!(rest > red_package_info.per_amount, REDPACKAGE_IS_EMPTY);

        let per_amount = red_package_info.per_amount;
        vector::push_back(&mut red_package_info.claimed, sender);
        
        let cliam_token = coin::split(&mut red_package_info.tokens, per_amount, ctx);
        transfer::transfer(cliam_token,sender);
    }


    fun sum(leafs: &vector<u64>): u64 {
        let i = 0;
        let sum = 0u64;
        let ll = vector::length(leafs);
        while (i < ll) {
            let x = *vector::borrow<u64>(leafs, i);
            sum = x + sum ;
            i = i + 1;
        };
        sum
    }

}