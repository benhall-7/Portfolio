use crate::utils::game::*;
use std::str::FromStr;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
// use yew::interval::{IntervalService, IntervalTask};
use gloo::timers::callback::Interval;

const DEFAULT_WIDTH: usize = 10;
const DEFAULT_HEIGHT: usize = 10;
const DEFAULT_SIZE: usize = 20;
// RGB
const DEFAULT_ALIVE_COLOR: &str = "#ffffff";
const DEFAULT_DEAD_COLOR: &str = "#808080";

#[derive(Debug)]
pub struct Conway {
    job: Option<Interval>,
    game: Game,
    width: usize,
    height: usize,
    form_width: Option<usize>,
    form_height: Option<usize>,
    canvas: NodeRef,
}

pub enum ConwayMessage {
    Start,
    Pause,
    Reset,
    Step,
    SetPreset(GamePreset),
    SetFormWidth(String),
    SetFormHeight(String),
    SetDimensions(),
    Click([usize; 2]),
}

impl Component for Conway {
    type Message = ConwayMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Conway {
            job: None,
            game: Game::new(DEFAULT_WIDTH, DEFAULT_HEIGHT, false),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            form_width: None,
            form_height: None,
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, context: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ConwayMessage::Start => {
                let link = context.link().clone();
                let handle =
                    Interval::new(1000 / 10, move || link.send_message(ConwayMessage::Step));
                self.job = Some(handle);
                true
            }
            ConwayMessage::Pause => {
                self.job = None;
                true
            }
            ConwayMessage::Reset => {
                self.game.clear();
                self.job = None;
                self.draw();
                true
            }
            ConwayMessage::Step => {
                self.game.step();
                self.draw();
                false
            }
            ConwayMessage::SetPreset(preset) => {
                self.set_preset(preset);
                self.draw();
                true
            }
            ConwayMessage::SetFormWidth(width) => {
                self.form_width = if width.is_empty() {
                    None
                } else {
                    usize::from_str(&width).ok()
                };
                false
            }
            ConwayMessage::SetFormHeight(height) => {
                self.form_height = if height.is_empty() {
                    None
                } else {
                    usize::from_str(&height).ok()
                };
                false
            }
            ConwayMessage::SetDimensions() => {
                self.set_dimensions(
                    self.form_width.unwrap_or(self.width),
                    self.form_height.unwrap_or(self.height),
                );
                self.draw();
                true
            }
            ConwayMessage::Click(mut coord) => {
                if coord[0] >= self.height {
                    coord[0] = self.height - 1;
                }
                if coord[1] >= self.width {
                    coord[0] = self.width - 1;
                }
                // TODO: this iteration (trying to avoid allocation) is ugly
                self.game.invert([coord].iter().map(|a| *a));
                self.draw();
                false
            }
        }
    }

    fn changed(&mut self, _: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }

    fn rendered(&mut self, _: &Context<Self>, first_render: bool) {
        if first_render {
            self.set_preset(DEFAULT_PRESET);
            self.draw();
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        let playing = self.job.is_some();
        html! { <div class="conway">
            <div class="conway-title">
                <h2>{"Conway's Game of Life"}</h2>
            </div>
            <h3>{"Presets"}</h3>
            <div class="conway-presets">
                <button
                    onclick={context.link().callback(|_| ConwayMessage::SetPreset(BLINKER_PRESET))}
                >{"Blinker"}</button>
                <button
                    onclick={context.link().callback(|_| ConwayMessage::SetPreset(PENTADEC_PRESET))}
                >{"Pentadecathlon"}</button>
                <button
                    onclick={context.link().callback(|_| ConwayMessage::SetPreset(LWSS_PRESET))}
                >{"Spaceship"}</button>
            </div>
            <h3>{"Size"}</h3>
            <form
                id="conway-size-form"
                onsubmit={context.link().callback(|e: SubmitEvent| {
                    e.prevent_default();
                    ConwayMessage::SetDimensions()
                })}
            >
                <div class="conway-size-inputs">
                <div>
                    <label for="conway-width">{"Rows"}</label>
                    <input
                        type="number"
                        min="1"
                        max="50"
                        oninput={context.link().callback(|e: InputEvent| ConwayMessage::SetFormWidth(e.data().unwrap_or_default()))}
                    />
                </div>
                <div>
                    <label for="conway-height">{"Columns"}</label>
                    <input
                        type="number"
                        min="1"
                        max="50"
                        oninput={context.link().callback(|e: InputEvent| ConwayMessage::SetFormHeight(e.data().unwrap_or_default()))}
                    />
                </div>
                </div>
                <button>{"Set"}</button>
            </form>
            <h3>{"Controls"}</h3>
            <div class="conway-controls">
                <button
                    id="conway-controls-play"
                    disabled={playing}
                    onclick={context.link().callback(|_| ConwayMessage::Start)}
                >{"Play"}</button>
                <button
                    id="conway-controls-pause"
                    disabled={!playing}
                    onclick={context.link().callback(|_| ConwayMessage::Pause)}
                >{"Pause"}</button>
                <button
                    id="conway-controls-step"
                    disabled={playing}
                    onclick={context.link().callback(|_| ConwayMessage::Step)}
                >{"Step"}</button>
                <button onclick={context.link().callback(|_| ConwayMessage::Reset)}>{"Reset"}</button>
            </div>
            <canvas
                ref={self.canvas.clone()}
                onclick={context.link().callback(|e: MouseEvent| {
                    // TODO: There seems to be an off by 1 issue with y coordinates, which is resolved manually here
                    ConwayMessage::Click([
                        (e.offset_y() as usize - 1)  / DEFAULT_SIZE,
                        (e.offset_x() as usize) / DEFAULT_SIZE,
                    ])
                })}
            ><p>{"This browser does not support the canvas element!"}</p></canvas>
        </div> }
    }
}

impl Conway {
    fn draw(&self) {
        let width = self.width;
        let height = self.height;
        let size = DEFAULT_SIZE;

        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
        canvas.set_width((width * size) as u32);
        canvas.set_height((height * size) as u32);
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context.set_fill_style(&JsValue::from(DEFAULT_DEAD_COLOR));
        context.fill_rect(0.0, 0.0, (width * size) as f64, (height * size) as f64);
        context.set_fill_style(&JsValue::from(DEFAULT_ALIVE_COLOR));
        for row in 0..height {
            for col in 0..width {
                if self.game.front()[[row as usize, col as usize]] != 0 {
                    context.fill_rect(
                        (col * size) as f64,
                        (row * size) as f64,
                        size as f64,
                        size as f64,
                    );
                }
            }
        }
    }

    fn set_preset(&mut self, preset: GamePreset) {
        self.set_dimensions(preset.width, preset.height);
        self.game.set_on(preset.cells.iter().map(|c| *c));
    }

    fn set_dimensions(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.game = Game::new(width, height, false);
    }
}
