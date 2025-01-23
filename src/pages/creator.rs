use cosmic::{
    app::Task,
    iced::{id, Length},
    style, task,
    widget::{self},
    Element,
};

use crate::{
    browser::{installed_browsers, Browser, BrowserModel},
    common::{self, icon_cache_get, url_valid, IconType},
    fl,
    pages::{self},
    warning::{WarnAction, WarnMessages, Warning},
};

#[derive(Debug, Clone)]
pub struct AppCreator {
    pub app_title_id: id::Id,
    pub app_title: String,
    pub app_url: String,
    pub app_url_id: id::Id,
    pub app_icon: String,
    pub app_parameters: String,
    pub app_categories: Vec<String>,
    pub app_category: String,
    pub selected_category: usize,
    pub app_browser: Option<Browser>,
    pub app_navbar: bool,
    pub app_incognito: bool,
    pub app_isolated: bool,
    pub selected_icon: Option<common::Icon>,
    pub app_browsers: Vec<Browser>,
    pub selected_browser: Option<usize>,
    pub edit_mode: bool,
    pub warning: Warning,
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
        let browsers = installed_browsers();
        let browser = if !browsers.is_empty() {
            Some(browsers[0].clone())
        } else {
            None
        };

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
            app_title_id: id::Id::new("app-title"),
            app_title: String::new(),
            app_url: String::new(),
            app_url_id: id::Id::new("app-url"),
            app_icon: String::new(),
            app_parameters: String::new(),
            app_categories: categories.to_vec(),
            app_category: categories[0].clone(),
            selected_category: 0,
            app_browser: browser,
            app_navbar: false,
            app_incognito: false,
            app_isolated: true,
            selected_icon: None,
            app_browsers: browsers,
            selected_browser: Some(0),
            edit_mode: false,
            warning: Warning::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<pages::Message> {
        let mut commands: Vec<Task<pages::Message>> = Vec::new();

        match message {
            Message::Title(title) => {
                self.app_title = title;

                if self.app_title.len() >= 3 {
                    commands.push(task::future(async {
                        pages::Message::Warning((WarnAction::Remove, WarnMessages::AppName))
                    }))
                } else {
                    commands.push(task::future(async {
                        pages::Message::Warning((WarnAction::Add, WarnMessages::AppName))
                    }))
                }
            }
            Message::Url(url) => {
                self.app_url = url;

                if url_valid(&self.app_url) {
                    commands.push(task::future(async {
                        pages::Message::Warning((WarnAction::Remove, WarnMessages::AppUrl))
                    }))
                } else {
                    commands.push(task::future(async {
                        pages::Message::Warning((WarnAction::Add, WarnMessages::AppUrl))
                    }))
                }
            }
            Message::Arguments(args) => {
                self.app_parameters = args;
            }
            Message::Browser(idx) => {
                let browser = &self.app_browsers[idx];
                self.selected_browser = Some(idx);
                self.app_browser = Some(browser.clone());

                commands.push(match browser.model {
                    None => task::future(async {
                        pages::Message::Warning((WarnAction::Add, WarnMessages::AppBrowser))
                    }),
                    Some(_) => task::future(async {
                        pages::Message::Warning((WarnAction::Remove, WarnMessages::AppBrowser))
                    }),
                })
            }
            Message::Category(idx) => {
                self.app_category.clone_from(&self.app_categories[idx]);
                self.selected_category = idx;
            }

            Message::Clicked(buttons) => match buttons {
                Buttons::Navbar(selected) => {
                    self.app_navbar = selected;
                }
                Buttons::IsolatedProfile(selected) => {
                    self.app_isolated = selected;
                }
                Buttons::Incognito(selected) => {
                    self.app_incognito = selected;
                }
            },
        };

        Task::batch(commands)
    }

    fn icon_picker_icon(&self, icon: Option<common::Icon>) -> Element<pages::Message> {
        let ico = if let Some(ico) = icon {
            match ico.icon {
                IconType::Raster(data) => widget::button::custom(widget::image(data))
                    .width(Length::Fixed(48.))
                    .height(Length::Fixed(48.))
                    .class(style::Button::Icon),

                IconType::Svg(data) => widget::button::custom(widget::svg(data))
                    .width(Length::Fixed(48.))
                    .height(Length::Fixed(48.))
                    .class(style::Button::Icon),
            }
        } else {
            widget::button::custom(icon_cache_get("folder-pictures-symbolic", 16))
                .width(Length::Fixed(48.))
                .height(Length::Fixed(48.))
                .class(style::Button::Icon)
        };

        widget::container(ico)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn download_button(&self) -> Element<pages::Message> {
        widget::container(
            widget::button::custom(icon_cache_get("folder-download-symbolic", 16))
                .width(Length::Fixed(48.))
                .height(Length::Fixed(48.))
                .class(style::Button::Icon),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    pub fn view(&self, warning: Warning) -> Element<pages::Message> {
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
        let download_button = widget::button::custom(download_button)
            .width(82.)
            .height(82.)
            .on_press(pages::Message::Clicked(pages::Buttons::SearchFavicon));

        let icon = self.icon_picker_icon(self.selected_icon.clone());
        let icon = widget::button::custom(icon)
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

        let categories_dropdown = widget::dropdown(
            &self.app_categories,
            Some(self.selected_category),
            move |index| pages::Message::Creator(Message::Category(index)),
        )
        .width(Length::Fixed(200.));

        let browser_specific = if let Some(browser) = &self.app_browser {
            match browser.model {
                Some(BrowserModel::Firefox) | Some(BrowserModel::Zen) => Some(
                    widget::toggler(self.app_navbar)
                        .label(fl!("navbar"))
                        .on_toggle(|b| {
                            pages::Message::Creator(Message::Clicked(Buttons::Navbar(b)))
                        })
                        .spacing(10),
                ),

                _ => Some(
                    widget::toggler(self.app_isolated)
                        .label(fl!("isolated-profile"))
                        .on_toggle(|b| {
                            pages::Message::Creator(Message::Clicked(Buttons::IsolatedProfile(b)))
                        })
                        .spacing(10),
                ),
            }
        } else {
            None
        };

        let incognito = widget::toggler(self.app_incognito)
            .label(fl!("private-mode"))
            .on_toggle(|b| pages::Message::Creator(Message::Clicked(Buttons::Incognito(b))))
            .spacing(10);

        let first_row = widget::row()
            .push(categories_dropdown)
            .push_maybe(browser_specific)
            .spacing(10);

        let app_browsers = widget::dropdown(&self.app_browsers, self.selected_browser, |idx| {
            pages::Message::Creator(Message::Browser(idx))
        })
        .width(Length::Fixed(200.));

        let save_btn = if self.edit_mode {
            widget::button::suggested(fl!("edit")).on_press(pages::Message::Clicked(
                pages::Buttons::DoneEdit((None, None)),
            ))
        } else {
            widget::button::suggested(fl!("create"))
                .on_press(pages::Message::Clicked(pages::Buttons::DoneCreate))
        };

        let creator_close =
            widget::button::standard(fl!("close")).on_press(pages::Message::CloseCreator);

        let spacer = widget::horizontal_space();

        let end_row = widget::row()
            .push(app_browsers)
            .push(incognito)
            .push(spacer)
            .push(creator_close)
            .push(save_btn)
            .spacing(10);

        let mut view_column = widget::column();

        if warning.show {
            view_column = view_column.push(widget::warning(warning.messages()))
        }

        view_column = view_column
            .push(row)
            .push(app_arguments)
            .push(first_row)
            .push(end_row)
            .spacing(10)
            .padding(30);

        widget::container(view_column).max_width(1000).into()
    }
}
