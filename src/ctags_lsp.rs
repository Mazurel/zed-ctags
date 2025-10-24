use zed_extension_api as zed;

type ErrorMessage = String;

const CTAGS_LSP_FOLDER_NAME: &str = "ctags-lsp-project";
const CTAGS_LSP_BINARY_NAME: &str = "ctags-lsp";

struct CtagsLspBinary {
    url: String,
    file_type: zed::DownloadedFileType,
}

fn get_ctags_lsp_binary_url() -> Result<CtagsLspBinary, ErrorMessage> {
    let (tarball, file_type) = match zed::current_platform() {
        (zed::Os::Linux, zed::Architecture::X8664) => (
            "ctags-lsp_Linux_x86_64.tar.gz",
            zed::DownloadedFileType::GzipTar,
        ),
        (zed::Os::Linux, zed::Architecture::Aarch64) => (
            "ctags-lsp_Linux_arm64.tar.gz",
            zed::DownloadedFileType::GzipTar,
        ),
        (zed::Os::Mac, zed::Architecture::X8664) => (
            "ctags-lsp_Darwin_x86_64.tar.gz",
            zed::DownloadedFileType::GzipTar,
        ),
        (zed::Os::Mac, zed::Architecture::Aarch64) => (
            "ctags-lsp_Darwin_arm64.tar.gz",
            zed::DownloadedFileType::GzipTar,
        ),
        (zed::Os::Windows, zed::Architecture::X8664) => {
            ("ctags-lsp_Windows_x86_64.zip", zed::DownloadedFileType::Zip)
        }
        (zed::Os::Windows, zed::Architecture::Aarch64) => {
            ("ctags-lsp_Windows_arm64.zip", zed::DownloadedFileType::Zip)
        }
        (os, arch) => Err(format!("Unsupported platform: {:?} {:?}", os, arch).to_string())?,
    };
    let url = format!(
        "https://github.com/netmute/ctags-lsp/releases/download/v0.8.1/{}",
        tarball
    );
    println!("Downloading ctags lsp from {}", url);

    Ok(CtagsLspBinary {
        url: url.to_string(),
        file_type: file_type,
    })
}

pub fn get_ctags_lsp_binary_path() -> String {
    format!("{}/{}", CTAGS_LSP_FOLDER_NAME, CTAGS_LSP_BINARY_NAME)
}

pub fn download_ctags_lsp_binary() -> Result<(), ErrorMessage> {
    let lsp_binary = get_ctags_lsp_binary_url()?;
    zed::download_file(&lsp_binary.url, CTAGS_LSP_FOLDER_NAME, lsp_binary.file_type)
        .map_err(|msg| format!("Error downloading ctags lsp: {}", msg))?;

    let binary_path = &get_ctags_lsp_binary_path();
    zed::make_file_executable(&binary_path)
        .map_err(|msg| format!("Error making ctags lsp executable: {}", msg))?;

    Ok(())
}
