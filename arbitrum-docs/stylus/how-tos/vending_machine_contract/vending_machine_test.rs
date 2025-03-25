use super::*;
use stylus_test::*;
use stylus_sdk::alloy_primitives::Address;

#[test]
fn test_give_cupcake() {
    // Setup test environment
    let vm = TestVM::default();
    let mut contract = VendingMachine::from(&vm);
    
    // Create a test user address
    let user = Address::repeat_byte(0x42);
    
    // Initial balance should be zero
    let initial_balance = contract.get_cupcake_balance_for(user);
    assert_eq!(initial_balance, U256::ZERO);
    
    // First cupcake should be given successfully
    let result = contract.give_cupcake_to(user);
    assert!(result);
    
    // Balance should be incremented to 1
    let balance_after_first = contract.get_cupcake_balance_for(user);
    assert_eq!(balance_after_first, U256::from(1));
    
    // Trying to get another cupcake immediately should fail (5 second cooldown)
    let result = contract.give_cupcake_to(user);
    assert!(!result);
    
    // Balance should still be 1
    let balance_after_second_attempt = contract.get_cupcake_balance_for(user);
    assert_eq!(balance_after_second_attempt, U256::from(1));
    
    // Advance time by 6 seconds
    vm.advance_time(6);
    
    // Now we should be able to get another cupcake
    let result = contract.give_cupcake_to(user);
    assert!(result);
    
    // Balance should be incremented to 2
    let final_balance = contract.get_cupcake_balance_for(user);
    assert_eq!(final_balance, U256::from(2));
}

#[test]
fn test_multiple_users() {
    // Setup test environment
    let vm = TestVM::default();
    let mut contract = VendingMachine::from(&vm);
    
    // Create two test user addresses
    let user1 = Address::repeat_byte(0x11);
    let user2 = Address::repeat_byte(0x22);
    
    // Give cupcake to first user
    let result = contract.give_cupcake_to(user1);
    assert!(result);
    
    // First user should have 1 cupcake
    let user1_balance = contract.get_cupcake_balance_for(user1);
    assert_eq!(user1_balance, U256::from(1));
    
    // Second user should have 0 cupcakes
    let user2_balance = contract.get_cupcake_balance_for(user2);
    assert_eq!(user2_balance, U256::ZERO);
    
    // Give cupcake to second user
    let result = contract.give_cupcake_to(user2);
    assert!(result);
    
    // Second user should now have 1 cupcake
    let user2_balance = contract.get_cupcake_balance_for(user2);
    assert_eq!(user2_balance, U256::from(1));
    
    // First user should still have 1 cupcake
    let user1_balance = contract.get_cupcake_balance_for(user1);
    assert_eq!(user1_balance, U256::from(1));
}
