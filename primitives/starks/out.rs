#![feature(prelude_import)]
#![warn(missing_docs)]
//!Substrate runtime standard library as compiled when linked with Rust's standard library.
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use sp_runtime_interface::{runtime_interface, Pointer};
#[cfg(feature = "std")]
use distaff::StarkProof;
pub mod starks {
    use super::*;
    #[doc(hidden)]
    extern crate sp_runtime_interface as proc_macro_runtime_interface;
    #[cfg(feature = "std")]
    pub fn verify(
        program_hash: &[u8; 32],
        public_inputs: &[u128],
        outputs: &[u128],
        proof: &[u8],
    ) -> Result<bool, ()> {
        verify_version_1(program_hash, public_inputs, outputs, proof)
    }
    #[cfg(feature = "std")]
    fn verify_version_1(
        program_hash: &[u8; 32],
        public_inputs: &[u128],
        outputs: &[u128],
        proof: &[u8],
    ) -> Result<bool, ()> {
        {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "verify_version_1",
                        "sp_starks::starks",
                        ::tracing::Level::TRACE,
                        Some("primitives/starks/src/lib.rs"),
                        Some(18u32),
                        Some("sp_starks::starks"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        }
        .in_scope(|| {
            proc_macro_runtime_interface::with_externalities(|mut __externalities__| {
                Starks::verify_version_1(
                    &mut __externalities__,
                    program_hash,
                    public_inputs,
                    outputs,
                    proof,
                )
            })
            .expect("`verify_version_1` called outside of an Externalities-provided environment.")
        })
    }
    #[cfg(feature = "std")]
    impl Starks for &mut dyn proc_macro_runtime_interface::Externalities {
        fn verify_version_1(
            &mut self,
            program_hash: &[u8; 32],
            public_inputs: &[u128],
            outputs: &[u128],
            proof: &[u8],
        ) -> Result<bool, ()> {
            let stark_proof = bincode::deserialize::<StarkProof>(&proof).unwrap();
            distaff::verify(program_hash, public_inputs, outputs, &stark_proof).map_err(|_e| ())
        }
    }
    trait Starks {
        fn verify_version_1(
            &mut self,
            program_hash: &[u8; 32],
            public_inputs: &[u128],
            outputs: &[u128],
            proof: &[u8],
        ) -> Result<bool, ()>;
    }
    /// Provides implementations for the extern host functions.
    #[cfg(feature = "std")]
    pub struct HostFunctions;
    #[cfg(feature = "std")]
    impl proc_macro_runtime_interface::sp_wasm_interface::HostFunctions for HostFunctions {
        fn host_functions(
        ) -> Vec<&'static dyn proc_macro_runtime_interface::sp_wasm_interface::Function> {
            <[_]>::into_vec(box [{
                struct ExtStarksVerifyVersion1;
                impl proc_macro_runtime_interface::sp_wasm_interface::Function for ExtStarksVerifyVersion1 {
                    fn name(&self) -> &str {
                        "ext_starks_verify_version_1"
                    }
                    fn signature(
                        &self,
                    ) -> proc_macro_runtime_interface::sp_wasm_interface::Signature
                    {
                        proc_macro_runtime_interface :: sp_wasm_interface :: Signature { args : std :: borrow :: Cow :: Borrowed (& [< < [u8 ; 32] as proc_macro_runtime_interface :: RIType > :: FFIType as proc_macro_runtime_interface :: sp_wasm_interface :: IntoValue > :: VALUE_TYPE , < < [u128] as proc_macro_runtime_interface :: RIType > :: FFIType as proc_macro_runtime_interface :: sp_wasm_interface :: IntoValue > :: VALUE_TYPE , < < [u128] as proc_macro_runtime_interface :: RIType > :: FFIType as proc_macro_runtime_interface :: sp_wasm_interface :: IntoValue > :: VALUE_TYPE , < < [u8] as proc_macro_runtime_interface :: RIType > :: FFIType as proc_macro_runtime_interface :: sp_wasm_interface :: IntoValue > :: VALUE_TYPE] [..]) , return_value : Some (< < Result < bool , () > as proc_macro_runtime_interface :: RIType > :: FFIType as proc_macro_runtime_interface :: sp_wasm_interface :: IntoValue > :: VALUE_TYPE) , }
                    }
                    fn execute(
                        &self,
                        __function_context__ : & mut dyn proc_macro_runtime_interface :: sp_wasm_interface :: FunctionContext,
                        args: &mut dyn Iterator<
                            Item = proc_macro_runtime_interface::sp_wasm_interface::Value,
                        >,
                    ) -> std::result::Result<
                        Option<proc_macro_runtime_interface::sp_wasm_interface::Value>,
                        String,
                    > {
                        let val = args . next () . ok_or_else (| | "Number of arguments given to `verify` does not match the expected number of arguments!") ? ;
                        let program_hash_ffi_value = < < [u8 ; 32] as proc_macro_runtime_interface :: RIType > :: FFIType as proc_macro_runtime_interface :: sp_wasm_interface :: TryFromValue > :: try_from_value (val) . ok_or_else (| | "Could not instantiate `program_hash` from wasm value while executing `verify` from interface `Starks`!") ? ;
                        let val = args . next () . ok_or_else (| | "Number of arguments given to `verify` does not match the expected number of arguments!") ? ;
                        let public_inputs_ffi_value = < < [u128] as proc_macro_runtime_interface :: RIType > :: FFIType as proc_macro_runtime_interface :: sp_wasm_interface :: TryFromValue > :: try_from_value (val) . ok_or_else (| | "Could not instantiate `public_inputs` from wasm value while executing `verify` from interface `Starks`!") ? ;
                        let val = args . next () . ok_or_else (| | "Number of arguments given to `verify` does not match the expected number of arguments!") ? ;
                        let outputs_ffi_value = < < [u128] as proc_macro_runtime_interface :: RIType > :: FFIType as proc_macro_runtime_interface :: sp_wasm_interface :: TryFromValue > :: try_from_value (val) . ok_or_else (| | "Could not instantiate `outputs` from wasm value while executing `verify` from interface `Starks`!") ? ;
                        let val = args . next () . ok_or_else (| | "Number of arguments given to `verify` does not match the expected number of arguments!") ? ;
                        let proof_ffi_value = < < [u8] as proc_macro_runtime_interface :: RIType > :: FFIType as proc_macro_runtime_interface :: sp_wasm_interface :: TryFromValue > :: try_from_value (val) . ok_or_else (| | "Could not instantiate `proof` from wasm value while executing `verify` from interface `Starks`!") ? ;
                        let program_hash = < [u8 ; 32] as proc_macro_runtime_interface :: host :: FromFFIValue > :: from_ffi_value (__function_context__ , program_hash_ffi_value) ? ;
                        let public_inputs = < [u128] as proc_macro_runtime_interface :: host :: FromFFIValue > :: from_ffi_value (__function_context__ , public_inputs_ffi_value) ? ;
                        let outputs = < [u128] as proc_macro_runtime_interface :: host :: FromFFIValue > :: from_ffi_value (__function_context__ , outputs_ffi_value) ? ;
                        let proof = < [u8] as proc_macro_runtime_interface :: host :: FromFFIValue > :: from_ffi_value (__function_context__ , proof_ffi_value) ? ;
                        let verify_result =
                            verify_version_1(&program_hash, &public_inputs, &outputs, &proof);
                        < Result < bool , () > as proc_macro_runtime_interface :: host :: IntoFFIValue > :: into_ffi_value (verify_result , __function_context__) . map (proc_macro_runtime_interface :: sp_wasm_interface :: IntoValue :: into_value) . map (Some)
                    }
                }
                &ExtStarksVerifyVersion1
                    as &dyn proc_macro_runtime_interface::sp_wasm_interface::Function
            }])
        }
    }
}
