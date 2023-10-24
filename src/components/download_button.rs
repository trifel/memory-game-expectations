use crate::expectations::Expectation;
use gloo_timers::future::TimeoutFuture;
use leptos::{html::a, *};
use web_sys::{Blob, Url};

#[component]
pub fn DownloadButton(expectation: Expectation) -> impl IntoView {
    let download_action = create_action(|value: &Expectation| {
        let expectation = value.to_owned();
        async move {
            TimeoutFuture::new(1).await;
            let csv = expectation.get_csv().await;
            let bytes = csv.into_bytes();
            let uint8arr =
                js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&bytes) }.into());
            let array = js_sys::Array::new();
            array.push(&uint8arr.buffer());
            let blob = Blob::new_with_u8_array_sequence_and_options(
                &array,
                web_sys::BlobPropertyBag::new().type_("text/csv"),
            )
            .unwrap();
            let download_url = Url::create_object_url_with_blob(&blob).unwrap();
            let anchor = a();
            let _ = anchor.set_attribute("href", &download_url);
            let _ = anchor.set_attribute("download", "expectations.csv");
            anchor.click();
            ()
        }
    });

    let pending = download_action.pending();

    let download_handler = move |_| {
        download_action.dispatch(expectation.clone());
    };

    view! {
        <div class="mx-auto p-8">
            <button
                class="inline-flex items-center gap-x-2 rounded-md bg-gray-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-gray-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-gray-500"
                on:click=download_handler
                disabled=move || pending()
            >
                {move || match pending() {
                    true => {
                        view! {
                            <svg
                                aria-hidden="true"
                                height="24"
                                width="24"
                                class="text-gray-200 animate-spin fill-indigo-500"
                                viewBox="0 0 100 101"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                                    fill="currentColor"
                                ></path>
                                <path
                                    d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                                    fill="currentFill"
                                ></path>
                            </svg>
                        }
                            .into_view()
                    }
                    false => {
                        view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="currentColor"
                                height="24"
                                viewBox="0 -960 960 960"
                                width="24"
                            >
                                <path d="M280-280h400v-80H280v80Zm200-120 160-160-56-56-64 62v-166h-80v166l-64-62-56 56 160 160Zm0 320q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q134 0 227-93t93-227q0-134-93-227t-227-93q-134 0-227 93t-93 227q0 134 93 227t227 93Zm0-320Z"></path>
                            </svg>
                        }
                            .into_view()
                    }
                }}

                "CSV-Download"
            </button>

        </div>
    }
}
