fn main() {
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("--std=c++11")
        .file("optick/src/optick_capi.cpp")
        .file("optick/src/optick_core.cpp")
        .file("optick/src/optick_gpu.cpp")
        .file("optick/src/optick_message.cpp")
        .file("optick/src/optick_miniz.cpp")
        .file("optick/src/optick_serialization.cpp")
        .file("optick/src/optick_server.cpp")
        .compile("optick");
}
