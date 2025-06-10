mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use pb::contract::v1::Events;
use pb::sf::ethereum::r#type::v2::Block;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!();

// substreams gui -e mainnet.eth.streamingfast.io:443 substreams.yaml map_events_calls -s 22471108 -t +1

// substreams gui -e mainnet.eth.streamingfast.io:443 substreams.yaml map_events -s 22471108 -t +1

const FACTORY_TRACKED_CONTRACTS: &[[u8; 20]] = &[
    hex!("10ea9e5303670331bdddfa66a4cea47dae4fcf3b"),
    ];


#[substreams::handlers::map]
fn map_events_calls(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<contract::EventsCalls, substreams::errors::Error> {
    Ok(contract::EventsCalls {
        events: Some(events),
        calls: Some(calls),
    })
}



#[substreams::handlers::map]
fn map_events(blk: Block) -> Result<Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    let logs = blk
        .transaction_traces
        .iter()
        .filter_map(|transaction| transaction.receipt.as_ref())
        .flat_map(|view| {
            view.logs
                .iter()
                .filter(|log| is_address_in_contracts(&log.address)).cloned()
        })
        .collect();

    events.logs = logs;

    Ok(events)
}



#[substreams::handlers::map]
fn map_calls(blk: Block) -> Result<contract::Calls, substreams::errors::Error> {
    let mut calls = contract::Calls::default();
    let transaction_calls = blk
        .transaction_traces
        .iter()
        .flat_map(|transaction| {
            transaction.calls
                .iter()
                .filter(|call| is_address_in_contracts(&call.address)).cloned()
        })
        .collect();
    calls.calls = transaction_calls;
    Ok(calls)
}


fn is_address_in_contracts(address: &Vec<u8>) -> bool {
    if address.len() != 20 {
        return false;
    }

    FACTORY_TRACKED_CONTRACTS.contains(&address.as_slice().try_into().unwrap())
}
