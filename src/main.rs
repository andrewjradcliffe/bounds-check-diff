use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

#[inline(never)]
fn diff_unsafe_push(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let m = n - 1;
    let mut dx: Vec<f64> = Vec::with_capacity(m);

    for i in 0..m {
        // We know this is in bounds given that the length is n, and
        // (n - 2 + 1) = n - 1 is the last offset accessed.
        dx.push(unsafe { *x.get_unchecked(i + 1) - *x.get_unchecked(i) });
    }

    dx
}

#[inline(never)]
fn diff_unsafe(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let m = n - 1;
    let mut dx: Vec<f64> = Vec::with_capacity(m);
    dx.resize(m, 0.0);

    for i in 0..m {
        // We know this is in bounds given that the length is n, and
        // (n - 2 + 1) = n - 1 is the last offset accessed.
        dx[i] = unsafe { *x.get_unchecked(i + 1) - *x.get_unchecked(i) };
    }

    dx
}

#[inline(never)]
fn diff_windows(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let m = n - 1;
    let mut dx: Vec<f64> = Vec::with_capacity(m);
    dx.resize(m, 0.0);

    for (i, w) in x.windows(2).enumerate() {
        dx[i] = w[1] - w[0];
    }
    dx
}

#[inline(never)]
fn diff_windows_zip(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let m = n - 1;
    let mut dx: Vec<f64> = Vec::with_capacity(m);
    dx.resize(m, 0.0);

    for (w, dx_i) in x.windows(2).zip(dx.iter_mut()) {
        *dx_i = w[1] - w[0];
    }
    dx
}

#[inline(never)]
fn diff_windows_zip_for_each(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let m = n - 1;
    let mut dx: Vec<f64> = Vec::with_capacity(m);
    dx.resize(m, 0.0);

    x.windows(2)
        .zip(dx.iter_mut())
        .for_each(|(w, dx_i)| *dx_i = w[1] - w[0]);

    dx
}

#[inline(never)]
fn diff_windows_zip_for_each_macro(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let m = n - 1;
    let mut dx: Vec<f64> = vec![0.0; m];

    x.windows(2)
        .zip(dx.iter_mut())
        .for_each(|(w, dx_i)| *dx_i = w[1] - w[0]);

    dx
}

#[inline(never)]
fn diff_windows_collect(x: &[f64]) -> Vec<f64> {
    x.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn main() {
    let mut args = std::env::args();
    let n: usize = match args.nth(1) {
        Some(s) => s.parse::<usize>().unwrap_or(1000),
        _ => 1000,
    };
    let version: usize = match args.nth(0) {
        Some(s) => s.parse::<usize>().unwrap_or(0),
        _ => 0,
    };

    let mut rng = Xoshiro256PlusPlus::from_rng(rand::thread_rng()).unwrap();

    let mut x: Vec<f64> = vec![0.0; n];
    rng.fill(&mut x[..]);

    println!("{:#?}", version);
    if version == 0 {
        for _ in 0..100 {
            println!("{:#?}", diff_unsafe_push(&x).into_iter().sum::<f64>());
        }
    } else if version == 1 {
        for _ in 0..100 {
            println!("{:#?}", diff_unsafe(&x).into_iter().sum::<f64>());
        }
    } else if version == 2 {
        for _ in 0..100 {
            println!("{:#?}", diff_windows(&x).into_iter().sum::<f64>());
        }
    } else if version == 3 {
        for _ in 0..100 {
            println!("{:#?}", diff_windows_zip(&x).into_iter().sum::<f64>());
        }
    } else if version == 4 {
        for _ in 0..100 {
            println!(
                "{:#?}",
                diff_windows_zip_for_each(&x).into_iter().sum::<f64>()
            );
        }
    } else if version == 5 {
        for _ in 0..100 {
            println!(
                "{:#?}",
                diff_windows_zip_for_each_macro(&x).into_iter().sum::<f64>()
            );
        }
    } else {
        for _ in 0..100 {
            println!("{:#?}", diff_windows_collect(&x).into_iter().sum::<f64>());
        }
    }
}
