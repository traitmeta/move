module color::rgb {
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};
    use sui::transfer;


    struct ColorObject has key{
        id :UID,
        red: u8,
        green: u8,
        blue: u8,
    }

    fun new (red:u8, green:u8, blue:u8, ctx:&mut TxContext): ColorObject{
        ColorObject{
            id: object::new(ctx),
            red,
            green,
            blue,
        }
    }

    public entry fun create(red :u8, green:u8, blue:u8, ctx: &mut TxContext){
        let color_object = new (red,green, blue,ctx);
        transfer::transfer(color_object, tx_context::sender(ctx));
    }

    public fun get_color(self: &ColorObject):(u8, u8, u8){
        (self.red, self.green, self.blue)
    }

    public entry fun copy_into(from_obj: &ColorObject, into_obj: &mut ColorObject) {
        into_obj.red = from_obj.red;
        into_obj.green = from_obj.green;
        into_obj.blue = from_obj.blue;
    }

    public entry fun delete(object: ColorObject) {
        let ColorObject { id, red: _, green: _, blue: _ } = object;
        object::delete(id); // UID must use this func to delete
    }

    public entry fun transfer(object: ColorObject, recipient: address, _ctx: &mut TxContext) {
        transfer::transfer(object, recipient)
    }

    #[test]
    fun test_transfer(){
        use sui::test_scenario;

        let owner = @0x1;
        // Create a ColorObject and transfer it to @owner.
        let scenario = &mut test_scenario::begin(&owner);
        {
            let ctx = test_scenario::ctx(scenario);
            create(255, 0, 255, ctx);
        };
        // Transfer the object to recipient.
        let recipient = @0x2;
        test_scenario::next_tx(scenario, &owner);
        {
            let object = test_scenario::take_owned<ColorObject>(scenario);
            let ctx = test_scenario::ctx(scenario);
            transfer(object, recipient, ctx);
        };

        // Check that owner no longer owns the object.
        test_scenario::next_tx(scenario, &owner);
        {
            assert!(!test_scenario::can_take_owned<ColorObject>(scenario), 0);
        };
        // Check that recipient now owns the object.
        test_scenario::next_tx(scenario, &recipient);
        {
            assert!(test_scenario::can_take_owned<ColorObject>(scenario), 0);
        };
    }

    #[test]
    fun test_delete_color(){
        use sui::test_scenario;

        let owner = @0x1;
        // Create a ColorObject and transfer it to @owner.
        let scenario = &mut test_scenario::begin(&owner);
        {
            let ctx = test_scenario::ctx(scenario);
            create(255, 0, 255, ctx);
        };
        // Delete the ColorObject we just created.
        test_scenario::next_tx(scenario, &owner);
        {
            let object = test_scenario::take_owned<ColorObject>(scenario);
            delete(object);
        };
        // Verify that the object was indeed deleted.
        test_scenario::next_tx(scenario, &owner);
        {
            assert!(!test_scenario::can_take_owned<ColorObject>(scenario), 0);
        }
    }

    #[test]
    fun test_new_color() {
        use sui::test_scenario;

        let owner  = @0x1;
        let scenario = &mut test_scenario::begin(&owner);
        {
            let ctx = test_scenario::ctx(scenario);
            create(255, 0, 255, ctx);
        };

        let not_owner = @0x2;
        test_scenario::next_tx(scenario, &not_owner);
        {
            assert!(!test_scenario::can_take_owned<ColorObject>(scenario), 0);
        };

        test_scenario::next_tx(scenario, &owner);
        {
            let object = test_scenario::take_owned<ColorObject>(scenario);
            let (red, green, blue) = get_color(&object);
            assert!(red == 255 && green == 0 && blue ==255, 0);
            test_scenario::return_owned(scenario, object);
        };
    }

    #[test]
    fun test_copy_color(){
        use sui::test_scenario;

        let owner = @0x1;
        let scenario = &mut test_scenario::begin(&owner);
        let (id1, id2) = {
            let ctx  = test_scenario::ctx(scenario);
            create(255, 255, 255, ctx);
            let id1 = object::id_from_address(tx_context::last_created_object_id(ctx));
            create(0, 0, 0, ctx);
            let id2 = object::id_from_address(tx_context::last_created_object_id(ctx));
            (id1, id2)
        };

        test_scenario::next_tx(scenario, &owner);
        {
            let obj1 = test_scenario::take_owned_by_id<ColorObject>(scenario, id1);
            let obj2 = test_scenario::take_owned_by_id<ColorObject>(scenario, id2);
            let (red, green, blue) = get_color(&obj1);
            assert!(red == 255 && green == 255 && blue == 255, 0);

            let _ctx = test_scenario::ctx(scenario);
            copy_into(&obj2, &mut obj1);
            test_scenario::return_owned(scenario, obj1);
            test_scenario::return_owned(scenario, obj2);
        };

        test_scenario::next_tx(scenario, &owner);
        {
            let obj1 = test_scenario::take_owned_by_id<ColorObject>(scenario, id1);
            let (red, green, blue) = get_color(&obj1);
            assert!(red == 0 && green == 0 && blue == 0, 0);
            test_scenario::return_owned(scenario, obj1);
        }
    }
}