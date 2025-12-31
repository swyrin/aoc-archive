use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse::Parse};

struct TestMacroArgs {
    input_type: syn::Type,
    sample: syn::LitStr,
    expected_out: syn::LitInt,
}

impl Parse for TestMacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut input_type = None;
        let mut sample = None;
        let mut expected = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "input_type" => {
                    if input_type.is_some() {
                        return Err(input.error("`input_type` specified more than once"));
                    }
                    input_type = Some(input.parse::<syn::Type>()?);
                }
                "sample" => {
                    if sample.is_some() {
                        return Err(input.error("`sample` specified more than once"));
                    }
                    sample = Some(input.parse::<syn::LitStr>()?);
                }
                "expected" | "expected_out" => {
                    if expected.is_some() {
                        return Err(input.error("`expected_out` specified more than once"));
                    }
                    expected = Some(input.parse::<syn::LitInt>()?);
                }
                other => {
                    return Err(input.error(format!(
                        "Unknown argument `{}` (expected `input_type`, `sample`, `expected_out`)",
                        other
                    )));
                }
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self {
            input_type: input_type.ok_or_else(|| input.error("Missing `input_type`"))?,
            sample: sample.ok_or_else(|| input.error("Missing `sample`"))?,
            expected_out: expected.ok_or_else(|| input.error("Missing `expected_out`"))?,
        })
    }
}

#[proc_macro_attribute]
pub fn test_should_output(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as TestMacroArgs);
    let func = syn::parse_macro_input!(input as ItemFn);

    let fn_name = &func.sig.ident;
    let sample = &args.sample;
    let input_type = &args.input_type;

    let expected: u128 = args
        .expected_out
        .base10_parse()
        .expect("expected_out must be a u128");

    let test_name = syn::Ident::new(&format!("result_{}", fn_name), fn_name.span());

    let expanded = quote! {
        #func

        use parameterized::parameterized;
        use aoc_lib::input::InputLike;

        #[parameterized(expected = { #expected })]
        fn #test_name(expected: u128)
        where
            #input_type: InputLike,
        {
            let input = <#input_type as InputLike>::from_str(#sample);

            assert_eq!(
                crate::#fn_name(input, true),
                Umi::from_number(expected)
            );
        }
    };

    expanded.into()
}

struct RunMacroArgs {
    day: syn::LitInt,
    input_type: syn::Type,
    run_part1: bool,
    run_part2: bool,
}

impl Parse for RunMacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut day = None;
        let mut input_type = None;
        let mut run_part1 = None;
        let mut run_part2 = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "day" => day = Some(input.parse::<syn::LitInt>()?),
                "input_type" => input_type = Some(input.parse::<syn::Type>()?),
                "run_part1" => run_part1 = Some(input.parse::<syn::LitBool>()?.value),
                "run_part2" => run_part2 = Some(input.parse::<syn::LitBool>()?.value),
                other => return Err(input.error(format!("Unknown argument `{}`", other))),
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self {
            day: day.ok_or_else(|| input.error("Missing `day`"))?,
            input_type: input_type.ok_or_else(|| input.error("Missing `input_type`"))?,
            run_part1: run_part1.unwrap_or(true),
            run_part2: run_part2.unwrap_or(true),
        })
    }
}

#[proc_macro]
pub fn aoc_run(input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(input as RunMacroArgs);

    let day = &args.day;
    let input_type = &args.input_type;
    let run_part1 = args.run_part1;
    let run_part2 = args.run_part2;

    let expanded = quote! {
        use aoc_lib::input::InputLike;

        const DAY_NUMBER: u8 = #day;

        let input_path = format!("input/day_{:02}.txt", DAY_NUMBER);
        if !std::path::Path::new(&input_path).exists() {
            panic!("Missing input file: {}", input_path);
        }

        if #run_part1 {
            let now = std::time::Instant::now();
            println!(
                "Part 1: {}",
                part_1(<#input_type as InputLike>::from_day_number(DAY_NUMBER), false)
            );
            println!(
                "Running part_1() took {} ms.",
                now.elapsed().as_millis()
            );
        }

        if #run_part2 {
            let now = std::time::Instant::now();
            println!(
                "Part 2: {}",
                part_2(<#input_type as InputLike>::from_day_number(DAY_NUMBER), false)
            );
            println!(
                "Running part_2() took {} ms.",
                now.elapsed().as_millis()
            );
        }
    };

    expanded.into()
}
