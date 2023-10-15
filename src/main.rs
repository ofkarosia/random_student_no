#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;

use app::{AppData, AppEvent, Config};
use vizia::prelude::*;

mod app;

const ICON_GITHUB_LINK: &str = "\u{eade}";

fn main() {
    let config_path = dirs::config_dir().unwrap().join("randno_rs\\config");
    let config: Config = fs::read(config_path.as_path())
        .map(|bytes| bitcode::decode(&bytes).unwrap_or_default())
        .ok()
        .unwrap_or_default();

    Application::new(move |cx| {
        cx.add_stylesheet(include_style!("src/app.css"))
            .expect("Failed to load stylesheet");
        cx.add_translation(langid!("zh_CN"), include_str!("res/zh_CN.ftl").to_owned());
        cx.add_translation(langid!("en_US"), include_str!("res/en_US.ftl").to_owned());

        cx.add_global_listener(move |cx, event| {
            event.map(|window_event: &WindowEvent, _| {
                if let WindowEvent::WindowClose = window_event {
                    let range = Config {
                        start: AppData::range_start.get(cx),
                        end: AppData::range_end.get(cx),
                        is_zh: cx.environment().locale.language.as_str() == "zh",
                    };

                    if !config_path.exists() {
                        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
                    }

                    fs::write(config_path.as_path(), bitcode::encode(&range).unwrap()).unwrap();
                }
            });
        });

        cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::DarkMode));
        cx.emit(EnvironmentEvent::SetLocale(if config.is_zh {
            langid!("zh_CN")
        } else {
            langid!("en_US")
        }));

        AppData {
            range_start: config.start,
            range_end: config.end,
            ..Default::default()
        }
        .build(cx);

        HStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("start"))
                    .class("range")
                    .describing("start");
                Textbox::new(cx, AppData::range_start)
                    .id("start")
                    .on_submit(|cx, input, _| cx.emit(AppEvent::SetRangeStart(input)))
                    .max_width(Pixels(50.0));
            })
            .width(Stretch(1.0))
            .height(Auto)
            .left(Pixels(20.0))
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(10.0));

            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("end"))
                    .class("range")
                    .describing("end");
                Textbox::new(cx, AppData::range_end)
                    .id("end")
                    .on_submit(|cx, input, _| cx.emit(AppEvent::SetRangeEnd(input)))
                    .max_width(Pixels(50.0));
            })
            .width(Stretch(1.0))
            .height(Auto)
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(10.0));
        })
        .width(Stretch(1.0))
        .height(Auto)
        .top(Pixels(10.0));

        VStack::new(cx, |cx| {
            Binding::new(cx, AppData::result, |cx, result| {
                if result.get(cx).is_some() {
                    Label::new(cx, result.unwrap())
                        .id("result")
                        .left(Stretch(1.0))
                        .right(Stretch(1.0))
                } else {
                    Label::new(cx, Localized::new("none"))
                        .id("result")
                        .left(Stretch(1.0))
                        .right(Stretch(1.0))
                };
            });

            Button::new(
                cx,
                |cx| cx.emit(AppEvent::Generate),
                |cx| Label::new(cx, Localized::new("gen")),
            )
            .disabled(AppData::button_disabled)
            .id("generate")
            .left(Stretch(1.0))
            .right(Stretch(1.0));
        })
        .width(Stretch(1.0))
        .space(Pixels(20.0))
        .row_between(Pixels(10.0))
        .height(Auto);

        HStack::new(cx, |cx| {
            Button::new(
                cx,
                |_| {
                    let _ = open::that_detached("https://github.com/ofkarosia/random_student_no");
                },
                |cx| Icon::new(cx, ICON_GITHUB_LINK),
            )
            .size(Auto)
            .id("link");

            Button::new(
                cx,
                |cx| {
                    if cx.environment().locale.language.as_str() == "zh" {
                        cx.emit(EnvironmentEvent::SetLocale(langid!("en_US")))
                    } else {
                        cx.emit(EnvironmentEvent::SetLocale(langid!("zh_CN")))
                    }
                },
                |cx| Label::new(cx, Localized::new("lang")),
            )
            .size(Auto)
            .left(Stretch(1.0));
        })
        .width(Stretch(1.0))
        .height(Auto)
        .col_between(Stretch(1.0))
        .child_top(Stretch(1.0));

        Binding::new(cx, Environment::locale, |cx, _| {
            cx.emit(WindowEvent::SetTitle(Localized::new("title").get_val(cx)))
        })
    })
    .min_inner_size(Some((260, 210)))
    .max_inner_size(Some((360, 210)))
    .run();
}
