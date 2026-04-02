// ===========================================

pub fn is_near_account_fun(account_id: &str) -> bool {
    // Must end with .near but not be an implicit account
    // Implicit accounts are 64 character hex strings (no dots)
    // .tg accounts end with .tg
    // Sub accounts like test.someaccount.near are valid
    
    if account_id.ends_with(".tg") {
        return false;
    }
    
    // Implicit accounts are 64 char hex strings with no dots
    if account_id.len() == 64 && account_id.chars().all(|c| c.is_ascii_hexdigit()) && !account_id.contains('.') {
        return false;
    }
    
    // Must end with .near
    account_id.ends_with(".near")
}
// ===========================================

pub fn get_account_type_fun(account_id: &str) -> &'static str {
    if account_id.ends_with(".tg") {
        "tg"
    } else if account_id.len() == 64 && account_id.chars().all(|c| c.is_ascii_hexdigit()) && !account_id.contains('.') {
        "implicit"
    } else if account_id.ends_with(".near") {
        "near"
    } else if account_id.contains('.') {
        "subaccount"
    } else {
        "unknown"
    }
}
// ===========================================
