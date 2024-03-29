use gclient::{EventProcessor, GearApi, Result};
use gstd::prelude::*;
use homm3::WASM_BINARY_OPT;
use homm3_io::*;

#[tokio::test]
#[ignore]
async fn gclient_test() -> Result<()> {
    let client = GearApi::dev_from_path(env!("GEAR_NODE_PATH")).await?;
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
        saver_id: 4.into(),
        archive: ArchiveDescription {
            filename: "save1".to_string(),
            hash: "Q4fdsW".to_string(),
        },
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
    let decoded_reply: homm3_io::Event =
        Decode::decode(&mut raw_reply.as_slice()).expect("Can't decode reply");

    println!(
        "raw_reply {:?}, decoded_reply = {:?}, encoded Event::Saved = {:?}",
        raw_reply,
        decoded_reply,
        Event::Saved.encode()
    );
    assert_eq!(Event::Saved, decoded_reply);

    Ok(())
}
