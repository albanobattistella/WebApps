use cosmic::widget::{text, warning};
use cosmic::{
    app::{message::app, Message as CosmicMessage},
    iced::{id, Length},
    style, theme,
    widget::{self, dropdown, toggler, Container},
    Command, Element,
};

use crate::{
    common::{get_supported_browsers, icon_cache_get, url_valid, Browser, BrowserType},
    fl,
    pages::{self, iconpicker::IconType},
    warning::{WarnAction, WarnMessages},
};

#[derive(Debug, Clone)]
pub struct AppCreator {
    pub app_codename: Option<String>,
    pub app_title_id: id::Id,
    pub app_title: String,
    pub app_url: String,
    pub app_url_id: id::Id,
    pub app_icon: String,
    pub app_parameters: String,
    pub app_categories: Vec<String>,
    pub app_category: String,
    pub selected_category: usize,
    pub app_browser_name: String,
    pub app_browser: Browser,
    pub app_navbar: bool,
    pub app_incognito: bool,
    pub app_isolated: bool,
    pub selected_icon: Option<pages::iconpicker::Icon>,
    pub app_browsers: Vec<Browser>,
    pub selected_browser: Option<usize>,
    pub dialog_open: bool,
    pub edit_mode: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Title(String),
    Url(String),
    Arguments(String),
    Browser(usize),
    Category(usize),

    Clicked(Buttons),
}

#[derive(Debug, Clone)]
pub enum Buttons {
    Navbar(bool),
    IsolatedProfile(bool),
    Incognito(bool),
}

impl AppCreator {
    pub fn new() -> Self {
        let browsers = get_supported_browsers();
        let browser = &browsers[0];

        let categories = [
            fl!("web"),
            fl!("accessories"),
            fl!("education"),
            fl!("games"),
            fl!("graphics"),
            fl!("internet"),
            fl!("office"),
            fl!("programming"),
            fl!("sound-and-video"),
        ];

        AppCreator {
            app_codename: None,
            app_title_id: id::Id::new("app-title"),
            app_title: String::new(),
            app_url: String::new(),
            app_url_id: id::Id::new("app-url"),
            app_icon: String::new(),
            app_parameters: String::new(),
            app_categories: categories.to_vec(),
            app_category: categories[0].clone(),
            selected_category: 0,
            app_browser_name: fl!("browser"),
            app_browser: browser.clone(),
            app_navbar: false,
            app_incognito: false,
            app_isolated: true,
            selected_icon: None,
            app_browsers: browsers,
            selected_browser: Some(0),
            dialog_open: false,
            edit_mode: false,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<CosmicMessage<pages::Message>> {
        match message {
            Message::Title(title) => {
                self.app_title = title;

                if self.app_title.len() >= 3 {
                    Command::perform(async {}, |_| {
                        app(pages::Message::Warning((
                            WarnAction::Remove,
                            WarnMessages::AppName,
                        )))
                    })
                } else {
                    Command::perform(async {}, |_| {
                        app(pages::Message::Warning((
                            WarnAction::Add,
                            WarnMessages::AppName,
                        )))
                    })
                }
            }
            Message::Url(url) => {
                self.app_url = url;

                if url_valid(&self.app_url) {
                    Command::perform(async {}, |_| {
                        app(pages::Message::Warning((
                            WarnAction::Remove,
                            WarnMessages::AppUrl,
                        )))
                    })
                } else {
                    Command::perform(async {}, |_| {
                        app(pages::Message::Warning((
                            WarnAction::Add,
                            WarnMessages::AppUrl,
                        )))
                    })
                }
            }
            Message::Arguments(args) => {
                self.app_parameters = args;
                Command::none()
            }
            Message::Browser(idx) => {
                let browser = &self.app_browsers[idx];
                self.selected_browser = Some(idx);
                self.app_browser = browser.clone();

                match browser._type {
                    BrowserType::NoBrowser => Command::perform(async {}, |_| {
                        app(pages::Message::Warning((
                            WarnAction::Add,
                            WarnMessages::AppBrowser,
                        )))
                    }),
                    _ => Command::perform(async {}, |_| {
                        app(pages::Message::Warning((
                            WarnAction::Remove,
                            WarnMessages::AppBrowser,
                        )))
                    }),
                }
            }
            Message::Category(idx) => {
                self.app_category.clone_from(&self.app_categories[idx]);
                self.selected_category = idx;
                Command::none()
            }

            Message::Clicked(buttons) => match buttons {
                Buttons::Navbar(selected) => {
                    self.app_navbar = selected;

                    Command::none()
                }
                Buttons::IsolatedProfile(selected) => {
                    self.app_isolated = selected;

                    Command::none()
                }
                Buttons::Incognito(selected) => {
                    self.app_incognito = selected;

                    Command::none()
                }
            },
        }
    }

    fn icon_picker_icon(&self, icon: Option<pages::iconpicker::Icon>) -> Element<pages::Message> {
        let ico = if let Some(ico) = icon {
            match ico.icon {
                IconType::Raster(data) => widget::button(widget::image(data))
                    .width(Length::Fixed(48.))
                    .height(Length::Fixed(48.))
                    .style(style::Button::Icon),

                IconType::Svg(data) => widget::button(widget::svg(data))
                    .width(Length::Fixed(48.))
                    .height(Length::Fixed(48.))
                    .style(style::Button::Icon),
            }
        } else {
            widget::button(icon_cache_get("folder-pictures-symbolic", 16))
                .width(Length::Fixed(48.))
                .height(Length::Fixed(48.))
                .style(style::Button::Icon)
        };

        Container::new(ico).center_x().center_y().into()
    }

    fn download_button(&self) -> Element<pages::Message> {
        Container::new(
            widget::button(icon_cache_get("folder-download-symbolic", 16))
                .width(Length::Fixed(48.))
                .height(Length::Fixed(48.))
                .style(style::Button::Icon),
        )
        .center_x()
        .center_y()
        .into()
    }

    pub fn view(&self, warnings: String) -> Element<pages::Message> {
        let app_title = widget::text_input(fl!("title"), &self.app_title)
            .id(self.app_title_id.clone())
            .on_input(|s| pages::Message::Creator(Message::Title(s)))
            .width(Length::Fill);
        let app_url = widget::text_input(fl!("url"), &self.app_url)
            .id(self.app_url_id.clone())
            .on_input(|s| pages::Message::Creator(Message::Url(s)))
            .width(Length::Fill);

        let app_data_inputs = widget::column().push(app_title).push(app_url).spacing(10);

        let download_button = self.download_button();
        let download_button = widget::button(download_button)
            .width(82.)
            .height(82.)
            .on_press(pages::Message::Clicked(pages::Buttons::SearchFavicon));

        let icon = self.icon_picker_icon(self.selected_icon.clone());
        let icon = widget::button(icon)
            .width(Length::Fixed(82.))
            .height(Length::Fixed(82.))
            .on_press(pages::Message::OpenIconPicker);

        let row = widget::row()
            .push(app_data_inputs)
            .push(download_button)
            .push(icon)
            .spacing(10)
            .width(Length::Fill);

        let app_arguments = widget::text_input(fl!("non-standard-arguments"), &self.app_parameters)
            .on_input(|s| pages::Message::Creator(Message::Arguments(s)))
            .width(Length::Fill);

        let categories_dropdown = dropdown(
            &self.app_categories,
            Some(self.selected_category),
            move |index| pages::Message::Creator(Message::Category(index)),
        )
        .width(Length::Fixed(200.));

        let navbar_toggle = toggler(fl!("navbar"), self.app_navbar, |b| {
            pages::Message::Creator(Message::Clicked(Buttons::Navbar(b)))
        })
        .width(Length::Fill);

        let browser_specific = match self.app_browser._type {
            BrowserType::Firefox => navbar_toggle,
            BrowserType::FirefoxFlatpak => navbar_toggle,

            _ => toggler(fl!("isolated-profile"), self.app_isolated, |b| {
                pages::Message::Creator(Message::Clicked(Buttons::IsolatedProfile(b)))
            })
            .width(Length::Fill),
        };

        let incognito = toggler(fl!("private-mode"), self.app_incognito, |b| {
            pages::Message::Creator(Message::Clicked(Buttons::Incognito(b)))
        })
        .width(Length::Fill);

        let save_btn = if self.edit_mode {
            widget::button(Container::new(text(fl!("edit"))).center_x())
                .on_press(pages::Message::DoneEdit)
                .width(Length::Fill)
                .style(theme::Button::Suggested)
        } else {
            widget::button(Container::new(text(fl!("create"))).center_x())
                .on_press(pages::Message::DoneCreate)
                .width(Length::Fill)
                .style(theme::Button::Suggested)
        };

        let first_row = widget::row()
            .push(categories_dropdown)
            .push(browser_specific)
            .push(save_btn)
            .spacing(10);

        let app_browsers = dropdown(&self.app_browsers, self.selected_browser, |idx| {
            pages::Message::Creator(Message::Browser(idx))
        })
        .width(Length::Fixed(200.));

        let creator_close = widget::button(Container::new(text(fl!("close"))).center_x())
            .on_press(pages::Message::CloseCreator)
            .width(Length::Fill);

        let end_row = widget::row()
            .push(app_browsers)
            .push(incognito)
            .push(creator_close)
            .spacing(10);

        let view_column = widget::column()
            .push(warning(warnings))
            .push(row)
            .push(app_arguments)
            .push(first_row)
            .push(end_row)
            .spacing(10)
            .padding(30);

        Container::new(view_column).max_width(1000).into()
    }
}
