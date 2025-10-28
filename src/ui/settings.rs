#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub fn settings() -> iced::window::Settings {
    iced::window::Settings::default()
}

#[cfg(target_os = "linux")]
pub fn settings() -> iced::window::Settings {
    iced::window::Settings {
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: "org.ejacques.rsfilesorter".to_string(),
            override_redirect: false,
        },
        ..Default::default()
    }
}

#[cfg(target_os = "macos")]
pub fn settings() -> iced::window::Settings {
    iced::window::Settings {
        platform_specific: iced::window::settings::PlatformSpecific {
            title_hidden: true,
            titlebar_transparent: true,
            fullsize_content_view: true,
        },
        ..Default::default()
    }
}

#[cfg(target_os = "windows")]
pub fn settings() -> iced::window::Settings {
    use iced::window;
    use image::EncodableLayout;

    let img = image::load_from_memory_with_format(
        include_bytes!("../../rsc/img/logo_71_71.png"),
        image::ImageFormat::Png,
    );
    match img {
        Ok(img) => match img.as_rgba8() {
            Some(icon) => window::Settings {
                icon: window::icon::from_rgba(
                    icon.as_bytes().to_vec(),
                    icon.width(),
                    icon.height(),
                )
                .ok(),
                ..Default::default()
            },
            None => window::Settings::default(),
        },
        Err(_) => window::Settings {
            ..Default::default()
        },
    }
}
