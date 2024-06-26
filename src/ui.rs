use leptos::{
    component,
    server_fn::{
        client::Client, codec::PostUrl, error::NoCustomError, request::ClientReq, ServerFn,
    },
    view, Action, AttributeValue, Children, IntoView, Serializable, ServerFnError,
};
use leptos_icons::Icon;
use leptos_router::ActionForm;
use serde::de::DeserializeOwned;

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="container mx-auto pt-6">{children()}</div> }
}

#[component]
pub fn CenteredCard(children: Children) -> impl IntoView {
    view! {
        <div class="size-full flex flex-col justify-center items-center">
            <div class="w-full max-w-xl pt-6 pb-8 px-20 bg-base-100 rounded-xl border border-neutral shadow-lg">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn ActionIcon<I, O, 'a>(
    action: Action<I, Result<O, ServerFnError>>,
    icon: icondata::Icon,
    class: &'a str,
    children: Children,
) -> impl IntoView
where
    I: Clone
        + ServerFn<InputEncoding = PostUrl, Output = O, Error = NoCustomError>
        + DeserializeOwned
        + 'static,
    O: Clone + Serializable + 'static,
    <<<I as ServerFn>::Client as Client<<I as ServerFn>::Error>>::Request as ClientReq<
        <I as ServerFn>::Error,
    >>::FormData: From<web_sys::FormData>,
{
    let class = class.to_string();

    view! {
        <ActionForm action>
            {children()} <button type="submit" class=format!("btn btn-square {class}")>
                <Icon icon class="text-2xl"/>
            </button>
        </ActionForm>
    }
}

#[component]
pub fn Form<I, O, 'a>(
    action: Action<I, Result<O, ServerFnError>>,
    title: &'a str,
    submit: &'a str,
    children: Children,
) -> impl IntoView
where
    I: Clone
        + ServerFn<InputEncoding = PostUrl, Output = O, Error = NoCustomError>
        + DeserializeOwned
        + 'static,
    O: Clone + Serializable + 'static,
    <<<I as ServerFn>::Client as Client<<I as ServerFn>::Error>>::Request as ClientReq<
        <I as ServerFn>::Error,
    >>::FormData: From<web_sys::FormData>,
{
    let title = title.to_string();
    let submit = submit.to_string();

    view! {
        <ActionForm action class="w-full flex flex-col items-center">
            <FormTitle text=&title/>
            <div class="w-full flex flex-col mt-4 gap-4 mb-6">{children()}</div>
            <FormSubmit msg=&submit/>
        </ActionForm>
    }
}

#[component]
pub fn FormTitle<'a>(text: &'a str) -> impl IntoView {
    let text = text.to_string();

    view! { <h1 class="text-primary text-2xl font-bold">{text}</h1> }
}

#[component]
pub fn FormInput<'a>(
    input_type: &'a str,
    name: &'a str,
    label: &'a str,
    placeholder: &'a str,
    // TODO: Add required
    #[prop(optional, into)] default_value: Option<AttributeValue>,
    #[prop(optional, into)] maxlength: Option<AttributeValue>,
) -> impl IntoView {
    let input_type = input_type.to_string();
    let name = name.to_string();
    let label = label.to_string();
    let placeholder = placeholder.to_string();

    view! {
        <div>
            <label class="form-control">
                <div class="label">
                    <span class="label-text text-lg font-bold">{label}</span>
                </div>
                <input
                    type=input_type
                    id=name.clone()
                    name=name
                    placeholder=placeholder
                    value=default_value
                    maxlength=maxlength
                    class="input input-accent w-full text-lg"
                />
            </label>
        </div>
    }
}

#[component]
pub fn FormCheckbox<'a>(label: &'a str, id: &'a str) -> impl IntoView {
    let label = label.to_string();
    let id = id.to_string();

    view! {
        <div>
            <label class="flex items-center">
                <input type="checkbox" name=id class="checkbox checkbox-accent"/>
                <span class="text-lg font-bold ml-2">{label}</span>
            </label>
        </div>
    }
}

#[component]
pub fn FormSubmit<'a>(msg: &'a str) -> impl IntoView {
    let msg = msg.to_string();

    view! {
        <button type="submit" class="btn btn-primary btn-wide text-lg">
            {msg}
        </button>
    }
}
