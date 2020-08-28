use yew::prelude::*;

struct Product {
    id: i32,
    name: String,
    description: String,
    image: String,
    price: f64,
}

struct State {
    products: Vec<Product>,
}

pub struct Home {
    state: State,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let products: Vec<Product> = vec![
            Product {
                id: 1,
                name: String::from("Apple"),
                description: String::from("An apple a day keeps the doctor away"),
                image: String::from("/products/apple.png"),
                price: 3.65,
            },
            Product {
                id: 2,
                name: String::from("Banana"),
                description: String::from("An old banana leaf was once young and green"),
                image: String::from("/products/banana.png"),
                price: 7.99,
            },
        ];

        Self {
            state: State { products },
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                html! {
                    <div>
                        <img src={&product.image} width="100" height="150"/>
                        <div>{&product.name}</div>
                        <div>{"$"}{&product.price}</div>
                    </div>
                }
            })
            .collect();
        html! { <span>{products}</span> }
    }
}
