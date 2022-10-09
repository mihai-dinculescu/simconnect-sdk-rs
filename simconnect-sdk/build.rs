use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=simconnect-sdk/ffi/lib");
    println!("cargo:rustc-link-lib=static=SimConnect");
    println!("cargo:rerun-if-changed=simconnect-sdk/ffi/include/SimConnect.h");
    println!("cargo:rerun-if-changed=simconnect-sdk/ffi/lib/SimConnect.lib");

    let bindings = bindgen::Builder::default()
        .header("ffi/include/SimConnect.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(&["-x", "c++"])
        .allowlist_function("SimConnect_Open")
        .allowlist_function("SimConnect_Close")
        .allowlist_function("SimConnect_MapClientEventToSimEvent")
        .allowlist_function("SimConnect_AddClientEventToNotificationGroup")
        .allowlist_function("SimConnect_SetNotificationGroupPriority")
        .allowlist_function("SimConnect_CallDispatch")
        .allowlist_function("SimConnect_GetNextDispatch")
        .allowlist_function("SimConnect_AddToDataDefinition")
        .allowlist_function("SimConnect_RequestDataOnSimObject")
        .allowlist_function("SimConnect_SubscribeToFacilities")
        .allowlist_function("SimConnect_RequestFacilitiesList")
        .allowlist_type("SIMCONNECT_RECV")
        .allowlist_type("SIMCONNECT_RECV_ID")
        .allowlist_type("SIMCONNECT_RECV_EVENT")
        .allowlist_type("SIMCONNECT_RECV_SIMOBJECT_DATA")
        .allowlist_type("SIMCONNECT_RECV_AIRPORT_LIST")
        .allowlist_type("SIMCONNECT_CLIENT_DATA_PERIOD")
        .allowlist_type("SIMCONNECT_RECV_OPEN")
        .allowlist_type("SIMCONNECT_RECV_EXCEPTION")
        .allowlist_var("SIMCONNECT_DATA_REQUEST_FLAG_CHANGED")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write the bindings!");
}
