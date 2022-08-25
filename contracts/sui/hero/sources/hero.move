module hero::game {
    use sui::object::{Self, UID};
    use sui::tx_context::{Self,TxContext};
    use sui::transfer;

    struct Sword has key, store {
        id: UID,
        magic: u64,
        strength: u64,
    }

    struct Forge has key, store{
        id: UID,
        swords_created: u64,
    }

    public fun magic(self: &Sword): u64 {
        self.magic
    }

    public fun strength(self: &Sword): u64 {
        self.strength
    }

    public fun swords_created(self: &Forge): u64 {
        self.swords_created
    }

    // module initializer to be executed when this module is published
    fun init(ctx: &mut TxContext) {
        let admin = Forge {
            id: object::new(ctx),
            swords_created: 0,
        };
        // transfer the forge object to the module/package publisher
        // (presumably the game admin)
        transfer::transfer(admin, tx_context::sender(ctx));
    }

    public entry fun sword_create(forge: &mut Forge, magic: u64, strength: u64, recipient: address, ctx: &mut TxContext) {
        // create a sword
        let sword = Sword {
            id: object::new(ctx),
            magic: magic,
            strength: strength,
        };
        // transfer the sword
        transfer::transfer(sword, recipient);
        forge.swords_created = forge.swords_created + 1;
    }

    public entry fun sword_transfer(sword: Sword, recipient: address, _ctx: &mut TxContext) {
        // transfer the sword
        transfer::transfer(sword, recipient);
    }

    #[test]
    public fun test_sword_create() {
        use sui::tx_context;
        use sui::transfer;

        // create a dummy TxContext for testing
        let ctx = tx_context::dummy();

        // create a sword
        let sword = Sword {
            id: object::new(&mut ctx),
            magic: 42,
            strength: 7,
        };

        // check if accessor functions return correct values
        assert!(magic(&sword) == 42 && strength(&sword) == 7, 1);

         // create a dummy address and transfer the sword
        let dummy_address = @0xCAFE;
        transfer::transfer(sword, dummy_address);
    }

    #[test]
    fun test_sword_transactions(){
        use sui::test_scenario;
        
        let admin = @0xABBA;
        let initial_owner = @0xCAFE;
        let final_owner = @0xFACE;

        let scenario = &mut test_scenario::begin(&admin);
        {
            init(test_scenario::ctx(scenario));
        };

        test_scenario::next_tx(scenario,&admin);{
            let forge = test_scenario::take_owned<Forge>(scenario);
            assert!(swords_created(&forge) == 0, 1);
            sword_create(&mut forge, 42, 7, initial_owner, test_scenario::ctx(scenario));
            assert!(swords_created(&forge) == 1, 1);
            test_scenario::return_owned(scenario, forge);
        };

        test_scenario::next_tx(scenario,&initial_owner);
        {
            let sword =  test_scenario::take_owned<Sword>(scenario);
            sword_transfer(sword, final_owner, test_scenario::ctx(scenario));
        };

        test_scenario::next_tx(scenario,&final_owner);
        {
            let sword = test_scenario::take_owned<Sword>(scenario);
            assert!(magic(&sword)==42 && strength(&sword) == 7, 1);
            test_scenario::return_owned(scenario, sword);
        };
    }

    #[test]
    public fun test_module_init() {
        use sui::test_scenario;

        // create test address representing game admin
        let admin = @0xABBA;

        // first transaction to emulate module initialization
        let scenario = &mut test_scenario::begin(&admin);
        {
            init(test_scenario::ctx(scenario));
        };
        // second transaction to check if the forge has been created
        // and has initial value of zero swords created
        test_scenario::next_tx(scenario, &admin);
        {
            // extract the Forge object
            let forge = test_scenario::take_owned<Forge>(scenario);
            // verify number of created swords
            assert!(swords_created(&forge) == 0, 1);
            // return the Forge object to the object pool
            test_scenario::return_owned(scenario, forge)
        }
    }
}