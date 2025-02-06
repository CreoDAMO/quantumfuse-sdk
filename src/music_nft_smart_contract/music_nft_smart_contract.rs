use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub artist: String,
    pub song_title: String,
    pub metadata_url: String,
    pub initial_price: Uint128,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let nft_data = MusicNFT {
        owner: msg.artist.clone(),
        song_title: msg.song_title,
        metadata_url: msg.metadata_url,
        price: msg.initial_price,
    };

    deps.storage.save("music_nft", &nft_data)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}
