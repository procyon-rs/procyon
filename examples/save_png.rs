use procyon::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plot_save_path = Path::new("examples/output/image.png");
    let chart = Procyon::chart(
            "https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/data/clustered_data.csv"
        )
        .mark_point()
        .encode_axis("x", "y").encode_color("cluster")
        .save(plot_save_path).await?;
    eprintln!("{:?}", chart);
    Ok(())
}
