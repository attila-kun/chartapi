use chrono::{Date, Duration, NaiveDate, Utc};
use dto::{HLOC};
use plotters::prelude::*;

pub fn create_chart(symbol: &str, points: Vec<HLOC>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let start = std::time::Instant::now();

    let font = ("sans-serif", 50.0).into_font();
    let width: u32 = 500;
    let height: u32 = 500;
    let buffer_size = width as usize * height as usize * 3;
    let mut buffer: Vec<u8> = vec![0; buffer_size as usize];

    {
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        root.fill(&WHITE)?;

        let (from_date, to_date) = (
            convert_date(points[0].date) - Duration::days(1),
            convert_date(points.last().unwrap().date) + Duration::days(1),
        );

        let mut min: f32 = points[0].open;
        let mut max: f32 = points[0].open;
        points.iter().for_each(|p| {
            min = min.min(p.high);
            min = min.min(p.low);
            min = min.min(p.open);
            min = min.min(p.close);

            max = max.max(p.high);
            max = max.max(p.low);
            max = max.max(p.open);
            max = max.max(p.close);
        });

        let diff = max - min;
        let padding = 0.05f32;
        min -= diff * padding;
        max += diff * padding;

        let mut chart = ChartBuilder::on(&root);
        chart.x_label_area_size(40);
        chart.right_y_label_area_size(40);

        chart.caption(symbol, font);

        let mut built_chart = chart.build_cartesian_2d(from_date..to_date, min..max)?;

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

        built_chart.draw_series(points.into_iter().map(|hloc: HLOC| {
            CandleStick::new(
                convert_date(hloc.date),
                hloc.open,
                hloc.high,
                hloc.low,
                hloc.close,
                &plotters::prelude::GREEN,
                &plotters::prelude::RED,
                6,
                true
            )
        })).unwrap();
    }

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    Ok(buffer)
}

fn convert_date(naive_date: NaiveDate) -> Date<Utc> {
    Date::<Utc>::from_utc(naive_date, Utc)
}

#[test]
fn write_chart_to_buffer() {
    let mut points: Vec<HLOC> = Vec::new();
    points.push(HLOC{
        high: 10f32,
        low: 2f32,
        open: 12f32,
        close: -2.5f32,
        date: NaiveDate::from_ymd(2021, 1, 2)
    });
    let result = create_chart("tsla", points.to_vec());
    assert!(result.is_ok());
}