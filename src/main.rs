use iced::widget::{button, column, text, text_input, Container, image}; // Attempt image import again
use iced::{Application, Command, Element, Settings, Theme, Background, Color, Border};
use std::path::PathBuf;

// Define the app's state
#[derive(Default)]
struct MostroLogin {
    seed_input: String,
    is_dark_mode: bool,
}

// Messages (events)
#[derive(Debug, Clone)]
enum Message {
    SeedChanged(String),
    LoginPressed,
    GenerateKeyPressed,
    ToggleTheme,
    ClosePressed,
}

// Custom style for the container
struct ContainerStyle {
    is_dark_mode: bool,
}

impl iced::widget::container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _theme: &Theme) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(Background::Color(if self.is_dark_mode {
                Color::from_rgb8(0x08, 0x08, 0x08) // #080808 for dark mode
            } else {
                Color::WHITE // White for light mode
            })),
            ..Default::default()
        }
    }
}

// Custom style for buttons
struct ButtonStyle;

impl iced::widget::button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(Background::Color(Color::WHITE)), // White background
            text_color: Color::BLACK, // Black text
            border: Border {
                radius: 7.0.into(), // 7px corner radius
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Application for MostroLogin {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            MostroLogin {
                seed_input: String::new(),
                is_dark_mode: true, // Start in dark mode
            },
            iced::window::change_mode(iced::window::Id::MAIN, iced::window::Mode::Windowed),
        )
    }

    fn title(&self) -> String {
        String::from("Mostro Desktop - Login")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SeedChanged(seed) => {
                self.seed_input = seed;
            }
            Message::LoginPressed => {
                if self.seed_input.split_whitespace().count() == 12 {
                    println!("Logging in with seed: {}", self.seed_input);
                    // Add Mostro Protocol login logic here
                } else {
                    println!("Invalid seed: Please enter exactly 12 words.");
                }
            }
            Message::GenerateKeyPressed => {
                // Simulate generating a seed phrase (12 random words)
                self.seed_input = "word1 word2 word3 word4 word5 word6 word7 word8 word9 word10 word11 word12".to_string();
                println!("Generated new seed: {}", self.seed_input);
            }
            Message::ToggleTheme => {
                self.is_dark_mode = !self.is_dark_mode;
            }
            Message::ClosePressed => {
                return iced::window::close(iced::window::Id::MAIN);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // Load Mostro logo (assumes word-logo.png is in assets/)
        let logo = image(PathBuf::from("assets/word-logo.png"))
            .width(200)
            .height(50);

        // Seed phrase input
        let seed_input = text_input("Enter your 12 seed phrases...", &self.seed_input)
            .on_input(Message::SeedChanged)
            .padding(10)
            .width(300)
            .style(if self.is_dark_mode {
                iced::theme::TextInput::Default
            } else {
                iced::theme::TextInput::Default
            });

        // Buttons
        let login_button = button(text("Login"))
            .on_press(Message::LoginPressed)
            .padding(10)
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle)));
        let generate_key_button = button(text("Generate Key"))
            .on_press(Message::GenerateKeyPressed)
            .padding(10)
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle)));
        let toggle_theme_button = button(text("Toggle Theme"))
            .on_press(Message::ToggleTheme)
            .padding(10)
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle)));
        let close_button = button(text("Close"))
            .on_press(Message::ClosePressed)
            .padding(10)
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle)));

        // Layout
        Container::new(
            column![
                logo,
                seed_input,
                login_button,
                generate_key_button,
                toggle_theme_button,
                close_button,
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle {
            is_dark_mode: self.is_dark_mode,
        })))
        .into()
    }

    fn theme(&self) -> Theme {
        if self.is_dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

fn main() -> iced::Result {
    MostroLogin::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(400.0, 600.0), // Adjusted size for login window
            ..Default::default()
        },
        ..Default::default()
    })
}