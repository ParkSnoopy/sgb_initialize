pub const POST_IDLE: u64 = 3;

pub const PARALLEL: u16 = 6;
pub const DL_LEVEL: u16 = 1;

pub const DL_1: &[(&str, &str)] = &[
	("https://github.com/ParkSnoopy/sgb_unlimiter/releases/download/v1.1.7/x86_64-pc-windows-msvc.zip", "SGB unlimiter v1.1.7.zip"), // SGB Unlimiter (v1.1.7)
	("https://kr.bandisoft.com/bandizip/dl.php?web", "Bandizip Setup.exe"), // BANDIZIP (latest)
	("https://pkgs.tailscale.com/stable/tailscale-setup-latest.exe", "Tailscale Setup.exe"), // TAILSCALE (latest)
	("https://github.com/naver/d2codingfont/releases/download/VER1.3.2/D2Coding-Ver1.3.2-20180524.zip", "D2Coding v1.3.2.zip"), // FONT (v1.3.2)
	("https://msdl.microsoft.com/download/symbols/cmd.exe/A7CB0DC067000/cmd.exe", "build20240829.blob.exe"), // CMD (build_20240829)
	("https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe", "Rustup Setup x64.exe"), // RUSTUP_INIT x64 (latest)
];

pub const DL_2: &[(&str, &str)] = &[
	("https://the.earth.li/~sgtatham/putty/latest/w64/putty-64bit-0.81-installer.msi", "Putty v0.81 Setup.msi"), // PUTTY (v0.81)
	("https://jaist.dl.sourceforge.net/project/winscp/WinSCP/6.3.4/WinSCP-6.3.4-Setup.exe?viasf=1", "WinSCP v6.2.4 Setup.exe"), // WinSCP (v6.3.4)
	("https://nchc.dl.sourceforge.net/project/eraser/Eraser%206/6.2/Eraser%206.2.0.2993.exe?viasf=1", "Eraser v6.2 Setup.exe"), // Eraser (v6.2.0.2993)
];

pub const DL_3: &[(&str, &str)] = &[
	("https://www.python.org/ftp/python/3.11.9/python-3.11.9-amd64.exe", "Python v3.11.9 x64.msi"), // Python (v3.11.9)
	("https://download.revouninstaller.com/download/RevoUninstaller_Portable.zip", "RevoUnins.zip"), // RevoUninstaller Portable (latest)
];
