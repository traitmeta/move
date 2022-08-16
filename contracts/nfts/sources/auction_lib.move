// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// This is a helper module for implementing two versions of an
/// English auction (https://en.wikipedia.org/wiki/English_auction),
/// one using single-owner objects only and the other using shared
/// objects.
module nfts::auction_lib {
    use std::option::{Self, Option};
    use std::signer;
    use aptos_token::token::{Self, Token};
    use aptos_std::coin::{Self, Coin};
    use aptos_std::guid::{Self,GUID};

    friend nfts::auction;
    friend nfts::shared_auction;

    /// Stores information about an auction bid.
    struct BidData<phantom CoinType> has store {
        /// Coin representing the current (highest) bid.
        funds: Coin<CoinType>,
        /// Address of the highest bidder.
        highest_bidder: address,
    }

    /// Maintains the state of the auction owned by a trusted
    /// auctioneer.
    struct Auction<phantom C> has key {
        id: GUID,
        /// Item to be sold. It only really needs to be wrapped in
        /// Option if Auction represents a shared object but we do it
        /// for single-owner Auctions for better code re-use.
        to_sell: Option<Token>,
        /// Owner of the time to be sold.
        owner: address,
        /// Data representing the highest bid (starts with no bid)
        bid_data: Option<BidData<C>>,
    }

    public(friend) fun auction_owner<T: key + store, C>(auction: &Auction<C>): address {
        auction.owner
    }

    /// Creates an auction. This is executed by the owner of the asset to be
    /// auctioned.
    public(friend) fun create_auction<C>(account: &signer, id: GUID, to_sell: Token): Auction<C> {
        // A question one might asked is how do we know that to_sell
        // is owned by the caller of this entry function and the
        // answer is that it's checked by the runtime.
        Auction<C> {
            id,
            to_sell: option::some(to_sell),
            owner: signer::address_of(account),
            bid_data: option::none(),
        }
    }

    /// Updates the auction based on the information in the bid
    /// (update auction if higher bid received and send coin back for
    /// bids that are too low).
    public fun update_auction<T: key + store,C>(
        auction: &mut Auction<C>,
        bidder: address,
        funds: Coin<C>,
    ) {
        if (option::is_none(&auction.bid_data)) {
            // first bid
            let bid_data = BidData {
                funds,
                highest_bidder: bidder,
            };
            option::fill(&mut auction.bid_data, bid_data);
        } else {
            let prev_bid_data = option::borrow(&auction.bid_data);
            if (coin::value(&funds) > coin::value(&prev_bid_data.funds)) {
                // a bid higher than currently highest bid received
                let new_bid_data = BidData {
                    funds,
                    highest_bidder: bidder
                };

                // update auction to reflect highest bid
                let BidData {
                    funds,
                    highest_bidder
                } = option::swap(&mut auction.bid_data, new_bid_data);

                // transfer previously highest bid to its bidder
                send_balance(funds, highest_bidder);
            } else {
                // a bid is too low - return funds to the bidder
                send_balance(funds, bidder);
            }
        }
    }

    /// Ends the auction - transfers item to the currently highest
    /// bidder or to the original owner if no bids have been placed.
    fun end_auction<T: key + store,C>(
        owner: &signer,
        to_sell: &Token,
        bid_data: &mut Option<BidData<C>>,
    ) {
        if (option::is_some<BidData<C>>(bid_data)) {
            // bids have been placed - send funds to the original item
            // owner and the item to the highest bidder
            let BidData {
                funds,
                highest_bidder
            } = option::extract(bid_data);
            let owner_account = signer::address_of(owner);
            send_balance(funds, owner_account);
            
            let token_id  = token::token_id(to_sell);
            let amount  = token::balance_of(owner_account,*token_id);
            token::transfer(owner, token_id, highest_bidder, amount);
        } else {
            // no bids placed - send the item back to the original owner
            transfer::transfer(item, owner);
        };
    }

    /// Ends auction and destroys auction object (can only be used if
    /// Auction is single-owner object) - transfers item to the
    /// currently highest bidder or to the original owner if no bids
    /// have been placed.
    public fun end_and_destroy_auction<T: key + store,C>(
        auction: Auction<C>, 
    ) {
        let Auction { id, to_sell, owner, bid_data } = auction;
        // object::delete(id);
        end_auction(&mut to_sell, owner, &mut bid_data);

        option::destroy_none(bid_data);
        option::destroy_none(to_sell);
    }

    /// Ends auction (should only be used if Auction is a shared
    /// object) - transfers item to the currently highest bidder or to
    /// the original owner if no bids have been placed.
    public fun end_shared_auction<T: key + store,C>(
        auction: &mut Auction<C>,
    ) {
        end_auction(&mut auction.to_sell, auction.owner, &mut auction.bid_data);
    }

    /// Helper for the most common operation - wrapping a balance and sending it
    fun send_balance<C>(balance: Coin<C>, to: address) {
        coin::deposit(to, balance);
    }

    /// exposes transfer::transfer
    public entry fun init_auction<C>(obj: Auction<C>, recipient: &signer) {
        move_to(recipient,obj);
    }
}
