use procyon::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chart = Procyon::chart(
            "https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/data/clustered_data.csv"
        )
        .mark_point()
        .encode_axis("x", "y").encode_color("cluster")
        .build();

    eprintln!("{:?}", chart);
    chart.show().unwrap();

    Ok(())
}
