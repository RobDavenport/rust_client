use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
  static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_dynamic_data(time: f32, canvas_width: f32, canvas_height: f32) {
  let mut data = APP_STATE.lock().unwrap();

  *data = Arc::new(AppState {
    canvas_width,
    canvas_height,
    time,
    ..*data.clone()
  });
}

pub fn get_current_state() -> Arc<AppState> {
  APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
  pub canvas_width: f32,
  pub canvas_height: f32,
  pub control_bottom: f32,
  pub control_top: f32,
  pub control_left: f32,
  pub control_right: f32,
  pub time: f32,
}

impl AppState {
  fn new() -> AppState {
    AppState {
      canvas_width: 0.,
      canvas_height: 0.,
      control_bottom: 0.,
      control_top: 0.,
      control_left: 0.,
      control_right: 0.,
      time: 0.
    }
  }
}