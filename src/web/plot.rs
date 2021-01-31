use gnuplot::{Figure, Caption, Color, AxesCommon, AutoOption};
use std::convert::{TryFrom};
//use rocket::http::ext::IntoCollection;
use std::collections::VecDeque;
//use gnuplot::MarginSide::{MarginLeft, MarginRight, MarginBottom, MarginTop};
use gnuplot::PlotOption::FillAlpha;
use gnuplot::Coordinate::Graph;
//use gnuplot::PlotOption::{PointSize, FillPattern, LineStyle};
use gnuplot::LegendOption::Reverse;
use gnuplot::LabelOption::{Font, TextColor, TextAlign};
use gnuplot::AlignType::{AlignLeft};


//use std::collections::VecDeque;

pub fn rysuj_cpu(width: u32, height: u32, dane: &Vec<u64>, tytul: &str, plik: &str)
{
    let mut max_y = 100;
    let kolor = String::from("#0f0431");
    let najwieksza = dane.iter().max().unwrap();
    if *najwieksza > 100_u64 {
        max_y = *najwieksza + 10;
        //kolor = String::from("red");
    }

    //tworzymy długi iter dla osi poziomej
    let ilosc = u64::try_from(dane.capacity()).ok().unwrap();
    //ilosc += 1;
    let mut pozioma: VecDeque<u64> = VecDeque::new();
    let mut pozioma_0: Vec<u64> = Vec::new();
    for a in 0..ilosc {
        pozioma.push_front(a);
        pozioma_0.push(0);
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title(tytul, &[Font("",12_f64), TextColor("#460000")])
        .set_x_label("Time (s)", &[Font("",10_f64), TextAlign(AlignLeft)])
        .set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(max_y as f64))
        .set_x_range(AutoOption::Fix((ilosc - 1) as f64), AutoOption::Fix(0.0))
        .set_x_reverse(true)
        .set_y_grid(true)
        .set_legend(
            Graph(0.95),
            Graph(0.9),
            &[Reverse],
            &[]
        )
        /*.boxes(
            &[90, 80, 70, 60, 50, 40, 30, 20, 10, 0],
            &[0,0,0,0,0,0,0,0,0,0],
            &[Color("#ff1010"), FillAlpha(1_f64)],)
        */
        .fill_between(
            &pozioma,//[-9, -8, -7, -6, -5, -4, -3, -2, -1, 0],
            &pozioma_0,
            dane.iter(),
            &[Color(kolor.as_str()), FillAlpha(1_f64)],
        )
        .lines(
            &pozioma, //&[-9, -8, -7, -6, -5, -4, -3, -2, -1, 0],
            dane.iter(),
            &[Color("black"), Caption(format!("{} %",dane.last().unwrap()).as_str())],
        );

    fg.save_to_svg(plik, width, height).unwrap();
}

pub fn rysuj_ram(width: u32, height: u32, dane: &Vec<u32>, tytul: &str, plik: &str)
{
    let max_y;// = 4000_u32;
    let kolor = String::from("green");
    let najwieksza = dane.iter().max().unwrap();
    //if *najwieksza > 4000_u32 {
    max_y = *najwieksza + 1000;
    //kolor = String::from("red");
    //}

    //tworzymy długi iter dla osi poziomej
    let ilosc = u64::try_from(dane.capacity()).ok().unwrap();
    //ilosc += 1;
    let mut pozioma: VecDeque<u64> = VecDeque::new();
    let mut pozioma_0: Vec<u64> = Vec::new();
    for a in 0..ilosc {
        pozioma.push_front(a);
        pozioma_0.push(0);
    }

    let mut fg = Figure::new();
    fg.axes2d()

        .set_title(tytul, &[Font("",12_f64), TextColor("#460000")])
        .set_x_label("Time (s)", &[Font("",10_f64)])

        .set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(max_y as f64))
        .set_x_range(AutoOption::Fix((ilosc - 1) as f64), AutoOption::Fix(0.0))
        .set_x_reverse(true)
        .set_y_grid(true)
        .set_legend(
            Graph(0.95),
            Graph(0.9),
            &[Reverse],
            &[]
        )
        .fill_between(
            &pozioma,//[-9, -8, -7, -6, -5, -4, -3, -2, -1, 0],
            &pozioma_0,
            dane.iter(),
            &[Color(kolor.as_str())],
        )
        .lines(
            &pozioma, //&[-9, -8, -7, -6, -5, -4, -3, -2, -1, 0],
            dane.iter(),
            &[Color("black"), Caption(format!("{} MB",dane.last().unwrap()).as_str())],
        );

    fg.save_to_svg(plik, width, height).unwrap();
}