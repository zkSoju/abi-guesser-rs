use ethers::{abi::ParamType, types::Bytes};
use eyre::Result;

// fn try_parse_offset(data: Vec<u8>, pos: usize) -> Result<u64> {
//     let word = data[pos..pos + 32].to_vec();

// }

pub fn guess_abi_encoded_data(bytes: Bytes) -> Result<Vec<ParamType>> {
    let bytes = bytes.to_vec();
    let static_len = bytes.len();

    let params = decode_well_formed_tuple(0, bytes, 0, vec![], static_len, None, None);

    let final_params: Vec<ParamType> = vec![];
    Ok(final_params)
}

fn decode_well_formed_tuple(
    depth: usize,
    data: Vec<u8>,
    param_idx: usize,
    collected_params: Vec<ParamType>,
    end_of_static_calldata: usize,
    expected_length: Option<usize>,
    is_dynamic_array_element: Option<bool>,
) -> Result<Vec<ParamType>> {
    let param_offset = param_idx * 32;

    if param_offset < end_of_static_calldata {
        if let Some(is_dynamic) = is_dynamic_array_element {
            Err(eyre::eyre!("Dynamic array element is not allowed in static calldata"))
        } else {
            let fragment = decode_well_formed_tuple(
                depth,
                data,
                param_idx + 1,
                collected_params,
                end_of_static_calldata,
                expected_length,
                is_dynamic_array_element,
            );

            let final_params: Vec<ParamType> = vec![];
            Ok(final_params)
        }
    } else {
        let final_params: Vec<ParamType> = vec![];
        Ok(final_params)
    }
}
