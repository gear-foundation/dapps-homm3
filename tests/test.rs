use gmeta::Encode;
use gtest::{Program, System};
use homm3_io::*;
// use homm3_state::{WASM_BINARY, WASM_EXPORTS};

#[test]
fn test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);
    let mut result = program.send_bytes(2, []);

    assert!(!result.main_failed());

    let name = "Q4fdsW";

    let demo_state = GameState {
        saver_id: 4.into(),
        archive: ArchiveDescription {
            filename: "save1".to_string(),
            hash: name.to_string(),
        },
    };
    let save_action = Action::Save(demo_state.clone());

    result = program.send(2, save_action);
    println!("Save reply = {:?}", result);

    // assert!(result.contains(&(2, vec![4, 1])));
    assert!(result.contains(&(2, Event::Saved.encode())));
    // assert!(result.contains(&Log::builder().payload(Event::Saved)));
    // assert!(result.contains(&Event::Saved));

    let result = program.send(
        2,
        Action::Load {
            hash: name.to_string(),
        },
    );

    println!("Load reply = {:?}", result);
    let expected = Event::Loaded(Some(demo_state)).encode();
    assert!(result.contains(&(2, expected)));

    // State reading

    // All state

    // let mut expected_state = vec![];

    // for mut actor in 0..=100 {
    //     actor += 2;
    //     result = program.send(actor, PingPong::Ping);

    //     assert!(result.contains(&Log::builder().payload(PingPong::Pong)));

    //     expected_state.push((actor.into(), 1))
    // }

    // let mut state: <ContractMetadata as Metadata>::State = program.read_state().unwrap();

    // expected_state.sort_unstable();
    // state.sort_unstable();

    // assert_eq!(state, expected_state);

    // `ping_count` metafunction

    // result = program.send(2, PingPong::Ping);

    // assert!(result.contains(&Log::builder().payload(PingPong::Pong)));

    // let ping_count: u128 = program
    // .read_state_using_wasm(WASM_EXPORTS[2], WASM_BINARY.into(), Some(ActorId::from(2)))
    // .unwrap();

    // assert_eq!(ping_count, 2);
}
