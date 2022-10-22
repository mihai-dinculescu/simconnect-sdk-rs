use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=ffi/include/Wrapper.h");
    println!("cargo:rerun-if-changed=ffi/include/WrapperFake.h");
    println!("cargo:rerun-if-changed=ffi/include/SimConnect.h");
    println!("cargo:rerun-if-changed=ffi/lib/SimConnect.lib");
    println!("cargo:rerun-if-changed=ffi/lib/SimConnect.dll");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_path.to_string_lossy());
    println!("cargo:rustc-link-lib=static=SimConnect");

    let builder = if let Ok(_) = env::var("DOCS_RS") {
        bindgen::Builder::default().header("ffi/include/WrapperFake.h")
    } else {
        bindgen::Builder::default().header("ffi/include/Wrapper.h")
    };

    let bindings = builder
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
        .allowlist_function("SimConnect_ClearDataDefinition")
        .allowlist_function("SimConnect_SubscribeToFacilities")
        .allowlist_function("SimConnect_UnsubscribeToFacilities")
        .allowlist_function("SimConnect_RequestFacilitiesList")
        .allowlist_type("SIMCONNECT_RECV")
        .allowlist_type("SIMCONNECT_RECV_ID")
        .allowlist_type("SIMCONNECT_RECV_EVENT")
        .allowlist_type("SIMCONNECT_RECV_SIMOBJECT_DATA")
        .allowlist_type("SIMCONNECT_RECV_FACILITIES_LIST")
        .allowlist_type("SIMCONNECT_RECV_AIRPORT_LIST")
        .allowlist_type("SIMCONNECT_RECV_WAYPOINT_LIST")
        .allowlist_type("SIMCONNECT_RECV_NDB_LIST")
        .allowlist_type("SIMCONNECT_RECV_VOR_LIST")
        .allowlist_type("SIMCONNECT_CLIENT_DATA_PERIOD")
        .allowlist_type("SIMCONNECT_RECV_OPEN")
        .allowlist_type("SIMCONNECT_RECV_EXCEPTION")
        .allowlist_var("SIMCONNECT_DATA_REQUEST_FLAG_CHANGED")
        .allowlist_var("SIMCONNECT_RECV_ID_VOR_LIST_HAS_NAV_SIGNAL")
        .allowlist_var("SIMCONNECT_RECV_ID_VOR_LIST_HAS_LOCALIZER")
        .allowlist_var("SIMCONNECT_RECV_ID_VOR_LIST_HAS_GLIDE_SLOPE")
        .allowlist_var("SIMCONNECT_RECV_ID_VOR_LIST_HAS_DME")
        .allowlist_var("SIMCONNECT_OBJECT_ID_USER")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write the bindings");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to find `CARGO_MANIFEST_DIR`");
    let lib_path = PathBuf::from(manifest_dir).join("ffi/lib");

    for file in &["SimConnect.lib", "SimConnect.dll"] {
        std::fs::copy(lib_path.join(file), out_path.join(file))
            .unwrap_or_else(|_| panic!("Failed to copy `{file}`"));
    }
}
