//TODO Get rid of this?

pub fn error(result: std::io::Result<()>) -> Result<(), String> {
    match result {
        Ok(r) => Ok(r),
        Err(r) => Err(r.to_string()),
    }
}

pub fn error_u64(result: std::io::Result<u64>) -> Result<u64, String> {
    match result {
        Ok(r) => Ok(r),
        Err(r) => Err(r.to_string()),
    }
}

pub fn error_usize(result: std::io::Result<usize>) -> Result<usize, String> {
    match result {
        Ok(r) => Ok(r),
        Err(r) => Err(r.to_string()),
    }
}
