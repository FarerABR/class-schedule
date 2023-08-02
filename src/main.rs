use plotters::prelude::*;
use std::{cmp, io};

fn main() {
    println!("pleaese enter the number of classes: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading the input");

    let n = input
        .trim()
        .parse::<u32>()
        .expect("the input is not in the correct format");

    let mut string = vec![String::new(); n as usize];
    for i in 0..n as usize {
        println!("enter the hours number {}: ", i + 1);
        io::stdin().read_line(&mut string[i]).unwrap();
    }
    let classes: Vec<(u32, u32)> = string
        .iter()
        .map(|item| -> (u32, u32) {
            let mut iter = item.split("..");
            return (
                iter.next().unwrap().trim().parse::<u32>().unwrap(),
                iter.next().unwrap().trim().parse::<u32>().unwrap(),
            );
        })
        .collect();

    println!("{:?}", min_class(classes));
}

fn min_class(vec: Vec<(u32, u32)>) -> usize {
    let mut times = Vec::new();
    for i in 0..vec.len() {
        times.push((vec[i].0, 1));
        times.push((vec[i].1, -1));
    }
    times.sort_by_key(|item| item.0);

    let mut high = 1;
    let mut tmp = 0;
    let mut hourly: Vec<(u32, i32)> = Vec::new();
    for i in 0..times.len() {
        tmp += times[i].1;
        high = cmp::max(high, tmp);
        if let Some(x) = hourly.iter().position(|item| item.0 == times[i].0) {
            hourly[x].1 = cmp::max(hourly[x].1, tmp)
        } else {
            hourly.push((times[i].0, tmp));
        }
    }
    draw(hourly, high as usize);
    high as usize
}
fn draw(vec: Vec<(u32, i32)>, max: usize) {
    let drawing_area = BitMapBackend::new("./figure.png", (1920, 1080)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("mininmum class in each hour", ("calibri", 40))
        .build_cartesian_2d(
            (vec[0].0 - 1) as u32..(vec[vec.len() - 1].0 + 1) as u32,
            0 as i32..(max + 1) as i32,
        )
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            vec.iter()
                .map(|&point| Circle::new(point, 10, Into::<ShapeStyle>::into(&RED).filled())),
        )
        .unwrap();
}
