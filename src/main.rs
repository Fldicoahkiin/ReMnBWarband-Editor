use anyhow::Result;
use remnb_warband_editor::App;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let app = App::new()?;
    app.run()
}

