error_chain! {
    // The type defined for this error. These are the conventional
    // and recommended names, but they can be arbitrarily chosen.
    //
    // It is also possible to leave this section out entirely, or
    // leave it empty, and these names will be used automatically.
    types {
        Error, ErrorKind, ResultExt;
    }

    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // wrapped in a new error with, in the first case, the
    // `ErrorKind::Fmt` variant. The description and cause will
    // forward to the description and cause of the original error.
    //
    // Optionally, some attributes can be added to a variant.
    //
    // This section can be empty.
    foreign_links {
        Regex(::regex::Error);
    }

    // Define additional `ErrorKind` variants.  Define custom responses with the
    // `description` and `display` calls.
    errors {
        BindingError(t: String) {
            description("Error when calling external API")
            display("Error when calling external API: '{}'", t)
        }
        ReaderError(t: String) {
            description("Invalid configuration file")
            display("Configuration file can not be loaded: '{}'", t)
        }
//        InvalidToolchainName(t: String) {
//            description("invalid toolchain name")
//            display("invalid toolchain name: '{}'", t)
//        }
//
//        // You can also add commas after description/display.
//        // This may work better with some editor auto-indentation modes:
//        UnknownToolchainVersion(v: String) {
//            description("unknown toolchain version"), // note the ,
//            display("unknown toolchain version: '{}'", v), // trailing comma is allowed
//        }
    }
}
