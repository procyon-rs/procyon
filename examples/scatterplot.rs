use procyon::*;
use showata::Showable;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chart = Procyon::chart(
            "https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/data/clustered_data.csv"
        )
        .mark_point()
        .encode_axis("x", "y").encode_color("cluster")
        .save(Path::new("/Users/aubrythomas/src/personal-git/procyon-rs/procyon/image.png")).await?;

    //chart.show().unwrap();

    Ok(())
}
