use cosmwasm_std::{
    entry_point, to_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::StreamNFT { nft_id } => record_stream(deps, info.sender.to_string(), nft_id),
    }
}

pub fn record_stream(deps: DepsMut, user: String, nft_id: String) -> StdResult<Response> {
    let mut streams: StreamRecord = deps.storage.load(&nft_id)?;
    streams.streams += 1;
    deps.storage.save(&nft_id, &streams)?;

    let royalty = Uint128::from(streams.streams * 5);
    let transfer_msg = BankMsg::Send {
        from_address: user,
        to_address: streams.artist.clone(),
        amount: vec![cosmwasm_std::Coin {
            denom: "qfc".to_string(),
            amount: royalty,
        }],
    };

    let cosmos_msg: CosmosMsg = transfer_msg.into();
    Ok(Response::new().add_message(cosmos_msg))
}

// Make handle_streaming_payments public
pub fn handle_streaming_payments() -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing streaming payments...");
    // Implement streaming payments handling logic here
    Ok(())
}
