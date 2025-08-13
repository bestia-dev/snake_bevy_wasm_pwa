// src/main_mod.rs

//! This module is like a main.rs module for a binary CLI executable.  
//! The `main_mod.rs` contains all input/output interface stuff.  
//! So the program logic can be separate from the interface.  

// The `lib_mod.rs` must not contains any input/output interface stuff.
// This `lib_mod.rs` can then be used as dependency crate for other projects.

// The `main_mod.rs` uses the `anyhow` error library.
// The `lib_mod.rs` uses the `thiserror` library.

use unwrap::unwrap;
use wasm_rs_dbg::dbg;

mod lib_mod;
use lib_mod::wsm;
pub use lib_mod::LibraryError;

/// entry point just like for cli-bin-executable
pub fn main() {
    // logging is essential for every project
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("main() started");

    let args = get_args_from_hash_fragment();
    routing_by_arguments(args);
}

/// get args from hash fragment
fn get_args_from_hash_fragment() -> Vec<String> {
    // region: In browser we can use 'local routing' on url path with # fragment
    // but sometimes it does not reload the page, because the browser thinks # is an anchor on the same page
    // So we need to add a listener also to this other event.
    // http://localhost:4000/snake_bevy_wasm_pwa/#arg_1/arg_2
    let location = wsm::window().location();
    let mut location_hash_fragment = unwrap!(location.hash());
    // the hash is not decoded automatically !
    // dbg! is now writing to the console, crate wasm-rs-dbg
    dbg!(&location_hash_fragment);
    dbg!(&wsm::now_time_as_string());

    // in std::env::args() the nth(0) is the exe name. Let's make it similar.
    if !location_hash_fragment.is_empty() {
        // replace # with delimiter /
        location_hash_fragment.replace_range(..1, "/");
    }
    let location_hash_fragment = format!("snake_bevy_wasm_pwa{location_hash_fragment}");
    dbg!(&location_hash_fragment);
    let args = location_hash_fragment.split("/");
    let args: Vec<String> = args.map(|x| x.to_string()).collect();
    dbg!(&args);
    args
}

/// routing by arguments  
/// routing can come from:  
/// 1. on page load and then read the window().location()  
/// 2. or from event change_hash  
/// 3. or can be called from a wasm function directly  
fn routing_by_arguments(args: Vec<String>) {
    // every page must have the header and onhashchange
    wsm::add_listener_for_onhashchange(&on_hash_change);
    header();
    // endregion

    // transforming Vec<String> to Vec<&str>, because we need that in the match expression
    let args: Vec<&str> = args.iter().map(|s| s as &str).collect();

    // super simple argument parsing.
    match args.get(1).copied() {
        None => page_with_inputs(),
        Some("page_with_inputs") => page_with_inputs(),
        Some("help") => print_help(),
        Some("print") => {
            match args.get(2).copied() {
                // second argument
                Some(greet_name) => print_greet_name(greet_name),
                None => wsm::set_html_element_inner_text("div_for_errors", "Error: Missing second argument for print."),
            }
        }
        Some("upper") => {
            match args.get(2).copied() {
                // second argument
                Some(greet_name) => {
                    // this can return an error. Here is the last place I can deal with the error.
                    match upper_greet_name(greet_name) {
                        // do nothing
                        Ok(()) => (),
                        // log error from anyhow
                        Err(err) => wsm::set_html_element_inner_text("div_for_errors", &format!("Error: {err}")),
                    }
                }
                None => wsm::set_html_element_inner_text("div_for_errors", "Error: Missing second argument for upper."),
            }
        }
        _ => wsm::set_html_element_inner_text(
            "div_for_errors",
            "Error: Unrecognized arguments. Try \n http://localhost:4000/snake_bevy_wasm_pwa/#help",
        ),
    }
}

/// the listener calls this function  
fn on_hash_change() {
    dbg!("on_hash_change");
    let args = get_args_from_hash_fragment();
    routing_by_arguments(args);
}

/// render header with Home and Help  
fn header() {
    let html_source_code = wsm::HtmlSourceCode::new(
        r#"
<div class="div_header">
    <a href="/snake_bevy_wasm_pwa/#page_with_inputs"><span class="fa-solid fa-home"></span>Home</a>
    &nbsp;
    <a href="/snake_bevy_wasm_pwa/#help"><span class="fa-solid fa-question-circle"></span>Help</a>
    &nbsp;
</div>
<div>&nbsp;</div>
<div id="div_body"></div>
"#,
    );
    html_source_code.inject_into_dom_element("div_for_wasm_html_injecting");
}

/// print help  
fn print_help() {
    wsm::set_html_element_inner_text(
        "div_body",
        r#"Welcome to snake_bevy_wasm_pwa !

This is a simple yet complete template for a PWA WASM program written in Rust.
The file structure is on purpose similar to a Rust CLI project and accepts similar arguments.

http://localhost:4000/snake_bevy_wasm_pwa/
http://localhost:4000/snake_bevy_wasm_pwa/#help
http://localhost:4000/snake_bevy_wasm_pwa/#print/world
http://localhost:4000/snake_bevy_wasm_pwa/#upper/world

This command should return an error:
http://localhost:4000/snake_bevy_wasm_pwa/#upper/WORLD

© 2025 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto
"#,
    );
}

/// render first page  
fn page_with_inputs() {
    // rust has `Raw string literals` that are great!
    // just add r# before the starting double quotes and # after the ending double quotes.
    let mut html_source_code = wsm::HtmlSourceCode::new(
        r#"<h1>snake_bevy_wasm_pwa</h1>
<p>Write a command in the Argument 1: print or upper</p>
<div class="input-wrap">
    <label for="arg_1">Argument 1:</label>  
    <input style="width:20%;" type="text" id="arg_1" value="{ph_arg_1}"/>
</div>
<p>Write a name in the Argument 2: world or WORLD</p>
<div class="input-wrap">
    <label for="arg_2">Argument 2:</label>  
    <input style="width:20%;" type="text" id="arg_2" value="{ph_arg_2}"/>
</div>
<p>Click on Run</p>
<div class="input-wrap">
    <input type="button" class="button" id="btn_run" value="Run"/>
</div>
{ph_elem_p_1}
        "#,
    );

    // {ph_...} is the prefix for placeholder to make the string unique and distinctive
    html_source_code.replace_attribute_value("{ph_arg_1}", "upper");
    html_source_code.replace_attribute_value("{ph_arg_2}", "world");

    let mut fragment = wsm::HtmlSourceCode::new(r#"<p class="{ph_attr_class_1}">{ph_text_node_1}</p>"#);
    fragment.replace_attribute_value("{ph_attr_class_1}", "small");
    fragment.replace_text_node("{ph_text_node_1}", "bestia.dev");
    html_source_code.replace_html_source_code("{ph_elem_p_1}", &fragment);

    dbg!(html_source_code.get_html());
    html_source_code.inject_into_dom_element("div_body");
    wsm::add_listener_to_button("btn_run", &on_click_btn_run);
}

/// the listener calls this function  
fn on_click_btn_run() {
    let arg_1 = wsm::get_input_element_value_string_by_id("arg_1");
    let arg_2 = wsm::get_input_element_value_string_by_id("arg_2");
    if !arg_1.is_empty() && !arg_2.is_empty() {
        // pass arguments as URL in a new tab
        let url = format!("/snake_bevy_wasm_pwa/#{arg_1}/{arg_2}");
        wsm::open_url(&url);
    } else {
        // write on the same web page
        wsm::set_html_element_inner_text("div_for_errors", "Error: Both arguments are mandatory.");
    }
}

/// print my name  
fn print_greet_name(greet_name: &str) {
    wsm::set_html_element_inner_text(
        "div_body",
        &format!(
            r#"The result is
{}
"#,
            lib_mod::format_hello_phrase(greet_name)
        ),
    );
}

/// print my name upper, can return error  
fn upper_greet_name(greet_name: &str) -> anyhow::Result<()> {
    // the function from `lib.rs`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = lib_mod::format_upper_hello_phrase(greet_name)?;
    wsm::set_html_element_inner_text(
        "div_body",
        &format!(
            r#"The result is
{upper}
"#
        ),
    );
    // return
    Ok(())
}
