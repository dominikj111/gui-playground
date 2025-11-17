use iced::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl ThemeType {
    pub const ALL: [ThemeType; 22] = [
        ThemeType::Light,
        ThemeType::Dark,
        ThemeType::Dracula,
        ThemeType::Nord,
        ThemeType::SolarizedLight,
        ThemeType::SolarizedDark,
        ThemeType::GruvboxLight,
        ThemeType::GruvboxDark,
        ThemeType::CatppuccinLatte,
        ThemeType::CatppuccinFrappe,
        ThemeType::CatppuccinMacchiato,
        ThemeType::CatppuccinMocha,
        ThemeType::TokyoNight,
        ThemeType::TokyoNightStorm,
        ThemeType::TokyoNightLight,
        ThemeType::KanagawaWave,
        ThemeType::KanagawaDragon,
        ThemeType::KanagawaLotus,
        ThemeType::Moonfly,
        ThemeType::Nightfly,
        ThemeType::Oxocarbon,
        ThemeType::Ferra,
    ];

    pub fn to_theme(self) -> Theme {
        match self {
            ThemeType::Light => Theme::Light,
            ThemeType::Dark => Theme::Dark,
            ThemeType::Dracula => Theme::Dracula,
            ThemeType::Nord => Theme::Nord,
            ThemeType::SolarizedLight => Theme::SolarizedLight,
            ThemeType::SolarizedDark => Theme::SolarizedDark,
            ThemeType::GruvboxLight => Theme::GruvboxLight,
            ThemeType::GruvboxDark => Theme::GruvboxDark,
            ThemeType::CatppuccinLatte => Theme::CatppuccinLatte,
            ThemeType::CatppuccinFrappe => Theme::CatppuccinFrappe,
            ThemeType::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            ThemeType::CatppuccinMocha => Theme::CatppuccinMocha,
            ThemeType::TokyoNight => Theme::TokyoNight,
            ThemeType::TokyoNightStorm => Theme::TokyoNightStorm,
            ThemeType::TokyoNightLight => Theme::TokyoNightLight,
            ThemeType::KanagawaWave => Theme::KanagawaWave,
            ThemeType::KanagawaDragon => Theme::KanagawaDragon,
            ThemeType::KanagawaLotus => Theme::KanagawaLotus,
            ThemeType::Moonfly => Theme::Moonfly,
            ThemeType::Nightfly => Theme::Nightfly,
            ThemeType::Oxocarbon => Theme::Oxocarbon,
            ThemeType::Ferra => Theme::Ferra,
        }
    }
}

impl std::fmt::Display for ThemeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ThemeType::Light => "Light",
                ThemeType::Dark => "Dark",
                ThemeType::Dracula => "Dracula",
                ThemeType::Nord => "Nord",
                ThemeType::SolarizedLight => "Solarized Light",
                ThemeType::SolarizedDark => "Solarized Dark",
                ThemeType::GruvboxLight => "Gruvbox Light",
                ThemeType::GruvboxDark => "Gruvbox Dark",
                ThemeType::CatppuccinLatte => "Catppuccin Latte",
                ThemeType::CatppuccinFrappe => "Catppuccin Frappé",
                ThemeType::CatppuccinMacchiato => "Catppuccin Macchiato",
                ThemeType::CatppuccinMocha => "Catppuccin Mocha",
                ThemeType::TokyoNight => "Tokyo Night",
                ThemeType::TokyoNightStorm => "Tokyo Night Storm",
                ThemeType::TokyoNightLight => "Tokyo Night Light",
                ThemeType::KanagawaWave => "Kanagawa Wave",
                ThemeType::KanagawaDragon => "Kanagawa Dragon",
                ThemeType::KanagawaLotus => "Kanagawa Lotus",
                ThemeType::Moonfly => "Moonfly",
                ThemeType::Nightfly => "Nightfly",
                ThemeType::Oxocarbon => "Oxocarbon",
                ThemeType::Ferra => "Ferra",
            }
        )
    }
}
