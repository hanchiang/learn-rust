// The #[macro_export] annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scop
// The body is similar to the structure of a match expression
// Here we have one arm with the pattern ( $( $x:expr ),* ), followed by => and the block of code associated with this pattern.
// First, a set of parentheses encompasses the whole pattern. A dollar sign ($) is next,
// followed by a set of parentheses that captures values that match the pattern within the parentheses for use in the replacement code.
// Within $() is $x:expr, which matches any Rust expression and gives the expression the name $x.
// The comma following $() indicates that a literal comma separator character could optionally appear
// after the code that matches the code in $(). The * specifies that the pattern matches zero or more of whatever precedes the *.
// In the body, temp_vec.push() within $()* is generated for each part that matches $() in the pattern
// zero or more times depending on how many times the pattern matches.
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }
}