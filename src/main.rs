
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // 建立 UI 視窗
    let app = AppWindow::new()?;

    // 讀取按鈕事件
    let weak_app = app.as_weak();
    let mut count = 0u32;

    app.on_read_voltage(move || {
        // 產生模擬電壓
        let voltage = 3.5 + (count % 12) as f64 * 0.073;
        count += 1;

        if let Some(app) = weak_app.upgrade() {
            app.set_voltage(
                format!("{:.3}", voltage).into()
            );
        }
    });

    // 重設按鈕事件
    let weak_app = app.as_weak();

    app.on_reset_voltage(move || {
        if let Some(app) = weak_app.upgrade() {
            app.set_voltage("0.000".into());
        }
    });

    // 啟動視窗
    app.run()
}
