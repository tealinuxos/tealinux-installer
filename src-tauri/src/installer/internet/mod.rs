use online::check;

pub fn is_online() -> bool
{
    check(None).is_ok()
}
