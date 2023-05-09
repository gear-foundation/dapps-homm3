use app_io::*;
use gmeta::Encode;
use gtest::{Program, System};

#[test]
fn test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);
    let mut result = program.send_bytes(2, []);

    assert!(!result.main_failed());

    let name = "WWW";

    let demo_state = GameState {
        saver_id: 2.into(),
        tar: ArchiveDescription {
            filename: "filename".to_string(),
            hash: "hash".to_string(),
            name: name.to_string(),
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
            hash: "hash".to_string(),
        },
    );

    println!("Load reply = {:?}", result);
    let expected = Event::Loaded(Some(demo_state)).encode();
    assert!(result.contains(&(2, expected)));
}
