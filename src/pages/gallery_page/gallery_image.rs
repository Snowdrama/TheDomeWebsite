use yew::{Component, Context, html, Html, Properties};


#[derive(Properties, PartialEq)]
pub struct Props{
    pub image_url: String,
}

pub struct GalleryImage{
    show_modal: bool,
}

impl GalleryImage{

}
pub enum Msg {
    OpenModal,
    CloseModal
}

impl Component for GalleryImage {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            show_modal:false,
        }
    }


    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OpenModal => {
                self.show_modal = true;
                true
            },
            Msg::CloseModal => {
                self.show_modal = false;
                true
            },
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let open_callback = ctx.link().callback(|_| {
            Msg::OpenModal
        });
        let close_callback = ctx.link().callback(|_| {
            Msg::CloseModal
        });
        let mut modal_class = "modal".to_string();
        if self.show_modal { 
            modal_class = "modal show".to_string();
        }
        html!{
            <>
            <div>
                <div class={modal_class} onclick={close_callback.to_owned()}>
                    <div class="modal-close" onclick={close_callback.to_owned()}>{"X"}</div>
                    <img src={ ctx.props().image_url.to_owned() }/>
                </div>
                <div class="image" onclick={open_callback}>
                    <img src={ ctx.props().image_url.to_owned() }/>
                </div>
            </div>
            </>
        }
    }
}