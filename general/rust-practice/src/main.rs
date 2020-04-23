#![allow(unused_imports)]
#![allow(dead_code)]

mod rect;
mod heap;
mod controlflow;
mod structex;
mod options;
mod array;
mod vectors;
mod strings;
mod tuples;
mod hashmap;
mod pattern_matching;
mod generics;
mod ownership;
mod closures;
mod hof;
mod traits;
mod trait_extend_types;
mod conversion_from_into;
mod operator_overloading;
mod dispatch;
mod vectors_different_objects;
mod lifetime;
mod smart_pointers;
mod too_many_lists;
mod too_many_lists_take2;
mod too_many_lists_take3_rc;
mod rc;
mod arc;
mod messages_between_threads;

fn square(x: i32, y: i32) -> i32 {
    return x * y;
}

fn cube(x: i32) -> i32 {
    return x * x * x;
}

fn template_function<A>(x: A) -> A {
    return x;
}

fn main() {
    // let x = 10;
    // let sqx = square(x, x);
    // let c = cube(x);
    // let area=rect::area(x, x);
    //
    // println!("\nFunctions test");
    // println!("The square of {:?} cube {:?} area={}", sqx, c, area );
    // let tf = template_function(10);
    //
    // println!("Template function returned {:?}", tf);
    //
    // println!("\nHeap variables test");
    // heap::heap_test();
    //
    // println!("\nControl flows test");
    // println!("\n .. if .. ");
    // controlflow::check_body_temperature(97);
    // controlflow::check_body_temperature(102);
    //
    // println!("\n .. while .. ");
    // controlflow::while_loop_println(5);
    //
    // println!("\n .. for loop until 5.. ");
    // controlflow::for_loop(5);
    //
    // println!("\n .. for loop range with index .. ");
    // controlflow::for_loop_range_with_index(10);
    //
    // println!("\n .. match test");
    // controlflow::match_country_code_phone_number(91);
    // controlflow::match_country_code_phone_number(9999);
    // controlflow::match_country_code_phone_number(34);
    //
    // println!("\n .. Circle color test .. ");
    // let circle = structex::get_circle(structex::Color::Red, structex::Point {x: 10, y: 10}, 32);
    // println!("circle @ ({}, {}) radius {} with color {:?}", circle.center.x, circle.center.y, circle.radius, circle.background_color);
    // println!("cicle area = {}", circle.area());
    //
    // println!("\n .. options test .. ");
    // options::options_test(10.0, 3.0);
    // options::options_test(10.0, 0.0);
    //
    // println!("\n .. Arrays ..");
    // array::array_test();
    //
    // println!("\n .. vectors ..");
    // vectors::vectors_test();
    //
    // println!("\n .. strings ..");
    // strings::strings_test();
    //
    // println!("\n .. tuples ..");
    // tuples::tuples_test();
    //
    // println!("\n .. hashmap ..");
    // hashmap::hashmap_test();
    //
    // println!("\n .. pattern matching ..");
    // pattern_matching::pm_test();
    //
    // println!("\n .. Generics test .. ");
    // generics::generics_test();
    //
    // // println!("\n .. Ownership test .. ");
    // // ownership::ownership_test();
    //
    // println!("\n Closures test.. \n");
    // closures::closures_test();
    //
    // hof::hof_test();
    //
    // println!("\n .. Traits test .. ");
    // traits::traits_test();
    //
    // println!("\n .. Traits extension .. ");
    // trait_extend_types::trait_extend_types_test();
    //
    // println!("\n .. Converstion between types .. ");
    // conversion_from_into::conversion_from_into_test();
    //
    // println!("\n .. operator_overloading test");
    // operator_overloading::operator_overloading_test();
    //
    // println!("\n .. static and dynamic dispatch");
    // dispatch::dispatch_test();
    //
    // println!("\n .. vectors with enum objects ..");
    // vectors_different_objects::vectors_enum_objects_test();
    //
    // println!("\n Lifetimes -> ");
    // lifetime::lifetime_test();
    //
    // println!("\n Smart pointers test ");
    // smart_pointers::smart_pointers_test();

    // println!("\n .. Too many lists ... ");
    // too_many_lists::too_many_lists_test();

    arc::shopping_arc_test();

    messages_between_threads::thread_messages_test();
}
