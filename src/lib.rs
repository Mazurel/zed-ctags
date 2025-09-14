use zed_extension_api as zed;

mod ctags_lsp;

struct CtagsExtension {}

impl zed::Extension for CtagsExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        ctags_lsp::download_ctags_lsp_binary().map_err(|msg| {
            let err_msg = format!("Error downloading clang lsp: {}", msg);
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Failed(err_msg.clone()),
            );
            err_msg
        })?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(zed::Command {
            command: ctags_lsp::get_ctags_lsp_binary_path(),
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(CtagsExtension);
