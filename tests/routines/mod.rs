use auction_io::*;
use codec::Encode;
use gear_lib::non_fungible_token::token::*;
use gtest::{Program, RunResult, System};

pub const USERS: &[u64] = &[4, 5, 6];
#[allow(dead_code)]
pub const DURATION: u32 = 7 * 24 * 60 * 60 * 1000;

pub fn init(sys: &System) -> Program {
    USERS
        .iter()
        .for_each(|user| sys.mint_to(*user, 1_000_000_000));
    let owner_user = USERS[0];

    sys.init_logger();

    let auction_program = Program::current(sys);

    auction_program.send(owner_user, ());

    init_nft(sys, owner_user);
    let result = update_auction(&auction_program, owner_user, 2, 1_000_000_000);

    assert!(result.contains(&(
        owner_user,
        Event::AuctionStarted {
            token_owner: owner_user.into(),
            price: 1_000_000_000,
            token_id: 0.into(),
        }
        .encode()
    )));

    auction_program
}

pub fn init_nft(sys: &System, owner: u64) {
    let nft_program = Program::from_file(sys, "./target/nft.wasm");

    nft_program.send(
        owner,
        nft_io::InitNFT {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            base_uri: String::from(""),
            royalties: None,
        },
    );

    nft_program.send(
        owner,
        nft_io::NFTAction::Mint {
            token_metadata: TokenMetadata {
                name: "MyNFT".to_string(),
                description: "NFTForAuction".to_string(),
                media: "".to_string(),
                reference: "".to_string(),
            },
        },
    );
    nft_program.send(
        owner,
        nft_io::NFTAction::Approve {
            to: 1.into(),
            token_id: 0.into(),
        },
    );
}

pub fn update_auction(
    auction: &Program,
    owner: u64,
    nft_contract_id: u64,
    starting_price: u128,
) -> RunResult {
    auction.send(
        owner,
        Action::Create(CreateConfig {
            nft_contract_actor_id: nft_contract_id.into(),
            token_owner: owner.into(),
            starting_price,
            discount_rate: 1_000,
            token_id: 0.into(),
            duration: Duration {
                days: 7,
                hours: 0,
                minutes: 0,
            },
        }),
    )
}
