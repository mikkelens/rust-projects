mod video;

use reqwasm::http::Request;
use video::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
	let videos = use_state(Vec::new);
	{
		let videos = videos.clone();
		use_effect_with_deps(
			move |_| {
				let videos = videos.clone();
				wasm_bindgen_futures::spawn_local(async move {
					let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
						.send()
						.await
						.unwrap()
						.json()
						.await
						.unwrap();
					videos.set(fetched_videos);
				});
				|| ()
			},
			()
		);
	}

	let selected_video = use_state(|| None);

	let on_video_select = {
		let selected_video = selected_video.clone();
		Callback::from(move |video: Video| selected_video.set(Some(video)))
	};

	let details = selected_video.as_ref().map(|video| {
		html! {
			<VideoDetails video={video.clone()} />
		}
	});

	html! {
		<>
			<h1>{ "RustConf Explorer" }</h1>
			<div>
				<h3>{"Videos to watch"}</h3>
				<VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
			</div>
			{
				for details
			}
		</>
	}
}

fn main() { yew::start_app::<App>(); }
