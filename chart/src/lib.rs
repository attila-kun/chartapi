use chrono::{Date, DateTime, Duration, Timelike, Utc};
use plotters::prelude::*;

pub fn create_chart() -> Result<(), Box<dyn std::error::Error>> {

    let start = std::time::Instant::now();

    let font = ("sans-serif", 50.0).into_font();
    let root = BitMapBackend::new("target/stock.png", (500, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let data = get_data();
    let (from_date, to_date) = (
        data[0].0 - Duration::days(1),
        data.last().unwrap().0 + Duration::days(1),
    );

    let mut chart = ChartBuilder::on(&root);
    chart.x_label_area_size(40);
    chart.right_y_label_area_size(40);

    chart.caption("MSFT Stock Price", font);

    let mut built_chart = chart.build_cartesian_2d(from_date..to_date, 130f32..200f32)?;

    let mut configure = built_chart.configure_mesh();

    let x_label_formatter = &(|&d: &Date<Utc>| {
        if from_date == d { return String::from("") }
        return format!("{}", d.format("%F"));
    });

    configure.x_label_formatter(x_label_formatter);
    configure.y_label_formatter(&|val| {
        return format!("{}", val);
    });
    configure.light_line_style(&WHITE).draw()?;

    built_chart.draw_series(
        data.iter()
            .map(|x| CandleStick::new(x.0, x.1, x.2, x.3, x.4, &GREEN, &RED, 6, true)),
    )?;

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    Ok(())
}

fn get_data() -> Vec<(Date<Utc>, f32, f32, f32, f32)> {

    let mut vec = Vec::new();
    let now = Utc::now().date();
    let seed = (Utc::now().timestamp_millis() % 10) as f32;

    for i in 1..30 {
        vec.push((now + Duration::days(i), 130.0600 + seed + i as f32 + f32::sin(i as f32)*10.0, 131.3700 + i as f32, 128.8300 + i as f32, 129.1500 + i as f32));
    }

    return vec;
}

#[test]
fn create_chart_test() {
  assert_eq!(create_chart().unwrap(), ());
}