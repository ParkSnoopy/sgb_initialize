use eyre::Result;


fn main() -> Result<()> {
    #[cfg(windows)] // Build host machine have to be WindowsOS in order to compile with manifest/icon
    {
        use winres;

        #[allow(unused_mut)]
        let mut res = winres::WindowsResource::new();

        // res.set_icon("assets/icon.ico");

        res.compile()?;
    }

    Ok(())
}
