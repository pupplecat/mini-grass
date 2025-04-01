#[macro_export]
macro_rules! validate {
    ($assert:expr, $err:expr) => {{
        if ($assert) { // Validation succeeds when condition is true
            Ok(())
        } else {
            let error_code: ErrorCode = $err;
            msg!("Error {} thrown at {}:{}", error_code as u32, file!(), line!()); // Log the error code and location
            Err(error_code) // Return the error
        }
    }};
    ($assert:expr, $err:expr, $($arg:tt)+) => {{
        if ($assert) { // Validation succeeds when condition is true
            Ok(())
        } else {
            let error_code: ErrorCode = $err;
            msg!("Error {} thrown at {}:{}", error_code as u32, file!(), line!());
            msg!($($arg)*); // Log custom error message
            Err(error_code)
        }
    }};
}
