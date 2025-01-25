
use polars_core::prelude::*;
use polars_io::prelude::*;
use plotters::prelude::*;
//https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html

fn load_wine_csv() -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some("wine.csv".into()))?
            .finish()
}

fn grater_than_given_value(df: &DataFrame,value: f64,column_name: &str) -> PolarsResult<DataFrame> {
    let values = df.column(column_name)?.as_materialized_series();
    let mask = values.gt(value)?;
    df.filter(&mask)
}

fn less_than_given_value(df: &DataFrame,value: f64,column_name:&str) -> PolarsResult<DataFrame> {
    let values = df.column(column_name)?.as_materialized_series();
    let mask = values.lt(value)?;
    df.filter(&mask)
}
fn in_range_of_values(df: &DataFrame,lower_bound: f64,upper_bound: f64,column_name_1:&str, column_name_2: &str) -> PolarsResult<DataFrame> {
    let values_1 = df.column(column_name_1)?.as_materialized_series();
    let values_2 = df.column(column_name_2)?.as_materialized_series();
    let mask = values_1.gt(upper_bound)? & values_2.lt(lower_bound)?;
    df.filter(&mask)
}

fn sort_by_alcohol(df: &DataFrame) -> PolarsResult<DataFrame> {
    df.sort(["alcohol"], Default::default())
}
fn sort_with_specific_order(df: &DataFrame, descending: bool) -> PolarsResult<DataFrame> {
    df.sort(
        ["alcohol"],
        SortMultipleOptions::new()
            .with_order_descending(descending)
    )
}

fn get_min_max_mean_median(df: &DataFrame,column_name: &str) -> (f64,f64,f64,f64) {
    let values = df.column(column_name).unwrap().f64().unwrap();
    let min = values.min();
    let max = values.max();
    let mean = values.mean();
    let median = values.median();
    (min.unwrap(),max.unwrap(),mean.unwrap(),median.unwrap())
}

fn plot_scatter(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let color_intensity = df.column("color_intensity")?.f64()?;
    let alcohol_content = df.column("alcohol")?.f64()?;
    
    let points: Vec<(f64, f64)> = color_intensity
        .into_iter()
        .zip(alcohol_content.into_iter())
        .filter_map(|(ci, alc)| Some((ci?, alc?))) // Filtrowanie None
        .collect();

    let root = BitMapBackend::new("scatter_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Zależność między kolorem a alkoholem", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..15.0, 0.0..15.0)?;

    chart.configure_mesh()
        .x_desc("Intensywność koloru")
        .y_desc("Zawartość alkoholu")
        .draw()?;

    chart.draw_series(
        points.into_iter().map(|(x, y)| Circle::new((x, y), 5, ShapeStyle::from(&BLUE).filled()))
    )?;

    Ok(())
}





fn main() {
    let df1 = load_wine_csv().unwrap();
    println!("{}", df1.head(Some(5)));
    println!("{}", df1.tail(Some(3)));
    println!("{:?}", df1.get_column_names());
    //chce znaleć tylko te wina, które maja kolor intensywniejszy ni 12
    println!("{}", grater_than_given_value(&df1,12.0,"color_intensity").unwrap().head(Some(2)));
    //magnez ponizej 100
    println!("{}", less_than_given_value(&df1,100.0,"magnesium").unwrap().head(Some(100)));
    //alkohol < 12 i magnes > 100
    println!("{}", in_range_of_values(&df1,12.0,100.0,"magnesium","alcohol").unwrap().head(Some(100)));

    //sortowanie rosnące po alkoholu
    println!("{:?}", sort_by_alcohol(&df1).unwrap());

    //sortowanie malejące po alkoholu
    println!("{:?}", sort_with_specific_order(&df1,true).unwrap());
    //alkohol min max itd
    println!("{:?}", get_min_max_mean_median(&df1,"alcohol"));

    plot_scatter(&df1).unwrap();
   
}
