use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics};

#[proc_macro_derive(Tower)]
pub fn derive_tower(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn new(position: Point) -> Self {
                Self {
                    aim: Aim::new(None),
                    position: position,
                    cooldown_clock: Clock::from_now(),
                }
            }
        }


        impl #impl_generics Tower for #name #ty_generics #where_clause {
            fn position(&self) -> &Point {
                &self.position
            }

            fn cost(&self) -> u64 {
                Self::COST
            }

            fn range(&self) -> f32 {
                Self::RANGE
            }
        }

        impl #impl_generics UpdatableObject for #name #ty_generics #where_clause  {
            fn on_update(&mut self, game_model: &mut dyn GameModel, _: Duration) {
                self.update_aim(game_model);
                if self.cooldown_clock.elapsed() > Self::COOLDOWN {
                    self.shoot(game_model);
                    self.cooldown_clock.tick();
                }
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            fn shoot(&mut self, game_model: &mut dyn GameModel) {
                self.aim.try_shoot(Self::DAMAGE, |reward| {
                    game_model.wallet_mut().add_money(reward);
                });
            }

            fn update_aim(&mut self, game_model: &dyn GameModel) {
                if !self.aim.is_in_shoot_range(self, game_model.trajectory())
                    || !self.aim.is_alive()
                {
                    self.aim = Aim::new(None);
                }

                if self.aim.is_some() {
                    return;
                }

                let random_chosen_enemy = game_model
                    .enemies()
                    .iter()
                    .filter(|enemy| {
                        let enemypos = game_model.trajectory().get_point(enemy.borrow().position());
                        enemypos.distance(self.position()) < self.range()
                    })
                    .map(|rc| rc.clone())
                    .choose(&mut rand::thread_rng());

                self.aim = Aim::new(random_chosen_enemy);
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::HeapSize));
        }
    }
    generics
}
