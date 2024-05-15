use crate::{error_template::{AppError, ErrorTemplate}, js::{videojs::{videojs, VideoJsPlayer}, wasp::WaspHlsPlayerW}};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

const UID_LIST: [&str; 6] = [
    "757b142d71614ed68f19cb794016ff06",
    "43ee1282abce49dfaf35356d0cff9b60",
    "a217e48bece94060aa270983a0876c40",
    "1808d521161a478998786d6e3bd6d1df",
    "37e6cf4cc8804ce4a6203fffa38d14a3",
    "8d4c64f54b3c4516b3a27d376527e4dd",
];

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/hls-test.css"/>
        <Stylesheet href="https://vjs.zencdn.net/8.10.0/video-js.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/vidjs" view=VideoJs/>
                    <Route path="/native" view=Native/>
                    <Route path="/hls" view=Hls/>
                </Routes>
            </main>
        </Router>
        <script src="https://vjs.zencdn.net/8.10.0/video.min.js"></script>
    }
}

#[component]
fn HlsPlayer(url: String) -> impl IntoView {
    let vid = create_node_ref::<html::Video>();
    let wasp_player = create_rw_signal(None::<WaspHlsPlayerW>);
    create_effect(move |_| {
        let vid_ref = vid()?;
        let wasp = WaspHlsPlayerW::new_recommended(&vid_ref);
        wasp.load(&url);
        wasp_player.set(Some(wasp));
        vid_ref.set_muted(true);
        vid_ref.set_loop(true);
        vid_ref.set_autoplay(true);
        Some(())
    });
    view! {
        <video _ref=vid class="object-contain h-[25vh]" preload="auto" loop autoplay="muted" muted/>
    }
}

#[component]
fn VidJsPlayer(url: String) -> impl IntoView {
    let node_ref = create_node_ref::<html::Video>();
    let player = create_rw_signal(None::<VideoJsPlayer>);

    node_ref.on_load(move |v| {
        _ = v.on_mount(move |v| {
            let vidjs = videojs(&v).unwrap();
            player.set(Some(vidjs));
        })
    });

    on_cleanup(move || {
        let Some(p) = player.get() else {
            return;
        };
        _ = p.dispose();
    });

    view! {
        <video _ref=node_ref class="video-js object-contain w-full" preload="auto" loop autoplay="muted" muted>
            <source src=url type="application/x-mpegURL"/>
        </video>

    }
}

#[component]
fn NativePlayer(url: String) -> impl IntoView {
    view! {
        <video src=url class="object-contain h-[25dvh]" preload="auto" loop autoplay="muted" muted>
        </video>
    }
}

#[component]
fn VidView(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col w-screen min-h-screen bg-black">
            {children()}
        </div>
    }
}

#[component]
fn VideoJs() -> impl IntoView {
    view! {
        <VidView>
            <For each=move || UID_LIST.clone() key=|id| id.to_string() children=move |uid| {
                let url = format!("https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{uid}/manifest/video.m3u8");
                view! {
                    <VidJsPlayer url/>
                }
            }/>
        </VidView>
    }
}

#[component]
fn Hls() -> impl IntoView {
    view! {
        <VidView>
            <For each=move || UID_LIST.clone() key=|id| id.to_string() children=move |uid| {
                let url = format!("https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{uid}/manifest/video.m3u8");
                view! {
                    <HlsPlayer url/>
                }
            }/>
        </VidView>
    }
}

#[component]
fn Native() -> impl IntoView {
    view! {
        <VidView>
            <For each=move || UID_LIST.clone() key=|id| id.to_string() children=move |uid| {
                let url = format!("https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{uid}/downloads/default.mp4");
                view! {
                    <NativePlayer url/>
                }
            }/>
        </VidView>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col w-screen min-h-screen bg-black text-white text-2xl justify-center items-center gap-8 underline">
            <a href="/vidjs">VideoJs</a>
            <a href="/native">Native</a>
            <a href="/hls">Hls</a>
        </div>
    }
}
