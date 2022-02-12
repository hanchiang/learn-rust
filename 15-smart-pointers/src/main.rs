mod r#box;
mod custom_box;
mod reference_counting;
mod refcell;
mod reference_cycle;

fn main() {
    r#box::main();
    custom_box::main();
    reference_counting::main();
    refcell::main();
    reference_cycle::main();
}
