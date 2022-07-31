module Ans::Names {
    use std::string;
    use aptos_std::simple_map;
    use std::error;
    use aptos_std::event;
    use std::signer;

    struct NameMaps has key{
        names : simple_map::SimpleMap<Name, address>,
    }

    struct Name has drop, store{
        value: string::String,
    }

    struct NameChangeEvent has drop, store {
        name: string::String,
        addr: address,
    }

    /// There is no message present
    const ENO_MESSAGE: u64 = 0;

    public fun get_address(name: string::String): string::String acquires MessageHolder {
        NameMaps.message.find(name)
        assert!(exists<Name>(name), error::not_found(ENO_MESSAGE));
        *&borrow_global<NameMaps>(addr).message
    }

    public entry fun set_message(account: signer, message_bytes: vector<u8>)
    acquires MessageHolder {
        let message = string::utf8(message_bytes);
        let account_addr = signer::address_of(&account);
        if (!exists<MessageHolder>(account_addr)) {
            move_to(&account, MessageHolder {
                message,
                message_change_events: event::new_event_handle<MessageChangeEvent>(&account),
            })
        } else {
            let old_message_holder = borrow_global_mut<MessageHolder>(account_addr);
            let from_message = *&old_message_holder.message;
            event::emit_event(&mut old_message_holder.message_change_events, MessageChangeEvent {
                from_message,
                to_message: copy message,
            });
            old_message_holder.message = message;
        }
    }

    #[test(account = @0x1)]
    public entry fun sender_can_set_message(account: signer) acquires MessageHolder {
        let addr = signer::address_of(&account);
        set_message(account,  b"Hello, Blockchain");

        assert!(
          get_message(addr) == string::utf8(b"Hello, Blockchain"),
          ENO_MESSAGE
        );
    }
}
