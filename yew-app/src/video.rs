use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideoListProps {
    pub videos: Vec<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideoListProps { videos }: &VideoListProps) -> Html {
    videos.iter().map(|video| html! {
        <p>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect()
}
