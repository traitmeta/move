module Msg::Message {
    use std::string;
    use std::error;
    use std::signer;

    const ENO_MESSAGE: u64 = 0;

    struct MessageHolder has key {
        message: string::String,
    }

    public fun get_message(addr: address): string::String acquires MessageHolder {
        assert!(exists<MessageHolder>(addr), error::not_found(ENO_MESSAGE));
        *&borrow_global<MessageHolder>(addr).message
    }

   
    public entry fun set_message(account: signer, message_bytes: vector<u8>)
    acquires MessageHolder {
        let message = string::utf8(message_bytes);
        let account_addr = signer::address_of(&account);
        if (!exists<MessageHolder>(account_addr)) {
            move_to(&account, MessageHolder {
                message,
            })
        } else {
            let old_message_holder = borrow_global_mut<MessageHolder>(account_addr);
            old_message_holder.message = message;
        }
    }

    #[test(account = @0x1)]
    public entry fun sender_can_set_message(account: signer) acquires MessageHolder {
        let addr = signer::address_of(&account);
        set_message(account,  b"Hello, Blockchain");

        assert!(
          get_message(addr) == string::utf8(b"Hello, Blockchain"),
          0
        );
    }
}