use app::WASM_BINARY_OPT;
use app_io::*;
use app_state::{WASM_BINARY, WASM_EXPORTS};
use gclient::{EventProcessor, EventListener, GearApi, Result};
use gstd::{prelude::*, ActorId};


const ALICE: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];

#[tokio::test]
#[ignore]
async fn gclient_test() -> Result<()> {
    let client = GearApi::dev().await.unwrap(); // GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;
    let mut listener = client.subscribe().await?;

    let mut gas_limit = client
        .calculate_upload_gas(None, WASM_BINARY_OPT.into(), vec![], 0, true)
        .await?
        .min_limit;
    

    let (mut message_id, program_id, _) = client
        .upload_program_bytes(
            WASM_BINARY_OPT,
            gclient::now_micros().to_le_bytes(),
            [],
            gas_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let demo_state = GameState {
        name: "autosave1".to_string(),
        data: vec![6, 6, 6, 6, 6, 6, 6, 6, 6],
    };
    let save_action = Action::Save(demo_state);

    gas_limit = client
        .calculate_handle_gas(None, program_id, save_action.encode(), 0, true)
        .await?
        .min_limit;
    (message_id, _) = client
        .send_message(program_id, save_action, gas_limit, 0)
        .await?;

    // assert!(listener.message_processed(message_id).await?.succeed());
    let (_m, raw_reply, _) = listener.reply_bytes_on(message_id).await?;
    let raw_reply = raw_reply.unwrap();
    let decoded_reply: app_io::Event =
        Decode::decode(&mut raw_reply.as_slice()).expect("Can't decode reply");

    println!(
        "raw_reply {:?}, decoded_reply = {:?}, encoded Event::Saved = {:?}",
        raw_reply,
        decoded_reply,
        Event::Saved.encode()
    );
    assert_eq!(Event::Saved, decoded_reply);

    // let state_binary = WASM_BINARY.to_vec();

    // assert_eq!(
    //     client
    //         .read_state_using_wasm::<_, u128>(
    //             program_id,
    //             WASM_EXPORTS[2],
    //             state_binary.clone(),
    //             Some(ActorId::from(ALICE))
    //         )
    //         .await?,
    //     1
    // );

    // assert_eq!(
    //     client
    //         .read_state_using_wasm::<(), Vec<ActorId>>(
    //             program_id,
    //             WASM_EXPORTS[1],
    //             state_binary,
    //             None
    //         )
    //         .await?,
    //     vec![ALICE.into()]
    // );

    Ok(())
}
