use auction_io::*;
use codec::Encode;
use gstd::ActorId;
use gtest::System;

mod routines;
use routines::*;

#[test]
fn buy() {
    let sys = System::new();

    let auction = init(&sys);

    let nft_program = sys.get_program(2);
    let res = nft_owner(&nft_program, USERS[0], 0.into());
    println!("{:?}", res.decoded_log::<ActorId>());

    let result = auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);

    assert!(result.contains(&(
        USERS[1],
        Event::Bought {
            price: 1_000_000_000,
        }
        .encode()
    )));

    let res = nft_owner(&nft_program, USERS[0], 0.into());
    let new_owner = ActorId::from(USERS[1]);
    println!("{:?}", res.decoded_log::<ActorId>());

    assert!(res.contains(&(USERS[0], new_owner.encode(),)));

    sys.claim_value_from_mailbox(USERS[0]);

    let buyer_balance = sys.balance_of(USERS[1]);
    let seller_balance = sys.balance_of(USERS[0]);

    assert_eq!(buyer_balance, 0);
    assert_eq!(seller_balance, 2_000_000_000);
}

#[test]
fn buy_later_with_lower_price() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(100_000_000);
    let result = auction.send_with_value(USERS[1], Action::Buy, 900_000_000);

    assert!(result.contains(&(USERS[1], Event::Bought { price: 900_000_000 }.encode())));

    sys.claim_value_from_mailbox(USERS[0]);

    let buyer_balance = sys.balance_of(USERS[1]);
    let seller_balance = sys.balance_of(USERS[0]);

    assert_eq!(buyer_balance, 100_000_000);
    assert_eq!(seller_balance, 1_900_000_000);
}

#[test]
fn buy_two_times() {
    let sys = System::new();

    let auction = init(&sys);
    auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);
    let result = auction.send_with_value(USERS[2], Action::Buy, 1_000_000_000);

    assert!(result.main_failed());
}

#[test]
fn buy_too_late() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(DURATION);
    let result = auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);

    assert!(result.main_failed());
}

#[test]
fn buy_with_less_money() {
    let sys = System::new();

    let auction = init(&sys);
    let result = auction.send_with_value(USERS[1], Action::Buy, 999_000_000);

    assert!(result.main_failed());
}

#[test]
fn create_auction_twice_in_a_row() {
    let sys = System::new();

    let auction = init(&sys);
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, 999_000_000);

    assert!(result.main_failed());
}

#[test]
fn create_auction_twice_after_time() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(DURATION);
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, 999_000_000);

    assert!(result.contains(&(
        USERS[1],
        Event::AuctionStarted {
            token_owner: USERS[1].into(),
            price: 999_000_000,
            token_id: 0.into(),
        }
        .encode()
    )));
}

#[test]
fn create_auction_with_low_price() {
    let sys = System::new();

    let auction = init(&sys);
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, (DURATION / 1000 - 1).into());

    assert!(result.main_failed());
}

#[test]
fn create_and_stop() {
    let sys = System::new();
    let owner_user = USERS[0];
    let auction = init(&sys);

    let result = auction.send(owner_user, Action::ForceStop);

    assert!(result.contains(&(
        owner_user,
        Event::AuctionStoped {
            token_owner: owner_user.into(),
            token_id: 0.into(),
        }
        .encode()
    )));
}
