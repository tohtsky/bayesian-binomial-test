use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use rand::{prelude::Distribution, SeedableRng};
use wasm_bindgen::prelude::*;

use rand_pcg::Pcg64;
use serde_json::Value;
use statrs::distribution::Beta;
use web_sys::HtmlCanvasElement;

fn _take_f64(v: &Value, key: &str) -> Result<f64, JsValue> {
    let target = &v[key];
    match target.as_f64() {
        Some(value) => Ok(value),
        None => Err(JsValue::from_str(key)),
    }
}

fn _take_f64_arr(v: &Value, key: &str) -> Result<Vec<f64>, JsValue> {
    let target = &v[key];
    let mut result = Vec::new();
    let arr = match target.as_array() {
        None => Err(JsValue::from_str(
            format!("key {} cannot be interpreted as f64 arr.", key).as_str(),
        )),
        Some(v) => Ok(v.clone()),
    }?;
    for x in arr.iter() {
        let v = match x.as_f64() {
            Some(v) => Ok(v),
            None => Err(JsValue::from_str(
                format!("key {} cannot be interpreted as f64 arr.", key).as_str(),
            )),
        }?;
        result.push(v);
    }
    Ok(result)
}

fn samples_to_hist(sample_vecs: &Vec<f64>, min: f64, n_bins: usize, delta: f64) -> (Vec<u32>, i64) {
    let indices: Vec<_> = (sample_vecs)
        .iter()
        .map(|x: &f64| ((*x - min) / delta).floor().min((n_bins - 1) as f64) as usize)
        .collect();
    let mut counts = vec![0 as u32; n_bins];
    for index in indices.iter() {
        counts[*index] = counts[*index] + 1;
    }
    let mut cnt_max = 0 as u32;
    for cnt in counts.iter() {
        cnt_max = cnt_max.max(*cnt);
    }
    (counts, cnt_max as i64)
}

#[wasm_bindgen]
pub fn compute(
    target: &str,
    dpr: f64,
    canvas_a_b: HtmlCanvasElement,
    canvas_diff: HtmlCanvasElement,
) -> Result<f64, JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let a_color = RGBColor(31, 119, 180).mix(0.7);
    let b_color = RGBColor(255, 127, 14).mix(0.7);
    let dpr_i = dpr as i32;

    use serde_json;
    let mut rng = Pcg64::seed_from_u64(0 as u64);
    let parsed: Value = match serde_json::from_str(target) {
        Ok(v) => Ok(v),
        Err(_) => Err(JsValue::from_str("Invalid JSON!")),
    }?;
    let a_tot = _take_f64(&parsed, "a_tot")?;
    let a_pos = _take_f64(&parsed, "a_pos")?;
    if a_tot < a_pos {
        return Err(JsValue::from_str("a_pos cannot be greater than a_tot!"));
    }
    let b_tot = _take_f64(&parsed, "b_tot").unwrap();
    let b_pos = _take_f64(&parsed, "b_pos").unwrap();
    if b_tot < b_pos {
        return Err(JsValue::from_str("b_pos cannot be greater than b_tot!"));
    }
    let prior_pos = match _take_f64(&parsed, "prior_pos") {
        Ok(v) => v,
        Err(_) => 1.0,
    };
    let prior_neg = match _take_f64(&parsed, "prior_neg") {
        Ok(v) => v,
        Err(_) => 1.0,
    };

    let n_samples = match _take_f64(&parsed, "n_samples") {
        Ok(f) => f as i64,
        Err(_) => 1000,
    };
    if n_samples <= 0 {
        return Err(JsValue::from_str("n_samples must be strictly positive!"));
    }
    let n_bins = match _take_f64(&parsed, "n_bins") {
        Ok(f) => f as i64,
        Err(_) => 100,
    };
    if n_bins <= 0 {
        return Err(JsValue::from_str("n_bins must be strictly positive!"));
    }

    let percentiles = match _take_f64_arr(&parsed, "diff_percentiles") {
        Ok(v) => v,
        Err(_) => vec![
            0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 0.75, 0.9, 0.95, 0.975, 0.99,
        ],
    };

    let dist_a = Beta::new(prior_pos + a_pos, prior_neg + a_tot - a_pos).unwrap();
    let dist_b = Beta::new(prior_pos + b_pos, prior_neg + b_tot - b_pos).unwrap();
    let mut a_samples = Vec::new();
    let mut b_samples = Vec::new();
    let mut diff_samples = Vec::new();

    let mut min_value: Option<f64> = None;
    let mut max_value: Option<f64> = None;
    let mut min_diff: Option<f64> = None;
    let mut max_diff: Option<f64> = None;

    let mut b_win: i64 = 0;
    for _ in 0..n_samples {
        let a_sample = dist_a.sample(&mut rng);
        let b_sample = dist_b.sample(&mut rng);
        let diff = b_sample - a_sample;
        if a_sample < b_sample {
            b_win += 1;
        }
        min_value = match min_value {
            Some(v) => Some(v.min(a_sample).min(b_sample)),
            None => Some(a_sample.min(b_sample)),
        };
        max_value = match max_value {
            Some(v) => Some(v.max(a_sample).max(b_sample)),
            None => Some(a_sample.max(b_sample)),
        };
        min_diff = match min_diff {
            Some(v) => Some(v.min(diff)),
            None => Some(diff),
        };
        max_diff = match max_diff {
            Some(v) => Some(v.max(diff)),
            None => Some(diff),
        };
        a_samples.push(a_sample);
        b_samples.push(b_sample);
        diff_samples.push(diff);
    }
    let option_to_f64 = |x| match x {
        Some(x) => Ok(x),
        None => Err(JsValue::from_str("to few sample!")),
    };
    let max_value = option_to_f64(max_value)?;
    let min_value = option_to_f64(min_value)?;

    {
        let delta = (max_value - min_value) / (n_bins as f64);
        let backend = CanvasBackend::with_canvas_object(canvas_a_b).unwrap();
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();

        let (a_hist, a_cnt_max) = samples_to_hist(&a_samples, min_value, n_bins as usize, delta);
        let (b_hist, b_cnt_max) = samples_to_hist(&b_samples, min_value, n_bins as usize, delta);
        let cnt_max = a_cnt_max.max(b_cnt_max) as f64;

        let mut chart = ChartBuilder::on(&root)
            .set_label_area_size(LabelAreaPosition::Left, 80.0 * dpr)
            .set_label_area_size(LabelAreaPosition::Bottom, 40.0 * dpr)
            .caption("Posterior rate density", ("sans-serif", 15 * dpr_i))
            .build_cartesian_2d(
                0.98 * min_value..(1.02 * max_value),
                0 as u32..((cnt_max * 1.01) as u32),
            )
            .unwrap();
        chart
            .configure_mesh()
            .x_labels(10)
            .x_label_style(("sans-serif", 10 * dpr_i))
            .disable_y_mesh()
            .disable_y_axis()
            .draw()
            .unwrap();

        chart
            .draw_series(a_hist.iter().enumerate().map(|(i, count)| {
                let left = min_value + delta * (i as f64);
                let right = left + delta;
                let bar = Rectangle::new([(left, 0), (right, *count)], a_color.filled());
                bar
            }))
            .unwrap()
            .label("A density")
            .legend(|(x, y)| {
                Rectangle::new(
                    [(x, y - 3 * dpr_i), (x + 10 * dpr_i, y + 3 * dpr_i)],
                    a_color.filled(),
                )
            });

        chart
            .draw_series(b_hist.iter().enumerate().map(|(i, count)| {
                let left = min_value + delta * (i as f64);
                let right = left + delta;
                let bar = Rectangle::new([(left, 0), (right, *count)], b_color.filled());
                bar
            }))
            .unwrap()
            .label("B density")
            .legend(|(x, y)| {
                Rectangle::new(
                    [(x, y - 3 * dpr_i), (x + 10 * dpr_i, y + 3 * dpr_i)],
                    b_color.filled(),
                )
            });
        chart
            .configure_series_labels()
            .label_font(("sans-serif", 12 * dpr_i))
            .border_style(&BLACK)
            .background_style(&WHITE.mix(0.8))
            .draw()
            .unwrap();
        root.present().unwrap();
    }
    let max_diff = option_to_f64(max_diff)?;
    let min_diff = option_to_f64(min_diff)?;
    diff_samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    {
        let delta = (max_diff - min_diff) / (n_bins as f64);
        let backend = CanvasBackend::with_canvas_object(canvas_diff).unwrap();
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();

        let (hist, cnt_max) = samples_to_hist(&diff_samples, min_diff, n_bins as usize, delta);
        let cnt_max = cnt_max as f64;

        let mut chart = ChartBuilder::on(&root)
            .set_label_area_size(LabelAreaPosition::Left, 80.0 as f64)
            .set_label_area_size(LabelAreaPosition::Bottom, 40.0 as f64)
            .caption(
                "Posterior difference distribution",
                ("sans-serif", 15 * dpr as i32),
            )
            .build_cartesian_2d(
                0.98 * min_diff..(1.02 * max_diff),
                0 as u32..((cnt_max * 1.01) as u32),
            )
            .unwrap();
        chart
            .configure_mesh()
            .x_labels(10)
            .x_label_style(("sans-serif", 10 * dpr_i))
            .disable_y_mesh()
            .disable_y_axis()
            .draw()
            .unwrap();

        chart
            .draw_series(hist.iter().enumerate().map(|(i, count)| {
                let left = min_diff + delta * (i as f64);
                let right = left + delta;
                let bar = Rectangle::new([(left, 0), (right, *count)], a_color.filled());
                bar
            }))
            .unwrap();
        for v in percentiles.iter() {
            let index = (v * (n_samples as f64)) as usize;
            let pval = diff_samples[index.min(diff_samples.len() - 1)];
            let index_ = ((pval - min_diff) / delta) as usize;
            let cnt_this = hist[index_];
            let gen = (0..2).map(|i| (pval + delta / 2.0, (i * cnt_this as u32)));
            chart.draw_series(LineSeries::new(gen, &BLACK)).unwrap();
            chart
                .draw_series(PointSeries::of_element(
                    vec![cnt_this],
                    10 * dpr_i,
                    &BLACK,
                    &|_c, _s, _st| {
                        return Text::new(
                            format!("{:.1}%", (v * 100.0)),
                            (pval, (cnt_this / 2) as u32),
                            ("sans-serif", 10 * dpr as i32),
                        );
                    },
                ))
                .unwrap();
        }

        root.present().unwrap();
    }
    Ok((b_win as f64) / n_samples as f64)
}
