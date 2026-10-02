pub fn initialise() {
    tracing_subscriber::fmt().json().flatten_event(true).init()
}
