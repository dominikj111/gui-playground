use std::time::Instant;

use crate::styles::theme::ThemeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Developer,
    Designer,
    Manager,
    Tester,
}

impl Role {
    pub const ALL: [Role; 4] = [Role::Developer, Role::Designer, Role::Manager, Role::Tester];
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::Developer => "Developer",
                Role::Designer => "Designer",
                Role::Manager => "Manager",
                Role::Tester => "Tester",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserOption {
    OptionA,
    OptionB,
    OptionC,
}

impl BrowserOption {
    pub const ALL: [BrowserOption; 3] = [
        BrowserOption::OptionA,
        BrowserOption::OptionB,
        BrowserOption::OptionC,
    ];
}

impl std::fmt::Display for BrowserOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BrowserOption::OptionA => "Option A",
                BrowserOption::OptionB => "Option B",
                BrowserOption::OptionC => "Option C",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Form,
    Chart,
    Summary,
    BrowserForm,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    NameChanged(String),
    EmailChanged(String),
    AgeChanged(u8),
    ExperienceChanged(f32),
    RoleSelected(Role),
    ThemeSelected(ThemeType),
    ViewChanged(ViewMode),
    Tick(Instant),
    ShowSecondButton,
    HideSecondButton,
    // Browser form messages
    BrowserTextChanged(String),
    BrowserPasswordChanged(String),
    BrowserSliderChanged(f32),
    BrowserCheckboxToggled(bool),
    BrowserOptionSelected(BrowserOption),
    BrowserFormSubmit,
    // Config menu
    ToggleConfigMenu,
    ConfigResetData,
    ConfigExportSettings,
    ConfigImportSettings,
    ConfigMenuInteraction, // Dummy message to block click-through
}
