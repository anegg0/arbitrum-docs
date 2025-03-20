use super::*;
use stylus_sdk::stylus_test::*;
use alloy_primitives::{address, Address, U256};

#[test]
fn test_initial_balance() {
    // Set up test VM and contract
    let vm = TestVM::default();
    let contract = VendingMachine::from(&vm);
    
    // Check initial balance is zero for a random address
    let test_address = address!("dCE82b5f92C98F27F116F70491a487EFFDb6a2a9");
    assert_eq!(contract.get_cupcake_balance_for(test_address), U256::ZERO);
}

#[test]
fn test_cupcake_distribution() {
    // Set up test VM with custom configuration
    let vm = TestVMBuilder::new()
        .block_timestamp(100)
        .build();
    let mut contract = VendingMachine::from(&vm);
    
    let test_address = address!("dCE82b5f92C98F27F116F70491a487EFFDb6a2a9");
    
    // First distribution should succeed
    assert!(contract.give_cupcake_to(test_address));
    assert_eq!(contract.get_cupcake_balance_for(test_address), U256::from(1));
    
    // Immediate second distribution should fail (less than 5 seconds)
    assert!(!contract.give_cupcake_to(test_address));
    assert_eq!(contract.get_cupcake_balance_for(test_address), U256::from(1));
    
    // Advance time by 6 seconds
    vm.set_block_timestamp(106);
    
    // Distribution should now succeed
    assert!(contract.give_cupcake_to(test_address));
    assert_eq!(contract.get_cupcake_balance_for(test_address), U256::from(2));
}

#[test]
fn test_multiple_users() {
    let vm = TestVMBuilder::new()
        .block_timestamp(100)
        .build();
    let mut contract = VendingMachine::from(&vm);
    
    let user1 = address!("dCE82b5f92C98F27F116F70491a487EFFDb6a2a9");
    let user2 = address!("DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF");
    
    // Give cupcakes to both users
    assert!(contract.give_cupcake_to(user1));
    assert!(contract.give_cupcake_to(user2));
    
    // Verify balances
    assert_eq!(contract.get_cupcake_balance_for(user1), U256::from(1));
    assert_eq!(contract.get_cupcake_balance_for(user2), U256::from(1));
    
    // Advance time
    vm.set_block_timestamp(106);
    
    // Give another cupcake to user1 only
    assert!(contract.give_cupcake_to(user1));
    
    // Verify updated balances
    assert_eq!(contract.get_cupcake_balance_for(user1), U256::from(2));
    assert_eq!(contract.get_cupcake_balance_for(user2), U256::from(1));
}
