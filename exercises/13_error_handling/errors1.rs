// TODO: This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was instead of just returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Change
// the function signature and body to return `Result<String, String>` instead
// of `Option<String>`.
/*!SECTION
 * // Construction / basic logic
Result::and
Result::and_then
Result::or
Result::or_else

// Status checks
Result::is_ok
Result::is_ok_and
Result::is_err
Result::is_err_and

// Conversions
Result::ok              // Result<T, E> -> Option<T>
Result::err             // Result<T, E> -> Option<E>
Result::transpose       // Result<Option<T>, E> -> Option<Result<T, E>>

// Borrowing / deref
Result::as_ref          // &Result<T, E> -> Result<&T, &E>
Result::as_mut          // &mut Result<T, E> -> Result<&mut T, &mut E>
Result::as_deref        // Result<T, E> where T: Deref -> Result<&T::Target, E>
Result::as_deref_mut    // Result<T, E> where T: DerefMut -> Result<&mut T::Target, E>

// Mapping / transforming
Result::map
Result::map_err
Result::map_or
Result::map_or_else
// (there is also an unstable map_or_default on nightly)

// Iteration-style helpers
Result::iter            // &Result<T, E> -> impl Iterator<Item = &T>
Result::iter_mut        // &mut Result<T, E> -> impl Iterator<Item = &mut T>
Result::cloned          // clones inner T when T: Clone
Result::copied          // copies inner T when T: Copy

// Debug-style side-effect hooks
Result::inspect
Result::inspect_err

// Consuming conversions
Result::into_ok
Result::into_err

// Unwrapping (panic on error, or on ok for *_err)
Result::unwrap
Result::expect
Result::unwrap_or
Result::unwrap_or_default
Result::unwrap_or_else
Result::unwrap_err
Result::expect_err

// Unsafe (nightly-only)
Result::unwrap_unchecked
Result::unwrap_err_unchecked
 */
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
