use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::inspect_message;

#[inspect_message]
fn inspect_message() {
    let method_name = ic_cdk::api::call::method_name();

    if &method_name[..] == "remove_notifications" {
        RUNTIME_STATE.with(|state| accept_if_push_service(&state.borrow()));
    } else {
        ic_cdk::api::call::accept_message();
    }
}

fn accept_if_push_service(runtime_state: &RuntimeState) {
    let caller = runtime_state.env.caller();
    if runtime_state.data.push_service_principals.contains(&caller) {
        ic_cdk::api::call::accept_message();
    }
}
