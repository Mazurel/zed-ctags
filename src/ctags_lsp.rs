use zed_extension_api as zed;

type ErrorMessage = String;

const CTAGS_LSP_FOLDER_NAME: &str = "ctags-lsp-project";
const CTAGS_LSP_BINARY_NAME: &str = "ctags-lsp";

fn get_ctags_lsp_binary_url() -> Result<String, ErrorMessage> {
    let tarball = match zed::current_platform() {
        (zed::Os::Linux, zed::Architecture::X8664) => "ctags-lsp_Linux_x86_64.tar.gz",
        (zed::Os::Linux, zed::Architecture::Aarch64) => "ctags-lsp_Linux_arm64.tar.gz",
        (zed::Os::Mac, zed::Architecture::X8664) => "ctags-lsp_Darwin_x86_64.tar.gz",
        (zed::Os::Mac, zed::Architecture::Aarch64) => "ctags-lsp_Darwin_arm64.tar.gz",
        (zed::Os::Windows, zed::Architecture::X8664) => "ctags-lsp_Windows_x86_64.tar.gz",
        (zed::Os::Windows, zed::Architecture::Aarch64) => "ctags-lsp_Windows_arm64.tar.gz",
        (os, arch) => Err(format!("Unsupported platform: {:?} {:?}", os, arch).to_string())?,
    };
    let url = format!(
        "https://github.com/netmute/ctags-lsp/releases/download/v0.8.1/{}",
        tarball
    );
    println!("Downloading ctags lsp from {}", url);

    Ok(url.to_string())
}

pub fn get_ctags_lsp_binary_path() -> String {
    format!("{}/{}", CTAGS_LSP_FOLDER_NAME, CTAGS_LSP_BINARY_NAME)
}

pub fn download_ctags_lsp_binary() -> Result<(), ErrorMessage> {
    let url = get_ctags_lsp_binary_url()?;
    zed::download_file(
        &url,
        CTAGS_LSP_FOLDER_NAME,
        zed::DownloadedFileType::GzipTar,
    )
    .map_err(|msg| format!("Error downloading ctags lsp: {}", msg))?;

    let binary_path = &get_ctags_lsp_binary_path();
    zed::make_file_executable(&binary_path)
        .map_err(|msg| format!("Error making ctags lsp executable: {}", msg))?;

    Ok(())
}
