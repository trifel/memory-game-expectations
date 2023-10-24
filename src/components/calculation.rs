use crate::{components::download_button::DownloadButton, expectations::Expectation};
use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, *};

#[component]
pub fn Calculation() -> impl IntoView {
    let pairs_of_cards_input_element: NodeRef<Input> = create_node_ref();

    let calc_action = create_action(|value: &i32| {
        let number_of_pairs = value.to_owned();
        async move {
            if number_of_pairs < 1 {
                return None;
            }
            TimeoutFuture::new(1).await;
            let value = Expectation::calculate(number_of_pairs).await;
            Some(value)
        }
    });

    let pending = calc_action.pending();
    let result = calc_action.value();

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center pt-10 sm:pt-20">

            <div class="p-6 tracking-tight text-white lg:col-span-7">
                <h2 class="text-4xl font-bold inline sm:block lg:inline xl:block pb-5">
                    Memory Game expectations
                </h2>
                <p>
                    "For those who always wanted to know what is the expectation value in a memory game with a perfect memory. The calculation is performed directly in the browser thanks to Web Assembly."
                </p>
            </div>

            <div>
                <label for="number-of-pairs" class="sr-only">
                    Number of pairs
                </label>
                <input
                    id="number-of-pairs"
                    node_ref=pairs_of_cards_input_element
                    type="number"
                    autocomplete="email"
                    required
                    class="w-52 min-w-0 flex-auto rounded-md border-0 bg-white/5 px-3.5 py-2 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:pb-3"
                    placeholder="Enter number of pairs"
                    on:focus=move |_| {
                        calc_action.dispatch(-1);
                    }
                />

                <button
                    type="submit"
                    class="ml-5 flex-none rounded-md bg-indigo-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                    on:click=move |_| {
                        let value = pairs_of_cards_input_element()
                            .expect("input element not found")
                            .value()
                            .parse::<i32>();
                        if let Ok(value) = value {
                            calc_action.dispatch(value);
                        }
                    }
                >

                    Calculate
                </button>
            </div>

            <p class="px-10 pb-10 pt-10 text-sm leading-6 text-gray-300">
                {move || match pending() {
                    true => view! { <span class="italic">"Calculating..."</span> }.into_view(),
                    false => {
                        if let Some(Some(expectation)) = result() {
                            view! {
                                "The calculated expectation value is "
                                <span class="text-lg text-white">
                                    {format!("{:.2}.", expectation.expectation_value)}
                                </span>
                                <DownloadButton expectation/>
                            }
                                .into_view()
                        } else {
                            view! { <span class="italic">"Start a new calculation."</span> }
                                .into_view()
                        }
                    }
                }}

            </p>
        </div>
    }
}
