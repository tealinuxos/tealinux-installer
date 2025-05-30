use tea_arch_chroot_lib::chroot::Account;

#[tauri::command]
pub async fn blueprint_set_account(fullname: String, username: String, hostname: String, password: String, autologin: bool)
{
    let account = Account::new(&fullname, &username, &hostname, &password, autologin);

    let mut blueprint = super::get_blueprint().unwrap();

    blueprint.account = Some(account);

    super::write_blueprint(blueprint).unwrap();
}
