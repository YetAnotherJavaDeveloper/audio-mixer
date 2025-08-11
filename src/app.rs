mod models;

use crate::core::extract_sample_info;
use crate::file::{read_music_samples_from_file};
use crate::media::MusicSamplesPlayer;

use iced::widget::{button, column, container, progress_bar, row, text};
use iced::{
     Element, Task, Theme, alignment,
};

pub use models::{AudioMixer, Message};

impl AudioMixer {
    pub fn new() -> Self {
        Self {
            is_loaded: false,
            is_playing: false,
            is_loading: false,
            length: 0.0,
            current_position: 0,
            file_path: String::new(),
            loaded_samples: None,
            media_player: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                if let Some(player) = &mut self.media_player {
                    self.current_position = player.current_position;
                }
                Task::none()
            }
            Message::OpenFileDialog => Task::perform(
                async {
                    rfd::AsyncFileDialog::new()
                        .add_filter("audiofile", &["mp3"])
                        .pick_file()
                        .await
                },
                |path_opt| {
                    path_opt
                        .map(|path| Message::FileChosen(path.path().to_string_lossy().to_string()))
                        .unwrap_or(Message::FileChosen(String::new()))
                },
            ),
            Message::FileChosen(path) => {
                if !path.is_empty() {
                    self.file_path = path.clone();
                    self.is_loading = true;

                    // Lancer la lecture dans un Task async
                    return Task::perform(
                        async move { read_music_samples_from_file(path) },
                        |result| match result {
                            Ok(samples) => {
                                extract_sample_info(&samples);
                                Message::FileLoaded(samples)
                            }
                            Err(e) => {
                                eprintln!("Error loading file: {}", e);
                                Message::Error(e.to_string())
                            }
                        },
                    );
                } else {
                    self.is_loaded = false;
                    self.is_loading = false;
                    self.file_path.clear();
                    eprintln!("No file chosen");
                }
                Task::none()
            }
            Message::FileLoaded(result) => {
                self.is_loaded = true;
                self.is_loading = false;
                self.length = result.all_samples[0].len() as f32 / result.sample_rate as f32;
                self.loaded_samples = Some(result);
                self.media_player = Some(MusicSamplesPlayer::new(
                    self.loaded_samples.clone().unwrap(),
                ));
                self.current_position = self.media_player.as_ref().unwrap().current_position;
                Task::none()
            }
            Message::PlayPause => {
                if self.loaded_samples.is_some() && self.media_player.is_some() {
                    if self.is_playing {
                        self.is_playing = false;
                        self.media_player.as_mut().unwrap().pause();
                    } else {
                        self.is_playing = true;
                        self.media_player.as_mut().unwrap().play();
                    }
                }
                Task::none()
            }
            Message::Error(err) => {
                eprintln!("Error: {}", err);
                self.is_loading = false;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message, Theme> {
        let info_content = container(column![
            text(format!("Is Loaded: {}", self.is_loaded)),
            text(format!(
                "File name: {}",
                std::path::Path::new(&self.file_path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("No file loaded")
            )),
            text(format!("Is Loading: {}", self.is_loading)),
            text(format!("Is Playing: {}", self.is_playing)),
            text(format!("Length: {:.2} seconds", self.length)),
            text(format!(
                "Current Position: {:.2} seconds",
                self.current_position
            )),
        ]);

        let main_content = container(
            row![
                text("Main Content").size(24),
                button("Action").on_press(Message::OpenFileDialog),
                text(self.is_loading.then(|| "Loading...").unwrap_or("Ready")),
            ]
            .spacing(10)
            .padding(20)
            .align_y(alignment::Vertical::Center),
        );

        let current_position = self
            .media_player
            .is_some()
            .then(|| {
                (self.media_player.as_ref().unwrap().current_position as f32
                    / self.loaded_samples.clone().unwrap().sample_rate as f32)
                    / self.length
            })
            .unwrap_or(0.0);

        let percent = current_position / self.length;

        let footer_content = container(
            row![
                button("Play/Pause").on_press(Message::PlayPause),
                text(
                    self.media_player
                        .is_some()
                        .then(|| format!(
                            "Playing at {:.2} seconds ({:.2}%)",
                            current_position, percent
                        ))
                        .unwrap_or("No media player available".to_string())
                ),
                progress_bar(0.0..=100.0, current_position * 100.0)
                    .width(iced::Length::Fill)
                    .height(20),
            ]
            .spacing(10)
            .padding(10),
        )
        .width(iced::Length::Fill)
        .align_bottom(1000);

        return column![info_content, main_content, footer_content,].into();
    }
}