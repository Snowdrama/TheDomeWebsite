use yew::{Component, Context, html, Html};
use crate::navigation::NavigationMenu;
use crate::common::Header;
use rand::prelude::*;

mod gallery_image;
pub struct GalleryPage{}

impl GalleryPage{
    fn get_photos() -> Vec<String> {
        let mut rng = thread_rng();
        let mut hand_picked = vec![
            "/images/the_dome/TheDome_HandPicked_1.png".to_string(),
            "/images/the_dome/TheDome_HandPicked_2.png".to_string(),
            "/images/the_dome/TheDome_HandPicked_3.png".to_string(),
            "/images/the_dome/TheDome_HandPicked_4.png".to_string(),
        ];
        hand_picked.shuffle(&mut rng);

        let mut others = vec![
            "/images/the_dome/TheDome_1.png".to_string(),
            "/images/the_dome/TheDome_2.png".to_string(),
            "/images/the_dome/TheDome_3.png".to_string(),
            // "/images/the_dome/TheDome_4.png".to_string(),
            "/images/the_dome/TheDome_5.png".to_string(),
            "/images/the_dome/TheDome_6.png".to_string(),
            "/images/the_dome/TheDome_7.png".to_string(),
            "/images/the_dome/TheDome_8.png".to_string(),
            "/images/the_dome/TheDome_9.png".to_string(),
            "/images/the_dome/TheDome_10.png".to_string(),
            "/images/the_dome/TheDome_11.png".to_string(),
            "/images/the_dome/TheDome_12.png".to_string(),
            "/images/the_dome/TheDome_13.png".to_string(),
            "/images/the_dome/TheDome_14.png".to_string(),
            "/images/the_dome/TheDome_15.png".to_string(),
            // "/images/the_dome/TheDome_16.png".to_string(),
            "/images/the_dome/TheDome_17.png".to_string(),
            "/images/the_dome/TheDome_18.png".to_string(),
            "/images/the_dome/TheDome_19.png".to_string(),
            "/images/the_dome/TheDome_20.png".to_string(),
            "/images/the_dome/TheDome_21.png".to_string(),
            "/images/the_dome/TheDome_22.png".to_string(),
            "/images/the_dome/TheDome_23.png".to_string(),
            "/images/the_dome/TheDome_24.png".to_string(),
            "/images/the_dome/TheDome_25.png".to_string(),
        ];
        others.shuffle(&mut rng);

        hand_picked.append(&mut others);

        hand_picked
    }
}

impl Component for GalleryPage {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let gallery_images = GalleryPage::get_photos().iter().map(|images| {
            html!{
                <gallery_image::GalleryImage image_url={images.to_owned()} />
            }
        }).collect::<Html>();
        html!{
            <>
                <Header/>
                <NavigationMenu/>
                <div class="gallery-grid">
                    {gallery_images}
                </div>
            </>
        }
    }
}