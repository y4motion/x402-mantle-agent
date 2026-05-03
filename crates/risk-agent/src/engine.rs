pub fn is_exposure_safe(current_exposure: f64, max_exposure: f64) -> bool {
    current_exposure < max_exposure
}
