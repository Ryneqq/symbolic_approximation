use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::LineStyle;
use itertools::Itertools;
mod var;
mod par;
mod fun;
mod gen;
mod gen_code;

pub use var::Var;
pub use par::Par;
pub use fun::Fun;
pub use gen::Gen;
pub use gen_code::GenCode;

fn main() {
    use self::Fun::*;
    use self::Par::*;
    use self::Var::*;




    // let s1: Plot = Plot::new(org)
    //     .line_style(LineStyle::new().colour("#0000ff"));
    // let s2: Plot = Plot::new(integral)
    //     .line_style(LineStyle::new().colour("#ff0000"));
    // let s3: Plot = Plot::new(integral_ideal)
    //     .line_style(LineStyle::new().colour("#00ff00"));
    // let v = ContinuousView::new()
    //     .add(s1)
    //     .add(s2)
    //     .add(s3)
    //     .x_range(-15., 15.);

    // Page::single(&v).save("integrals.svg").unwrap();
}
